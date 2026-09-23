// Rapid Download Manager - Background Service Worker
const PRIMARY_SERVER = "http://127.0.0.1:9669/add_download";
const FALLBACK_SERVER = "http://localhost:9669/add_download";
const STATUS_SERVER = "http://127.0.0.1:9669/";

// Prevent duplicate processing of the same download item
const processedDownloadIds = new Set();

// Active media streams per tab: tabId -> { video, audio, m3u8 }
const tabMediaStreams = new Map();

// ---------------------------------------------------------
// 0. Live Network Stream Sniffer (YouTube, GoogleVideo, HLS)
// ---------------------------------------------------------
chrome.webRequest.onBeforeRequest.addListener(
  (details) => {
    if (!details.url || details.tabId < 0) return;
    try {
      let u = new URL(details.url);

      // 1. YouTube & Google Video Streams
      if (u.hostname.includes("googlevideo.com") && u.pathname.includes("videoplayback")) {
        // Strip byte range and chunking params to get full unsegmented stream
        u.searchParams.delete("range");
        u.searchParams.delete("rn");
        u.searchParams.delete("rbuf");
        let fullStreamUrl = u.toString();

        let mime = (u.searchParams.get("mime") || "").toLowerCase();
        let itag = u.searchParams.get("itag") || "";

        let tabData = tabMediaStreams.get(details.tabId) || { video: null, audio: null, m3u8: null };

        if (mime.startsWith("video") || itag === "18" || itag === "22") {
          tabData.video = {
            url: fullStreamUrl,
            mime: mime,
            itag: itag,
            timestamp: Date.now()
          };
        } else if (mime.startsWith("audio") || itag === "140" || itag === "251") {
          tabData.audio = {
            url: fullStreamUrl,
            mime: mime,
            itag: itag,
            timestamp: Date.now()
          };
        }
        tabMediaStreams.set(details.tabId, tabData);
      }

      // 2. HLS (.m3u8) Streams
      if (u.pathname.includes(".m3u8") || details.url.includes(".m3u8")) {
        let tabData = tabMediaStreams.get(details.tabId) || { video: null, audio: null, m3u8: null };
        tabData.m3u8 = {
          url: details.url,
          timestamp: Date.now()
        };
        tabMediaStreams.set(details.tabId, tabData);
      }
    } catch (e) {}
  },
  {
    urls: [
      "*://*.googlevideo.com/videoplayback*",
      "*://*/*.m3u8*",
      "*://*/*.m3u8"
    ]
  }
);

// Clean up memory when tab is closed
chrome.tabs.onRemoved.addListener((tabId) => {
  tabMediaStreams.delete(tabId);
});

// ---------------------------------------------------------
// 1. Context Menu Setup
// ---------------------------------------------------------
chrome.runtime.onInstalled.addListener(() => {
  chrome.contextMenus.create({
    id: "rapid-download-link",
    title: "⚡ Download with Rapid Download Manager",
    contexts: ["link", "image", "video", "audio"]
  });

  chrome.contextMenus.create({
    id: "rapid-scan-page",
    title: "🔍 Scan page with Rapid Media Scanner",
    contexts: ["page"]
  });
});

chrome.contextMenus.onClicked.addListener(async (info, tab) => {
  if (info.menuItemId === "rapid-scan-page") {
    if (tab && tab.id) {
      chrome.action.openPopup().catch(() => {});
    }
    return;
  }

  let targetUrl = info.linkUrl || info.srcUrl;
  if (!targetUrl) return;

  let pageReferrer = tab && tab.url ? tab.url : "";
  let isGoogle = isGoogleDriveUrl(targetUrl);
  let cookies = await extractCookiesForUrl(targetUrl, isGoogle, pageReferrer);

  let success = await sendToRapidApp({
    url: targetUrl,
    referrer: pageReferrer,
    filename: "",
    cookies: cookies,
    user_agent: navigator.userAgent,
    is_gdrive: isGoogle
  });

  if (success) {
    showNotification("⚡ Rapid Download Manager", "Download dispatched to Rapid Download Manager!");
  } else {
    showNotification("⚡ Rapid Download Manager Offline", "Please ensure rapid-gui.exe is open on your computer.");
  }
});

// Helper: Check if URL belongs to Google Drive
function isGoogleDriveUrl(urlStr) {
  try {
    let u = new URL(urlStr);
    return u.hostname.includes("drive.google.com") || 
           u.hostname.includes("googleusercontent.com") || 
           u.hostname.includes("usercontent.google.com") ||
           u.hostname.includes("docs.google.com") ||
           u.hostname.includes("takeout-download-drive") ||
           u.hostname.includes("google.com");
  } catch (e) {
    return false;
  }
}

// Helper: Extract cookies for target URL + originating referrer page
async function extractCookiesForUrl(targetUrl, isGoogle, pageReferrer) {
  try {
    let isYT = targetUrl.includes("googlevideo.com") || 
               targetUrl.includes("youtube.com") || 
               (pageReferrer && pageReferrer.includes("youtube.com"));

    if (isGoogle || isYT) {
      const [driveCookies, googleCookies, rootGoogleCookies, ytCookies, rootYtCookies, contentCookies, dotContentCookies, userContentCookies, dotUserContentCookies, accountsCookies, urlCookies, refCookies] = await Promise.all([
        chrome.cookies.getAll({ domain: "drive.google.com" }).catch(() => []),
        chrome.cookies.getAll({ domain: ".google.com" }).catch(() => []),
        chrome.cookies.getAll({ domain: "google.com" }).catch(() => []),
        chrome.cookies.getAll({ domain: ".youtube.com" }).catch(() => []),
        chrome.cookies.getAll({ domain: "youtube.com" }).catch(() => []),
        chrome.cookies.getAll({ domain: "googleusercontent.com" }).catch(() => []),
        chrome.cookies.getAll({ domain: ".googleusercontent.com" }).catch(() => []),
        chrome.cookies.getAll({ domain: "usercontent.google.com" }).catch(() => []),
        chrome.cookies.getAll({ domain: ".usercontent.google.com" }).catch(() => []),
        chrome.cookies.getAll({ domain: "accounts.google.com" }).catch(() => []),
        chrome.cookies.getAll({ url: targetUrl }).catch(() => []),
        pageReferrer ? chrome.cookies.getAll({ url: pageReferrer }).catch(() => []) : []
      ]);

      const cookieMap = new Map();
      [
        ...googleCookies,
        ...rootGoogleCookies,
        ...driveCookies,
        ...ytCookies,
        ...rootYtCookies,
        ...contentCookies,
        ...dotContentCookies,
        ...userContentCookies,
        ...dotUserContentCookies,
        ...accountsCookies,
        ...urlCookies,
        ...(refCookies || [])
      ].forEach(c => {
        if (c && c.name) {
          cookieMap.set(c.name, c.value);
        }
      });
      return Array.from(cookieMap.entries()).map(([k, v]) => `${k}=${v}`).join("; ");
    } else {
      let [cookies, refCookies] = await Promise.all([
        chrome.cookies.getAll({ url: targetUrl }).catch(() => []),
        pageReferrer ? chrome.cookies.getAll({ url: pageReferrer }).catch(() => []) : []
      ]);
      const cookieMap = new Map();
      [...cookies, ...(refCookies || [])].forEach(c => {
        if (c && c.name) {
          cookieMap.set(c.name, c.value);
        }
      });
      return Array.from(cookieMap.entries()).map(([k, v]) => `${k}=${v}`).join("; ");
    }
  } catch (e) {
    console.warn("[Rapid] Failed to get cookies:", e);
    return "";
  }
}

// Helper: Send payload to Rapid Desktop App
async function sendToRapidApp(payload) {
  for (const endpoint of [PRIMARY_SERVER, FALLBACK_SERVER]) {
    try {
      console.log(`[Rapid] Dispatching to ${endpoint}:`, payload.url);
      let controller = new AbortController();
      let timeoutId = setTimeout(() => controller.abort(), 3500);
      let resp = await fetch(endpoint, {
        method: "POST",
        mode: "cors",
        headers: { 
          "Content-Type": "application/json"
        },
        body: JSON.stringify(payload),
        signal: controller.signal
      });
      clearTimeout(timeoutId);
      if (resp.ok) {
        let data = await resp.json().catch(() => ({}));
        if (data.status === "ok" || resp.status === 200) {
          console.log("[Rapid] Successfully dispatched download:", payload.url);
          return true;
        }
      } else {
        console.warn(`[Rapid] Server returned status ${resp.status} for ${endpoint}`);
      }
    } catch (err) {
      console.warn(`[Rapid] Connection to ${endpoint} failed:`, err);
    }
  }
  return false;
}

// Helper: Show Chrome desktop notification
function showNotification(title, message) {
  try {
    chrome.notifications.create({
      type: "basic",
      iconUrl: "icon128.png",
      title: title,
      message: message
    });
  } catch (e) {}
}

// ---------------------------------------------------------
// 2. Message Router for Content Script and Popup
// ---------------------------------------------------------
chrome.runtime.onMessage.addListener((msg, sender, sendResponse) => {
  if (!msg || !msg.type) return;

  // Query captured streams for current tab
  if (msg.type === "RAPID_GET_TAB_STREAM") {
    let tabId = sender.tab ? sender.tab.id : null;
    let tabData = tabId ? tabMediaStreams.get(tabId) : null;
    sendResponse({ status: "ok", data: tabData || null });
    return true;
  }

  // 1. Direct link click interception
  if (msg.type === "RAPID_INTERCEPT_LINK") {
    (async () => {
      let isGoogle = isGoogleDriveUrl(msg.url);
      let ref = msg.referrer || (sender.tab && sender.tab.url ? sender.tab.url : "");
      let cookies = await extractCookiesForUrl(msg.url, isGoogle, ref);
      let success = await sendToRapidApp({
        url: msg.url,
        referrer: ref,
        filename: msg.filename || "",
        cookies: cookies,
        user_agent: navigator.userAgent,
        is_gdrive: isGoogle
      });
      if (success) {
        showNotification("⚡ Rapid Download Manager", `Accelerating: ${msg.filename || "Link"}`);
      } else {
        chrome.downloads.download({ url: msg.url }).catch(() => {});
      }
    })();
    return true;
  }

  // 2. In-Page Media Sniffer Interception
  if (msg.type === "RAPID_INTERCEPT_MEDIA") {
    (async () => {
      let targetUrl = msg.url || "";
      let tabId = sender.tab ? sender.tab.id : null;
      let tabStreams = tabId ? tabMediaStreams.get(tabId) : null;
      let pageUrl = (sender.tab && sender.tab.url) ? sender.tab.url : (msg.referrer || "");
      let isYT = msg.is_youtube || pageUrl.includes("youtube.com") || pageUrl.includes("youtu.be") || targetUrl.includes("googlevideo.com");

      if (isYT) {
        let ytUrl = (pageUrl.includes("youtube.com") || pageUrl.includes("youtu.be")) ? pageUrl : targetUrl;
        let cookies = await extractCookiesForUrl(ytUrl, false, pageUrl);
        let fn = msg.filename || (sender.tab && sender.tab.title ? sender.tab.title.replace(/\s*-\s*YouTube$/i, "") : "") || "YouTube_Video.mp4";
        let success = await sendToRapidApp({
          url: ytUrl,
          referrer: pageUrl,
          filename: fn,
          cookies: cookies,
          user_agent: navigator.userAgent,
          is_youtube: true
        });
        if (success) {
          showNotification("Rapid YouTube Download", `Captured: ${fn}`);
          sendResponse({ success: true });
        } else {
          showNotification("Rapid Download Manager Offline", "Please start rapid-gui.exe to accelerate this video.");
          sendResponse({ success: false });
        }
        return;
      }

      // If incoming URL is a blob or missing, resolve from sniffed tab streams
      if (!targetUrl || targetUrl.startsWith("blob:") || msg.is_blob) {
        if (tabStreams) {
          if (msg.mediaType === "audio" && tabStreams.audio) {
            targetUrl = tabStreams.audio.url;
          } else if (tabStreams.video) {
            targetUrl = tabStreams.video.url;
          } else if (tabStreams.m3u8) {
            targetUrl = tabStreams.m3u8.url;
          }
        }
      }

      // If still unresolved blob, notify user to trigger stream
      if (!targetUrl || targetUrl.startsWith("blob:")) {
        showNotification(
          "Rapid Media Capture",
          "Please play 1-2 seconds of the video so Rapid can capture the high-speed stream!"
        );
        sendResponse({ success: false, reason: "needs_playback" });
        return;
      }

      let isGoogle = isGoogleDriveUrl(targetUrl);
      let ref = msg.referrer || (sender.tab && sender.tab.url ? sender.tab.url : "");
      let cookies = await extractCookiesForUrl(targetUrl, isGoogle, ref);
      let success = await sendToRapidApp({
        url: targetUrl,
        referrer: ref,
        filename: msg.filename || "",
        cookies: cookies,
        user_agent: navigator.userAgent,
        is_gdrive: isGoogle
      });
      if (success) {
        showNotification("⚡ Rapid Media Capture", `Captured: ${msg.filename || "Media stream"}`);
        sendResponse({ success: true });
      } else {
        showNotification("⚡ Rapid Download Manager Offline", "Please start rapid-gui.exe to accelerate this media.");
        sendResponse({ success: false });
      }
    })();
    return true;
  }

  // 3. Update Extension Action Badge Count
  if (msg.type === "RAPID_UPDATE_MEDIA_COUNT") {
    if (sender.tab && sender.tab.id) {
      let count = msg.count || 0;
      let badgeText = count > 0 ? String(count) : "";
      chrome.action.setBadgeText({ text: badgeText, tabId: sender.tab.id }).catch(() => {});
      chrome.action.setBadgeBackgroundColor({ color: "#00D2FF", tabId: sender.tab.id }).catch(() => {});
    }
    return true;
  }

  // 4. Batch Download from Extension Popup Scanner
  if (msg.type === "RAPID_BATCH_DOWNLOAD") {
    (async () => {
      let items = msg.items || [];
      let successCount = 0;
      for (let item of items) {
        let isGoogle = isGoogleDriveUrl(item.url);
        let cookies = await extractCookiesForUrl(item.url, isGoogle, item.referrer || "");
        let ok = await sendToRapidApp({
          url: item.url,
          referrer: item.referrer || "",
          filename: item.title || item.filename || "",
          cookies: cookies,
          user_agent: navigator.userAgent,
          is_gdrive: isGoogle
        });
        if (ok) successCount++;
        await new Promise(r => setTimeout(r, 120));
      }
      showNotification("⚡ Rapid Batch Downloader", `Dispatched ${successCount} items to Rapid Download Manager.`);
      sendResponse({ success: true, count: successCount });
    })();
    return true;
  }

  // 5. Check desktop engine status
  if (msg.type === "RAPID_CHECK_STATUS") {
    (async () => {
      try {
        let controller = new AbortController();
        let timeoutId = setTimeout(() => controller.abort(), 1500);
        let resp = await fetch(STATUS_SERVER, { signal: controller.signal });
        clearTimeout(timeoutId);
        sendResponse({ online: resp.ok });
      } catch (e) {
        sendResponse({ online: false });
      }
    })();
    return true;
  }
});

// ---------------------------------------------------------
// 3. Intercept browser native downloads (IDM style)
// ---------------------------------------------------------
chrome.downloads.onDeterminingFilename.addListener((downloadItem, suggest) => {
  let targetUrl = downloadItem.finalUrl || downloadItem.url;

  if (!targetUrl || 
      targetUrl.startsWith("blob:") || 
      targetUrl.startsWith("data:") || 
      targetUrl.startsWith("chrome://") ||
      targetUrl.startsWith("chrome-extension://")) {
    suggest();
    return false;
  }

  if (processedDownloadIds.has(downloadItem.id)) {
    suggest({ filename: downloadItem.filename || "download", conflictAction: "uniquify" });
    return false;
  }
  processedDownloadIds.add(downloadItem.id);

  if (processedDownloadIds.size > 200) {
    const first = processedDownloadIds.values().next().value;
    processedDownloadIds.delete(first);
  }

  (async () => {
    try {
      let isGoogle = isGoogleDriveUrl(targetUrl);
      let ref = downloadItem.referrer || (isGoogle ? "https://drive.google.com/" : "");
      if (!ref) {
        let tabs = await chrome.tabs.query({ active: true, currentWindow: true }).catch(() => []);
        if (tabs && tabs[0] && tabs[0].url) {
          ref = tabs[0].url;
        }
      }
      let cookies = await extractCookiesForUrl(targetUrl, isGoogle, ref);
      let rawFilename = (downloadItem.filename || "").trim();
      let resolvedFilename = "";
      let lowerFn = rawFilename.toLowerCase();
      if (rawFilename && 
          !["download", "download.bin", "download.crdownload", "uc", "file", "document"].includes(lowerFn) &&
          !lowerFn.startsWith("download.") &&
          !lowerFn.startsWith("uc.")) {
        resolvedFilename = rawFilename;
      }

      let success = await sendToRapidApp({
        url: targetUrl,
        referrer: ref,
        filename: resolvedFilename,
        cookies: cookies,
        user_agent: navigator.userAgent,
        is_gdrive: isGoogle
      });

      if (success) {
        suggest({ filename: resolvedFilename || "download", conflictAction: "uniquify" });
        try {
          await chrome.downloads.cancel(downloadItem.id);
          await chrome.downloads.erase({ id: downloadItem.id });
        } catch (e) {}

        showNotification("⚡ Rapid Download Manager", `Accelerating: ${resolvedFilename || "File"}`);
      } else {
        suggest({ filename: resolvedFilename || downloadItem.filename || "download", conflictAction: "uniquify" });
      }
    } catch (err) {
      console.warn("[Rapid] Interception error:", err);
      suggest();
    }
  })();

  return true;
});
// Rapid Download Manager — Background Service Worker
const PRIMARY_SERVER = "http://127.0.0.1:9669/add_download";
const FALLBACK_SERVER = "http://localhost:9669/add_download";

// Prevent duplicate processing of the same download item
const processedDownloadIds = new Set();

// 1. Context Menu: Right click on any link, image, video, audio
chrome.runtime.onInstalled.addListener(() => {
  chrome.contextMenus.create({
    id: "rapid-download-link",
    title: "⚡ Download with Rapid Download Manager",
    contexts: ["link", "image", "video", "audio"]
  });
});

chrome.contextMenus.onClicked.addListener(async (info, tab) => {
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
    showNotification("⚠ Rapid Download Manager Offline", "Please ensure rapid-gui.exe is open on your computer.");
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
    if (isGoogle) {
      const [driveCookies, googleCookies, rootGoogleCookies, contentCookies, dotContentCookies, userContentCookies, dotUserContentCookies, accountsCookies, urlCookies, refCookies] = await Promise.all([
        chrome.cookies.getAll({ domain: "drive.google.com" }).catch(() => []),
        chrome.cookies.getAll({ domain: ".google.com" }).catch(() => []),
        chrome.cookies.getAll({ domain: "google.com" }).catch(() => []),
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
      let timeoutId = setTimeout(() => controller.abort(), 3000);
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

// 1. Message listener from content script (Direct link click interceptor)
chrome.runtime.onMessage.addListener((msg, sender, sendResponse) => {
  if (msg && msg.type === "RAPID_INTERCEPT_LINK") {
    (async () => {
      let isGoogle = isGoogleDriveUrl(msg.url);
      let ref = msg.referrer || (sender.tab && sender.tab.url ? sender.tab.url : "");
      let cookies = await extractCookiesForUrl(msg.url, isGoogle, ref);
      let success = await sendToRapidApp({
        url: msg.url,
        referrer: msg.referrer || (sender.tab && sender.tab.url ? sender.tab.url : ""),
        filename: msg.filename || "",
        cookies: cookies,
        user_agent: navigator.userAgent,
        is_gdrive: isGoogle
      });
      if (success) {
        showNotification("⚡ Rapid Download Manager", `Accelerating: ${msg.filename || "Link"}`);
      } else {
        // App is offline, fallback to browser native download
        chrome.downloads.download({ url: msg.url }).catch(() => {});
      }
    })();
    return true;
  }
});

// 2. Intercept browser native downloads (IDM style - onDeterminingFilename)
// NOTE: We deliberately do NOT listen to onCreated because onCreated runs before filename
// determination and causes race conditions with Chrome's native Save dialog.
chrome.downloads.onDeterminingFilename.addListener((downloadItem, suggest) => {
  let targetUrl = downloadItem.finalUrl || downloadItem.url;

  // Ignore internal/browser URLs
  if (!targetUrl || 
      targetUrl.startsWith("blob:") || 
      targetUrl.startsWith("data:") || 
      targetUrl.startsWith("chrome://") ||
      targetUrl.startsWith("chrome-extension://")) {
    suggest();
    return false;
  }

  // Prevent duplicate execution for the same download ID
  if (processedDownloadIds.has(downloadItem.id)) {
    suggest({ filename: downloadItem.filename || "download", conflictAction: "uniquify" });
    return false;
  }
  processedDownloadIds.add(downloadItem.id);

  // Keep set bounded to prevent memory growth
  if (processedDownloadIds.size > 200) {
    const first = processedDownloadIds.values().next().value;
    processedDownloadIds.delete(first);
  }

  // Return true to tell Chrome to wait for suggest() asynchronously
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
        // Complete suggest first to satisfy Chrome API event lifecycle, then cancel native download
        suggest({ filename: resolvedFilename || "download", conflictAction: "uniquify" });
        try {
          await chrome.downloads.cancel(downloadItem.id);
          await chrome.downloads.erase({ id: downloadItem.id });
        } catch (e) {}

        showNotification("⚡ Rapid Download Manager", `Accelerating: ${resolvedFilename || "File"}`);
      } else {
        // Desktop app not running, let Chrome download natively
        suggest({ filename: resolvedFilename || downloadItem.filename || "download", conflictAction: "uniquify" });
      }
    } catch (err) {
      console.warn("[Rapid] Interception error:", err);
      suggest();
    }
  })();

  return true;
});
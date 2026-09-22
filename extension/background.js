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
  let cookies = await extractCookiesForUrl(targetUrl, isGoogle);

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
           u.hostname.includes("google.com");
  } catch (e) {
    return false;
  }
}

// Helper: Extract cookies for target URL
async function extractCookiesForUrl(targetUrl, isGoogle) {
  try {
    if (isGoogle) {
      const [driveCookies, googleCookies, rootGoogleCookies, contentCookies, dotContentCookies, userContentCookies, dotUserContentCookies, accountsCookies, urlCookies] = await Promise.all([
        chrome.cookies.getAll({ domain: "drive.google.com" }).catch(() => []),
        chrome.cookies.getAll({ domain: ".google.com" }).catch(() => []),
        chrome.cookies.getAll({ domain: "google.com" }).catch(() => []),
        chrome.cookies.getAll({ domain: "googleusercontent.com" }).catch(() => []),
        chrome.cookies.getAll({ domain: ".googleusercontent.com" }).catch(() => []),
        chrome.cookies.getAll({ domain: "usercontent.google.com" }).catch(() => []),
        chrome.cookies.getAll({ domain: ".usercontent.google.com" }).catch(() => []),
        chrome.cookies.getAll({ domain: "accounts.google.com" }).catch(() => []),
        chrome.cookies.getAll({ url: targetUrl }).catch(() => [])
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
        ...urlCookies
      ].forEach(c => {
        if (c && c.name) {
          cookieMap.set(c.name, c.value);
        }
      });
      return Array.from(cookieMap.entries()).map(([k, v]) => `${k}=${v}`).join("; ");
    } else {
      let cookies = await chrome.cookies.getAll({ url: targetUrl });
      return cookies.map(c => `${c.name}=${c.value}`).join("; ");
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
      let resp = await fetch(endpoint, {
        method: "POST",
        headers: { 
          "Content-Type": "application/json"
        },
        body: JSON.stringify(payload)
      });
      if (resp.ok) {
        let data = await resp.json().catch(() => ({}));
        if (data.status === "ok" || resp.status === 200) {
          console.log("[Rapid] Successfully dispatched download:", payload.url);
          return true;
        }
      }
    } catch (err) {
      console.warn(`[Rapid] Connection to ${endpoint} failed:`, err.message);
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

// 2. Intercept browser native downloads (IDM style)
// We use onDeterminingFilename so that final Content-Disposition filenames and Google Drive dynamic zips are fully resolved
chrome.downloads.onDeterminingFilename.addListener((downloadItem, suggest) => {
  // Pass-through blob:, data:, chrome://, or extensions
  if (!downloadItem.url || 
      downloadItem.url.startsWith("blob:") || 
      downloadItem.url.startsWith("data:") || 
      downloadItem.url.startsWith("chrome://") ||
      downloadItem.url.startsWith("chrome-extension://")) {
    suggest();
    return false;
  }

  // If already processed, let it proceed
  if (processedDownloadIds.has(downloadItem.id)) {
    suggest();
    return false;
  }

  // Handle asynchronously
  (async () => {
    try {
      processedDownloadIds.add(downloadItem.id);

      let isGoogle = isGoogleDriveUrl(downloadItem.url);
      let cookies = await extractCookiesForUrl(downloadItem.url, isGoogle);
      let ref = downloadItem.referrer || (isGoogle ? "https://drive.google.com/" : "");
      let resolvedFilename = downloadItem.filename || "";

      let success = await sendToRapidApp({
        url: downloadItem.url,
        referrer: ref,
        filename: resolvedFilename,
        cookies: cookies,
        user_agent: navigator.userAgent,
        is_gdrive: isGoogle
      });

      if (success) {
        // Cancel browser download only because Rapid successfully accepted it
        chrome.downloads.cancel(downloadItem.id, () => {
          chrome.downloads.erase({ id: downloadItem.id });
        });
        showNotification("⚡ Rapid Download Manager", `Accelerating: ${resolvedFilename || "File"}`);
      } else {
        // Rapid app is not running; allow Chrome to download the file natively!
        console.warn("[Rapid] Desktop app offline. Allowing browser native download.");
        suggest();
      }
    } catch (err) {
      console.error("[Rapid] Interception error:", err);
      suggest();
    }
  })();

  // Returning true informs Chrome that suggest() might be called asynchronously if not cancelled
  return true;
});
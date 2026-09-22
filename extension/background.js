// Rapid Download Manager â€” Background Service Worker
const RAPID_LOCAL_SERVER = "http://127.0.0.1:9669/add_download";

// Context Menu: Right click on any link or media
chrome.runtime.onInstalled.addListener(() => {
  chrome.contextMenus.create({
    id: "rapid-download-link",
    title: "âš¡ Download with Rapid Download Manager",
    contexts: ["link", "image", "video", "audio"]
  });
});

chrome.contextMenus.onClicked.addListener((info, tab) => {
  let targetUrl = info.linkUrl || info.srcUrl;
  if (targetUrl) {
    sendToRapidApp(targetUrl, tab ? tab.url : "");
  }
});

// Intercept browser native downloads automatically (IDM style)
chrome.downloads.onCreated.addListener(async (downloadItem) => {
  if (!downloadItem.url || downloadItem.url.startsWith("blob:") || downloadItem.url.startsWith("data:")) {
    return;
  }

  // Cancel native single-threaded browser download
  chrome.downloads.cancel(downloadItem.id, () => {
    chrome.downloads.erase({ id: downloadItem.id });
  });

  let isGoogle = false;
  try {
    let urlObj = new URL(downloadItem.url);
    isGoogle = urlObj.hostname.includes("google.com") || urlObj.hostname.includes("googleusercontent.com");
  } catch(e) {}

  // Extract cookies
  let cookieString = "";
  try {
    if (isGoogle) {
      // Get all cookies across Google domains (.google.com, drive.google.com, googleusercontent.com, and download url)
      const [driveCookies, googleCookies, contentCookies, urlCookies] = await Promise.all([
        chrome.cookies.getAll({ domain: "drive.google.com" }).catch(() => []),
        chrome.cookies.getAll({ domain: ".google.com" }).catch(() => []),
        chrome.cookies.getAll({ domain: "googleusercontent.com" }).catch(() => []),
        chrome.cookies.getAll({ url: downloadItem.url }).catch(() => [])
      ]);

      const cookieMap = new Map();
      [...googleCookies, ...driveCookies, ...contentCookies, ...urlCookies].forEach(c => {
        if (c && c.name) {
          cookieMap.set(c.name, c.value);
        }
      });
      cookieString = Array.from(cookieMap.entries()).map(([k, v]) => `${k}=${v}`).join('; ');
    } else {
      let cookies = await chrome.cookies.getAll({ url: downloadItem.url });
      cookieString = cookies.map(c => `${c.name}=${c.value}`).join('; ');
    }
  } catch (e) {
    console.warn("Failed to get cookies:", e);
  }

  // Determine referrer
  let ref = downloadItem.referrer || "";
  if (!ref && isGoogle) {
    ref = "https://drive.google.com/";
  }

  // Forward to Rapid Download Manager native engine
  sendToRapidApp(
    downloadItem.url,
    ref,
    downloadItem.filename || "",
    cookieString,
    navigator.userAgent,
    isGoogle
  );
});

function sendToRapidApp(url, referrer, filename = "", cookies = "", user_agent = "", is_gdrive = false) {
  fetch(RAPID_LOCAL_SERVER, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      url: url,
      referrer: referrer,
      filename: filename,
      cookies: cookies,
      user_agent: user_agent,
      is_gdrive: is_gdrive
    })
  })
  .then(resp => resp.json())
  .then(data => {
    console.log("Rapid Download Manager received URL:", data);
  })
  .catch(err => {
    console.warn("Could not connect to Rapid Download Manager local server (is it running?):", err);
  });
}
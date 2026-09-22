// Rapid Download Manager — Background Service Worker
const RAPID_LOCAL_SERVER = "http://127.0.0.1:9669/add_download";

// Context Menu: Right click on any link or media
chrome.runtime.onInstalled.addListener(() => {
  chrome.contextMenus.create({
    id: "rapid-download-link",
    title: "⚡ Download with Rapid Download Manager",
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

  // Extract cookies for the download URL
  let cookieString = "";
  try {
    let cookies = await chrome.cookies.getAll({ url: downloadItem.url });
    cookieString = cookies.map(c => `${c.name}=${c.value}`).join('; ');
  } catch (e) {
    console.warn("Failed to get cookies:", e);
  }

  // Forward to Rapid Download Manager native engine
  sendToRapidApp(downloadItem.url, downloadItem.referrer || "", downloadItem.filename || "", cookieString, navigator.userAgent);
});

function sendToRapidApp(url, referrer, filename = "", cookies = "", user_agent = "") {
  fetch(RAPID_LOCAL_SERVER, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({
      url: url,
      referrer: referrer,
      filename: filename,
      cookies: cookies,
      user_agent: user_agent
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

// Rapid Download Manager - Direct Link Click Interceptor
const DOWNLOAD_EXTENSIONS = new Set([
  "zip", "rar", "7z", "tar", "gz", "bz2", "xz", "iso", "img", "bin",
  "exe", "msi", "dmg", "pkg", "deb", "rpm", "apk", "appimage",
  "pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx", "csv",
  "mp4", "mkv", "avi", "mov", "webm", "flv", "mp3", "flac", "wav", "aac", "ogg", "m4a",
  "tgz", "zst", "jar", "war", "whl", "torrent"
]);

function isDownloadableLink(urlStr) {
  try {
    let u = new URL(urlStr, window.location.href);
    if (!["http:", "https:"].includes(u.protocol)) return false;
    let pathname = u.pathname.toLowerCase();
    let ext = pathname.split(".").pop();
    return DOWNLOAD_EXTENSIONS.has(ext);
  } catch (e) {
    return false;
  }
}

document.addEventListener("click", (e) => {
  // Holding Alt key bypasses interception (IDM style)
  if (e.altKey) return;

  let target = e.target.closest("a");
  if (!target || !target.href) return;

  let href = target.href;
  let hasDownloadAttr = target.hasAttribute("download");

  if (hasDownloadAttr || isDownloadableLink(href)) {
    e.preventDefault();
    e.stopPropagation();

    let dlAttr = (target.getAttribute("download") || "").trim();
    let fnVal = "";
    if (dlAttr && !["download", "true", "false", "undefined", "null", "file"].includes(dlAttr.toLowerCase())) {
      fnVal = dlAttr;
    }

    chrome.runtime.sendMessage({
      type: "RAPID_INTERCEPT_LINK",
      url: href,
      referrer: window.location.href,
      filename: fnVal
    });
  }
}, true);

// Rapid Download Manager - Content Script
// Link Click Interception, In-Page Floating Media Sniffer, and Page Scanner

const DOWNLOAD_EXTENSIONS = new Set([
  "zip", "rar", "7z", "tar", "gz", "bz2", "xz", "iso", "img", "bin",
  "exe", "msi", "dmg", "pkg", "deb", "rpm", "apk", "appimage",
  "pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx", "csv",
  "mp4", "mkv", "avi", "mov", "webm", "flv", "mp3", "flac", "wav", "aac", "ogg", "m4a",
  "tgz", "zst", "jar", "war", "whl", "torrent"
]);

const VIDEO_EXTENSIONS = new Set(["mp4", "mkv", "avi", "mov", "webm", "flv", "m4v", "3gp", "ts", "m3u8", "mpd"]);
const AUDIO_EXTENSIONS = new Set(["mp3", "wav", "aac", "flac", "ogg", "m4a", "wma", "opus"]);
const IMAGE_EXTENSIONS = new Set(["jpg", "jpeg", "png", "gif", "webp", "bmp", "svg", "tiff", "ico", "avif"]);
const DOC_EXTENSIONS = new Set(["pdf", "zip", "rar", "7z", "tar", "gz", "iso", "exe", "msi", "apk", "doc", "docx", "xls", "xlsx", "ppt", "pptx"]);

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

// ---------------------------------------------------------
// 1. Direct Link Click Interception (Alt key bypasses)
// ---------------------------------------------------------
document.addEventListener("click", (e) => {
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

// ---------------------------------------------------------
// 2. In-Page Floating Media Sniffer (IDM-style Floating Overlay)
// ---------------------------------------------------------
const hookedMedia = new WeakSet();

function getMediaSource(el) {
  if (!el) return "";
  if (el.currentSrc && !el.currentSrc.startsWith("blob:") && el.currentSrc.startsWith("http")) return el.currentSrc;
  if (el.src && !el.src.startsWith("blob:") && el.src.startsWith("http")) return el.src;

  let sources = el.querySelectorAll("source");
  for (let s of sources) {
    let src = s.src || s.getAttribute("src");
    if (src && !src.startsWith("blob:") && src.startsWith("http")) {
      return src;
    }
  }

  // Do NOT return blob: directly, as external clients cannot download RAM blobs
  return "";
}

function sanitizeTitleString(str) {
  if (!str) return "";
  // Strip YouTube notification count e.g. "(40) "
  let cleaned = str.replace(/^\(\d+\)\s*/, "");
  // Strip trailing platform branding e.g. " - YouTube"
  cleaned = cleaned.replace(/\s*-\s*YouTube$/i, "");
  // Strip characters forbidden in filenames: < > : " / \ | ? *
  cleaned = cleaned.replace(/[<>:"/\\|?*]/g, "_").trim();
  // Collapse whitespace
  cleaned = cleaned.replace(/\s+/g, " ");
  return cleaned;
}

function resolveMediaTitle(el, type) {
  // Special handling for YouTube
  if (window.location.hostname.includes("youtube.com")) {
    const ytSelectors = [
      "h1.ytd-watch-metadata yt-formatted-string",
      "#title h1 yt-formatted-string",
      "h1.title yt-formatted-string",
      "h1.title",
      "#video-title"
    ];
    for (let sel of ytSelectors) {
      let node = document.querySelector(sel);
      if (node && node.textContent && node.textContent.trim()) {
        return sanitizeTitleString(node.textContent.trim());
      }
    }
  }

  let title = (el.getAttribute("title") || el.getAttribute("aria-label") || "").trim();
  if (title) return sanitizeTitleString(title);

  // Check parent figure or container
  let container = el.closest("figure, article, [data-video-id], .video-js, .player");
  if (container) {
    let heading = container.querySelector("h1, h2, h3, h4, .title, .caption");
    if (heading && heading.textContent.trim()) {
      return sanitizeTitleString(heading.textContent.trim());
    }
  }

  // Fallback to document title
  let docTitle = document.title.trim();
  if (docTitle) return sanitizeTitleString(docTitle);

  return (type === "audio" ? "Audio" : "Video") + "_" + Date.now();
}

function attachFloatingWidget(mediaEl, type) {
  if (hookedMedia.has(mediaEl)) return;
  hookedMedia.add(mediaEl);

  // Create isolated Shadow DOM overlay host
  const host = document.createElement("div");
  host.className = "rapid-floating-overlay-host";
  host.style.cssText = "position: absolute; top: 0; left: 0; width: 0; height: 0; pointer-events: none; z-index: 2147483647;";
  
  const shadow = host.attachShadow({ mode: "open" });
  const style = document.createElement("style");
  style.textContent = `
    * {
      box-sizing: border-box;
      margin: 0;
      padding: 0;
      font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
      user-select: none;
    }
    .widget-container {
      position: absolute;
      display: flex;
      align-items: center;
      gap: 6px;
      padding: 5px 8px;
      background: rgba(11, 14, 20, 0.94);
      backdrop-filter: blur(14px);
      -webkit-backdrop-filter: blur(14px);
      border: 1px solid rgba(0, 210, 255, 0.45);
      border-radius: 8px;
      box-shadow: 0 8px 24px rgba(0, 0, 0, 0.65), 0 0 12px rgba(0, 210, 255, 0.25);
      opacity: 0;
      pointer-events: none;
      transform: translateY(-4px) scale(0.96);
      transition: opacity 0.22s cubic-bezier(0.16, 1, 0.3, 1), transform 0.22s cubic-bezier(0.16, 1, 0.3, 1);
    }
    .widget-container.active {
      opacity: 1;
      pointer-events: auto;
      transform: translateY(0) scale(1);
    }
    .dl-btn {
      display: inline-flex;
      align-items: center;
      gap: 6px;
      background: linear-gradient(135deg, #00D2FF 0%, #0077FE 100%);
      color: #FFFFFF;
      font-size: 11.5px;
      font-weight: 700;
      padding: 5px 10px;
      border: none;
      border-radius: 5px;
      cursor: pointer;
      outline: none;
      box-shadow: 0 2px 8px rgba(0, 210, 255, 0.4);
      text-shadow: 0 1px 2px rgba(0,0,0,0.3);
      transition: all 0.15s ease;
      white-space: nowrap;
    }
    .dl-btn:hover {
      transform: translateY(-1px);
      filter: brightness(1.12);
      box-shadow: 0 4px 14px rgba(0, 210, 255, 0.6);
    }
    .dl-btn:active {
      transform: translateY(0);
      filter: brightness(0.95);
    }
    .dl-btn.success {
      background: linear-gradient(135deg, #10B981 0%, #059669 100%);
      box-shadow: 0 2px 10px rgba(16, 185, 129, 0.5);
    }
    .dl-btn.sniffing {
      background: linear-gradient(135deg, #F59E0B 0%, #D97706 100%);
      box-shadow: 0 2px 10px rgba(245, 158, 11, 0.5);
    }
    .icon {
      font-size: 13px;
      display: inline-block;
      filter: drop-shadow(0 1px 1px rgba(0,0,0,0.4));
    }
    .format-badge {
      background: rgba(0, 210, 255, 0.12);
      border: 1px solid rgba(0, 210, 255, 0.3);
      color: #00D2FF;
      font-size: 9.5px;
      font-weight: 800;
      padding: 2px 5px;
      border-radius: 4px;
      letter-spacing: 0.5px;
      text-transform: uppercase;
    }
    .close-btn {
      display: flex;
      align-items: center;
      justify-content: center;
      width: 18px;
      height: 18px;
      background: transparent;
      border: none;
      color: #8A99AD;
      font-size: 12px;
      border-radius: 4px;
      cursor: pointer;
      transition: all 0.15s;
    }
    .close-btn:hover {
      color: #FF5252;
      background: rgba(255, 82, 82, 0.18);
    }
  `;

  const widget = document.createElement("div");
  widget.className = "widget-container";

  const iconSpan = document.createElement("span");
  iconSpan.className = "icon";
  iconSpan.textContent = type === "audio" ? "🎵" : "⚡";

  const btnText = document.createElement("span");
  btnText.textContent = type === "audio" ? "Download Audio" : "Download Video";

  const dlButton = document.createElement("button");
  dlButton.className = "dl-btn";
  dlButton.appendChild(iconSpan);
  dlButton.appendChild(btnText);

  const formatBadge = document.createElement("span");
  formatBadge.className = "format-badge";
  formatBadge.textContent = type === "audio" ? "MP3" : "MP4";

  const closeButton = document.createElement("button");
  closeButton.className = "close-btn";
  closeButton.innerHTML = "✕";
  closeButton.title = "Dismiss download button";

  widget.appendChild(dlButton);
  widget.appendChild(formatBadge);
  widget.appendChild(closeButton);

  shadow.appendChild(style);
  shadow.appendChild(widget);
  document.body.appendChild(host);

  let isHovered = false;
  let isMediaHovered = false;
  let hideTimer = null;
  let isDismissed = false;

  function updatePosition() {
    if (isDismissed || !document.body.contains(mediaEl)) return;
    const rect = mediaEl.getBoundingClientRect();
    if (rect.width < 120 || rect.height < 60 || rect.bottom < 0 || rect.top > window.innerHeight) {
      widget.classList.remove("active");
      return;
    }

    let top = rect.top + window.scrollY + 10;
    let left = (rect.right + window.scrollX) - (widget.offsetWidth || 165) - 10;

    widget.style.top = `${top}px`;
    widget.style.left = `${Math.max(10, left)}px`;
  }

  function showWidget() {
    if (isDismissed) return;
    clearTimeout(hideTimer);
    updatePosition();
    widget.classList.add("active");
  }

  function scheduleHide() {
    clearTimeout(hideTimer);
    hideTimer = setTimeout(() => {
      if (!isHovered && !isMediaHovered) {
        widget.classList.remove("active");
      }
    }, 1200);
  }

  // Hover listeners on media element
  mediaEl.addEventListener("mouseenter", () => {
    isMediaHovered = true;
    showWidget();
  });
  mediaEl.addEventListener("mouseleave", () => {
    isMediaHovered = false;
    scheduleHide();
  });
  mediaEl.addEventListener("play", () => {
    showWidget();
    scheduleHide();
  });

  // Hover listeners on widget
  widget.addEventListener("mouseenter", () => {
    isHovered = true;
    showWidget();
  });
  widget.addEventListener("mouseleave", () => {
    isHovered = false;
    scheduleHide();
  });

  // Reposition on window resize and scroll
  window.addEventListener("scroll", updatePosition, { passive: true });
  window.addEventListener("resize", updatePosition, { passive: true });

  // Close / dismiss
  closeButton.addEventListener("click", (e) => {
    e.stopPropagation();
    isDismissed = true;
    widget.classList.remove("active");
  });

  // Helper to query sniffed stream from background service worker
  async function querySniffedStream() {
    try {
      let resp = await new Promise((res) => {
        chrome.runtime.sendMessage({ type: "RAPID_GET_TAB_STREAM" }, res);
      });
      if (resp && resp.data) {
        if (type === "audio" && resp.data.audio) {
          return resp.data.audio.url;
        } else if (resp.data.video) {
          return resp.data.video.url;
        } else if (resp.data.m3u8) {
          return resp.data.m3u8.url;
        }
      }
    } catch (e) {}
    return null;
  }

  // Click to Download via Rapid Download Manager
  dlButton.addEventListener("click", async (e) => {
    e.stopPropagation();
    e.preventDefault();

    let mediaSrc = getMediaSource(mediaEl);
    let title = resolveMediaTitle(mediaEl, type);
    let ext = type === "audio" ? "mp3" : "mp4";
    if (!title.toLowerCase().endsWith("." + ext)) {
      title += "." + ext;
    }

    let isYouTube = window.location.hostname.includes("youtube.com") || window.location.hostname.includes("youtu.be");
    if (isYouTube) {
      chrome.runtime.sendMessage({
        type: "RAPID_INTERCEPT_MEDIA",
        url: window.location.href,
        referrer: window.location.href,
        filename: title,
        mediaType: type,
        is_youtube: true
      }, (res) => {
        if (res && res.success) {
          btnText.textContent = "Dispatched!";
          dlButton.classList.add("success");
          setTimeout(() => {
            btnText.textContent = type === "audio" ? "Download Audio" : "Download Video";
            dlButton.classList.remove("success");
          }, 2000);
        }
      });
      return;
    }

    // If media source is not direct (e.g. YouTube MediaSource blob)
    if (!mediaSrc) {
      btnText.textContent = "Sniffing stream...";
      dlButton.classList.add("sniffing");

      // 1. Check if background sniffer has already captured stream
      let captured = await querySniffedStream();

      // 2. If not yet captured and video is paused, trigger play briefly to initiate stream request
      if (!captured && mediaEl.paused) {
        try {
          await mediaEl.play();
        } catch (err) {}
        await new Promise((r) => setTimeout(r, 700));
        captured = await querySniffedStream();
      } else if (!captured) {
        await new Promise((r) => setTimeout(r, 600));
        captured = await querySniffedStream();
      }

      dlButton.classList.remove("sniffing");

      if (captured) {
        mediaSrc = captured;
      }
    }

    if (!mediaSrc) {
      btnText.textContent = "Play video 1s to capture!";
      setTimeout(() => {
        btnText.textContent = type === "audio" ? "Download Audio" : "Download Video";
      }, 2500);
      return;
    }

    btnText.textContent = "Sending to Rapid...";
    dlButton.classList.add("success");

    chrome.runtime.sendMessage({
      type: "RAPID_INTERCEPT_MEDIA",
      url: mediaSrc,
      filename: title,
      referrer: window.location.href,
      mediaType: type
    }, (resp) => {
      if (resp && resp.success) {
        btnText.textContent = "✓ Sent to Rapid!";
      } else {
        btnText.textContent = "Play video 1s to capture!";
      }
      setTimeout(() => {
        btnText.textContent = type === "audio" ? "Download Audio" : "Download Video";
        dlButton.classList.remove("success");
      }, 2500);
    });
  });
}

function scanMediaElements() {
  const videos = document.querySelectorAll("video");
  videos.forEach(v => attachFloatingWidget(v, "video"));

  const audios = document.querySelectorAll("audio");
  audios.forEach(a => attachFloatingWidget(a, "audio"));

  let totalMedia = videos.length + audios.length;
  chrome.runtime.sendMessage({
    type: "RAPID_UPDATE_MEDIA_COUNT",
    count: totalMedia
  }).catch(() => {});
}

// Initial scan and MutationObserver for dynamically loaded players
if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", scanMediaElements);
} else {
  scanMediaElements();
}

const observer = new MutationObserver(() => {
  scanMediaElements();
});
observer.observe(document.documentElement, { childList: true, subtree: true });

// ---------------------------------------------------------
// 3. Full Page Media Scanner (Listener for Extension Popup)
// ---------------------------------------------------------
chrome.runtime.onMessage.addListener((msg, sender, sendResponse) => {
  if (msg && msg.type === "RAPID_SCAN_PAGE_MEDIA") {
    (async () => {
      let collected = {
        videos: [],
        audio: [],
        images: [],
        files: []
      };

      const seenUrls = new Set();

      // Query any sniffed streams from background worker for MSE players
      let sniffedData = null;
      try {
        let resp = await new Promise((res) => {
          chrome.runtime.sendMessage({ type: "RAPID_GET_TAB_STREAM" }, res);
        });
        if (resp && resp.data) {
          sniffedData = resp.data;
        }
      } catch (e) {}

      // 1. Gather Videos
      document.querySelectorAll("video").forEach(v => {
        let src = getMediaSource(v);
        if (!src && sniffedData && sniffedData.video) {
          src = sniffedData.video.url;
        } else if (!src && sniffedData && sniffedData.m3u8) {
          src = sniffedData.m3u8.url;
        }

        if (src && !seenUrls.has(src)) {
          seenUrls.add(src);
          let title = resolveMediaTitle(v, "video");
          collected.videos.push({
            url: src,
            title: title,
            type: "video",
            poster: v.poster || ""
          });
        }
      });

      // 2. Gather Audio
      document.querySelectorAll("audio").forEach(a => {
        let src = getMediaSource(a);
        if (!src && sniffedData && sniffedData.audio) {
          src = sniffedData.audio.url;
        }
        if (src && !seenUrls.has(src)) {
          seenUrls.add(src);
          let title = resolveMediaTitle(a, "audio");
          collected.audio.push({
            url: src,
            title: title,
            type: "audio"
          });
        }
      });

      // 3. Gather Large Images
      document.querySelectorAll("img").forEach(img => {
        let src = img.currentSrc || img.src || img.getAttribute("data-src") || "";
        if (!src || src.startsWith("data:") || seenUrls.has(src)) return;

        let w = img.naturalWidth || img.width || 0;
        let h = img.naturalHeight || img.height || 0;

        if ((w >= 60 && h >= 60) || isImageExtension(src)) {
          seenUrls.add(src);
          let alt = (img.alt || img.title || "").trim();
          let fn = alt || extractFilenameFromUrl(src) || "image.jpg";
          collected.images.push({
            url: src,
            title: fn,
            type: "image",
            thumbnail: src,
            width: w,
            height: h
          });
        }
      });

      // 4. Gather Downloadable Links / Documents
      document.querySelectorAll("a[href]").forEach(a => {
        let href = a.href;
        if (!href || !href.startsWith("http") || seenUrls.has(href)) return;

        let ext = (href.split("?")[0].split("#")[0].split(".").pop() || "").toLowerCase();
        if (VIDEO_EXTENSIONS.has(ext)) {
          seenUrls.add(href);
          collected.videos.push({
            url: href,
            title: a.textContent.trim() || extractFilenameFromUrl(href),
            type: "video"
          });
        } else if (AUDIO_EXTENSIONS.has(ext)) {
          seenUrls.add(href);
          collected.audio.push({
            url: href,
            title: a.textContent.trim() || extractFilenameFromUrl(href),
            type: "audio"
          });
        } else if (IMAGE_EXTENSIONS.has(ext)) {
          seenUrls.add(href);
          collected.images.push({
            url: href,
            title: a.textContent.trim() || extractFilenameFromUrl(href),
            type: "image",
            thumbnail: href
          });
        } else if (DOC_EXTENSIONS.has(ext) || a.hasAttribute("download")) {
          seenUrls.add(href);
          collected.files.push({
            url: href,
            title: a.getAttribute("download") || a.textContent.trim() || extractFilenameFromUrl(href),
            type: "file",
            ext: ext
          });
        }
      });

      sendResponse({ status: "ok", data: collected });
    })();
    return true;
  }
});

function isImageExtension(urlStr) {
  try {
    let ext = urlStr.split("?")[0].split("#")[0].split(".").pop().toLowerCase();
    return IMAGE_EXTENSIONS.has(ext);
  } catch (e) {
    return false;
  }
}

function extractFilenameFromUrl(urlStr) {
  try {
    let clean = urlStr.split("?")[0].split("#")[0];
    let fn = clean.substring(clean.lastIndexOf("/") + 1);
    return decodeURIComponent(fn);
  } catch (e) {
    return "download";
  }
}
// Rapid Download Manager — Extension Popup Script
// Universal Media Scanner and Batch Downloader UI

let allMediaItems = [];
let currentTabFilter = "all";
let activeTabUrl = "";

document.addEventListener("DOMContentLoaded", async () => {
  checkEngineStatus();
  setupEventListeners();
  await scanCurrentTabMedia();
});

function setupEventListeners() {
  // Filter Tabs
  document.querySelectorAll(".tab-btn").forEach(btn => {
    btn.addEventListener("click", () => {
      document.querySelectorAll(".tab-btn").forEach(b => b.classList.remove("active"));
      btn.classList.add("active");
      currentTabFilter = btn.dataset.tab;
      renderMediaList();
    });
  });

  // Select All checkbox
  const selectAllCb = document.getElementById("select-all");
  selectAllCb.addEventListener("change", (e) => {
    const isChecked = e.target.checked;
    const filtered = getFilteredItems();
    filtered.forEach(item => item.selected = isChecked);
    renderMediaList();
    updateBatchButtonState();
  });

  // Download Selected Button
  document.getElementById("download-selected-btn").addEventListener("click", () => {
    const selected = allMediaItems.filter(item => item.selected);
    if (selected.length === 0) return;

    chrome.runtime.sendMessage({
      type: "RAPID_BATCH_DOWNLOAD",
      items: selected.map(s => ({
        url: s.url,
        title: s.title,
        referrer: activeTabUrl
      }))
    }, (res) => {
      window.close();
    });
  });

  // Rescan Page Button
  document.getElementById("rescan-btn").addEventListener("click", () => {
    scanCurrentTabMedia();
  });
}

// Check desktop engine connection status on 127.0.0.1:9669
function checkEngineStatus() {
  const dot = document.getElementById("status-dot");
  const text = document.getElementById("status-text");

  chrome.runtime.sendMessage({ type: "RAPID_CHECK_STATUS" }, (res) => {
    if (res && res.online) {
      dot.className = "status-dot online";
      text.textContent = "Engine Connected";
    } else {
      dot.className = "status-dot";
      text.textContent = "Engine Offline";
    }
  });
}

// Request media scan from active tab's content script
async function scanCurrentTabMedia() {
  const mediaList = document.getElementById("media-list");
  const emptyState = document.getElementById("empty-state");

  mediaList.innerHTML = "<div style='text-align:center; padding: 24px; color:#8A99AD;'>Scanning page for media...</div>";
  emptyState.style.display = "none";

  let tabs = await chrome.tabs.query({ active: true, currentWindow: true });
  if (!tabs || tabs.length === 0) return;

  activeTabUrl = tabs[0].url || "";

  chrome.tabs.sendMessage(tabs[0].id, { type: "RAPID_SCAN_PAGE_MEDIA" }, (resp) => {
    if (chrome.runtime.lastError || !resp || !resp.data) {
      mediaList.innerHTML = "";
      emptyState.style.display = "block";
      updateCounts(0, 0, 0, 0, 0);
      return;
    }

    const { videos, audio, images, files } = resp.data;

    // Flatten all items with unique IDs
    allMediaItems = [
      ...(videos || []).map((v, i) => ({ ...v, id: "v_" + i, type: "video", selected: true })),
      ...(audio || []).map((a, i) => ({ ...a, id: "a_" + i, type: "audio", selected: true })),
      ...(images || []).map((img, i) => ({ ...img, id: "img_" + i, type: "image", selected: false })),
      ...(files || []).map((f, i) => ({ ...f, id: "f_" + i, type: "file", selected: true }))
    ];

    updateCounts(
      allMediaItems.length,
      (videos || []).length,
      (audio || []).length,
      (images || []).length,
      (files || []).length
    );

    renderMediaList();
    updateBatchButtonState();
  });
}

function updateCounts(all, v, a, img, f) {
  document.getElementById("count-all").textContent = all;
  document.getElementById("count-videos").textContent = v;
  document.getElementById("count-audio").textContent = a;
  document.getElementById("count-images").textContent = img;
  document.getElementById("count-files").textContent = f;
}

function getFilteredItems() {
  if (currentTabFilter === "all") return allMediaItems;
  if (currentTabFilter === "videos") return allMediaItems.filter(i => i.type === "video");
  if (currentTabFilter === "audio") return allMediaItems.filter(i => i.type === "audio");
  if (currentTabFilter === "images") return allMediaItems.filter(i => i.type === "image");
  if (currentTabFilter === "files") return allMediaItems.filter(i => i.type === "file");
  return allMediaItems;
}

function renderMediaList() {
  const container = document.getElementById("media-list");
  const emptyState = document.getElementById("empty-state");
  const filtered = getFilteredItems();

  container.innerHTML = "";

  if (filtered.length === 0) {
    emptyState.style.display = "block";
    return;
  }
  emptyState.style.display = "none";

  filtered.forEach(item => {
    const card = document.createElement("div");
    card.className = "media-card";

    // Checkbox
    const cb = document.createElement("input");
    cb.type = "checkbox";
    cb.checked = !!item.selected;
    cb.addEventListener("change", (e) => {
      item.selected = e.target.checked;
      updateBatchButtonState();
    });

    // Thumbnail / Icon
    let thumbEl;
    if (item.type === "image" && item.thumbnail) {
      thumbEl = document.createElement("img");
      thumbEl.className = "media-thumb";
      thumbEl.src = item.thumbnail;
      thumbEl.onerror = () => { thumbEl.replaceWith(createIconSpan("🖼")); };
    } else if (item.type === "video" && item.poster) {
      thumbEl = document.createElement("img");
      thumbEl.className = "media-thumb";
      thumbEl.src = item.poster;
      thumbEl.onerror = () => { thumbEl.replaceWith(createIconSpan("🎥")); };
    } else {
      let iconChar = item.type === "video" ? "🎥" : item.type === "audio" ? "🎵" : item.type === "image" ? "🖼" : "📁";
      thumbEl = createIconSpan(iconChar);
    }

    // Info
    const info = document.createElement("div");
    info.className = "media-info";

    const titleEl = document.createElement("div");
    titleEl.className = "media-title";
    titleEl.textContent = item.title || "Media File";
    titleEl.title = item.url;

    const metaEl = document.createElement("div");
    metaEl.className = "media-meta";

    const extBadge = document.createElement("span");
    extBadge.className = "badge-ext";
    let ext = (item.ext || item.url.split("?")[0].split(".").pop() || item.type).toUpperCase();
    if (ext.length > 5) ext = item.type.toUpperCase();
    extBadge.textContent = ext;

    const typeDesc = document.createElement("span");
    typeDesc.textContent = item.type.toUpperCase();

    metaEl.appendChild(extBadge);
    metaEl.appendChild(typeDesc);
    info.appendChild(titleEl);
    info.appendChild(metaEl);

    // Single Download Button
    const dlBtn = document.createElement("button");
    dlBtn.className = "item-dl-btn";
    dlBtn.innerHTML = "⬇";
    dlBtn.title = "Download with Rapid";
    dlBtn.addEventListener("click", () => {
      dlBtn.innerHTML = "✓";
      chrome.runtime.sendMessage({
        type: "RAPID_INTERCEPT_MEDIA",
        url: item.url,
        filename: item.title,
        referrer: activeTabUrl,
        mediaType: item.type
      });
    });

    card.appendChild(cb);
    card.appendChild(thumbEl);
    card.appendChild(info);
    card.appendChild(dlBtn);

    container.appendChild(card);
  });
}

function createIconSpan(char) {
  const span = document.createElement("div");
  span.className = "media-thumb";
  span.textContent = char;
  return span;
}

function updateBatchButtonState() {
  const selectedCount = allMediaItems.filter(i => i.selected).length;
  const btn = document.getElementById("download-selected-btn");
  const badge = document.getElementById("selected-count-badge");

  badge.textContent = `(${selectedCount})`;
  btn.disabled = selectedCount === 0;
}

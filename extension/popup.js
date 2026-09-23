// Rapid Download Manager — Extension Popup Logic
async function checkEngineStatus() {
  const dot = document.getElementById("status-dot");
  const text = document.getElementById("status-text");
  const desc = document.getElementById("status-desc");

  for (const host of ["http://127.0.0.1:9669", "http://localhost:9669"]) {
    try {
      const resp = await fetch(host, { method: "GET", mode: "cors" });
      if (resp.ok) {
        const data = await resp.json().catch(() => ({}));
        dot.className = "status-dot online";
        text.innerText = "Engine Connected & Active";
        text.style.color = "#2ECC71";
        desc.innerText = "Browser downloads are automatically intercepted and accelerated with high-speed multi-threading.";
        return;
      }
    } catch (e) {
      console.warn(`[Rapid Popup] Failed connection to ${host}:`, e);
    }
  }

  dot.className = "status-dot offline";
  text.innerText = "Desktop App Offline";
  text.style.color = "#E74C3C";
  desc.innerText = "Please ensure rapid-gui.exe is open on your computer. When the app is running, downloads will be intercepted automatically.";
}

document.addEventListener("DOMContentLoaded", () => {
  checkEngineStatus();
  // Periodically refresh status while popup is open
  setInterval(checkEngineStatus, 3000);
});

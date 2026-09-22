# ⚡ Rapid Download Manager

> **High-Speed Multi-Threaded Download Manager with Native Browser Interception, Cyber-Obsidian UI, and System Tray Integration.** Built in **Rust** using `tokio` and `eframe`/`egui`.

---

## ✨ Features

- **🚀 Ultra-Fast Multi-Part Engine:** Splits downloads into up to 32 parallel HTTP connections with adaptive segment chunking, dynamic rebalancing, and resume support.
- **🌐 Browser Download Interception (IDM Style):** Native WebExtension (Manifest V3) automatically captures browser downloads across **Google Chrome**, **Microsoft Edge**, and **Brave Browser** and routes them directly to the native engine via `127.0.0.1:9669`.
- **🖱️ Right-Click Context Menu:** Download any file, image, audio, or video link with *"⚡ Download with Rapid Download Manager"*.
- **🎨 Cyber-Obsidian Dark Theme:** Custom-tailored dark theme (`#0B0E14`, `#151922`, `#00D2FF`) engineered for low eye fatigue and sleek modern aesthetics.
- **🌟 Animated Splash Screen:** Smooth 1.8-second startup animation with pulsating neon lightning and expanding radar glow.
- **📊 Custom Cyber Status Bar:** 
  - **Engine Beacon:** Live green beacon indicating listener status on `127.0.0.1:9669`.
  - **Dynamic Speed Pill:** Real-time aggregate multi-stream download throughput badge.
  - **Color-Coded Status Counters:** Active (`📥`), Paused (`⏸`), Finished (`✔`), and Total Transferred (`📊`).
  - **One-Click Quick Action Chips:** Direct links to reveal Downloads folder and trigger Extension Hook.
- **🛡️ Windows System Tray & Background Downloads:**
  - Resides in Windows Notification Area (System Tray).
  - Hover tooltip displays live transfer speeds and task progress.
  - Right-click tray menu to open, pause all, resume all, or exit.
  - Minimizing or clicking window close keeps downloads running uninterrupted in the background.
- **🛠️ Interactive Table Actions:**
  - Compact icon-only buttons with hover tooltips (`⏸ Pause`, `▶ Resume`, `🔄 Redownload`, `🗑 Delete`).
  - Single-click to open completed files directly using default system viewer.
  - `📁` button to open and highlight file in Windows File Explorer.

---

## 🏗️ Architecture & Project Structure

```
rapid_download_manager/
├── Cargo.toml                # Cargo workspace configuration
├── crates/
│   ├── rapid-core/           # Multi-threaded download engine & segment coordinator
│   │   ├── src/lib.rs
│   │   ├── src/segment.rs
│   │   └── src/task.rs
│   ├── rapid-cli/            # Command-line interface
│   │   └── src/main.rs
│   └── rapid-gui/            # Desktop GUI application (eframe / egui)
│       ├── src/main.rs       # UI, state management & Tokio HTTP bridge
│       └── src/tray.rs       # Native Win32 System Tray implementation
├── extension/                # Manifest V3 Browser Extension
│   ├── manifest.json
│   ├── background.js
│   ├── popup.html
│   └── icon128.png
├── install_extension.bat     # One-click Windows Registry browser installer
└── run_rapid_gui.bat         # Launcher script
```

---

## 🚀 Quick Start

### Prerequisites

- [Rust & Cargo](https://rustup.rs/) (edition 2021)
- Windows 10/11 (for System Tray integration and desktop GUI)

### 1. Build and Run GUI

```bash
cargo run -p rapid-gui
```

Or run via the provided batch script:
```cmd
run_rapid_gui.bat
```

### 2. Install Browser Extension

#### Option A: One-Click Registry Installer
Double-click `install_extension.bat` in the root folder.

#### Option B: Developer Mode (Chrome / Edge / Brave)
1. Open `chrome://extensions` or `edge://extensions`.
2. Toggle **Developer mode** on (top right).
3. Click **Load unpacked** and select the `extension/` directory.

---

## 📜 License

MIT License. Developed with ❤️ in Rust.

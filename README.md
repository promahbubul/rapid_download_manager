# ⚡ Rapid Download Manager (Neon Pink Velvet Edition)

[![Release](https://img.shields.io/badge/Release-v1.0.5-FF2A85?style=for-the-badge&logo=github)](https://github.com/promahbubul/rapid_download_manager/releases/tag/v1.0.5)
[![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%7C%2011%20(x64)-1A0F24?style=for-the-badge&logo=windows)](https://github.com/promahbubul/rapid_download_manager/releases)
[![Rust](https://img.shields.io/badge/Language-Rust%202021-DEA584?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-00E5FF?style=for-the-badge)](LICENSE)

> **Rapid Download Manager** is an ultra-fast, multi-stream desktop download accelerator engineered from the ground up in **Rust**. Featuring a bespoke **Neon Pink Velvet & Cyber Magenta** visual design system, native Chromium browser interception, background system tray persistence, native MEGA.nz / Google Drive folder acceleration, and up to 32 parallel download streams.

---

## 📥 Download Production Software

Choose your preferred format below to download the latest **v1.0.5 Production Release**:

| Package Type | Description | Primary Download (GitHub Releases) | Direct Mirror (Repository) |
| :--- | :--- | :--- | :--- |
| 🏪 **Microsoft Store Package (.msix)** | **Store Certified:** Pure modern Windows MSIX package with sandboxed security and zero admin privileges required. | [⬇️ **Download Store v1.0.5 (.msix)**](https://github.com/promahbubul/rapid_download_manager/releases/latest) | [⬇️ Mirror (.msix)](https://github.com/promahbubul/rapid_download_manager/raw/main/dist/installer/RapidDownloadManager_v1.0.5.msix) |
| 💿 **Windows Installer (Setup .exe)** | **Classic Installer:** Full guided Windows setup wizard, Start Menu shortcuts, Desktop icon, autostart toggle, and uninstaller. | [⬇️ **Download Setup v1.0.4 (.exe)**](https://github.com/promahbubul/rapid_download_manager/releases/latest) | [⬇️ Mirror (.exe)](https://github.com/promahbubul/rapid_download_manager/raw/main/dist/installer/RapidDownloadManager_Setup_v1.0.4.exe) |
| 📦 **Portable Standalone (.zip)** | No installation required. Extract anywhere and run `rapid-gui.exe` immediately. | [⬇️ **Download Portable v1.0.5 (.zip)**](https://github.com/promahbubul/rapid_download_manager/releases/latest) | [⬇️ Mirror (.zip)](https://github.com/promahbubul/rapid_download_manager/raw/main/dist/installer/RapidDownloadManager_v1.0.5_Portable.zip) |

🔗 **View all releases & changelogs:** [GitHub Releases Page](https://github.com/promahbubul/rapid_download_manager/releases)

---

## 🌟 Software Overview & Key Highlights

Rapid Download Manager departs completely from outdated 90s-style download utilities (such as legacy IDM and FDM clones) by introducing a 100% proprietary, copyright-safe, modern **Neon Velvet** interface and a high-concurrency Rust engine.

```
┌────────────────────────────────────────────────────────────────────────┐
│  ⚡ RAPID DOWNLOAD MANAGER v1.0.5                    [─] [□] [✕]      │
├────────────────────────────────────────────────────────────────────────┤
│  [＋ New Download]  [▶ Resume All]  [⏸ Pause All]   [📂 Open Folder]   │
├────────────────────────────────────────────────────────────────────────┤
│  File Name         │ Progress │ Speed    │ Status    │ Actions        │
│  ──────────────────┼──────────┼──────────┼───────────┼─────────────── │
│  ubuntu-24.04.iso  │ ██████░░ │ 24.5 MB/s│ Active    │ [⏸] [🔄] [🗑] │
│  rust-setup.exe    │ ████████ │ Completed│ Finished  │ [▶] [📁] [🗑] │
├────────────────────────────────────────────────────────────────────────┤
│  ● ENGINE ACTIVE   ⚡ 24.5 MB/s │ 📥 1 Active  ⏸ 0 Paused  ✔ 1 Done   │
└────────────────────────────────────────────────────────────────────────┘
```

---

## 🚀 In-Depth Features

### 1. 🎨 Proprietary Neon Pink Velvet & Cyber Magenta UI
- **Original Color Palette:** Built with a luxurious dark velvet foundation (`#0A060E`, `#1A0F24`, `#2D1236`) complemented by vibrant electric neon accents (`#FF2A85`, `#FF6BB5`, `#00E5FF`).
- **Command Dock Architecture:** The legacy top menu and toolbar are replaced with a sleek horizontal Command Dock featuring glowing action pills and smooth hover animations.
- **Custom Vector Icon Rendering Engine:** Instead of relying on external raster images or generic web icon fonts, all icons (Plus, Play, Pause, Folder, Refresh, Trash, Bolt, Eye, Checkmark) are computed directly on the GPU using anti-aliased mathematical Bézier geometry.
- **Cyber Splash Screen:** A seamless 1.8-second startup sequence with pulsating neon lightning, radial gradient ripples, and live engine status updates.

### 2. ⚡ High-Speed Multi-Part Concurrency Engine
- **Parallel Segment Chunking:** Downloads are split into **4 to 32 concurrent HTTP/HTTPS worker threads** requesting specific byte ranges (`Range: bytes=start-end`).
- **Adaptive Rebalancing:** High-latency threads automatically yield remaining chunks to faster connections, maximizing your available bandwidth.
- **Segmented Resume Support:** Network dropouts or pauses do not corrupt files. Segment manifests record exact byte boundaries and dynamically resume from the last validated chunk.
- **Asynchronous Tokio Core:** All network I/O is non-blocking and executes independently of the GUI thread, ensuring 60 FPS buttery-smooth desktop interactivity.

### 3. ☁️ Native MEGA.nz Decrypted Streaming & Folder Acceleration
- **Automatic Folder Tree Crawling:** Paste any public MEGA folder link (`https://mega.nz/folder/...#...`), and Rapid Download Manager immediately explores the entire hierarchy.
- **Local Directory Structure Preservation:** Files are automatically saved into their respective subdirectories inside your download directory.
- **On-the-Fly AES-128-CTR Decryption:** Streams encrypted blocks directly from MEGA CDN servers and decrypts them in memory as data arrives, avoiding intermediate disk thrashing.
- **Resilient Range Resume:** Supports resuming interrupted or paused MEGA downloads directly at exact byte boundaries.

### 4. 🌐 Browser Auto-Interception (IDM Style)
- **Chromium Ecosystem Support:** Full compatibility with **Google Chrome**, **Microsoft Edge**, **Brave Browser**, **Opera**, and **Vivaldi**.
- **Manifest V3 Extension:** Lightweight background script intercepts browser download events and hands them over to Rapid Download Manager automatically.
- **Right-Click Context Menu:** Right-click on any image, video, audio link, or file download button and choose *"⚡ Download with Rapid Download Manager"*.
- **Native Local RPC Daemon:** The desktop application runs a lightning-fast HTTP listener on `127.0.0.1:9669` to receive tasks with file metadata, user agent, and referrer headers.
- **One-Click Extension Setup:** Includes `install_extension.bat` which configures Windows Registry extension policies in seconds.

### 5. 📊 Custom Cyber Status Bar & System Tray
- **Engine Beacon:** Live status indicator showing active local daemon health on `127.0.0.1:9669`.
- **Dynamic Speed Pill:** Real-time aggregate bandwidth meter displaying combined throughput across all multi-part worker connections.
- **Categorized Download Counters:** Instant counters for Active (`📥`), Paused (`⏸`), and Completed (`✔`) files.
- **Background Persistence (System Tray):** Minimizing or closing the window minimizes to the Windows Notification Area, allowing uninterrupted long-running downloads. Hovering over the tray icon displays current speed and status.

### 6. 🛠️ Rich Task Interaction & Direct File Access
- **Single-Click File Launch:** Clicking any completed file automatically executes it using your operating system's default media player or document viewer.
- **Folder Reveal Button (`📁`):** Instantly highlights and selects the downloaded file in Windows File Explorer.
- **Full Control Actions:** Inline compact buttons to Pause (`⏸`), Resume (`▶`), Redownload (`🔄`), or Delete (`🗑`) tasks with custom confirmation safeguards.

---

## 💻 Installation & Setup Guide

### Method 1: Microsoft Store Package (.msix) (Recommended)
1. Download [`RapidDownloadManager_v1.0.5.msix`](https://github.com/promahbubul/rapid_download_manager/releases/latest).
2. Double-click the installer to launch the modern Windows App installer.
3. Click **Install** and launch directly from Windows Start Menu.

### Method 2: Portable Standalone Package
1. Download [`RapidDownloadManager_v1.0.5_Portable.zip`](https://github.com/promahbubul/rapid_download_manager/releases/latest).
2. Extract the archive into any folder on your PC.
3. Double-click `rapid-gui.exe` (or `run_rapid_gui.bat`).

---

## 🔌 Installing the Browser Extension

To enable automatic download interception from Chrome, Edge, or Brave:

### Option A: Automatic One-Click Setup
1. Inside the application folder, right-click `install_extension.bat` and select **Run as administrator** (or double-click to install for the current user).
2. Restart your browser.

### Option B: Manual Developer Mode
1. Open your browser and navigate to:
   - **Chrome / Brave:** `chrome://extensions`
   - **Edge:** `edge://extensions`
2. Turn on **Developer mode** (top right switch).
3. Click **Load unpacked** (top left).
4. Select the `extension/` directory inside your Rapid Download Manager installation folder.
5. The **⚡ Rapid Download Manager Interceptor** icon will appear in your browser toolbar!

---

## 🛠️ Building from Source

If you want to build the project from scratch or customize the Rust source code:

### Prerequisites
- [Rust & Cargo](https://rustup.rs/) (Stable 1.75+ or newer)
- Windows 10/11 x64
- [Inno Setup 6](https://jrsoftware.org/isdl.php) (optional, for compiling the setup installer)

### 1. Clone the Repository
```bash
git clone https://github.com/promahbubul/rapid_download_manager.git
cd rapid_download_manager
```

### 2. Run Desktop GUI in Debug Mode
```bash
cargo run -p rapid-gui
```

### 3. Build Optimized Production Binary
```bash
cargo build --release -p rapid-gui
```

The optimized production executable will be created at:
```
target/release/rapid-gui.exe
```

### 4. Build Microsoft Store Package (.msix)
```powershell
python msix/build_msix.py
```
The compiled MSIX package will be generated in `dist/installer/RapidDownloadManager_v1.0.5.msix`.

---

## 🏗️ Repository Architecture

```
rapid_download_manager/
├── .github/
│   └── workflows/
│       └── build-release.yml    # Automated CI/CD release workflow
├── crates/
│   ├── rapid-core/              # Multi-threaded download engine & segment coordinator
│   │   ├── src/lib.rs
│   │   ├── src/mega.rs          # Native MEGA.nz folder crawler & AES decrypter
│   │   ├── src/gdrive.rs        # Google Drive folder/file resolver
│   │   ├── src/segment.rs       # Byte-range chunking & worker threads
│   │   └── src/task.rs          # Task state machine & manifests
│   ├── rapid-cli/               # Standalone command-line interface
│   │   └── src/main.rs
│   └── rapid-gui/               # Desktop GUI (eframe / egui)
│       ├── src/main.rs          # Neon Pink velvet UI, vector icons, HTTP bridge
│       └── src/tray.rs          # Win32 system tray & notification area
├── dist/
│   └── installer/               # Production executables & releases
│       ├── RapidDownloadManager_v1.0.5.msix
│       └── RapidDownloadManager_v1.0.5_Portable.zip
├── extension/                   # Manifest V3 browser integration
│   ├── manifest.json
│   ├── background.js            # Interception & RPC dispatcher
│   ├── popup.html
│   └── icon128.png
├── msix/                        # Microsoft Store AppxManifest & packaging
│   ├── AppxManifest.xml
│   ├── build_msix.py
│   └── Assets/
├── installer/
│   └── RapidDownloadManager.iss # Inno Setup 6 compilation script
├── install_extension.bat        # Windows Registry browser installer
└── run_rapid_gui.bat            # Quick launcher script
```

---

## 🤝 Contributing

Contributions, bug reports, and feature suggestions are always welcome! Feel free to open an issue or submit a pull request on GitHub.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

---

## 📜 License

Distributed under the **MIT License**. See `LICENSE` for more information.

Developed with ❤️ and Rust by [promahbubul](https://github.com/promahbubul).

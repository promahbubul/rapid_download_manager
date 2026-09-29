# Changelog

All notable changes to **Rapid Download Manager** will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [1.0.3] - 2026-09-29

### Added
- **Native Bengali Font Shaping Engine:** Integrated `rustybuzz` and PUA glyph mapping (`kalpurush-pua.ttf`) in `bengali.rs`, delivering flawless complex Bengali text shaping with zero broken ligatures or dotted circles.
- **Ultra-Clean Non-Colliding Status Bar:** Redesigned bottom status bar inspired by VS Code and Arc with mathematically partitioned left status line and right quick action dock (`Downloads`, `Browser`, `About`, `Tray`).
- **Responsive Compact Mode:** Status bar automatically adapts to narrow window widths (< 600px) with icon-only action dock and GPU clipping.
- **Embedded LLVM Runtime:** Bundled `libunwind.dll` directly inside all distribution and MSIX staging packages (`dist/msix_staging`, `dist/RapidDownloadManager`), permanently resolving the `libunwind.dll missing` system error on clean Windows and Microsoft Store installations.

### Fixed
- Fixed UI collisions and text overlapping on high DPI scaling and small screen resolutions.
- Fixed 404 download URLs in documentation and website release links.

---

## [1.0.0] - 2026-09-24

### Initial Production Release

#### Added
- **Core Multi-Segment Download Engine (`rapid-core`):**
  - Accelerated multi-connection HTTP/2 downloads with up to 32 parallel TCP streams.
  - Dynamic stream balancing to eliminate lagging download segments.
  - Multi-stream HLS (.m3u8) video/audio segment downloader and auto-merger.
  - Google Drive large file confirmation and virus-scan bypass engine.
  - YouTube and multi-platform media stream extractor.
- **Enterprise Storage & Database Layer (`storage.rs`):**
  - Resilient atomic write operations via temporary files (`.tmp`) and atomic rename to prevent file corruption during sudden power loss.
  - Dual-layer automatic `.bak` backups for `history.json`, `config.json`, and `scheduler.json`.
  - Robust forward and backward schema migration using Serde default fallback attributes.
- **Windows System Integration & Security Architecture:**
  - Standardized Windows user storage paths: configuration in `%APPDATA%\RapidDownloadManager` and logs/cache in `%LOCALAPPDATA%\RapidDownloadManager`.
  - Zero-leak credential sanitizer: automatic redaction of API keys, tokens, and passwords in diagnostic logs.
  - Three-tier enterprise logging: `app.log`, `error.log`, and symbolized `crash.log`.
  - Embedded Windows PE application manifest with `requestedExecutionLevel="asInvoker"`, PerMonitorV2 high-DPI awareness, and long path support.
- **Modern Cyber-Obsidian GUI (`rapid-gui`):**
  - Ultra-responsive immediate-mode desktop interface built with Eframe and Egui.
  - Real-time parallel connection visualizer with individual stream progress bars.
  - Time-based download scheduler with automatic PC shutdown and sleep timer.
  - Dynamic bandwidth speed limiter (Unlimited, 10 MB/s, 5 MB/s, 1 MB/s, 512 KB/s).
  - Background system tray minimization with active download notifications.
  - In-app live update checker querying the official GitHub Releases API.
- **Rapid CLI Tool (`rapid-cli`):**
  - Lightweight terminal-based download accelerator featuring multi-bar progress gauges.
  - Headless inspection and URL header probe commands.
- **Automated Browser Integration:**
  - One-click native registry registration for Google Chrome, Microsoft Edge, Brave, Opera, and Mozilla Firefox.
  - High-performance background localhost listener on port 9669.
- **Production Packaging & Store Readiness:**
  - Inno Setup Windows installer (`RapidDownloadManager_Setup_v1.0.0.exe`) with `PrivilegesRequired=lowest`, `PrivilegesRequiredOverridesAllowed=dialog`, and clean uninstaller.
  - Standalone portable ZIP package (`RapidDownloadManager_v1.0.0_Portable.zip`).
  - Microsoft Store MSIX package (`RapidDownloadManager_v1.0.0.msix`) with 100% WACK (Windows App Certification Kit) pre-flight audit pass.

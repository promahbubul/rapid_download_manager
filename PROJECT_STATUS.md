# PROJECT HANDOVER & AUDIT REPORT

## 1. Executive Summary
- **Repository**: `rapid_download_manager` (Rust Workspace + Browser Extension)
- **Current Branch**: `main`
- **Compiler Status**: `cargo check --workspace` passing with **0 errors**.
- **Target OS**: Windows (portable across Linux/macOS in architecture).
- **Core Status**: Verified end-to-end with high-speed multi-threaded downloads and Google Drive dynamic multi-part ZIP downloads.

---

## 2. Work Completed (Current Implementation & Features)

### A. Full Google Drive & Google Takeout Support System (Verified Working)
1. **Dynamic ZIP & Google Takeout Interception (`crates/rapid-core`, `extension/background.js`)**:
   - Supports `takeout-download-drive.usercontent.google.com`, `drive.google.com`, `googleusercontent.com`, and `usercontent.google.com`.
   - Comprehensive authentication cookie harvesting across `.google.com`, `google.com`, `accounts.google.com`, `drive.google.com`, and all Google CDN subdomains.
   - Direct HTTP `Cookie` and `Referer` injection in `DownloadWorker::run` ensuring Google never serves `ServiceLogin` challenges.
   - Enforces single-stream safe mode for dynamic zip generation tokens to prevent stream invalidation.
   - Cookie persistence in `.rapid` manifest files (`DownloadTaskState`) so paused/interrupted Google Drive downloads can be resumed without losing session auth.

2. **In-App Google Drive Folder & File Resolver (`crates/rapid-core/src/gdrive.rs`)**:
   - `GDriveResolver::parse_resource_type`: Identifies whether a Google Drive URL is a File or a Folder ID.
   - `GDriveResolver::resolve_file_download_url`: For single file links, queries Google Drive, automatically parses the virus scan warning form (`confirm=t&uuid=...`), and extracts the confirmed streaming download URL, real filename, and size.
   - `GDriveResolver::crawl_folder_default`: For folder links, recursively crawls the folder and subfolders to find all items without requiring browser zipping.
   - `rapid-gui/src/main.rs`: When a folder URL is pasted in `+ Add Download`, it crawls the folder in the background and queues each file individually.

3. **Stream Protection & Anti-Corruption Guard (`crates/rapid-core/src/worker.rs`)**:
   - HTML Content Guard prevents writing HTML challenge/auth pages into archives.
   - Automatically handles Google Drive virus warning redirects.

### B. Chrome / Edge Browser Extension Integration (`extension/`)
- Upgraded to Manifest V3 with non-blocking async interception (`onDeterminingFilename`).
- Chrome Private Network Access (PNA) compliance (`Access-Control-Allow-Private-Network: true` on loopback `127.0.0.1:9669`).
- Safe fallback mechanism: if the desktop app is offline, browser downloads are not cancelled, preventing lost files.
- Live connectivity status indicator in `popup.html`.

### C. Desktop UI (`crates/rapid-gui`)
- Rust + `egui` / `eframe` with dark glassmorphic velvet theme.
- Frameless custom glass velvet title bar (with_decorations(false) + with_taskbar(true)), full Windows taskbar visibility, and smooth drag/caption controls.
- Local background HTTP receiver on `127.0.0.1:9669`.
- Multi-segment live connection visualizer, ETA, speed gauge, and pause/resume controls.

### D. Production Distribution
- Portable standalone binary: `rapid-gui.exe`.
- Inno Setup installer script: `installer/setup.iss`.
- Automated GitHub Actions release workflow: `.github/workflows/release.yml`.

---

## 3. Next Implementation Roadmap (Master Prompt Universal Vision)

- **Phase 0 (Complete)**: Full Audit & Google Drive Dual Support (Folder/File Resolver + Browser Takeout/ZIP Interception).
- **Phase 1**: Universal URL Analyzer & Media Model (`core/url`, `core/media`).
- **Phase 2**: Chrome Native Messaging Host (`rapid-host` binary connecting Chrome/Edge/Firefox).
- **Phase 3**: Generic Media Detection (In-page Video/Audio/Image detector content scripts).
- **Phase 4**: Provider / Adapter System (`MediaProvider` trait for YouTube, Social Media, etc.).
- **Phase 5**: Browser UI Enhancements (Hover download button, media popup scanner).
- **Phase 6**: Persistent Queue & Database Management (SQLite/sled history, reordering).
- **Phase 7**: Production Hardening & Testing.


### C. Selection, Window Design & Original Filename Preservation Updates (September 2026)
1. **Permanent Custom Window Title Bar**:
   - Fixed title bar flickering and resetting back to Windows default bar by configuring `ViewportBuilder::default().with_decorations(false).with_taskbar(true)` in `rapid-gui/src/main.rs`.
   - Rendered a custom dark velvet glassmorphism title bar (`render_custom_title_bar(ctx)`) with drag-to-move, minimize, maximize, and close controls.

2. **Selection & Multi-Delete Synchronization**:
   - Reconciled single-selection (`selected_task_index`) and multi-selection (`selected_tasks` HashSet).
   - Entire row is clickable to select; checkboxes seamlessly toggle multi-selection and update the top status bar.
   - Delete action properly handles all selected/checked downloads simultaneously.

3. **Original Filename & Extension Preservation**:
   - **RFC 5987 / RFC 6266 Precedence**: `Probe::extract_filename_from_cd` now strictly prioritizes `filename*=` (UTF-8 percent-encoded) over `filename=`, handling all quotes, whitespace, and path traversal characters.
   - **Google Drive CDN Probe Fix**: Replaced rejected `HEAD` requests on confirmed Google CDN URLs with `GET` requests using `Range: bytes=0-0`. This guarantees Google CDN returns the real `Content-Disposition` header with the original filename (e.g. `Lecture_01.mp4`) and `Content-Range` with the exact total file size.
   - **Automatic MIME Extension Fallback**: Added `extension_from_mime` in `probe.rs`. If a download URL is dynamic (e.g., `/download/stream?id=...` or `/get.php`) and has no file extension or server content disposition, the real extension is deduced from `Content-Type` (e.g. `video/mp4` -> `.mp4`, `application/pdf` -> `.pdf`).
   - **Generic Placeholder Overrides**: Added `is_generic_placeholder` and `resolve_best_filename` in `engine.rs`. If the browser extension or URL passes generic names like `"download"`, `"download.bin"`, `"uc"`, `"file"`, etc., the engine defers to the real probed filename. If a custom name has no extension, the real probed extension is automatically preserved and attached.
   - **Browser Extension Cleaning**: Updated `content.js` and `background.js` to discard generic `<a download="Download">` attributes and Chrome fallback names so only verified original filenames reach the desktop client.


## 🌟 Latest Completed Milestones (v1.0.5 Release)
1. **Native MEGA.nz Folder Crawler & Decrypted Streaming Engine**:
   - Implemented `crates/rapid-core/src/mega.rs` with full recursive folder hierarchy crawling for public MEGA folder URLs (`https://mega.nz/folder/...#...`).
   - Dynamic tree traversal resolving folders and subfolders, automatically preserving directory structure on disk.
   - On-the-fly AES-128-CTR hardware-accelerated decryption streaming directly from MEGA CDN servers without intermediate temporary file bloat.
   - Byte-accurate HTTP Range resume support on MEGA CDN endpoints.
   - Integrated into GUI: pasting a MEGA folder link in `+ Add Download` instantly discovers all files and enqueues them for parallel accelerated download.
2. **Lock-Safe Redownload & Resume Engine**:
   - Fixed concurrency deadlocks on Redownload/Resume by deferring worker task aborts and file locks outside active Mutex guards.
   - Immediate responsive UI feedback and state re-initialization.
3. **Microsoft Store MSIX Packaging & Metadata Update**:
   - Bumped package version to `1.0.5.0` (`RapidDownloadManager_v1.0.5.msix`).
   - Generated release packages in `dist/installer/` with updated AppxManifest and store listing.

---

## 🌟 Previous Milestones (v1.0.3 - v1.0.4 Release)
1. **Bengali Complex Script Shaping Engine**: Built native HarfBuzz-level text shaping using `rustybuzz 0.20.1` and custom PUA Unicode glyph map in `crates/rapid-gui/src/bengali.rs` + `assets/fonts/kalpurush-pua.ttf`. Unit tested and verified across complex sentences.
2. **Non-Colliding Glassmorphism Status Bar**: Replaced fragile discrete chip layout with mathematically bounded dual-rectangle partition (`left_rect` + `right_rect`), preventing any text collisions on small screens or DPI scaling.
3. **Microsoft Store & Windows Runtime Compatibility**: Bundled `libunwind.dll` into MSIX staging and installer directories, fixing runtime dependency errors on end-user machines.
4. **Documentation & Release Links Synchronization**: Fixed 404 download endpoints, unified latest release links, and synchronized changelogs.

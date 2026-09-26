# Rapid Download Manager — Next.js & TailwindCSS Website Blueprint

**Project:** Rapid Download Manager Official Web Application  
**Target Directory:** `D:\mahbub\project\rapid_download_manager\website`  
**Tech Stack:** Next.js (App Router), React, TailwindCSS, Lucide Icons, TypeScript  
**Theme:** Cyber-Obsidian (matching `rapid-gui` native app) with dynamic Dark, Light & System theme switcher dropdown.

---

## 1. Design System & Theme Alignment
The website will faithfully inherit the visual identity of the native desktop app (`crates/rapid-gui`):

### Color Tokens
* **Dark Mode (Default / Native App Match):**
  - Background Base: `#0B0F19` (Deep Obsidian / Midnight Slate)
  - Surface / Cards: `#111827` / `#1E293B` (Elevated Charcoal with subtle borders `#334155`)
  - Primary Accent: Gradient `#6366F1` (Indigo) to `#8B5CF6` (Violet / Purple)
  - Success / Active: `#10B981` (Emerald Cyber-Green)
  - Text: Main `#F8FAFC`, Muted `#94A3B8`
* **Light Mode:**
  - Background Base: `#F8FAFC` (Clean Modern Slate)
  - Surface / Cards: `#FFFFFF` (Crisp White with delicate glassmorphism `#E2E8F0` borders)
  - Primary Accent: `#4F46E5` / `#7C3AED`
  - Text: Main `#0F172A`, Muted `#64748B`
* **Glassmorphism & Micro-Interactions:**
  - Backdrop blur (`backdrop-blur-md`), glowing borders, and smooth transitions on hover.

---

## 2. Component & Architecture Breakdown

### `components/Navbar.tsx`
* **Brand Logo & Title:** Sleek lightning-bolt badge + "Rapid Download Manager".
* **Nav Links:** Features, Architecture, Benchmarks, Downloads, Extension, Docs.
* **Theme Switcher Dropdown (Custom Requested):**
  - Single icon button displaying active mode (☀️ Sun, 🌙 Moon, 💻 Monitor).
  - Clicking triggers an animated floating dropdown with 3 options:
    1. ☀️ **Light Mode**
    2. 🌙 **Dark Mode**
    3. 💻 **System Default**
  - Instant toggle with `next-themes` / CSS class persistence in `localStorage`.
* **Action Button:** "Download Free" button with Microsoft Store badge icon.

### `components/HeroSection.tsx`
* **Badge:** `v1.0.0 Ready for Windows 10/11`
* **Headline:** "Turbocharged Multi-Segment Download Accelerator for Windows"
* **Subtitle:** "Engineered in 100% memory-safe Rust. Turbo multi-connection engine, automatic browser integration, and intelligent link capture with zero bloatware."
* **CTA Buttons:**
  - "Get from Microsoft Store" (Primary gradient button with Windows logo)
  - "Direct Windows Setup (.exe)" (Secondary glassmorphic button)
  - "View on GitHub" (Outlined button with stars)
* **Interactive Live Simulation / Visual Showcase:**
  - Pixel-perfect recreation of the native GUI window with active animated download streams, segmented chunk progress bar, speed graph, and category sidebar!

### `components/FeaturesGrid.tsx`
* 6 pixel-perfect feature cards with glowing borders:
  1. ⚡ **16x Dynamic Multi-Chunk Acceleration** (Dynamic HTTP range socket balancing)
  2. 🌐 **1-Click Browser Integration** (Instant extension for Chrome, Edge, Brave, Opera, Firefox)
  3. 🛡️ **Zero Tracking, 100% Privacy** (Zero ads, credential redacting engine, 100% local storage)
  4. 🦀 **Memory-Safe Rust Engine** (Ultra-lightweight <25MB RAM, instant startup)
  5. 🎬 **Smart Video & Media Sniffer** (Integrated media stream capture)
  6. 🔄 **Resilient Resume & Crash Guard** (Seamless download continuation after disconnects)

### `components/ComparisonTable.tsx`
* Modern comparison matrix showing:
  - Rapid Download Manager vs IDM vs Free Download Manager vs Browser Built-in
  - Metrics: Download Speed, Memory Footprint, Memory-Safe Rust, Tracking/Ads, Modern UI, Modern TLS/Rustls.

### `components/DownloadCenter.tsx`
* Dedicated download hub with 4 download cards:
  - **Microsoft Store Package** (`.msix` - Auto update)
  - **Standard Installer** (`.exe` - Inno Setup)
  - **Portable Edition** (`.zip` - Zero install)
  - **Browser Extension** (`.zip` / Web Store ready)

### `components/Footer.tsx`
* Documentation links, GitHub repo, Privacy Policy (`privacy.html`), Open Source Licenses (`licenses.html`), and copyright.

---

## 3. Implementation Steps
1. Initialize Next.js project with TailwindCSS and TypeScript inside `website/`.
2. Configure Tailwind theme tokens (colors, animations, fonts, dark mode class).
3. Build atomic UI components (Navbar, ThemeDropdown, Hero, AppPreview, Features, Table, DownloadCards, Footer).
4. Integrate `next-themes` for seamless dark/light/system theme toggling.
5. Export static build (`next build` / `next export`) to allow deployment anywhere, including GitHub Pages.

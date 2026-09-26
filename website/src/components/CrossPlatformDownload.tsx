"use client";

import React, { useState, useEffect } from "react";
import {
  Apple,
  Terminal,
  Download,
  Copy,
  Check,
  Package,
  Layers,
  Sparkles,
  ExternalLink,
  Laptop,
  CheckCircle2,
} from "lucide-react";
import { ChromeIcon, WindowsIcon } from "./Icons";
import { useTheme } from "../context/ThemeContext";

type Platform = "windows" | "macos" | "linux" | "extension" | "cli";

export default function CrossPlatformDownload() {
  const { resolvedTheme } = useTheme();
  const isDark = resolvedTheme === "dark";

  const [detectedOS, setDetectedOS] = useState<Platform>("windows");
  const [selectedPlatform, setSelectedPlatform] = useState<Platform>("windows");
  const [copiedCmd, setCopiedCmd] = useState<string | null>(null);

  useEffect(() => {
    if (typeof window === "undefined") return;
    const ua = window.navigator.userAgent.toLowerCase();
    if (ua.includes("win")) {
      setDetectedOS("windows");
      setSelectedPlatform("windows");
    } else if (ua.includes("mac") || ua.includes("darwin")) {
      setDetectedOS("macos");
      setSelectedPlatform("macos");
    } else if (ua.includes("linux")) {
      setDetectedOS("linux");
      setSelectedPlatform("linux");
    } else {
      setDetectedOS("windows");
      setSelectedPlatform("windows");
    }
  }, []);

  const copyCommand = (cmd: string, id: string) => {
    navigator.clipboard.writeText(cmd);
    setCopiedCmd(id);
    setTimeout(() => setCopiedCmd(null), 2000);
  };

  return (
    <section id="download" className="py-20 border-t"
      style={{
        backgroundColor: isDark ? "rgba(11, 15, 23, 0.5)" : "rgba(248, 250, 252, 0.7)",
        borderColor: "var(--border-subtle)",
      }}
    >
      <div className="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
        {/* Section Header */}
        <div className="text-center max-w-2xl mx-auto mb-10 space-y-3">
          <h2 className="text-3xl sm:text-4xl font-extrabold tracking-tight" style={{ color: "var(--text-heading)" }}>
            Download & Installation
          </h2>
          <p className="text-sm sm:text-base" style={{ color: "var(--text-muted)" }}>
            Choose the recommended binary for your operating system or install via your preferred package manager.
          </p>

          {/* Detected Device Badge */}
          <div className="inline-flex items-center gap-2 px-3.5 py-1.5 rounded-full border text-xs font-medium"
            style={{
              backgroundColor: "var(--bg-card)",
              borderColor: "var(--border-subtle)",
            }}
          >
            <span style={{ color: "var(--text-muted)" }}>Your System:</span>
            <span className="font-semibold text-indigo-400 capitalize flex items-center gap-1.5">
              {detectedOS === "windows" && <WindowsIcon className="w-3.5 h-3.5" />}
              {detectedOS === "macos" && <Apple className="w-3.5 h-3.5" />}
              {detectedOS === "linux" && <Terminal className="w-3.5 h-3.5" />}
              <span>{detectedOS} (Ready to Install)</span>
            </span>
          </div>
        </div>

        {/* Platform Selection Tabs */}
        <div className="flex flex-wrap items-center justify-center gap-2 mb-10">
          {[
            { id: "windows", name: "Windows", icon: <WindowsIcon className="w-4 h-4" /> },
            { id: "macos", name: "macOS", icon: <Apple className="w-4 h-4" /> },
            { id: "linux", name: "Linux", icon: <Terminal className="w-4 h-4" /> },
            { id: "extension", name: "Browser Extension", icon: <ChromeIcon className="w-4 h-4" /> },
            { id: "cli", name: "Terminal CLI", icon: <Terminal className="w-4 h-4" /> },
          ].map((tab) => {
            const isSelected = selectedPlatform === tab.id;
            return (
              <button
                key={tab.id}
                onClick={() => setSelectedPlatform(tab.id as Platform)}
                className={`flex items-center gap-2 px-4 py-2.5 rounded-xl text-xs sm:text-sm font-medium transition-colors cursor-pointer ${
                  isSelected
                    ? "bg-indigo-600 text-white font-semibold shadow-sm"
                    : "border hover:bg-slate-500/10"
                }`}
                style={{
                  borderColor: isSelected ? "transparent" : "var(--border-subtle)",
                  backgroundColor: isSelected ? undefined : "var(--bg-card)",
                  color: isSelected ? "#ffffff" : "var(--text-heading)",
                }}
              >
                {tab.icon}
                <span>{tab.name}</span>
              </button>
            );
          })}
        </div>

        {/* Tab 1: WINDOWS */}
        {selectedPlatform === "windows" && (
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
            {/* Windows 1: MSIX */}
            <div className="p-6 rounded-xl border flex flex-col justify-between"
              style={{ backgroundColor: "var(--bg-card)", borderColor: "var(--border-subtle)" }}
            >
              <div className="space-y-3">
                <div className="flex items-center justify-between">
                  <div className="w-9 h-9 rounded-lg bg-indigo-500/10 flex items-center justify-center text-indigo-400">
                    <WindowsIcon className="w-5 h-5" />
                  </div>
                  <span className="text-[10px] font-bold px-2 py-0.5 rounded-full bg-emerald-500/20 text-emerald-400 border border-emerald-500/30">
                    Recommended
                  </span>
                </div>
                <h3 className="text-base font-bold" style={{ color: "var(--text-heading)" }}>
                  MSIX Package
                </h3>
                <p className="text-xs leading-relaxed" style={{ color: "var(--text-muted)" }}>
                  Official Microsoft Store package. Clean sandbox installation, zero registry clutter, and automated background updates.
                </p>
                <div className="text-[11px] font-mono text-slate-400 pt-1">
                  Size: <span className="text-indigo-400 font-semibold">5.71 MB</span> • 64-bit
                </div>
              </div>
              <div className="pt-6">
                <a
                  href="https://github.com/promahbubul/rapid_download_manager/releases/download/v1.0.0/RapidDownloadManager_1.0.0.0_x64.msix"
                  className="w-full flex items-center justify-center gap-2 px-4 py-2.5 rounded-lg bg-indigo-600 hover:bg-indigo-500 text-white font-semibold text-xs shadow-sm transition-colors"
                >
                  <Download className="w-4 h-4" />
                  <span>Download .MSIX (5.71 MB)</span>
                </a>
              </div>
            </div>

            {/* Windows 2: EXE */}
            <div className="p-6 rounded-xl border flex flex-col justify-between"
              style={{ backgroundColor: "var(--bg-card)", borderColor: "var(--border-subtle)" }}
            >
              <div className="space-y-3">
                <div className="w-9 h-9 rounded-lg bg-purple-500/10 flex items-center justify-center text-purple-400">
                  <Package className="w-5 h-5" />
                </div>
                <h3 className="text-base font-bold" style={{ color: "var(--text-heading)" }}>
                  Setup Installer (.exe)
                </h3>
                <p className="text-xs leading-relaxed" style={{ color: "var(--text-muted)" }}>
                  Standard Windows desktop installer wizard. Creates Start Menu shortcuts and native protocol handlers.
                </p>
                <div className="text-[11px] font-mono text-slate-400 pt-1">
                  Size: <span className="text-indigo-400 font-semibold">5.85 MB</span> • 64-bit
                </div>
              </div>
              <div className="pt-6">
                <a
                  href="https://github.com/promahbubul/rapid_download_manager/releases/download/v1.0.0/RapidDownloadManager-Setup-1.0.0.exe"
                  className="w-full flex items-center justify-center gap-2 px-4 py-2.5 rounded-lg border font-semibold text-xs transition-colors hover:bg-slate-500/10"
                  style={{
                    borderColor: "var(--border-subtle)",
                    backgroundColor: "var(--bg-card)",
                    color: "var(--text-heading)",
                  }}
                >
                  <Download className="w-4 h-4" />
                  <span>Download .EXE Setup</span>
                </a>
              </div>
            </div>

            {/* Windows 3: WinGet */}
            <div className="p-6 rounded-xl border flex flex-col justify-between"
              style={{ backgroundColor: "var(--bg-card)", borderColor: "var(--border-subtle)" }}
            >
              <div className="space-y-3">
                <div className="w-9 h-9 rounded-lg bg-cyan-500/10 flex items-center justify-center text-cyan-400">
                  <Terminal className="w-5 h-5" />
                </div>
                <h3 className="text-base font-bold" style={{ color: "var(--text-heading)" }}>
                  Windows Package Manager
                </h3>
                <p className="text-xs leading-relaxed" style={{ color: "var(--text-muted)" }}>
                  Install directly from PowerShell or Windows Terminal using official Microsoft WinGet repo.
                </p>
                <div className="p-2.5 rounded-lg bg-slate-900 border border-slate-800 font-mono text-[11px] text-indigo-300 overflow-x-auto">
                  <span>winget install promahbubul.RapidDownloadManager</span>
                </div>
              </div>
              <div className="pt-6">
                <button
                  onClick={() => copyCommand("winget install promahbubul.RapidDownloadManager", "winget")}
                  className="w-full flex items-center justify-center gap-2 px-4 py-2.5 rounded-lg bg-slate-800 hover:bg-slate-700 text-white font-semibold text-xs transition-colors cursor-pointer"
                >
                  {copiedCmd === "winget" ? <Check className="w-4 h-4 text-emerald-400" /> : <Copy className="w-4 h-4" />}
                  <span>{copiedCmd === "winget" ? "Copied Command!" : "Copy WinGet Command"}</span>
                </button>
              </div>
            </div>
          </div>
        )}

        {/* Tab 2: MACOS */}
        {selectedPlatform === "macos" && (
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
            {/* macOS Apple Silicon */}
            <div className="p-6 rounded-xl border flex flex-col justify-between"
              style={{ backgroundColor: "var(--bg-card)", borderColor: "var(--border-subtle)" }}
            >
              <div className="space-y-3">
                <div className="w-9 h-9 rounded-lg bg-indigo-500/10 flex items-center justify-center text-indigo-400">
                  <Apple className="w-5 h-5" />
                </div>
                <h3 className="text-base font-bold" style={{ color: "var(--text-heading)" }}>
                  Apple Silicon DMG
                </h3>
                <p className="text-xs leading-relaxed" style={{ color: "var(--text-muted)" }}>
                  Tailored native ARM64 build for M1, M2, M3, and M4 Macs. Excellent energy efficiency.
                </p>
                <div className="text-[11px] font-mono text-slate-400 pt-1">
                  Size: 6.2 MB • macOS 12 Monterey+
                </div>
              </div>
              <div className="pt-6">
                <a
                  href="https://github.com/promahbubul/rapid_download_manager/releases/download/v1.0.0/RapidDownloadManager-arm64.dmg"
                  className="w-full flex items-center justify-center gap-2 px-4 py-2.5 rounded-lg bg-indigo-600 hover:bg-indigo-500 text-white font-semibold text-xs shadow-sm transition-colors"
                >
                  <Download className="w-4 h-4" />
                  <span>Download .DMG (Apple Silicon)</span>
                </a>
              </div>
            </div>

            {/* macOS Intel */}
            <div className="p-6 rounded-xl border flex flex-col justify-between"
              style={{ backgroundColor: "var(--bg-card)", borderColor: "var(--border-subtle)" }}
            >
              <div className="space-y-3">
                <div className="w-9 h-9 rounded-lg bg-slate-500/10 flex items-center justify-center text-slate-400">
                  <Apple className="w-5 h-5" />
                </div>
                <h3 className="text-base font-bold" style={{ color: "var(--text-heading)" }}>
                  Intel x86_64 DMG
                </h3>
                <p className="text-xs leading-relaxed" style={{ color: "var(--text-muted)" }}>
                  Optimized for Intel-based MacBook Pro, iMac, and Mac mini hardware.
                </p>
                <div className="text-[11px] font-mono text-slate-400 pt-1">
                  Size: 6.8 MB • macOS 11 Big Sur+
                </div>
              </div>
              <div className="pt-6">
                <a
                  href="https://github.com/promahbubul/rapid_download_manager/releases/download/v1.0.0/RapidDownloadManager-x64.dmg"
                  className="w-full flex items-center justify-center gap-2 px-4 py-2.5 rounded-lg border font-semibold text-xs transition-colors hover:bg-slate-500/10"
                  style={{
                    borderColor: "var(--border-subtle)",
                    backgroundColor: "var(--bg-card)",
                    color: "var(--text-heading)",
                  }}
                >
                  <Download className="w-4 h-4" />
                  <span>Download .DMG (Intel)</span>
                </a>
              </div>
            </div>

            {/* Homebrew */}
            <div className="p-6 rounded-xl border flex flex-col justify-between"
              style={{ backgroundColor: "var(--bg-card)", borderColor: "var(--border-subtle)" }}
            >
              <div className="space-y-3">
                <div className="w-9 h-9 rounded-lg bg-amber-500/10 flex items-center justify-center text-amber-400">
                  <Terminal className="w-5 h-5" />
                </div>
                <h3 className="text-base font-bold" style={{ color: "var(--text-heading)" }}>
                  Homebrew Cask
                </h3>
                <p className="text-xs leading-relaxed" style={{ color: "var(--text-muted)" }}>
                  Install and update smoothly using standard macOS command line tools.
                </p>
                <div className="p-2.5 rounded-lg bg-slate-900 border border-slate-800 font-mono text-[11px] text-amber-300 overflow-x-auto">
                  <span>brew install --cask rapid-download-manager</span>
                </div>
              </div>
              <div className="pt-6">
                <button
                  onClick={() => copyCommand("brew install --cask rapid-download-manager", "brew")}
                  className="w-full flex items-center justify-center gap-2 px-4 py-2.5 rounded-lg bg-slate-800 hover:bg-slate-700 text-white font-semibold text-xs transition-colors cursor-pointer"
                >
                  {copiedCmd === "brew" ? <Check className="w-4 h-4 text-emerald-400" /> : <Copy className="w-4 h-4" />}
                  <span>{copiedCmd === "brew" ? "Copied Brew Command!" : "Copy Brew Command"}</span>
                </button>
              </div>
            </div>
          </div>
        )}

        {/* Tab 3: LINUX */}
        {selectedPlatform === "linux" && (
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
            {/* Linux 1: .deb */}
            <div className="p-6 rounded-xl border flex flex-col justify-between"
              style={{ backgroundColor: "var(--bg-card)", borderColor: "var(--border-subtle)" }}
            >
              <div className="space-y-3">
                <div className="w-9 h-9 rounded-lg bg-rose-500/10 flex items-center justify-center text-rose-400">
                  <Package className="w-5 h-5" />
                </div>
                <h3 className="text-base font-bold" style={{ color: "var(--text-heading)" }}>
                  Debian / Ubuntu (.deb)
                </h3>
                <p className="text-xs leading-relaxed" style={{ color: "var(--text-muted)" }}>
                  Native DEB package for Ubuntu, Debian, Pop!_OS, and Linux Mint.
                </p>
                <div className="p-2.5 rounded-lg bg-slate-900 border border-slate-800 font-mono text-[11px] text-rose-300 overflow-x-auto">
                  <span>sudo dpkg -i rdm_1.0.0_amd64.deb</span>
                </div>
              </div>
              <div className="pt-6">
                <a
                  href="https://github.com/promahbubul/rapid_download_manager/releases/download/v1.0.0/rapid-download-manager_1.0.0_amd64.deb"
                  className="w-full flex items-center justify-center gap-2 px-4 py-2.5 rounded-lg bg-indigo-600 hover:bg-indigo-500 text-white font-semibold text-xs shadow-sm transition-colors"
                >
                  <Download className="w-4 h-4" />
                  <span>Download .DEB (5.1 MB)</span>
                </a>
              </div>
            </div>

            {/* Linux 2: AppImage */}
            <div className="p-6 rounded-xl border flex flex-col justify-between"
              style={{ backgroundColor: "var(--bg-card)", borderColor: "var(--border-subtle)" }}
            >
              <div className="space-y-3">
                <div className="w-9 h-9 rounded-lg bg-emerald-500/10 flex items-center justify-center text-emerald-400">
                  <Layers className="w-5 h-5" />
                </div>
                <h3 className="text-base font-bold" style={{ color: "var(--text-heading)" }}>
                  Universal AppImage
                </h3>
                <p className="text-xs leading-relaxed" style={{ color: "var(--text-muted)" }}>
                  Single standalone binary that runs on any modern Linux distribution without installation.
                </p>
                <div className="p-2.5 rounded-lg bg-slate-900 border border-slate-800 font-mono text-[11px] text-emerald-300 overflow-x-auto">
                  <span>chmod +x RapidDownloadManager.AppImage</span>
                </div>
              </div>
              <div className="pt-6">
                <a
                  href="https://github.com/promahbubul/rapid_download_manager/releases/download/v1.0.0/RapidDownloadManager-x86_64.AppImage"
                  className="w-full flex items-center justify-center gap-2 px-4 py-2.5 rounded-lg border font-semibold text-xs transition-colors hover:bg-slate-500/10"
                  style={{
                    borderColor: "var(--border-subtle)",
                    backgroundColor: "var(--bg-card)",
                    color: "var(--text-heading)",
                  }}
                >
                  <Download className="w-4 h-4" />
                  <span>Download .AppImage</span>
                </a>
              </div>
            </div>

            {/* Linux 3: Cargo & AUR */}
            <div className="p-6 rounded-xl border flex flex-col justify-between"
              style={{ backgroundColor: "var(--bg-card)", borderColor: "var(--border-subtle)" }}
            >
              <div className="space-y-3">
                <div className="w-9 h-9 rounded-lg bg-cyan-500/10 flex items-center justify-center text-cyan-400">
                  <Terminal className="w-5 h-5" />
                </div>
                <h3 className="text-base font-bold" style={{ color: "var(--text-heading)" }}>
                  Rust Cargo & Arch AUR
                </h3>
                <p className="text-xs leading-relaxed" style={{ color: "var(--text-muted)" }}>
                  Compile with native CPU optimization from crates.io or install with pacman/yay.
                </p>
                <div className="p-2.5 rounded-lg bg-slate-900 border border-slate-800 font-mono text-[11px] text-cyan-300 overflow-x-auto">
                  <span>cargo install rdm-cli --locked</span>
                </div>
              </div>
              <div className="pt-6">
                <button
                  onClick={() => copyCommand("cargo install rdm-cli --locked", "cargo")}
                  className="w-full flex items-center justify-center gap-2 px-4 py-2.5 rounded-lg bg-slate-800 hover:bg-slate-700 text-white font-semibold text-xs transition-colors cursor-pointer"
                >
                  {copiedCmd === "cargo" ? <Check className="w-4 h-4 text-emerald-400" /> : <Copy className="w-4 h-4" />}
                  <span>{copiedCmd === "cargo" ? "Copied Cargo Command!" : "Copy Cargo Command"}</span>
                </button>
              </div>
            </div>
          </div>
        )}

        {/* Tab 4: EXTENSIONS */}
        {selectedPlatform === "extension" && (
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6 max-w-3xl mx-auto">
            <div className="p-6 rounded-xl border flex flex-col justify-between"
              style={{ backgroundColor: "var(--bg-card)", borderColor: "var(--border-subtle)" }}
            >
              <div className="space-y-3">
                <div className="w-9 h-9 rounded-lg bg-indigo-500/10 flex items-center justify-center text-indigo-400">
                  <ChromeIcon className="w-5 h-5" />
                </div>
                <h3 className="text-base font-bold" style={{ color: "var(--text-heading)" }}>
                  Chrome, Brave & Edge
                </h3>
                <p className="text-xs leading-relaxed" style={{ color: "var(--text-muted)" }}>
                  Manifest V3 compliant extension with background service worker and instant context-menu download integration.
                </p>
              </div>
              <div className="pt-6">
                <a
                  href="/extension"
                  className="w-full flex items-center justify-center gap-2 px-4 py-2.5 rounded-lg bg-indigo-600 hover:bg-indigo-500 text-white font-semibold text-xs shadow-sm transition-colors"
                >
                  <ExternalLink className="w-4 h-4" />
                  <span>Extension Setup & Installation</span>
                </a>
              </div>
            </div>

            <div className="p-6 rounded-xl border flex flex-col justify-between"
              style={{ backgroundColor: "var(--bg-card)", borderColor: "var(--border-subtle)" }}
            >
              <div className="space-y-3">
                <div className="w-9 h-9 rounded-lg bg-orange-500/10 flex items-center justify-center text-orange-400">
                  <Sparkles className="w-5 h-5" />
                </div>
                <h3 className="text-base font-bold" style={{ color: "var(--text-heading)" }}>
                  Mozilla Firefox Add-on
                </h3>
                <p className="text-xs leading-relaxed" style={{ color: "var(--text-muted)" }}>
                  Full support for Mozilla Firefox with secure native messaging and automatic link interception.
                </p>
              </div>
              <div className="pt-6">
                <a
                  href="/extension"
                  className="w-full flex items-center justify-center gap-2 px-4 py-2.5 rounded-lg border font-semibold text-xs transition-colors hover:bg-slate-500/10"
                  style={{
                    borderColor: "var(--border-subtle)",
                    backgroundColor: "var(--bg-card)",
                    color: "var(--text-heading)",
                  }}
                >
                  <ExternalLink className="w-4 h-4" />
                  <span>Firefox Installation</span>
                </a>
              </div>
            </div>
          </div>
        )}

        {/* Tab 5: CLI */}
        {selectedPlatform === "cli" && (
          <div className="max-w-2xl mx-auto p-6 rounded-xl border space-y-4"
            style={{ backgroundColor: "var(--bg-card)", borderColor: "var(--border-subtle)" }}
          >
            <div className="flex items-center gap-3">
              <div className="w-9 h-9 rounded-lg bg-indigo-500/10 flex items-center justify-center text-indigo-400">
                <Terminal className="w-5 h-5" />
              </div>
              <div>
                <h3 className="text-base font-bold" style={{ color: "var(--text-heading)" }}>
                  Terminal CLI & Automation
                </h3>
                <p className="text-xs" style={{ color: "var(--text-muted)" }}>
                  Headless daemon for bash scripts, remote servers, and NAS devices.
                </p>
              </div>
            </div>

            <div className="space-y-1.5">
              <div className="text-xs font-semibold text-slate-400">1-Line Shell Installer:</div>
              <div className="p-3 rounded-lg bg-slate-900 border border-slate-800 font-mono text-xs text-indigo-400 flex items-center justify-between">
                <span>curl -sSL https://rdm.app/install.sh | bash</span>
                <button
                  onClick={() => copyCommand("curl -sSL https://rdm.app/install.sh | bash", "cli-curl")}
                  className="p-1 rounded hover:bg-slate-800 text-slate-400 hover:text-white"
                >
                  {copiedCmd === "cli-curl" ? <Check className="w-3.5 h-3.5 text-emerald-400" /> : <Copy className="w-3.5 h-3.5" />}
                </button>
              </div>
            </div>
          </div>
        )}
      </div>
    </section>
  );
}

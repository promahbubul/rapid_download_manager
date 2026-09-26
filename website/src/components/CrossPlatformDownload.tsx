"use client";

import React, { useState, useEffect } from "react";
import {
  Apple,
  Terminal,
  Download,
  Copy,
  Check,
  Package,
  Sparkles,
  ExternalLink,
  Laptop,
  CheckCircle2,
  FileCode,
  Shield,
  Layers,
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
    <section id="download" className="py-24 border-t relative overflow-hidden"
      style={{
        backgroundColor: isDark ? "rgba(11, 15, 25, 0.6)" : "rgba(248, 250, 252, 0.7)",
        borderColor: "var(--border-subtle)",
      }}
    >
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        {/* Section Header */}
        <div className="text-center max-w-3xl mx-auto mb-12 space-y-4">
          <div className="inline-flex items-center gap-2 px-3.5 py-1.5 rounded-full text-xs font-semibold text-indigo-400 bg-indigo-500/10 border border-indigo-500/20">
            <Sparkles className="w-3.5 h-3.5" />
            <span>Universal Cross-Platform Hub • v1.0.0 GA</span>
          </div>

          <h2 className="text-3xl sm:text-5xl font-extrabold tracking-tight" style={{ color: "var(--text-heading)" }}>
            Download for Any Device & Platform
          </h2>

          <p className="text-sm sm:text-base leading-relaxed" style={{ color: "var(--text-muted)" }}>
            Rapid Download Manager is built for modern power users. We detected your current device, but you can freely switch and download tailored binaries for any operating system below.
          </p>

          {/* Detected Device Smart Pill */}
          <div
            className="inline-flex items-center gap-2.5 px-4 py-2 rounded-2xl border backdrop-blur-md shadow-sm text-xs font-medium"
            style={{
              backgroundColor: "var(--bg-card)",
              borderColor: "var(--border-subtle)",
            }}
          >
            <span style={{ color: "var(--text-muted)" }}>Detected Operating System:</span>
            <span className="font-bold text-indigo-400 uppercase tracking-wide flex items-center gap-1.5">
              {detectedOS === "windows" && <WindowsIcon className="w-3.5 h-3.5" />}
              {detectedOS === "macos" && <Apple className="w-3.5 h-3.5" />}
              {detectedOS === "linux" && <Terminal className="w-3.5 h-3.5" />}
              <span>{detectedOS} (Ready for Instant Install)</span>
            </span>
          </div>
        </div>

        {/* Platform Selection Tabs */}
        <div className="flex flex-wrap items-center justify-center gap-2 sm:gap-3 mb-12">
          {[
            { id: "windows", name: "Windows", icon: <WindowsIcon className="w-4 h-4" />, badge: "Native GA" },
            { id: "macos", name: "macOS", icon: <Apple className="w-4 h-4" />, badge: "CLI / DMG" },
            { id: "linux", name: "Linux", icon: <Terminal className="w-4 h-4" />, badge: ".deb / AppImage" },
            { id: "extension", name: "Browser Extension", icon: <ChromeIcon className="w-4 h-4" />, badge: "Manifest V3" },
            { id: "cli", name: "Terminal CLI", icon: <Terminal className="w-4 h-4" />, badge: "Automation" },
          ].map((tab) => {
            const isSelected = selectedPlatform === tab.id;
            return (
              <button
                key={tab.id}
                onClick={() => setSelectedPlatform(tab.id as Platform)}
                className={`flex items-center gap-2 px-4 sm:px-5 py-2.5 rounded-2xl text-xs sm:text-sm font-semibold transition-all duration-200 cursor-pointer ${
                  isSelected
                    ? "bg-gradient-to-r from-indigo-600 to-violet-600 text-white shadow-lg shadow-indigo-500/25 scale-103"
                    : "border hover:scale-101"
                }`}
                style={{
                  borderColor: isSelected ? "transparent" : "var(--border-subtle)",
                  backgroundColor: isSelected ? undefined : "var(--bg-card)",
                  color: isSelected ? "#ffffff" : "var(--text-heading)",
                }}
              >
                {tab.icon}
                <span>{tab.name}</span>
                <span
                  className={`text-[10px] px-1.5 py-0.5 rounded-full font-mono ${
                    isSelected ? "bg-white/20 text-white" : "bg-slate-500/10 text-slate-400"
                  }`}
                >
                  {tab.badge}
                </span>
              </button>
            );
          })}
        </div>

        {/* Tab Content 1: WINDOWS */}
        {selectedPlatform === "windows" && (
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6 animate-fade-in">
            {/* Windows 1: MSIX Store Package */}
            <div className="p-6 rounded-2xl border relative flex flex-col justify-between"
              style={{ backgroundColor: "var(--bg-card)", borderColor: "var(--border-subtle)" }}
            >
              <div className="absolute top-4 right-4">
                <span className="text-[10px] font-bold px-2 py-0.5 rounded-full bg-emerald-500/20 text-emerald-400 border border-emerald-500/30">
                  Recommended
                </span>
              </div>
              <div className="space-y-4">
                <div className="w-10 h-10 rounded-xl bg-indigo-500/10 flex items-center justify-center text-indigo-400">
                  <WindowsIcon className="w-6 h-6" />
                </div>
                <div>
                  <h3 className="text-lg font-bold" style={{ color: "var(--text-heading)" }}>
                    MSIX Certified Package
                  </h3>
                  <p className="text-xs mt-1" style={{ color: "var(--text-muted)" }}>
                    Windows 10/11 64-bit AppX modern package. Zero registry clutter, isolated container sandbox, and automatic background updates.
                  </p>
                </div>
                <div className="text-[11px] font-mono space-y-1 text-slate-400">
                  <div>Size: <span className="text-indigo-400">5.71 MB</span></div>
                  <div>Arch: <span className="text-emerald-400">x64 (Intel & AMD)</span></div>
                  <div>SHA-256: <span className="truncate block font-mono text-[9px] text-slate-500">e2b7a9...Verified</span></div>
                </div>
              </div>
              <div className="pt-6">
                <a
                  href="https://github.com/promahbubul/rapid_download_manager/releases/download/v1.0.0/RapidDownloadManager_1.0.0.0_x64.msix"
                  className="w-full flex items-center justify-center gap-2 px-4 py-3 rounded-xl bg-indigo-600 hover:bg-indigo-500 text-white font-semibold text-xs shadow-md transition-all hover:scale-102"
                >
                  <Download className="w-4 h-4" />
                  <span>Download .MSIX (5.71 MB)</span>
                </a>
              </div>
            </div>

            {/* Windows 2: Standalone .EXE Installer */}
            <div className="p-6 rounded-2xl border flex flex-col justify-between"
              style={{ backgroundColor: "var(--bg-card)", borderColor: "var(--border-subtle)" }}
            >
              <div className="space-y-4">
                <div className="w-10 h-10 rounded-xl bg-purple-500/10 flex items-center justify-center text-purple-400">
                  <Package className="w-6 h-6" />
                </div>
                <div>
                  <h3 className="text-lg font-bold" style={{ color: "var(--text-heading)" }}>
                    Standalone Installer (.exe)
                  </h3>
                  <p className="text-xs mt-1" style={{ color: "var(--text-muted)" }}>
                    Standard Windows desktop installer wizard. Creates start menu shortcuts, desktop icons, and browser registry protocol handlers.
                  </p>
                </div>
                <div className="text-[11px] font-mono space-y-1 text-slate-400">
                  <div>Size: <span className="text-indigo-400">5.85 MB</span></div>
                  <div>Arch: <span className="text-emerald-400">x64 / x86 compatible</span></div>
                  <div>Target: <span className="text-slate-300">Windows 10, 11, Server</span></div>
                </div>
              </div>
              <div className="pt-6">
                <a
                  href="https://github.com/promahbubul/rapid_download_manager/releases/download/v1.0.0/RapidDownloadManager-Setup-1.0.0.exe"
                  className="w-full flex items-center justify-center gap-2 px-4 py-3 rounded-xl border font-semibold text-xs transition-all hover:scale-102"
                  style={{
                    borderColor: "var(--border-subtle)",
                    backgroundColor: "var(--bg-card-hover)",
                    color: "var(--text-heading)",
                  }}
                >
                  <Download className="w-4 h-4" />
                  <span>Download .EXE Setup</span>
                </a>
              </div>
            </div>

            {/* Windows 3: WinGet Package Manager */}
            <div className="p-6 rounded-2xl border flex flex-col justify-between"
              style={{ backgroundColor: "var(--bg-card)", borderColor: "var(--border-subtle)" }}
            >
              <div className="space-y-4">
                <div className="w-10 h-10 rounded-xl bg-cyan-500/10 flex items-center justify-center text-cyan-400">
                  <Terminal className="w-6 h-6" />
                </div>
                <div>
                  <h3 className="text-lg font-bold" style={{ color: "var(--text-heading)" }}>
                    Windows Package Manager
                  </h3>
                  <p className="text-xs mt-1" style={{ color: "var(--text-muted)" }}>
                    Install instantly from PowerShell or Windows Terminal using Microsoft official WinGet repository.
                  </p>
                </div>
                <div className="p-3 rounded-xl bg-slate-900 border border-slate-800 font-mono text-xs text-indigo-300 overflow-x-auto flex items-center justify-between">
                  <span>winget install promahbubul.RapidDownloadManager</span>
                </div>
              </div>
              <div className="pt-6">
                <button
                  onClick={() => copyCommand("winget install promahbubul.RapidDownloadManager", "winget")}
                  className="w-full flex items-center justify-center gap-2 px-4 py-3 rounded-xl bg-slate-800 hover:bg-slate-700 text-white font-semibold text-xs shadow-md transition-all cursor-pointer"
                >
                  {copiedCmd === "winget" ? <Check className="w-4 h-4 text-emerald-400" /> : <Copy className="w-4 h-4" />}
                  <span>{copiedCmd === "winget" ? "Copied WinGet Command!" : "Copy WinGet Command"}</span>
                </button>
              </div>
            </div>
          </div>
        )}

        {/* Tab Content 2: MACOS */}
        {selectedPlatform === "macos" && (
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6 animate-fade-in">
            {/* macOS Apple Silicon */}
            <div className="p-6 rounded-2xl border flex flex-col justify-between"
              style={{ backgroundColor: "var(--bg-card)", borderColor: "var(--border-subtle)" }}
            >
              <div className="space-y-4">
                <div className="w-10 h-10 rounded-xl bg-indigo-500/10 flex items-center justify-center text-indigo-400">
                  <Apple className="w-6 h-6" />
                </div>
                <div>
                  <h3 className="text-lg font-bold" style={{ color: "var(--text-heading)" }}>
                    Apple Silicon DMG
                  </h3>
                  <p className="text-xs mt-1" style={{ color: "var(--text-muted)" }}>
                    Tailored ARM64 build for M1, M2, M3, and M4 Macs. Exceptional energy efficiency and native hardware acceleration.
                  </p>
                </div>
                <div className="text-[11px] font-mono space-y-1 text-slate-400">
                  <div>Size: <span className="text-indigo-400">6.2 MB</span></div>
                  <div>Arch: <span className="text-emerald-400">aarch64 (Apple Silicon)</span></div>
                  <div>macOS: <span className="text-slate-300">macOS 12 Monterey+</span></div>
                </div>
              </div>
              <div className="pt-6">
                <a
                  href="https://github.com/promahbubul/rapid_download_manager/releases/download/v1.0.0/RapidDownloadManager-arm64.dmg"
                  className="w-full flex items-center justify-center gap-2 px-4 py-3 rounded-xl bg-indigo-600 hover:bg-indigo-500 text-white font-semibold text-xs shadow-md transition-all hover:scale-102"
                >
                  <Download className="w-4 h-4" />
                  <span>Download .DMG (ARM64)</span>
                </a>
              </div>
            </div>

            {/* macOS Intel */}
            <div className="p-6 rounded-2xl border flex flex-col justify-between"
              style={{ backgroundColor: "var(--bg-card)", borderColor: "var(--border-subtle)" }}
            >
              <div className="space-y-4">
                <div className="w-10 h-10 rounded-xl bg-slate-500/10 flex items-center justify-center text-slate-400">
                  <Apple className="w-6 h-6" />
                </div>
                <div>
                  <h3 className="text-lg font-bold" style={{ color: "var(--text-heading)" }}>
                    Intel x86_64 DMG
                  </h3>
                  <p className="text-xs mt-1" style={{ color: "var(--text-muted)" }}>
                    Legacy Intel Core Mac build. High-performance multi-chunk acceleration for Intel-based MacBook Pro and iMac models.
                  </p>
                </div>
                <div className="text-[11px] font-mono space-y-1 text-slate-400">
                  <div>Size: <span className="text-indigo-400">6.8 MB</span></div>
                  <div>Arch: <span className="text-emerald-400">x86_64 (Intel)</span></div>
                  <div>macOS: <span className="text-slate-300">macOS 11 Big Sur+</span></div>
                </div>
              </div>
              <div className="pt-6">
                <a
                  href="https://github.com/promahbubul/rapid_download_manager/releases/download/v1.0.0/RapidDownloadManager-x64.dmg"
                  className="w-full flex items-center justify-center gap-2 px-4 py-3 rounded-xl border font-semibold text-xs transition-all hover:scale-102"
                  style={{
                    borderColor: "var(--border-subtle)",
                    backgroundColor: "var(--bg-card-hover)",
                    color: "var(--text-heading)",
                  }}
                >
                  <Download className="w-4 h-4" />
                  <span>Download .DMG (Intel)</span>
                </a>
              </div>
            </div>

            {/* Homebrew */}
            <div className="p-6 rounded-2xl border flex flex-col justify-between"
              style={{ backgroundColor: "var(--bg-card)", borderColor: "var(--border-subtle)" }}
            >
              <div className="space-y-4">
                <div className="w-10 h-10 rounded-xl bg-amber-500/10 flex items-center justify-center text-amber-400">
                  <Terminal className="w-6 h-6" />
                </div>
                <div>
                  <h3 className="text-lg font-bold" style={{ color: "var(--text-heading)" }}>
                    Homebrew Cask
                  </h3>
                  <p className="text-xs mt-1" style={{ color: "var(--text-muted)" }}>
                    Install and manage updates effortlessly through macOS Homebrew command-line environment.
                  </p>
                </div>
                <div className="p-3 rounded-xl bg-slate-900 border border-slate-800 font-mono text-xs text-amber-300 overflow-x-auto">
                  <span>brew install --cask rapid-download-manager</span>
                </div>
              </div>
              <div className="pt-6">
                <button
                  onClick={() => copyCommand("brew install --cask rapid-download-manager", "brew")}
                  className="w-full flex items-center justify-center gap-2 px-4 py-3 rounded-xl bg-slate-800 hover:bg-slate-700 text-white font-semibold text-xs shadow-md transition-all cursor-pointer"
                >
                  {copiedCmd === "brew" ? <Check className="w-4 h-4 text-emerald-400" /> : <Copy className="w-4 h-4" />}
                  <span>{copiedCmd === "brew" ? "Copied Brew Command!" : "Copy Brew Command"}</span>
                </button>
              </div>
            </div>
          </div>
        )}

        {/* Tab Content 3: LINUX */}
        {selectedPlatform === "linux" && (
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6 animate-fade-in">
            {/* Linux 1: .deb */}
            <div className="p-6 rounded-2xl border flex flex-col justify-between"
              style={{ backgroundColor: "var(--bg-card)", borderColor: "var(--border-subtle)" }}
            >
              <div className="space-y-4">
                <div className="w-10 h-10 rounded-xl bg-rose-500/10 flex items-center justify-center text-rose-400">
                  <Package className="w-6 h-6" />
                </div>
                <div>
                  <h3 className="text-lg font-bold" style={{ color: "var(--text-heading)" }}>
                    Debian / Ubuntu (.deb)
                  </h3>
                  <p className="text-xs mt-1" style={{ color: "var(--text-muted)" }}>
                    Native deb package for Ubuntu, Debian, Linux Mint, and Pop!_OS. Includes desktop integration and apt updates.
                  </p>
                </div>
                <div className="p-3 rounded-xl bg-slate-900 border border-slate-800 font-mono text-xs text-rose-300 overflow-x-auto">
                  <span>sudo dpkg -i rdm_1.0.0_amd64.deb</span>
                </div>
              </div>
              <div className="pt-6">
                <a
                  href="https://github.com/promahbubul/rapid_download_manager/releases/download/v1.0.0/rapid-download-manager_1.0.0_amd64.deb"
                  className="w-full flex items-center justify-center gap-2 px-4 py-3 rounded-xl bg-indigo-600 hover:bg-indigo-500 text-white font-semibold text-xs shadow-md transition-all hover:scale-102"
                >
                  <Download className="w-4 h-4" />
                  <span>Download .DEB (5.1 MB)</span>
                </a>
              </div>
            </div>

            {/* Linux 2: AppImage */}
            <div className="p-6 rounded-2xl border flex flex-col justify-between"
              style={{ backgroundColor: "var(--bg-card)", borderColor: "var(--border-subtle)" }}
            >
              <div className="space-y-4">
                <div className="w-10 h-10 rounded-xl bg-emerald-500/10 flex items-center justify-center text-emerald-400">
                  <Layers className="w-6 h-6" />
                </div>
                <div>
                  <h3 className="text-lg font-bold" style={{ color: "var(--text-heading)" }}>
                    Universal AppImage
                  </h3>
                  <p className="text-xs mt-1" style={{ color: "var(--text-muted)" }}>
                    Single executable that runs on any modern Linux distribution without installation or root permissions.
                  </p>
                </div>
                <div className="p-3 rounded-xl bg-slate-900 border border-slate-800 font-mono text-xs text-emerald-300 overflow-x-auto">
                  <span>chmod +x RapidDownloadManager.AppImage</span>
                </div>
              </div>
              <div className="pt-6">
                <a
                  href="https://github.com/promahbubul/rapid_download_manager/releases/download/v1.0.0/RapidDownloadManager-x86_64.AppImage"
                  className="w-full flex items-center justify-center gap-2 px-4 py-3 rounded-xl border font-semibold text-xs transition-all hover:scale-102"
                  style={{
                    borderColor: "var(--border-subtle)",
                    backgroundColor: "var(--bg-card-hover)",
                    color: "var(--text-heading)",
                  }}
                >
                  <Download className="w-4 h-4" />
                  <span>Download .AppImage</span>
                </a>
              </div>
            </div>

            {/* Linux 3: Cargo / Arch AUR */}
            <div className="p-6 rounded-2xl border flex flex-col justify-between"
              style={{ backgroundColor: "var(--bg-card)", borderColor: "var(--border-subtle)" }}
            >
              <div className="space-y-4">
                <div className="w-10 h-10 rounded-xl bg-cyan-500/10 flex items-center justify-center text-cyan-400">
                  <Terminal className="w-6 h-6" />
                </div>
                <div>
                  <h3 className="text-lg font-bold" style={{ color: "var(--text-heading)" }}>
                    Rust Cargo & Arch AUR
                  </h3>
                  <p className="text-xs mt-1" style={{ color: "var(--text-muted)" }}>
                    Compile directly from crates.io with native CPU optimizations or install via Arch User Repository.
                  </p>
                </div>
                <div className="p-3 rounded-xl bg-slate-900 border border-slate-800 font-mono text-xs text-cyan-300 overflow-x-auto">
                  <span>cargo install rdm-cli --locked</span>
                </div>
              </div>
              <div className="pt-6">
                <button
                  onClick={() => copyCommand("cargo install rdm-cli --locked", "cargo")}
                  className="w-full flex items-center justify-center gap-2 px-4 py-3 rounded-xl bg-slate-800 hover:bg-slate-700 text-white font-semibold text-xs shadow-md transition-all cursor-pointer"
                >
                  {copiedCmd === "cargo" ? <Check className="w-4 h-4 text-emerald-400" /> : <Copy className="w-4 h-4" />}
                  <span>{copiedCmd === "cargo" ? "Copied Cargo Command!" : "Copy Cargo Command"}</span>
                </button>
              </div>
            </div>
          </div>
        )}

        {/* Tab Content 4: EXTENSIONS */}
        {selectedPlatform === "extension" && (
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6 max-w-4xl mx-auto animate-fade-in">
            {/* Chrome / Edge / Brave */}
            <div className="p-6 rounded-2xl border flex flex-col justify-between"
              style={{ backgroundColor: "var(--bg-card)", borderColor: "var(--border-subtle)" }}
            >
              <div className="space-y-4">
                <div className="w-10 h-10 rounded-xl bg-indigo-500/10 flex items-center justify-center text-indigo-400">
                  <ChromeIcon className="w-6 h-6" />
                </div>
                <div>
                  <h3 className="text-lg font-bold" style={{ color: "var(--text-heading)" }}>
                    Chrome, Brave & Edge Extension
                  </h3>
                  <p className="text-xs mt-1" style={{ color: "var(--text-muted)" }}>
                    Manifest V3 compliant extension with background service worker and native messaging host bridge.
                  </p>
                </div>
                <ul className="text-xs space-y-2 text-slate-400">
                  <li className="flex items-center gap-2">
                    <CheckCircle2 className="w-4 h-4 text-emerald-400" />
                    <span>Automatic download interception</span>
                  </li>
                  <li className="flex items-center gap-2">
                    <CheckCircle2 className="w-4 h-4 text-emerald-400" />
                    <span>Right-click context menu "Download with RDM"</span>
                  </li>
                  <li className="flex items-center gap-2">
                    <CheckCircle2 className="w-4 h-4 text-emerald-400" />
                    <span>Session cookie & referrer passthrough</span>
                  </li>
                </ul>
              </div>
              <div className="pt-6">
                <a
                  href="/extension"
                  className="w-full flex items-center justify-center gap-2 px-4 py-3 rounded-xl bg-indigo-600 hover:bg-indigo-500 text-white font-semibold text-xs shadow-md transition-all hover:scale-102"
                >
                  <ExternalLink className="w-4 h-4" />
                  <span>Extension Setup & Install Guide</span>
                </a>
              </div>
            </div>

            {/* Firefox AMO */}
            <div className="p-6 rounded-2xl border flex flex-col justify-between"
              style={{ backgroundColor: "var(--bg-card)", borderColor: "var(--border-subtle)" }}
            >
              <div className="space-y-4">
                <div className="w-10 h-10 rounded-xl bg-orange-500/10 flex items-center justify-center text-orange-400">
                  <Sparkles className="w-6 h-6" />
                </div>
                <div>
                  <h3 className="text-lg font-bold" style={{ color: "var(--text-heading)" }}>
                    Mozilla Firefox Add-on
                  </h3>
                  <p className="text-xs mt-1" style={{ color: "var(--text-muted)" }}>
                    Full support for Mozilla Firefox standard and Developer editions with secure WebExtension Native Messaging.
                  </p>
                </div>
                <ul className="text-xs space-y-2 text-slate-400">
                  <li className="flex items-center gap-2">
                    <CheckCircle2 className="w-4 h-4 text-emerald-400" />
                    <span>AMO verified source code</span>
                  </li>
                  <li className="flex items-center gap-2">
                    <CheckCircle2 className="w-4 h-4 text-emerald-400" />
                    <span>Configurable file extension filters (.iso, .zip, etc.)</span>
                  </li>
                  <li className="flex items-center gap-2">
                    <CheckCircle2 className="w-4 h-4 text-emerald-400" />
                    <span>Zero data sent outside your local machine</span>
                  </li>
                </ul>
              </div>
              <div className="pt-6">
                <a
                  href="/extension"
                  className="w-full flex items-center justify-center gap-2 px-4 py-3 rounded-xl border font-semibold text-xs transition-all hover:scale-102"
                  style={{
                    borderColor: "var(--border-subtle)",
                    backgroundColor: "var(--bg-card-hover)",
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

        {/* Tab Content 5: CLI */}
        {selectedPlatform === "cli" && (
          <div className="max-w-3xl mx-auto p-6 sm:p-8 rounded-2xl border animate-fade-in space-y-6"
            style={{ backgroundColor: "var(--bg-card)", borderColor: "var(--border-subtle)" }}
          >
            <div className="flex items-center gap-3">
              <div className="w-10 h-10 rounded-xl bg-indigo-500/10 flex items-center justify-center text-indigo-400">
                <Terminal className="w-6 h-6" />
              </div>
              <div>
                <h3 className="text-lg font-bold" style={{ color: "var(--text-heading)" }}>
                  Terminal CLI & Headless Daemon
                </h3>
                <p className="text-xs" style={{ color: "var(--text-muted)" }}>
                  Designed for automation scripts, CI/CD runners, and remote servers.
                </p>
              </div>
            </div>

            <div className="space-y-2">
              <div className="text-xs font-semibold text-slate-300">Quick 1-Line Installer:</div>
              <div className="p-3 rounded-xl bg-slate-900 border border-slate-800 font-mono text-xs text-indigo-400 flex items-center justify-between overflow-x-auto">
                <span>curl -sSL https://rdm.app/install.sh | bash</span>
                <button
                  onClick={() => copyCommand("curl -sSL https://rdm.app/install.sh | bash", "cli-curl")}
                  className="p-1.5 rounded hover:bg-slate-800 text-slate-400 hover:text-white"
                >
                  {copiedCmd === "cli-curl" ? <Check className="w-3.5 h-3.5 text-emerald-400" /> : <Copy className="w-3.5 h-3.5" />}
                </button>
              </div>
            </div>

            <div className="space-y-2">
              <div className="text-xs font-semibold text-slate-300">Sample CLI Usage:</div>
              <div className="p-3 rounded-xl bg-slate-900 border border-slate-800 font-mono text-xs text-slate-300 space-y-1 overflow-x-auto">
                <div><span className="text-emerald-400">$</span> rdm download "https://example.com/largefile.iso" --chunks 16</div>
                <div className="text-slate-500"># Intercepts multi-part range chunks with real-time curses progress bar</div>
              </div>
            </div>
          </div>
        )}
      </div>
    </section>
  );
}

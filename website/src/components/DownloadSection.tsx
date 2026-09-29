"use client";

import { ChromeIcon } from "./Icons";

import React from "react";
import { Download, Monitor, HardDrive, Package,  CheckCircle, ExternalLink } from "lucide-react";
import { useTheme } from "../context/ThemeContext";

export default function DownloadSection() {
  const { resolvedTheme } = useTheme();
  const isDark = resolvedTheme === "dark";

  return (
    <section id="download" className="py-20 border-t"
      style={{
        backgroundColor: isDark ? "rgba(11, 15, 25, 0.4)" : "rgba(248, 250, 252, 0.6)",
        borderColor: "var(--border-subtle)",
      }}
    >
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="text-center max-w-3xl mx-auto mb-16 space-y-4">
          <div className="inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-xs font-semibold uppercase tracking-wider text-indigo-400 bg-indigo-500/10 border border-indigo-500/20">
            Official Release v1.0.4
          </div>
          <h2 className="text-3xl sm:text-4xl font-extrabold tracking-tight" style={{ color: "var(--text-heading)" }}>
            Get Rapid Download Manager
          </h2>
          <p className="text-base" style={{ color: "var(--text-muted)" }}>
            Choose your preferred distribution package. All releases are 100% free, virus-scanned, and open-source.
          </p>
        </div>

        {/* 4 Download Cards Grid */}
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
          {/* Card 1: Microsoft Store */}
          <div className="p-6 rounded-2xl border flex flex-col justify-between relative overflow-hidden transition-all duration-300 hover:scale-102 hover:shadow-xl group"
            style={{
              backgroundColor: "var(--bg-card)",
              borderColor: "var(--border-subtle)",
            }}
          >
            <div className="absolute top-0 right-0 bg-gradient-to-l from-indigo-500 to-purple-500 text-white text-[10px] font-bold px-3 py-1 rounded-bl-xl uppercase tracking-wider">
              Recommended
            </div>
            <div>
              <div className="w-12 h-12 rounded-xl bg-indigo-500/15 text-indigo-400 flex items-center justify-center mb-4">
                <Monitor className="w-6 h-6" />
              </div>
              <h3 className="text-lg font-bold mb-1" style={{ color: "var(--text-heading)" }}>
                Microsoft Store
              </h3>
              <p className="text-xs mb-4" style={{ color: "var(--text-muted)" }}>
                Official Windows MSIX package with automatic delta background updates.
              </p>
              <div className="text-xs space-y-1.5 font-medium mb-6" style={{ color: "var(--text-muted)" }}>
                <div className="flex items-center gap-2">
                  <CheckCircle className="w-3.5 h-3.5 text-emerald-400" />
                  <span>Size: 5.71 MB</span>
                </div>
                <div className="flex items-center gap-2">
                  <CheckCircle className="w-3.5 h-3.5 text-emerald-400" />
                  <span>Windows 10 / 11 (x64, ARM64)</span>
                </div>
                <div className="flex items-center gap-2">
                  <CheckCircle className="w-3.5 h-3.5 text-emerald-400" />
                  <span>Zero Admin Required</span>
                </div>
              </div>
            </div>
            <a
              href="https://apps.microsoft.com/"
              target="_blank"
              rel="noopener noreferrer"
              className="w-full py-2.5 px-4 rounded-xl bg-gradient-to-r from-indigo-600 to-purple-600 text-white text-xs font-semibold flex items-center justify-center gap-2 shadow-md shadow-indigo-500/25 hover:shadow-indigo-500/40 transition-all"
            >
              <Download className="w-4 h-4" />
              <span>Get on Windows Store</span>
            </a>
          </div>

          {/* Card 2: Windows Installer (.exe) */}
          <div className="p-6 rounded-2xl border flex flex-col justify-between transition-all duration-300 hover:scale-102 hover:shadow-xl"
            style={{
              backgroundColor: "var(--bg-card)",
              borderColor: "var(--border-subtle)",
            }}
          >
            <div>
              <div className="w-12 h-12 rounded-xl bg-emerald-500/15 text-emerald-400 flex items-center justify-center mb-4">
                <HardDrive className="w-6 h-6" />
              </div>
              <h3 className="text-lg font-bold mb-1" style={{ color: "var(--text-heading)" }}>
                Standard Setup
              </h3>
              <p className="text-xs mb-4" style={{ color: "var(--text-muted)" }}>
                Inno Setup single-click installer with desktop shortcut & full uninstaller.
              </p>
              <div className="text-xs space-y-1.5 font-medium mb-6" style={{ color: "var(--text-muted)" }}>
                <div className="flex items-center gap-2">
                  <CheckCircle className="w-3.5 h-3.5 text-emerald-400" />
                  <span>Size: 5.72 MB (.exe)</span>
                </div>
                <div className="flex items-center gap-2">
                  <CheckCircle className="w-3.5 h-3.5 text-emerald-400" />
                  <span>Clean Auto-Uninstaller</span>
                </div>
                <div className="flex items-center gap-2">
                  <CheckCircle className="w-3.5 h-3.5 text-emerald-400" />
                  <span>Offline Ready</span>
                </div>
              </div>
            </div>
            <a
              href="https://github.com/promahbubul/rapid_download_manager/releases/latest"
              target="_blank"
              rel="noopener noreferrer"
              className="w-full py-2.5 px-4 rounded-xl border text-xs font-semibold flex items-center justify-center gap-2 transition-all hover:border-indigo-500 hover:text-indigo-400"
              style={{
                borderColor: "var(--border-subtle)",
                backgroundColor: "var(--bg-page)",
                color: "var(--text-heading)",
              }}
            >
              <Download className="w-4 h-4" />
              <span>Download .EXE</span>
            </a>
          </div>

          {/* Card 3: Standalone Portable (.zip) */}
          <div className="p-6 rounded-2xl border flex flex-col justify-between transition-all duration-300 hover:scale-102 hover:shadow-xl"
            style={{
              backgroundColor: "var(--bg-card)",
              borderColor: "var(--border-subtle)",
            }}
          >
            <div>
              <div className="w-12 h-12 rounded-xl bg-purple-500/15 text-purple-400 flex items-center justify-center mb-4">
                <Package className="w-6 h-6" />
              </div>
              <h3 className="text-lg font-bold mb-1" style={{ color: "var(--text-heading)" }}>
                Portable Edition
              </h3>
              <p className="text-xs mb-4" style={{ color: "var(--text-muted)" }}>
                Zero installation. Extract and run directly from a USB stick on any PC.
              </p>
              <div className="text-xs space-y-1.5 font-medium mb-6" style={{ color: "var(--text-muted)" }}>
                <div className="flex items-center gap-2">
                  <CheckCircle className="w-3.5 h-3.5 text-emerald-400" />
                  <span>Size: 5.95 MB (.zip)</span>
                </div>
                <div className="flex items-center gap-2">
                  <CheckCircle className="w-3.5 h-3.5 text-emerald-400" />
                  <span>No Registry Modifications</span>
                </div>
                <div className="flex items-center gap-2">
                  <CheckCircle className="w-3.5 h-3.5 text-emerald-400" />
                  <span>100% Self-Contained</span>
                </div>
              </div>
            </div>
            <a
              href="https://github.com/promahbubul/rapid_download_manager/releases/latest"
              target="_blank"
              rel="noopener noreferrer"
              className="w-full py-2.5 px-4 rounded-xl border text-xs font-semibold flex items-center justify-center gap-2 transition-all hover:border-indigo-500 hover:text-indigo-400"
              style={{
                borderColor: "var(--border-subtle)",
                backgroundColor: "var(--bg-page)",
                color: "var(--text-heading)",
              }}
            >
              <Download className="w-4 h-4" />
              <span>Download Portable</span>
            </a>
          </div>

          {/* Card 4: Browser Extension (.zip) */}
          <div id="extension" className="p-6 rounded-2xl border flex flex-col justify-between transition-all duration-300 hover:scale-102 hover:shadow-xl"
            style={{
              backgroundColor: "var(--bg-card)",
              borderColor: "var(--border-subtle)",
            }}
          >
            <div>
              <div className="w-12 h-12 rounded-xl bg-cyan-500/15 text-cyan-400 flex items-center justify-center mb-4">
                <ChromeIcon className="w-6 h-6" />
              </div>
              <h3 className="text-lg font-bold mb-1" style={{ color: "var(--text-heading)" }}>
                Browser Extension
              </h3>
              <p className="text-xs mb-4" style={{ color: "var(--text-muted)" }}>
                Manifest V3 integration for  Edge, Brave, Opera, and Firefox.
              </p>
              <div className="text-xs space-y-1.5 font-medium mb-6" style={{ color: "var(--text-muted)" }}>
                <div className="flex items-center gap-2">
                  <CheckCircle className="w-3.5 h-3.5 text-emerald-400" />
                  <span>Size: 206 KB (.zip)</span>
                </div>
                <div className="flex items-center gap-2">
                  <CheckCircle className="w-3.5 h-3.5 text-emerald-400" />
                  <span>Auto Link Interception</span>
                </div>
                <div className="flex items-center gap-2">
                  <CheckCircle className="w-3.5 h-3.5 text-emerald-400" />
                  <span>Cookie & Auth Handover</span>
                </div>
              </div>
            </div>
            <a
              href="https://github.com/promahbubul/rapid_download_manager/releases/latest"
              target="_blank"
              rel="noopener noreferrer"
              className="w-full py-2.5 px-4 rounded-xl border text-xs font-semibold flex items-center justify-center gap-2 transition-all hover:border-indigo-500 hover:text-indigo-400"
              style={{
                borderColor: "var(--border-subtle)",
                backgroundColor: "var(--bg-page)",
                color: "var(--text-heading)",
              }}
            >
              <Download className="w-4 h-4" />
              <span>Get Extension (.zip)</span>
            </a>
          </div>
        </div>
      </div>
    </section>
  );
}
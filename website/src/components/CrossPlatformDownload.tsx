"use client";

import React, { useState } from "react";
import Link from "next/link";
import {
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
  FolderArchive,
} from "lucide-react";
import { ChromeIcon, WindowsIcon } from "./Icons";
import { useTheme } from "../context/ThemeContext";

type TabType = "windows" | "extension" | "cli";

export default function CrossPlatformDownload() {
  const { resolvedTheme } = useTheme();
  const isDark = resolvedTheme === "dark";

  const [activeTab, setActiveTab] = useState<TabType>("windows");
  const [copiedCmd, setCopiedCmd] = useState<string | null>(null);

  const copyCommand = (cmd: string, id: string) => {
    navigator.clipboard.writeText(cmd);
    setCopiedCmd(id);
    setTimeout(() => setCopiedCmd(null), 2000);
  };

  const latestReleaseUrl = "https://github.com/promahbubul/rapid_download_manager/releases/latest";

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
          <div className="inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-xs font-semibold uppercase tracking-wider text-indigo-400 bg-indigo-500/10 border border-indigo-500/20">
            Official Distribution
          </div>
          <h2 className="text-3xl sm:text-4xl font-extrabold tracking-tight" style={{ color: "var(--text-heading)" }}>
            Download Rapid Download Manager
          </h2>
          <p className="text-sm sm:text-base" style={{ color: "var(--text-muted)" }}>
            Engineered in 100% memory-safe Rust for Windows. Free, open source, and zero telemetry.
          </p>
        </div>

        {/* Category Tabs */}
        <div className="flex flex-wrap items-center justify-center gap-2 mb-10">
          {[
            { id: "windows", name: "Windows Desktop", icon: <WindowsIcon className="w-4 h-4" /> },
            { id: "extension", name: "Browser Extension", icon: <ChromeIcon className="w-4 h-4" /> },
            { id: "cli", name: "Terminal CLI", icon: <Terminal className="w-4 h-4" /> },
          ].map((tab) => {
            const isSelected = activeTab === tab.id;
            return (
              <button
                key={tab.id}
                onClick={() => setActiveTab(tab.id as TabType)}
                className={`flex items-center gap-2 px-5 py-2.5 rounded-xl text-xs sm:text-sm font-medium transition-colors cursor-pointer ${
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

        {/* Tab 1: WINDOWS DESKTOP */}
        {activeTab === "windows" && (
          <div className="space-y-6">
            <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
              {/* Option 1: MSIX */}
              <div
                className="p-6 rounded-2xl border flex flex-col justify-between transition-all duration-300 hover:shadow-xl group"
                style={{ backgroundColor: "var(--bg-card)", borderColor: "var(--border-subtle)" }}
              >
                <div className="space-y-3">
                  <div className="flex items-center justify-between">
                    <div className="w-10 h-10 rounded-xl bg-indigo-500/10 flex items-center justify-center text-indigo-400">
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
                    Official Microsoft Store format. Clean sandbox isolation, verified digital signature, and zero registry clutter.
                  </p>
                  <div className="text-[11px] font-mono text-slate-400 pt-1">
                    v1.0.5 GA • 64-bit Windows 10/11
                  </div>
                </div>
                <div className="pt-6">
                  <a
                    href={latestReleaseUrl}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="w-full flex items-center justify-center gap-2 px-4 py-2.5 rounded-xl bg-indigo-600 hover:bg-indigo-500 text-white font-semibold text-xs shadow-sm transition-colors cursor-pointer"
                  >
                    <Download className="w-4 h-4" />
                    <span>Download .MSIX</span>
                  </a>
                </div>
              </div>

              {/* Option 2: EXE Setup */}
              <div
                className="p-6 rounded-2xl border flex flex-col justify-between transition-all duration-300 hover:shadow-xl group"
                style={{ backgroundColor: "var(--bg-card)", borderColor: "var(--border-subtle)" }}
              >
                <div className="space-y-3">
                  <div className="w-10 h-10 rounded-xl bg-purple-500/10 flex items-center justify-center text-purple-400">
                    <Package className="w-5 h-5" />
                  </div>
                  <h3 className="text-base font-bold" style={{ color: "var(--text-heading)" }}>
                    Setup Installer (.exe)
                  </h3>
                  <p className="text-xs leading-relaxed" style={{ color: "var(--text-muted)" }}>
                    Standard Windows installation wizard built with Inno Setup. Creates Start Menu shortcuts and native URI associations.
                  </p>
                  <div className="text-[11px] font-mono text-slate-400 pt-1">
                    v1.0.5 GA • Standard User (No Admin Required)
                  </div>
                </div>
                <div className="pt-6">
                  <a
                    href={latestReleaseUrl}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="w-full flex items-center justify-center gap-2 px-4 py-2.5 rounded-xl border font-semibold text-xs transition-colors hover:bg-slate-500/10 cursor-pointer"
                    style={{
                      borderColor: "var(--border-subtle)",
                      backgroundColor: "var(--bg-card)",
                      color: "var(--text-heading)",
                    }}
                  >
                    <Download className="w-4 h-4" />
                    <span>Download Setup .EXE</span>
                  </a>
                </div>
              </div>

              {/* Option 3: Portable ZIP */}
              <div
                className="p-6 rounded-2xl border flex flex-col justify-between transition-all duration-300 hover:shadow-xl group"
                style={{ backgroundColor: "var(--bg-card)", borderColor: "var(--border-subtle)" }}
              >
                <div className="space-y-3">
                  <div className="w-10 h-10 rounded-xl bg-cyan-500/10 flex items-center justify-center text-cyan-400">
                    <FolderArchive className="w-5 h-5" />
                  </div>
                  <h3 className="text-base font-bold" style={{ color: "var(--text-heading)" }}>
                    Portable Archive (.zip)
                  </h3>
                  <p className="text-xs leading-relaxed" style={{ color: "var(--text-muted)" }}>
                    Completely standalone. Extract to any folder or run directly from a USB stick without installing or modifying system settings.
                  </p>
                  <div className="text-[11px] font-mono text-slate-400 pt-1">
                    Includes: GUI, CLI & Extension installer
                  </div>
                </div>
                <div className="pt-6">
                  <a
                    href={latestReleaseUrl}
                    target="_blank"
                    rel="noopener noreferrer"
                    className="w-full flex items-center justify-center gap-2 px-4 py-2.5 rounded-xl border font-semibold text-xs transition-colors hover:bg-slate-500/10 cursor-pointer"
                    style={{
                      borderColor: "var(--border-subtle)",
                      backgroundColor: "var(--bg-card)",
                      color: "var(--text-heading)",
                    }}
                  >
                    <Download className="w-4 h-4" />
                    <span>Download Portable .ZIP</span>
                  </a>
                </div>
              </div>
            </div>

            {/* Platform Roadmap Notice */}
            <div
              className="p-4 rounded-xl border text-center text-xs flex flex-col sm:flex-row items-center justify-center gap-2"
              style={{
                backgroundColor: isDark ? "rgba(30, 41, 59, 0.4)" : "rgba(241, 245, 249, 0.8)",
                borderColor: "var(--border-subtle)",
                color: "var(--text-muted)",
              }}
            >
              <Laptop className="w-4 h-4 text-indigo-400" />
              <span>Looking for macOS or Linux? Native UNIX builds are actively being developed on our open-source roadmap.</span>
              <a
                href="https://github.com/promahbubul/rapid_download_manager"
                target="_blank"
                rel="noopener noreferrer"
                className="text-indigo-400 font-semibold hover:underline"
              >
                Follow on GitHub &rarr;
              </a>
            </div>
          </div>
        )}

        {/* Tab 2: BROWSER EXTENSION */}
        {activeTab === "extension" && (
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6 max-w-4xl mx-auto">
            {/* Firefox AMO */}
            <div
              className="p-6 rounded-2xl border flex flex-col justify-between"
              style={{ backgroundColor: "var(--bg-card)", borderColor: "var(--border-subtle)" }}
            >
              <div className="space-y-3">
                <div className="flex items-center justify-between">
                  <span className="text-[10px] font-bold px-2 py-0.5 rounded-full bg-orange-500/20 text-orange-400 border border-orange-500/30">
                    Official Add-on
                  </span>
                </div>
                <h3 className="text-base font-bold" style={{ color: "var(--text-heading)" }}>
                  Mozilla Firefox
                </h3>
                <p className="text-xs leading-relaxed" style={{ color: "var(--text-muted)" }}>
                  Verified extension on Mozilla Add-ons (AMO). Intercepts file downloads with 1-click installation directly from your browser.
                </p>
                <div className="flex items-center gap-1.5 text-xs text-emerald-400 font-medium pt-1">
                  <CheckCircle2 className="w-3.5 h-3.5" />
                  <span>Works with Firefox & Firefox-based forks</span>
                </div>
              </div>
              <div className="pt-6">
                <a
                  href="https://addons.mozilla.org/en-US/firefox/addon/rapid-download-manager-integra/"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="w-full flex items-center justify-center gap-2 px-4 py-2.5 rounded-xl bg-gradient-to-r from-orange-600 to-amber-600 hover:opacity-95 text-white font-semibold text-xs shadow-sm transition-opacity cursor-pointer"
                >
                  <ExternalLink className="w-4 h-4" />
                  <span>Install from Firefox Add-ons</span>
                </a>
              </div>
            </div>

            {/* Chromium Extension */}
            <div
              className="p-6 rounded-2xl border flex flex-col justify-between"
              style={{ backgroundColor: "var(--bg-card)", borderColor: "var(--border-subtle)" }}
            >
              <div className="space-y-3">
                <div className="flex items-center justify-between">
                  <span className="text-[10px] font-bold px-2 py-0.5 rounded-full bg-cyan-500/20 text-cyan-400 border border-cyan-500/30">
                    Manifest V3
                  </span>
                </div>
                <h3 className="text-base font-bold" style={{ color: "var(--text-heading)" }}>
                  Chrome, Edge, Brave & Opera
                </h3>
                <p className="text-xs leading-relaxed" style={{ color: "var(--text-muted)" }}>
                  Companion browser package included with every desktop release. Register once with <code className="px-1 py-0.5 rounded bg-slate-500/10 font-mono text-[11px]">install_extension.bat</code>.
                </p>
                <div className="flex items-center gap-1.5 text-xs text-emerald-400 font-medium pt-1">
                  <CheckCircle2 className="w-3.5 h-3.5" />
                  <span>Preserves cookies, session auth, and referrers</span>
                </div>
              </div>
              <div className="pt-6">
                <Link
                  href="/extension"
                  className="w-full flex items-center justify-center gap-2 px-4 py-2.5 rounded-xl border font-semibold text-xs transition-colors hover:bg-slate-500/10 cursor-pointer"
                  style={{
                    borderColor: "var(--border-subtle)",
                    backgroundColor: "var(--bg-card)",
                    color: "var(--text-heading)",
                  }}
                >
                  <span>View Extension Installation Guide &rarr;</span>
                </Link>
              </div>
            </div>
          </div>
        )}

        {/* Tab 3: CLI BINARY */}
        {activeTab === "cli" && (
          <div
            className="p-6 sm:p-8 rounded-2xl border max-w-4xl mx-auto space-y-6"
            style={{ backgroundColor: "var(--bg-card)", borderColor: "var(--border-subtle)" }}
          >
            <div>
              <div className="flex items-center gap-2 text-indigo-400 text-xs font-semibold uppercase tracking-wider mb-1">
                <Terminal className="w-4 h-4" />
                <span>Headless High-Speed CLI</span>
              </div>
              <h3 className="text-lg font-bold" style={{ color: "var(--text-heading)" }}>
                Rapid CLI (`rapid-cli.exe`)
              </h3>
              <p className="text-xs sm:text-sm mt-1" style={{ color: "var(--text-muted)" }}>
                The standalone CLI binary is included in the portable release package. It provides the same 16-chunk acceleration engine directly from Windows Terminal or PowerShell scripts.
              </p>
            </div>

            <div className="space-y-3">
              <div className="flex items-center justify-between text-xs" style={{ color: "var(--text-muted)" }}>
                <span className="font-semibold">Example Usage:</span>
                <button
                  onClick={() => copyCommand("rapid-cli.exe --url https://example.com/largefile.iso --chunks 16", "cli-cmd")}
                  className="flex items-center gap-1.5 text-indigo-400 hover:text-indigo-300 cursor-pointer"
                >
                  {copiedCmd === "cli-cmd" ? <Check className="w-3.5 h-3.5 text-emerald-400" /> : <Copy className="w-3.5 h-3.5" />}
                  <span>{copiedCmd === "cli-cmd" ? "Copied!" : "Copy Command"}</span>
                </button>
              </div>

              <div className="p-3.5 rounded-xl bg-slate-950 border border-slate-800 font-mono text-xs text-indigo-300 overflow-x-auto">
                <code>rapid-cli.exe --url https://example.com/largefile.iso --chunks 16</code>
              </div>
            </div>

            <div className="pt-2 flex flex-wrap items-center justify-between gap-4 border-t border-slate-700/20">
              <div className="text-xs" style={{ color: "var(--text-muted)" }}>
                Requires no administrative permissions and runs seamlessly in CI/CD pipelines or automated batch scripts.
              </div>
              <Link
                href="/docs"
                className="text-xs font-semibold text-indigo-400 hover:underline inline-flex items-center gap-1"
              >
                <span>Read CLI Documentation</span>
                <span>&rarr;</span>
              </Link>
            </div>
          </div>
        )}
      </div>
    </section>
  );
}

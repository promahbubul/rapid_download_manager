"use client";

import React, { useState } from "react";
import Link from "next/link";
import Navbar from "@/components/Navbar";
import Footer from "@/components/Footer";
import {
  BookOpen,
  Terminal,
  Settings,
  HelpCircle,
  Copy,
  Check,
  ArrowRight,
  Shield,
  Zap,
  FolderOpen,
  ArrowLeft,
} from "lucide-react";
import { ChromeIcon } from "@/components/Icons";
import { useTheme } from "@/context/ThemeContext";

export default function DocsPage() {
  const { resolvedTheme } = useTheme();
  const isDark = resolvedTheme === "dark";
  const [activeTab, setActiveTab] = useState<"gui" | "extension" | "cli" | "troubleshooting">("gui");
  const [copiedCode, setCopiedCode] = useState<string | null>(null);

  const copyToClipboard = (text: string, id: string) => {
    navigator.clipboard.writeText(text);
    setCopiedCode(id);
    setTimeout(() => setCopiedCode(null), 2000);
  };

  return (
    <div className="min-h-screen flex flex-col">
      <Navbar />

      <main className="flex-1 max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
        <Link
          href="/"
          className="inline-flex items-center gap-2 text-xs font-semibold text-indigo-400 hover:text-indigo-300 mb-6 transition-colors"
        >
          <ArrowLeft className="w-4 h-4" />
          <span>Back to Home</span>
        </Link>

        {/* Page Header */}
        <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 mb-10 pb-6 border-b"
          style={{ borderColor: "var(--border-subtle)" }}
        >
          <div className="space-y-1">
            <div className="flex items-center gap-2">
              <div className="w-9 h-9 rounded-xl bg-indigo-500/15 text-indigo-400 flex items-center justify-center">
                <BookOpen className="w-5 h-5" />
              </div>
              <h1 className="text-3xl font-extrabold tracking-tight" style={{ color: "var(--text-heading)" }}>
                Documentation Hub
              </h1>
            </div>
            <p className="text-xs sm:text-sm" style={{ color: "var(--text-muted)" }}>
              Complete reference manual, browser integration setup, and CLI automation guides.
            </p>
          </div>

          <div className="inline-flex items-center gap-2 px-3 py-1.5 rounded-full border text-xs font-medium"
            style={{
              backgroundColor: "var(--bg-card)",
              borderColor: "var(--border-subtle)",
              color: "var(--text-muted)",
            }}
          >
            <span>Current Version:</span>
            <span className="font-bold text-indigo-400">v1.0.0 (x64)</span>
          </div>
        </div>

        {/* Navigation Tabs */}
        <div className="flex flex-wrap gap-2 mb-8 border-b pb-4" style={{ borderColor: "var(--border-subtle)" }}>
          <button
            onClick={() => setActiveTab("gui")}
            className={`px-4 py-2 rounded-xl text-xs sm:text-sm font-semibold flex items-center gap-2 transition-all ${
              activeTab === "gui"
                ? "bg-indigo-600 text-white shadow-md shadow-indigo-500/25"
                : "hover:bg-slate-500/10 text-slate-400"
            }`}
          >
            <Settings className="w-4 h-4" />
            <span>Desktop GUI Guide</span>
          </button>

          <button
            onClick={() => setActiveTab("extension")}
            className={`px-4 py-2 rounded-xl text-xs sm:text-sm font-semibold flex items-center gap-2 transition-all ${
              activeTab === "extension"
                ? "bg-indigo-600 text-white shadow-md shadow-indigo-500/25"
                : "hover:bg-slate-500/10 text-slate-400"
            }`}
          >
            <ChromeIcon className="w-4 h-4" />
            <span>Browser Extension Setup</span>
          </button>

          <button
            onClick={() => setActiveTab("cli")}
            className={`px-4 py-2 rounded-xl text-xs sm:text-sm font-semibold flex items-center gap-2 transition-all ${
              activeTab === "cli"
                ? "bg-indigo-600 text-white shadow-md shadow-indigo-500/25"
                : "hover:bg-slate-500/10 text-slate-400"
            }`}
          >
            <Terminal className="w-4 h-4" />
            <span>CLI Terminal Guide</span>
          </button>

          <button
            onClick={() => setActiveTab("troubleshooting")}
            className={`px-4 py-2 rounded-xl text-xs sm:text-sm font-semibold flex items-center gap-2 transition-all ${
              activeTab === "troubleshooting"
                ? "bg-indigo-600 text-white shadow-md shadow-indigo-500/25"
                : "hover:bg-slate-500/10 text-slate-400"
            }`}
          >
            <HelpCircle className="w-4 h-4" />
            <span>Troubleshooting & FAQ</span>
          </button>
        </div>

        {/* TAB 1: GUI GUIDE */}
        {activeTab === "gui" && (
          <div className="space-y-8 animate-in fade-in">
            <div className="p-6 rounded-2xl border space-y-4"
              style={{
                backgroundColor: "var(--bg-card)",
                borderColor: "var(--border-subtle)",
              }}
            >
              <h2 className="text-xl font-bold flex items-center gap-2" style={{ color: "var(--text-heading)" }}>
                <Zap className="w-5 h-5 text-indigo-400" />
                <span>1. Multi-Segment Download Acceleration</span>
              </h2>
              <p className="text-sm leading-relaxed" style={{ color: "var(--text-body)" }}>
                Rapid Download Manager automatically queries the remote HTTP server for byte-range support (<code>Accept-Ranges: bytes</code>). When supported, it splits the file into up to 16 equal segments and streams them simultaneously over parallel TCP connections.
              </p>
              <div className="p-4 rounded-xl border space-y-2 text-xs font-mono"
                style={{
                  backgroundColor: isDark ? "#090D16" : "#F1F5F9",
                  borderColor: "var(--border-subtle)",
                  color: isDark ? "#cbd5e1" : "#334155",
                }}
              >
                <div className="text-indigo-400 font-bold">// How Rapid DM allocates parallel connections</div>
                <div>Chunk #1: Bytes 000,000,000 - 099,999,999 [Active • Socket 1]</div>
                <div>Chunk #2: Bytes 100,000,000 - 199,999,999 [Active • Socket 2]</div>
                <div>Chunk #3: Bytes 200,000,000 - 299,999,999 [Active • Socket 3]</div>
                <div>...</div>
                <div>Chunk #16: Bytes 1,500,000,000 - 1,599,999,999 [Active • Socket 16]</div>
              </div>
            </div>

            <div className="p-6 rounded-2xl border space-y-4"
              style={{
                backgroundColor: "var(--bg-card)",
                borderColor: "var(--border-subtle)",
              }}
            >
              <h2 className="text-xl font-bold flex items-center gap-2" style={{ color: "var(--text-heading)" }}>
                <FolderOpen className="w-5 h-5 text-purple-400" />
                <span>2. Application Data & Storage Locations</span>
              </h2>
              <p className="text-sm leading-relaxed" style={{ color: "var(--text-body)" }}>
                Rapid Download Manager strictly conforms to Windows certified filesystem guidelines. All user files are neatly segregated:
              </p>
              <div className="grid grid-cols-1 md:grid-cols-3 gap-4 text-xs">
                <div className="p-3.5 rounded-xl border space-y-1" style={{ borderColor: "var(--border-subtle)" }}>
                  <div className="font-bold text-indigo-400">Download Directory</div>
                  <div className="font-mono text-[11px]" style={{ color: "var(--text-muted)" }}>%USERPROFILE%\Downloads</div>
                  <div style={{ color: "var(--text-muted)" }}>Completed and categorized downloads.</div>
                </div>
                <div className="p-3.5 rounded-xl border space-y-1" style={{ borderColor: "var(--border-subtle)" }}>
                  <div className="font-bold text-indigo-400">Configuration & State</div>
                  <div className="font-mono text-[11px]" style={{ color: "var(--text-muted)" }}>%APPDATA%\RapidDownloadManager</div>
                  <div style={{ color: "var(--text-muted)" }}>Download history and user settings.</div>
                </div>
                <div className="p-3.5 rounded-xl border space-y-1" style={{ borderColor: "var(--border-subtle)" }}>
                  <div className="font-bold text-indigo-400">Log Files & Cache</div>
                  <div className="font-mono text-[11px]" style={{ color: "var(--text-muted)" }}>%LOCALAPPDATA%\RapidDownloadManager</div>
                  <div style={{ color: "var(--text-muted)" }}>Redacted error and crash diagnostic logs.</div>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* TAB 2: EXTENSION SETUP */}
        {activeTab === "extension" && (
          <div className="space-y-8 animate-in fade-in">
            <div className="p-6 rounded-2xl border space-y-4"
              style={{
                backgroundColor: "var(--bg-card)",
                borderColor: "var(--border-subtle)",
              }}
            >
              <h2 className="text-xl font-bold flex items-center gap-2" style={{ color: "var(--text-heading)" }}>
                <ChromeIcon className="w-5 h-5 text-indigo-400" />
                <span>Browser Extension Installation (Chrome, Edge, Brave)</span>
              </h2>
              <p className="text-sm leading-relaxed" style={{ color: "var(--text-body)" }}>
                The companion extension intercepts downloads in Chromium and Firefox browsers and hands over cookies and session headers to the native accelerator:
              </p>

              <div className="space-y-4 pt-2">
                <div className="flex gap-4 items-start">
                  <div className="w-7 h-7 rounded-full bg-indigo-500/20 text-indigo-400 font-bold flex items-center justify-center shrink-0 text-xs">1</div>
                  <div className="space-y-1">
                    <div className="text-sm font-bold" style={{ color: "var(--text-heading)" }}>One-Click Automatic Integration (Recommended)</div>
                    <div className="text-xs" style={{ color: "var(--text-muted)" }}>
                      Simply double-click <code>install_extension.bat</code> located in your Rapid Download Manager installation folder. It automatically hooks Chrome, Edge, and Brave.
                    </div>
                  </div>
                </div>

                <div className="flex gap-4 items-start">
                  <div className="w-7 h-7 rounded-full bg-indigo-500/20 text-indigo-400 font-bold flex items-center justify-center shrink-0 text-xs">2</div>
                  <div className="space-y-1">
                    <div className="text-sm font-bold" style={{ color: "var(--text-heading)" }}>Manual Developer Mode Loading</div>
                    <div className="text-xs" style={{ color: "var(--text-muted)" }}>
                      Open <code>chrome://extensions</code> (or <code>edge://extensions</code>), toggle <strong>Developer mode</strong> ON in the top right, click <strong>Load unpacked</strong>, and select the <code>extension</code> folder inside your Rapid Download Manager directory.
                    </div>
                  </div>
                </div>

                <div className="flex gap-4 items-start">
                  <div className="w-7 h-7 rounded-full bg-indigo-500/20 text-indigo-400 font-bold flex items-center justify-center shrink-0 text-xs">3</div>
                  <div className="space-y-1">
                    <div className="text-sm font-bold" style={{ color: "var(--text-heading)" }}>Local Loopback Security</div>
                    <div className="text-xs" style={{ color: "var(--text-muted)" }}>
                      Communication occurs exclusively over encrypted local loopback (<code>127.0.0.1:18942</code>). No internet connections or cloud relays are ever used.
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* TAB 3: CLI GUIDE */}
        {activeTab === "cli" && (
          <div className="space-y-8 animate-in fade-in">
            <div className="p-6 rounded-2xl border space-y-4"
              style={{
                backgroundColor: "var(--bg-card)",
                borderColor: "var(--border-subtle)",
              }}
            >
              <h2 className="text-xl font-bold flex items-center gap-2" style={{ color: "var(--text-heading)" }}>
                <Terminal className="w-5 h-5 text-indigo-400" />
                <span>Command-Line Interface (rapid-cli)</span>
              </h2>
              <p className="text-sm leading-relaxed" style={{ color: "var(--text-body)" }}>
                For power users, automated server scripts, and CI/CD pipelines, Rapid Download Manager includes a blazing fast terminal binary: <code>rapid-cli.exe</code>.
              </p>

              {/* Code Snippet 1 */}
              <div className="space-y-2">
                <div className="flex items-center justify-between text-xs font-semibold" style={{ color: "var(--text-muted)" }}>
                  <span>Basic Multi-Chunk Download</span>
                  <button
                    onClick={() => copyToClipboard("rapid-cli.exe --url https://example.com/largefile.iso --chunks 16", "c1")}
                    className="flex items-center gap-1 text-indigo-400 hover:text-indigo-300 transition-colors"
                  >
                    {copiedCode === "c1" ? <Check className="w-3.5 h-3.5 text-emerald-400" /> : <Copy className="w-3.5 h-3.5" />}
                    <span>{copiedCode === "c1" ? "Copied" : "Copy"}</span>
                  </button>
                </div>
                <pre className="p-4 rounded-xl text-xs font-mono overflow-x-auto"
                  style={{
                    backgroundColor: isDark ? "#090D16" : "#F1F5F9",
                    color: isDark ? "#38bdf8" : "#0369a1",
                  }}
                >
                  rapid-cli.exe --url https://example.com/largefile.iso --chunks 16
                </pre>
              </div>

              {/* Code Snippet 2 */}
              <div className="space-y-2">
                <div className="flex items-center justify-between text-xs font-semibold" style={{ color: "var(--text-muted)" }}>
                  <span>Custom Destination Directory</span>
                  <button
                    onClick={() => copyToClipboard('rapid-cli.exe --url https://example.com/dataset.zip --output "D:\Datasets" --chunks 16', "c2")}
                    className="flex items-center gap-1 text-indigo-400 hover:text-indigo-300 transition-colors"
                  >
                    {copiedCode === "c2" ? <Check className="w-3.5 h-3.5 text-emerald-400" /> : <Copy className="w-3.5 h-3.5" />}
                    <span>{copiedCode === "c2" ? "Copied" : "Copy"}</span>
                  </button>
                </div>
                <pre className="p-4 rounded-xl text-xs font-mono overflow-x-auto"
                  style={{
                    backgroundColor: isDark ? "#090D16" : "#F1F5F9",
                    color: isDark ? "#38bdf8" : "#0369a1",
                  }}
                >
                  rapid-cli.exe --url https://example.com/dataset.zip --output "D:\Datasets" --chunks 16
                </pre>
              </div>
            </div>
          </div>
        )}

        {/* TAB 4: TROUBLESHOOTING */}
        {activeTab === "troubleshooting" && (
          <div className="space-y-8 animate-in fade-in">
            <div className="p-6 rounded-2xl border space-y-4"
              style={{
                backgroundColor: "var(--bg-card)",
                borderColor: "var(--border-subtle)",
              }}
            >
              <h2 className="text-xl font-bold flex items-center gap-2" style={{ color: "var(--text-heading)" }}>
                <HelpCircle className="w-5 h-5 text-indigo-400" />
                <span>Frequently Encountered Questions</span>
              </h2>

              <div className="space-y-4 divide-y" style={{ borderColor: "var(--border-subtle)" }}>
                <div className="pt-4 first:pt-0 space-y-1">
                  <h3 className="text-sm font-bold" style={{ color: "var(--text-heading)" }}>
                    Q: Does Rapid Download Manager require administrator rights?
                  </h3>
                  <p className="text-xs leading-relaxed" style={{ color: "var(--text-muted)" }}>
                    No. The executable runs with standard user privileges (<code>asInvoker</code>). It writes configurations to <code>%APPDATA%</code> and downloads directly to your user folder, ensuring absolute system safety without UAC prompts.
                  </p>
                </div>

                <div className="pt-4 space-y-1">
                  <h3 className="text-sm font-bold" style={{ color: "var(--text-heading)" }}>
                    Q: Why doesn't a specific server accelerate beyond 1 connection?
                  </h3>
                  <p className="text-xs leading-relaxed" style={{ color: "var(--text-muted)" }}>
                    Some download hosts explicitly disallow HTTP byte-range slicing (returning no <code>Accept-Ranges</code> header). In such rare cases, Rapid Download Manager seamlessly falls back to high-speed single-stream transfer to prevent data corruption.
                  </p>
                </div>

                <div className="pt-4 space-y-1">
                  <h3 className="text-sm font-bold" style={{ color: "var(--text-heading)" }}>
                    Q: How do I report an unexpected crash or bug?
                  </h3>
                  <p className="text-xs leading-relaxed" style={{ color: "var(--text-muted)" }}>
                    Crash diagnostics are automatically logged to <code>%LOCALAPPDATA%\RapidDownloadManager\logs\crash.log</code> with credentials redacted. You can open an issue on our GitHub repository and attach this log.
                  </p>
                </div>
              </div>
            </div>
          </div>
        )}
      </main>

      <Footer />
    </div>
  );
}

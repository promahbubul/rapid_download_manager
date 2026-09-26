"use client";

import { GithubIcon } from "./Icons";

import React, { useState } from "react";
import {
  Zap,
  Download,
  ShieldCheck,
  Cpu,
  Layers,
  Sparkles,
  ExternalLink,
    Play,
  Pause,
  CheckCircle2,
  FolderOpen,
  ArrowDownToLine,
  Activity,
} from "lucide-react";
import { useTheme } from "../context/ThemeContext";

export default function Hero() {
  const { resolvedTheme } = useTheme();
  const isDark = resolvedTheme === "dark";

  return (
    <section className="relative pt-12 pb-24 overflow-hidden">
      {/* Background Gradient Glows */}
      <div className="absolute top-1/4 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[700px] h-[400px] bg-gradient-to-tr from-indigo-600/20 via-purple-600/15 to-pink-500/10 rounded-full blur-3xl pointer-events-none -z-10" />

      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        {/* Top Badges & Headline */}
        <div className="text-center max-w-3xl mx-auto space-y-6">
          <div className="inline-flex items-center gap-2 px-3.5 py-1.5 rounded-full border text-xs font-semibold backdrop-blur-md shadow-sm transition-all hover:scale-102"
            style={{
              backgroundColor: isDark ? "rgba(17, 24, 39, 0.6)" : "rgba(255, 255, 255, 0.8)",
              borderColor: isDark ? "rgba(99, 102, 241, 0.4)" : "rgba(99, 102, 241, 0.3)",
              color: isDark ? "#a5b4fc" : "#4f46e5",
            }}
          >
            <Sparkles className="w-3.5 h-3.5 text-indigo-400 animate-pulse" />
            <span>Pure Memory-Safe Rust • Windows 10/11 Certified • 0 Telemetry</span>
          </div>

          <h1 className="text-4xl sm:text-5xl lg:text-6xl font-extrabold tracking-tight leading-[1.15]"
            style={{ color: "var(--text-heading)" }}
          >
            Turbocharged Speed.{" "}
            <span className="bg-gradient-to-r from-indigo-500 via-purple-500 to-pink-500 bg-clip-text text-transparent">
              Zero Compromises.
            </span>
          </h1>

          <p className="text-base sm:text-lg leading-relaxed max-w-2xl mx-auto"
            style={{ color: "var(--text-muted)" }}
          >
            Experience lightning-fast file downloads powered by 16-stream parallel range socket balancing, automated browser integration, and memory-safe Rust cryptography. Free, open source, and lightweight.
          </p>

          {/* Action CTAs */}
          <div className="flex flex-wrap items-center justify-center gap-4 pt-2">
            <a
              href="#download"
              className="inline-flex items-center gap-2.5 px-6 py-3.5 rounded-xl bg-gradient-to-r from-indigo-600 via-indigo-500 to-violet-600 text-white font-semibold text-sm shadow-lg shadow-indigo-500/25 hover:shadow-indigo-500/40 hover:scale-103 transition-all duration-200"
            >
              <Download className="w-4 h-4" />
              <span>Download for Windows</span>
              <span className="text-xs opacity-75 font-normal ml-1">v1.0.0</span>
            </a>

            <a
              href="https://github.com/promahbubul/rapid_download_manager"
              target="_blank"
              rel="noopener noreferrer"
              className="inline-flex items-center gap-2 px-5 py-3.5 rounded-xl border text-sm font-semibold transition-all hover:scale-102"
              style={{
                borderColor: "var(--border-subtle)",
                backgroundColor: "var(--bg-card)",
                color: "var(--text-heading)",
              }}
            >
              <GithubIcon className="w-4 h-4" />
              <span>Star on GitHub</span>
            </a>
          </div>

          {/* 4 Trust Metrics */}
          <div className="grid grid-cols-2 sm:grid-cols-4 gap-4 pt-6 max-w-2xl mx-auto text-left">
            <div className="p-3 rounded-xl border" style={{ borderColor: "var(--border-subtle)", backgroundColor: "var(--bg-card)" }}>
              <div className="text-xl font-bold bg-gradient-to-r from-indigo-400 to-purple-400 bg-clip-text text-transparent">Up to 16x</div>
              <div className="text-xs" style={{ color: "var(--text-muted)" }}>Chunk Acceleration</div>
            </div>
            <div className="p-3 rounded-xl border" style={{ borderColor: "var(--border-subtle)", backgroundColor: "var(--bg-card)" }}>
              <div className="text-xl font-bold text-emerald-400">&lt; 25 MB</div>
              <div className="text-xs" style={{ color: "var(--text-muted)" }}>Idle RAM Footprint</div>
            </div>
            <div className="p-3 rounded-xl border" style={{ borderColor: "var(--border-subtle)", backgroundColor: "var(--bg-card)" }}>
              <div className="text-xl font-bold text-cyan-400">100% Rust</div>
              <div className="text-xs" style={{ color: "var(--text-muted)" }}>Memory-Safe Core</div>
            </div>
            <div className="p-3 rounded-xl border" style={{ borderColor: "var(--border-subtle)", backgroundColor: "var(--bg-card)" }}>
              <div className="text-xl font-bold text-amber-400">0 Ads</div>
              <div className="text-xs" style={{ color: "var(--text-muted)" }}>Zero Telemetry / Trackers</div>
            </div>
          </div>
        </div>

        {/* Live Interactive UI Showcase / Desktop App Window */}
        <div id="preview" className="mt-14 max-w-5xl mx-auto rounded-2xl border shadow-2xl overflow-hidden backdrop-blur-xl transition-all duration-300 hover:shadow-indigo-500/10"
          style={{
            backgroundColor: isDark ? "#0E1526" : "#FFFFFF",
            borderColor: isDark ? "rgba(99, 102, 241, 0.3)" : "rgba(203, 213, 225, 0.8)",
          }}
        >
          {/* Windows Titlebar */}
          <div className="px-4 py-2.5 border-b flex items-center justify-between"
            style={{
              backgroundColor: isDark ? "#0B0F19" : "#F1F5F9",
              borderColor: isDark ? "#1E293B" : "#E2E8F0",
            }}
          >
            <div className="flex items-center gap-2.5">
              <div className="flex items-center gap-1.5">
                <div className="w-3 h-3 rounded-full bg-rose-500/80" />
                <div className="w-3 h-3 rounded-full bg-amber-500/80" />
                <div className="w-3 h-3 rounded-full bg-emerald-500/80" />
              </div>
              <span className="text-xs font-semibold tracking-wide ml-2" style={{ color: isDark ? "#94A3B8" : "#475569" }}>
                Rapid Download Manager — [Active Downloads: 3]
              </span>
            </div>
            <div className="flex items-center gap-3 text-xs" style={{ color: isDark ? "#64748B" : "#94A3B8" }}>
              <span className="flex items-center gap-1 text-emerald-400 font-mono font-medium">
                <span className="w-2 h-2 rounded-full bg-emerald-400 animate-ping inline-block" />
                Live Speed: 119.4 MB/s
              </span>
            </div>
          </div>

          {/* App Body Grid */}
          <div className="grid grid-cols-1 md:grid-cols-4 min-h-[380px]">
            {/* Sidebar */}
            <div className="border-r p-4 space-y-4 hidden md:block"
              style={{
                backgroundColor: isDark ? "#090D16" : "#F8FAFC",
                borderColor: isDark ? "#1E293B" : "#E2E8F0",
              }}
            >
              <div className="space-y-1">
                <div className="text-[11px] font-bold uppercase tracking-wider text-indigo-400 px-2 py-1">
                  Categories
                </div>
                <div className="px-2.5 py-1.5 rounded-lg text-xs font-semibold bg-indigo-500/15 text-indigo-400 flex items-center justify-between">
                  <span>Downloading</span>
                  <span className="px-1.5 py-0.5 rounded bg-indigo-500/20 text-[10px]">3</span>
                </div>
                <div className="px-2.5 py-1.5 rounded-lg text-xs font-medium hover:bg-slate-500/10 flex items-center justify-between" style={{ color: "var(--text-muted)" }}>
                  <span>Completed</span>
                  <span className="px-1.5 py-0.5 rounded bg-slate-500/20 text-[10px]">48</span>
                </div>
                <div className="px-2.5 py-1.5 rounded-lg text-xs font-medium hover:bg-slate-500/10 flex items-center justify-between" style={{ color: "var(--text-muted)" }}>
                  <span>Failed / Paused</span>
                  <span className="px-1.5 py-0.5 rounded bg-slate-500/20 text-[10px]">0</span>
                </div>
              </div>

              <div className="pt-4 border-t border-slate-700/20 space-y-2">
                <div className="text-[11px] font-bold uppercase tracking-wider text-slate-400 px-2">
                  System Stats
                </div>
                <div className="text-xs px-2 space-y-1" style={{ color: "var(--text-muted)" }}>
                  <div className="flex justify-between">
                    <span>Engine:</span>
                    <span className="font-mono text-indigo-400 font-semibold">Rust Tokio</span>
                  </div>
                  <div className="flex justify-between">
                    <span>TLS:</span>
                    <span className="font-mono text-emerald-400 font-semibold">Rustls 0.23</span>
                  </div>
                  <div className="flex justify-between">
                    <span>Connections:</span>
                    <span className="font-mono font-semibold">48 Sockets</span>
                  </div>
                </div>
              </div>
            </div>

            {/* Main Download Items List */}
            <div className="md:col-span-3 p-4 sm:p-6 space-y-4">
              {/* Item 1: In-Flight Multi-chunk File */}
              <div className="p-4 rounded-xl border transition-all duration-200"
                style={{
                  backgroundColor: isDark ? "#111827" : "#FFFFFF",
                  borderColor: isDark ? "#1F2937" : "#E2E8F0",
                }}
              >
                <div className="flex items-center justify-between gap-3 mb-2">
                  <div className="flex items-center gap-2.5 overflow-hidden">
                    <div className="w-8 h-8 rounded-lg bg-indigo-500/20 flex items-center justify-center shrink-0">
                      <ArrowDownToLine className="w-4 h-4 text-indigo-400" />
                    </div>
                    <div>
                      <div className="text-xs sm:text-sm font-bold truncate" style={{ color: "var(--text-heading)" }}>
                        Windows_11_24H2_Pro_x64.iso
                      </div>
                      <div className="text-[11px]" style={{ color: "var(--text-muted)" }}>
                        3.74 GB of 4.80 GB • 16 Chunks Active • ETA: 18s
                      </div>
                    </div>
                  </div>
                  <div className="text-right shrink-0">
                    <span className="text-xs sm:text-sm font-mono font-bold text-emerald-400">
                      42.5 MB/s
                    </span>
                    <div className="text-[11px] font-semibold text-indigo-400">78%</div>
                  </div>
                </div>

                {/* Animated Segmented Multi-Chunk Bar */}
                <div className="space-y-1.5">
                  <div className="w-full h-2.5 rounded-full overflow-hidden flex gap-0.5 bg-slate-800/40 p-0.5">
                    {[
                      "w-[6.25%] bg-emerald-500", "w-[6.25%] bg-emerald-400", "w-[6.25%] bg-indigo-500", "w-[6.25%] bg-emerald-500",
                      "w-[6.25%] bg-indigo-400", "w-[6.25%] bg-emerald-500", "w-[6.25%] bg-cyan-400", "w-[6.25%] bg-emerald-500",
                      "w-[6.25%] bg-indigo-500", "w-[6.25%] bg-emerald-400", "w-[6.25%] bg-emerald-500", "w-[6.25%] bg-cyan-400",
                      "w-[6.25%] bg-indigo-400", "w-[4%] bg-amber-400 animate-pulse", "w-[2%] bg-slate-700", "w-[0%] bg-slate-700"
                    ].map((chunk, idx) => (
                      <div key={idx} className={`h-full rounded-sm ${chunk}`} />
                    ))}
                  </div>
                  <div className="flex justify-between text-[10px] font-mono text-slate-400">
                    <span>Range Socket #1-#16 Synchronized</span>
                    <span className="text-emerald-400">100% Chunk Integrity Verified</span>
                  </div>
                </div>
              </div>

              {/* Item 2: RustRover Setup */}
              <div className="p-4 rounded-xl border transition-all duration-200"
                style={{
                  backgroundColor: isDark ? "#111827" : "#FFFFFF",
                  borderColor: isDark ? "#1F2937" : "#E2E8F0",
                }}
              >
                <div className="flex items-center justify-between gap-3 mb-2">
                  <div className="flex items-center gap-2.5 overflow-hidden">
                    <div className="w-8 h-8 rounded-lg bg-purple-500/20 flex items-center justify-center shrink-0">
                      <Cpu className="w-4 h-4 text-purple-400" />
                    </div>
                    <div>
                      <div className="text-xs sm:text-sm font-bold truncate" style={{ color: "var(--text-heading)" }}>
                        RustRover-2026.1.exe
                      </div>
                      <div className="text-[11px]" style={{ color: "var(--text-muted)" }}>
                        1.12 GB of 1.20 GB • 8 Chunks • ETA: 4s
                      </div>
                    </div>
                  </div>
                  <div className="text-right shrink-0">
                    <span className="text-xs sm:text-sm font-mono font-bold text-emerald-400">
                      18.2 MB/s
                    </span>
                    <div className="text-[11px] font-semibold text-indigo-400">94%</div>
                  </div>
                </div>
                <div className="w-full h-2 rounded-full overflow-hidden bg-slate-800/40">
                  <div className="h-full bg-gradient-to-r from-indigo-500 to-purple-500 rounded-full w-[94%]" />
                </div>
              </div>

              {/* Item 3: DeepSeek Model Weights */}
              <div className="p-4 rounded-xl border transition-all duration-200"
                style={{
                  backgroundColor: isDark ? "#111827" : "#FFFFFF",
                  borderColor: isDark ? "#1F2937" : "#E2E8F0",
                }}
              >
                <div className="flex items-center justify-between gap-3 mb-2">
                  <div className="flex items-center gap-2.5 overflow-hidden">
                    <div className="w-8 h-8 rounded-lg bg-pink-500/20 flex items-center justify-center shrink-0">
                      <Layers className="w-4 h-4 text-pink-400" />
                    </div>
                    <div>
                      <div className="text-xs sm:text-sm font-bold truncate" style={{ color: "var(--text-heading)" }}>
                        DeepSeek_v3_Model_Weights.bin
                      </div>
                      <div className="text-[11px]" style={{ color: "var(--text-muted)" }}>
                        4.82 GB of 14.20 GB • 16 Chunks Active • ETA: 2m 14s
                      </div>
                    </div>
                  </div>
                  <div className="text-right shrink-0">
                    <span className="text-xs sm:text-sm font-mono font-bold text-emerald-400">
                      58.7 MB/s
                    </span>
                    <div className="text-[11px] font-semibold text-indigo-400">34%</div>
                  </div>
                </div>
                <div className="w-full h-2 rounded-full overflow-hidden bg-slate-800/40">
                  <div className="h-full bg-gradient-to-r from-pink-500 via-purple-500 to-indigo-500 rounded-full w-[34%]" />
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}
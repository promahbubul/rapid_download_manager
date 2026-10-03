"use client";

import React, { useState } from "react";
import Link from "next/link";
import {
  CheckCircle2,
  ExternalLink,
  Maximize2,
  X,
} from "lucide-react";
import { GithubIcon, WindowsIcon } from "./Icons";
import { useTheme } from "../context/ThemeContext";
import { assetUrl } from "@/utils/assets";

export default function Hero() {
  const { resolvedTheme } = useTheme();
  const isDark = resolvedTheme === "dark";
  const [lightboxOpen, setLightboxOpen] = useState(false);

  const repoBase = "https://github.com/promahbubul/rapid_download_manager";
  const exeUrl = `${repoBase}/releases/download/v1.0.5/RapidDownloadManager_Setup_v1.0.5.exe`;
  const msixUrl = `${repoBase}/raw/main/dist/installer/RapidDownloadManager_v1.0.5.msix`;
  const zipUrl = `${repoBase}/releases/download/v1.0.5/RapidDownloadManager_v1.0.5_Portable.zip`;

  return (
    <section className="relative pt-10 pb-16 overflow-hidden">
      {/* Subtle Ambient Radial Lighting */}
      <div className="absolute top-0 left-1/2 -translate-x-1/2 w-[800px] h-[350px] bg-indigo-500/10 dark:bg-indigo-500/15 rounded-full blur-3xl pointer-events-none -z-10" />

      <div className="max-w-5xl mx-auto px-4 sm:px-6 lg:px-8 text-center space-y-6">
        {/* Release Pill Badge */}
        <div
          className="inline-flex items-center gap-2 px-3.5 py-1 rounded-full border text-xs font-medium backdrop-blur-md"
          style={{
            backgroundColor: isDark ? "rgba(30, 41, 59, 0.6)" : "rgba(255, 255, 255, 0.9)",
            borderColor: isDark ? "rgba(99, 102, 241, 0.3)" : "rgba(99, 102, 241, 0.2)",
            color: isDark ? "#c7d2fe" : "#4338ca",
          }}
        >
          <img src={assetUrl("/assets/app_icon.png")} alt="icon" className="w-3.5 h-3.5" />
          <span className="font-semibold">Version 1.0.5</span>
          <span className="opacity-40">•</span>
          <span>Windows 10 & 11 (64-bit)</span>
          <span className="opacity-40">•</span>
          <span className="text-emerald-400 font-semibold">100% Free & Open Source</span>
        </div>

        {/* Headline */}
        <div className="space-y-3 max-w-3xl mx-auto">
          <h1
            className="text-4xl sm:text-5xl font-black tracking-tight leading-[1.15]"
            style={{ color: "var(--text-heading)" }}
          >
            High-Speed Download Accelerator.{" "}
            <span className="bg-gradient-to-r from-indigo-500 via-purple-500 to-indigo-400 bg-clip-text text-transparent">
              Built in Rust.
            </span>
          </h1>

          <p
            className="text-sm sm:text-base leading-relaxed max-w-xl mx-auto"
            style={{ color: "var(--text-muted)" }}
          >
            Up to 32 parallel streams, recursive cloud folder acceleration, and 1-click browser integration. Ultra-lightweight with zero telemetry.
          </p>
        </div>

        {/* Direct Download Actions */}
        <div className="space-y-3 pt-1">
          <div className="flex flex-wrap items-center justify-center gap-2.5">
            {/* Primary Setup .exe Button */}
            <a
              href={exeUrl}
              className="inline-flex items-center gap-2 px-5 py-2.5 rounded-xl bg-indigo-600 hover:bg-indigo-500 text-white font-semibold text-xs sm:text-sm shadow-md shadow-indigo-600/25 transition-all hover:scale-102 cursor-pointer"
            >
              <WindowsIcon className="w-4 h-4" />
              <span>Download Setup (.exe)</span>
              <span className="text-[10px] opacity-80 font-mono px-1.5 py-0.2 rounded bg-white/20">
                v1.0.5
              </span>
            </a>

            {/* MSIX Store Package Button */}
            <a
              href={msixUrl}
              className="inline-flex items-center gap-2 px-4 py-2.5 rounded-xl border text-xs sm:text-sm font-semibold transition-colors hover:bg-slate-500/10 cursor-pointer"
              style={{
                borderColor: "var(--border-subtle)",
                backgroundColor: "var(--bg-card)",
                color: "var(--text-heading)",
              }}
            >
              <span>Store (MSIX)</span>
            </a>

            {/* Portable .zip Button */}
            <a
              href={zipUrl}
              className="inline-flex items-center gap-2 px-4 py-2.5 rounded-xl border text-xs sm:text-sm font-semibold transition-colors hover:bg-slate-500/10 cursor-pointer"
              style={{
                borderColor: "var(--border-subtle)",
                backgroundColor: "var(--bg-card)",
                color: "var(--text-heading)",
              }}
            >
              <span>Portable (.zip)</span>
            </a>

            {/* GitHub Star Button */}
            <a
              href="https://github.com/promahbubul/rapid_download_manager"
              target="_blank"
              rel="noopener noreferrer"
              className="p-2.5 rounded-xl border transition-colors hover:bg-slate-500/10 cursor-pointer flex items-center justify-center"
              style={{
                borderColor: "var(--border-subtle)",
                backgroundColor: "var(--bg-card)",
                color: "var(--text-heading)",
              }}
              title="Star on GitHub"
            >
              <GithubIcon className="w-4 h-4" />
            </a>
          </div>

          {/* Secondary Browser Extension Link */}
          <div className="flex items-center justify-center gap-1.5 text-xs" style={{ color: "var(--text-muted)" }}>
            <span>Auto-capture downloads with</span>
            <Link href="/extension" className="text-indigo-400 hover:underline font-medium">
              Companion Browser Extension →
            </Link>
          </div>
        </div>

        {/* Real Application Window Preview (Hero Showcase) */}
        <div className="pt-6 max-w-5xl mx-auto">
          <div
            className="rounded-2xl border shadow-2xl overflow-hidden transition-all duration-300 relative group"
            style={{
              backgroundColor: isDark ? "#0f172a" : "#ffffff",
              borderColor: isDark ? "rgba(99, 102, 241, 0.25)" : "rgba(226, 232, 240, 1)",
              boxShadow: isDark
                ? "0 25px 60px -15px rgba(0, 0, 0, 0.7), 0 0 40px rgba(99, 102, 241, 0.15)"
                : "0 20px 45px -10px rgba(0, 0, 0, 0.1)",
            }}
          >
            {/* Windows 11 Style Title Bar */}
            <div
              className="h-10 px-4 border-b flex items-center justify-between text-xs select-none"
              style={{
                backgroundColor: isDark ? "rgba(15, 23, 42, 0.95)" : "rgba(241, 245, 249, 0.95)",
                borderColor: "var(--border-subtle)",
              }}
            >
              <div className="flex items-center gap-2">
                <img src={assetUrl("/assets/app_icon.png")} alt="app icon" className="w-4 h-4" />
                <span className="font-semibold text-[11px]" style={{ color: "var(--text-heading)" }}>
                  Rapid Download Manager v1.0.5
                </span>
                <span className="px-1.5 py-0.2 rounded text-[10px] bg-emerald-500/15 text-emerald-400 font-mono">
                  ACTIVE
                </span>
              </div>
              <div className="flex items-center gap-2">
                <button
                  onClick={() => setLightboxOpen(true)}
                  className="flex items-center gap-1 px-2.5 py-1 rounded text-[11px] font-medium transition-colors hover:bg-slate-500/10 cursor-pointer"
                  style={{ color: "var(--text-muted)" }}
                >
                  <Maximize2 className="w-3 h-3" />
                  <span>Expand Preview</span>
                </button>
                <div className="flex items-center gap-1.5 ml-2">
                  <span className="w-2.5 h-2.5 rounded-full bg-slate-400/40" />
                  <span className="w-2.5 h-2.5 rounded-full bg-slate-400/40" />
                  <span className="w-2.5 h-2.5 rounded-full bg-rose-500/80" />
                </div>
              </div>
            </div>

            {/* Dashboard Screenshot */}
            <div className="relative cursor-pointer overflow-hidden" onClick={() => setLightboxOpen(true)}>
              <img
                src={assetUrl("/assets/screenshot_dashboard.png")}
                alt="Rapid Download Manager Dashboard"
                className="w-full h-auto object-cover transition-transform duration-500 group-hover:scale-[1.01]"
              />
              <div className="absolute inset-0 bg-gradient-to-t from-black/40 via-transparent to-transparent opacity-0 group-hover:opacity-100 transition-opacity flex items-end justify-center pb-6">
                <span className="px-4 py-2 rounded-xl bg-slate-900/90 text-white text-xs font-semibold backdrop-blur-md flex items-center gap-2 shadow-lg">
                  <Maximize2 className="w-3.5 h-3.5" />
                  <span>Click to view full-resolution dashboard</span>
                </span>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Lightbox Modal */}
      {lightboxOpen && (
        <div
          className="fixed inset-0 z-50 bg-black/85 backdrop-blur-md flex items-center justify-center p-4 sm:p-8 animate-in fade-in duration-200"
          onClick={() => setLightboxOpen(false)}
        >
          <div className="relative max-w-6xl w-full max-h-[90vh] flex flex-col items-center">
            <button
              onClick={() => setLightboxOpen(false)}
              className="absolute -top-12 right-0 p-2 text-white/80 hover:text-white transition-colors cursor-pointer"
              title="Close"
            >
              <X className="w-6 h-6" />
            </button>
            <img
              src={assetUrl("/assets/screenshot_dashboard.png")}
              alt="Rapid Download Manager Full View"
              className="w-full h-auto max-h-[85vh] object-contain rounded-xl shadow-2xl border border-white/10"
              onClick={(e) => e.stopPropagation()}
            />
          </div>
        </div>
      )}
    </section>
  );
}

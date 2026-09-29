"use client";

import React from "react";
import Link from "next/link";
import Navbar from "@/components/Navbar";
import Footer from "@/components/Footer";
import { Sparkles, ArrowLeft, Download, CheckCircle2, Tag, Calendar } from "lucide-react";
import { useTheme } from "@/context/ThemeContext";

export default function ChangelogPage() {
  const { resolvedTheme } = useTheme();
  const isDark = resolvedTheme === "dark";

  const releases = [
    {
      version: "v1.0.4",
      date: "September 26, 2026",
      tagline: "First Official General Availability (GA) Production Release",
      isLatest: true,
      features: [
        "16-chunk dynamic HTTP byte-range segmentation engine written in Tokio/Rustls.",
        "Full desktop GUI with Cyber-Obsidian dark mode, high-DPI scaling, and real-time chunk visualizer.",
        "Companion browser extension with 1-click automatic browser hooking for Chrome, Edge, and Brave.",
        "Dual-layer local storage architecture with atomic flush and automatic recovery backups.",
        "Zero-telemetry privacy design with automated credential redactor for all diagnostic logs.",
        "Native Win32 security integration: runs under standard user privileges (asInvoker) without UAC prompts.",
        "WACK certified official Microsoft Store package (MSIX) and standalone Inno Setup installer.",
      ],
      fixes: [
        "Fixed OPC content type manifest schema for seamless Microsoft Store ingestion.",
        "Optimized TCP socket buffer memory usage to sub-25MB under heavy 16-chunk download load.",
      ],
    },
  ];

  return (
    <div className="min-h-screen flex flex-col">
      <Navbar />

      <main className="flex-1 max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
        <Link
          href="/"
          className="inline-flex items-center gap-2 text-xs font-semibold text-indigo-400 hover:text-indigo-300 mb-6 transition-colors"
        >
          <ArrowLeft className="w-4 h-4" />
          <span>Back to Home</span>
        </Link>

        {/* Page Header */}
        <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 mb-12 pb-6 border-b"
          style={{ borderColor: "var(--border-subtle)" }}
        >
          <div className="space-y-1">
            <div className="flex items-center gap-2">
              <div className="w-9 h-9 rounded-xl bg-indigo-500/15 text-indigo-400 flex items-center justify-center">
                <Tag className="w-5 h-5" />
              </div>
              <h1 className="text-3xl font-extrabold tracking-tight" style={{ color: "var(--text-heading)" }}>
                Release Notes & Changelog
              </h1>
            </div>
            <p className="text-xs sm:text-sm" style={{ color: "var(--text-muted)" }}>
              Detailed tracking of every version, improvement, and security patch.
            </p>
          </div>

          <a
            href="https://github.com/promahbubul/rapid_download_manager/releases"
            target="_blank"
            rel="noopener noreferrer"
            className="inline-flex items-center gap-2 px-4 py-2 rounded-xl border text-xs font-semibold hover:border-indigo-500 transition-colors"
            style={{
              backgroundColor: "var(--bg-card)",
              borderColor: "var(--border-subtle)",
              color: "var(--text-heading)",
            }}
          >
            <span>GitHub Releases</span>
          </a>
        </div>

        {/* Release Timeline */}
        <div className="space-y-12">
          {releases.map((rel, idx) => (
            <div
              key={idx}
              className="p-6 sm:p-8 rounded-2xl border space-y-6 relative overflow-hidden"
              style={{
                backgroundColor: "var(--bg-card)",
                borderColor: "var(--border-subtle)",
              }}
            >
              {rel.isLatest && (
                <div className="absolute top-0 right-0 bg-gradient-to-l from-emerald-500 to-indigo-500 text-white text-[10px] font-bold px-3 py-1 rounded-bl-xl uppercase tracking-wider">
                  Latest Stable
                </div>
              )}

              {/* Version & Date */}
              <div className="space-y-1">
                <div className="flex items-center gap-3">
                  <span className="text-2xl font-black font-mono text-indigo-400">{rel.version}</span>
                  <span className="text-xs font-medium flex items-center gap-1" style={{ color: "var(--text-muted)" }}>
                    <Calendar className="w-3.5 h-3.5" />
                    <span>{rel.date}</span>
                  </span>
                </div>
                <p className="text-sm font-semibold" style={{ color: "var(--text-heading)" }}>
                  {rel.tagline}
                </p>
              </div>

              {/* New Features */}
              <div className="space-y-2">
                <h3 className="text-xs font-bold uppercase tracking-wider text-emerald-400">
                  New Features & Capabilities
                </h3>
                <ul className="space-y-1.5 text-xs" style={{ color: "var(--text-body)" }}>
                  {rel.features.map((f, i) => (
                    <li key={i} className="flex items-start gap-2">
                      <CheckCircle2 className="w-4 h-4 text-emerald-400 shrink-0 mt-0.5" />
                      <span>{f}</span>
                    </li>
                  ))}
                </ul>
              </div>

              {/* Bug Fixes */}
              <div className="space-y-2">
                <h3 className="text-xs font-bold uppercase tracking-wider text-cyan-400">
                  Reliability & Bug Fixes
                </h3>
                <ul className="space-y-1.5 text-xs" style={{ color: "var(--text-body)" }}>
                  {rel.fixes.map((fx, i) => (
                    <li key={i} className="flex items-start gap-2">
                      <CheckCircle2 className="w-4 h-4 text-cyan-400 shrink-0 mt-0.5" />
                      <span>{fx}</span>
                    </li>
                  ))}
                </ul>
              </div>

              {/* Download Buttons for this release */}
              <div className="pt-4 border-t flex flex-wrap gap-3" style={{ borderColor: "var(--border-subtle)" }}>
                <a
                  href="https://github.com/promahbubul/rapid_download_manager/releases/latest"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="px-4 py-2 rounded-xl bg-indigo-600 text-white text-xs font-semibold flex items-center gap-2 hover:bg-indigo-500 transition-colors"
                >
                  <Download className="w-3.5 h-3.5" />
                  <span>Download .EXE (5.72 MB)</span>
                </a>

                <a
                  href="https://github.com/promahbubul/rapid_download_manager/releases/latest"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="px-4 py-2 rounded-xl border text-xs font-semibold flex items-center gap-2 hover:border-indigo-500 transition-colors"
                  style={{
                    backgroundColor: "var(--bg-page)",
                    borderColor: "var(--border-subtle)",
                    color: "var(--text-heading)",
                  }}
                >
                  <Download className="w-3.5 h-3.5" />
                  <span>Download .MSIX (5.71 MB)</span>
                </a>
              </div>
            </div>
          ))}
        </div>
      </main>

      <Footer />
    </div>
  );
}

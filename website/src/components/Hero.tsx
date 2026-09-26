"use client";

import React, { useState, useEffect } from "react";
import {
  Download,
  Apple,
  Terminal,
  CheckCircle2,
  ChevronRight,
  Shield,
  Zap,
  Cpu,
  Eye,
} from "lucide-react";
import { GithubIcon, WindowsIcon, ChromeIcon } from "./Icons";
import { useTheme } from "../context/ThemeContext";

type PlatformType = "windows" | "macos" | "linux";

export default function Hero() {
  const { resolvedTheme } = useTheme();
  const isDark = resolvedTheme === "dark";

  const [detectedPlatform, setDetectedPlatform] = useState<PlatformType>("windows");
  const [detectedLabel, setDetectedLabel] = useState("Windows 10 / 11 (64-bit)");

  useEffect(() => {
    if (typeof window === "undefined") return;
    const ua = window.navigator.userAgent.toLowerCase();
    if (ua.includes("win")) {
      setDetectedPlatform("windows");
      setDetectedLabel("Windows 10 / 11 (64-bit)");
    } else if (ua.includes("mac") || ua.includes("darwin")) {
      setDetectedPlatform("macos");
      setDetectedLabel("macOS (Apple Silicon & Intel)");
    } else if (ua.includes("linux")) {
      setDetectedPlatform("linux");
      setDetectedLabel("Linux (.deb / AppImage)");
    } else {
      setDetectedPlatform("windows");
      setDetectedLabel("Windows 10 / 11 (64-bit)");
    }
  }, []);

  return (
    <section className="relative pt-12 pb-16 overflow-hidden">
      {/* Subtle Ambient Radial Lighting */}
      <div className="absolute top-0 left-1/2 -translate-x-1/2 w-[800px] h-[350px] bg-indigo-500/10 dark:bg-indigo-500/15 rounded-full blur-3xl pointer-events-none -z-10" />

      <div className="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 text-center space-y-8">
        {/* Release Pill Badge */}
        <div className="inline-flex items-center gap-2.5 px-4 py-1.5 rounded-full border text-xs font-medium backdrop-blur-md"
          style={{
            backgroundColor: isDark ? "rgba(30, 41, 59, 0.6)" : "rgba(255, 255, 255, 0.9)",
            borderColor: isDark ? "rgba(99, 102, 241, 0.3)" : "rgba(99, 102, 241, 0.2)",
            color: isDark ? "#c7d2fe" : "#4338ca",
          }}
        >
          <img src="/assets/app_icon.png" alt="icon" className="w-3.5 h-3.5" />
          <span className="font-semibold">Rapid Download Manager v1.0.0 GA</span>
          <span className="opacity-40">•</span>
          <span>Pure Memory-Safe Rust</span>
        </div>

        {/* Headline */}
        <div className="space-y-4 max-w-4xl mx-auto">
          <h1
            className="text-4xl sm:text-6xl font-black tracking-tight leading-[1.12]"
            style={{ color: "var(--text-heading)" }}
          >
            High-Speed Download Accelerator.{" "}
            <span className="bg-gradient-to-r from-indigo-500 via-purple-500 to-indigo-400 bg-clip-text text-transparent">
              Engineered for Power Users.
            </span>
          </h1>

          <p
            className="text-base sm:text-lg leading-relaxed max-w-2xl mx-auto"
            style={{ color: "var(--text-muted)" }}
          >
            Turbocharged file downloads powered by 16-stream parallel range socket balancing, automated browser interception, and memory-safe Rust cryptography. Free, lightweight, and zero telemetry.
          </p>
        </div>

        {/* Dynamic Device-Aware CTA & Platform Selector */}
        <div className="space-y-4 pt-2">
          <div className="flex flex-wrap items-center justify-center gap-3">
            {/* Primary Download Button for Detected Device */}
            <a
              href="#download"
              className="inline-flex items-center gap-3 px-6 py-3.5 rounded-xl bg-indigo-600 hover:bg-indigo-500 text-white font-semibold text-sm sm:text-base shadow-lg shadow-indigo-600/20 transition-all hover:scale-102"
            >
              {detectedPlatform === "windows" && <WindowsIcon className="w-5 h-5" />}
              {detectedPlatform === "macos" && <Apple className="w-5 h-5" />}
              {detectedPlatform === "linux" && <Terminal className="w-5 h-5" />}
              <span>Download for {detectedPlatform === "windows" ? "Windows" : detectedPlatform === "macos" ? "macOS" : "Linux"}</span>
              <span className="text-xs opacity-75 font-normal px-2 py-0.5 rounded-md bg-white/20">
                5.7 MB
              </span>
            </a>

            {/* GitHub Star Button */}
            <a
              href="https://github.com/promahbubul/rapid_download_manager"
              target="_blank"
              rel="noopener noreferrer"
              className="inline-flex items-center gap-2 px-5 py-3.5 rounded-xl border text-sm font-semibold transition-colors hover:bg-slate-500/10"
              style={{
                borderColor: "var(--border-subtle)",
                backgroundColor: "var(--bg-card)",
                color: "var(--text-heading)",
              }}
            >
              <GithubIcon className="w-4 h-4" />
              <span>GitHub</span>
            </a>
          </div>

          {/* Quick Platform Switcher Navigation */}
          <div className="flex flex-wrap items-center justify-center gap-2 text-xs" style={{ color: "var(--text-muted)" }}>
            <span className="flex items-center gap-1.5 text-emerald-400 font-medium">
              <CheckCircle2 className="w-3.5 h-3.5" />
              <span>Detected: {detectedLabel}</span>
            </span>
            <span className="opacity-40">•</span>
            <span>Switch to:</span>
            <a href="#download" className="hover:text-indigo-400 font-medium underline">Windows (.msix / .exe)</a>
            <span className="opacity-40">•</span>
            <a href="#download" className="hover:text-indigo-400 font-medium underline">macOS (.dmg)</a>
            <span className="opacity-40">•</span>
            <a href="#download" className="hover:text-indigo-400 font-medium underline">Linux (.deb / AppImage)</a>
            <span className="opacity-40">•</span>
            <a href="/extension" className="hover:text-indigo-400 font-medium underline">Browser Extension</a>
          </div>
        </div>

        {/* 4 Feature Highlights */}
        <div className="grid grid-cols-2 sm:grid-cols-4 gap-3 sm:gap-4 pt-4 max-w-3xl mx-auto text-left">
          <div className="p-3.5 rounded-xl border" style={{ borderColor: "var(--border-subtle)", backgroundColor: "var(--bg-card)" }}>
            <div className="flex items-center gap-2 text-indigo-400 mb-1">
              <Zap className="w-4 h-4" />
              <span className="text-xs font-semibold uppercase tracking-wider">Speed</span>
            </div>
            <div className="text-lg font-bold" style={{ color: "var(--text-heading)" }}>16x Parallel</div>
            <div className="text-xs" style={{ color: "var(--text-muted)" }}>Socket chunk balancing</div>
          </div>

          <div className="p-3.5 rounded-xl border" style={{ borderColor: "var(--border-subtle)", backgroundColor: "var(--bg-card)" }}>
            <div className="flex items-center gap-2 text-emerald-400 mb-1">
              <Cpu className="w-4 h-4" />
              <span className="text-xs font-semibold uppercase tracking-wider">Memory</span>
            </div>
            <div className="text-lg font-bold text-emerald-400">&lt; 25 MB</div>
            <div className="text-xs" style={{ color: "var(--text-muted)" }}>Idle RAM footprint</div>
          </div>

          <div className="p-3.5 rounded-xl border" style={{ borderColor: "var(--border-subtle)", backgroundColor: "var(--bg-card)" }}>
            <div className="flex items-center gap-2 text-cyan-400 mb-1">
              <Shield className="w-4 h-4" />
              <span className="text-xs font-semibold uppercase tracking-wider">Safety</span>
            </div>
            <div className="text-lg font-bold" style={{ color: "var(--text-heading)" }}>100% Rust</div>
            <div className="text-xs" style={{ color: "var(--text-muted)" }}>Memory-safe Tokio core</div>
          </div>

          <div className="p-3.5 rounded-xl border" style={{ borderColor: "var(--border-subtle)", backgroundColor: "var(--bg-card)" }}>
            <div className="flex items-center gap-2 text-purple-400 mb-1">
              <Eye className="w-4 h-4" />
              <span className="text-xs font-semibold uppercase tracking-wider">Privacy</span>
            </div>
            <div className="text-lg font-bold" style={{ color: "var(--text-heading)" }}>0 Telemetry</div>
            <div className="text-xs" style={{ color: "var(--text-muted)" }}>Audited zero logging</div>
          </div>
        </div>
      </div>
    </section>
  );
}

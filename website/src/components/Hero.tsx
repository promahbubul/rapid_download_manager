"use client";

import React, { useState, useEffect } from "react";
import {
  Download,
  Sparkles,
  Apple,
  Terminal,
  ArrowDownToLine,
  CheckCircle2,
  ChevronDown,
  Layers,
  Activity,
  Cpu,
} from "lucide-react";
import { GithubIcon, WindowsIcon, ChromeIcon } from "./Icons";
import { useTheme } from "../context/ThemeContext";

type PlatformType = "windows" | "macos" | "linux" | "other";

export default function Hero() {
  const { resolvedTheme } = useTheme();
  const isDark = resolvedTheme === "dark";

  const [detectedPlatform, setDetectedPlatform] = useState<PlatformType>("windows");
  const [detectedLabel, setDetectedLabel] = useState("Windows (64-bit)");

  useEffect(() => {
    if (typeof window === "undefined") return;
    const ua = window.navigator.userAgent.toLowerCase();
    if (ua.includes("win")) {
      setDetectedPlatform("windows");
      setDetectedLabel("Windows 10/11 (x64 / ARM64)");
    } else if (ua.includes("mac") || ua.includes("darwin")) {
      setDetectedPlatform("macos");
      setDetectedLabel("macOS (Apple Silicon & Intel)");
    } else if (ua.includes("linux")) {
      setDetectedPlatform("linux");
      setDetectedLabel("Linux (.deb / AppImage)");
    } else {
      setDetectedPlatform("windows");
      setDetectedLabel("Universal Cross-Platform");
    }
  }, []);

  return (
    <section className="relative pt-10 pb-20 overflow-hidden">
      {/* Dynamic 3D Radial Glow Backdrop */}
      <div className="absolute top-1/4 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[720px] h-[450px] bg-gradient-to-tr from-indigo-600/25 via-purple-600/20 to-pink-500/15 rounded-full blur-3xl pointer-events-none -z-10" />

      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="text-center max-w-4xl mx-auto space-y-6">
          {/* Top Holographic Pill */}
          <div
            className="inline-flex items-center gap-2 px-4 py-1.5 rounded-full border text-xs font-semibold backdrop-blur-md shadow-sm transition-all hover:scale-102"
            style={{
              backgroundColor: isDark ? "rgba(17, 24, 39, 0.7)" : "rgba(255, 255, 255, 0.85)",
              borderColor: isDark ? "rgba(99, 102, 241, 0.4)" : "rgba(99, 102, 241, 0.3)",
              color: isDark ? "#a5b4fc" : "#4f46e5",
            }}
          >
            <Sparkles className="w-3.5 h-3.5 text-indigo-400 animate-pulse" />
            <span>Pure Memory-Safe Rust • 16-Socket Chunk Accelerator • 0 Telemetry</span>
          </div>

          {/* Official Software Logo 3D Holographic Card */}
          <div className="flex justify-center items-center py-2">
            <div
              className="relative group p-4 sm:p-5 rounded-3xl border backdrop-blur-2xl shadow-2xl transition-all duration-300 hover:scale-104 cursor-pointer"
              style={{
                backgroundColor: isDark ? "rgba(17, 24, 39, 0.75)" : "rgba(255, 255, 255, 0.85)",
                borderColor: isDark ? "rgba(99, 102, 241, 0.4)" : "rgba(99, 102, 241, 0.25)",
                boxShadow: isDark
                  ? "0 20px 50px -10px rgba(99, 102, 241, 0.3), 0 0 30px rgba(168, 85, 247, 0.2)"
                  : "0 20px 40px -10px rgba(99, 102, 241, 0.15)",
              }}
            >
              {/* Outer Rotating Glowing Ring */}
              <div className="absolute -inset-0.5 rounded-3xl bg-gradient-to-r from-indigo-500 via-purple-500 to-pink-500 opacity-30 blur-sm group-hover:opacity-70 transition duration-500" />

              <div className="relative flex items-center gap-4">
                <img
                  src="/assets/app_icon.png"
                  alt="Rapid Download Manager Official Logo"
                  className="w-14 h-14 sm:w-16 sm:h-16 object-contain drop-shadow-xl group-hover:rotate-6 transition-transform duration-300"
                />
                <div className="text-left">
                  <div className="flex items-center gap-2">
                    <span className="text-lg sm:text-2xl font-black tracking-tight" style={{ color: "var(--text-heading)" }}>
                      Rapid Download Manager
                    </span>
                    <span className="text-[10px] font-mono px-2 py-0.5 rounded-full bg-emerald-500/20 text-emerald-400 border border-emerald-500/30">
                      v1.0.0 GA
                    </span>
                  </div>
                  <p className="text-xs sm:text-sm text-slate-400">
                    Official Release • Powered by Tokio & Rustls 0.23
                  </p>
                </div>
              </div>
            </div>
          </div>

          {/* Hero Headline */}
          <h1
            className="text-4xl sm:text-6xl lg:text-7xl font-black tracking-tight leading-[1.12]"
            style={{ color: "var(--text-heading)" }}
          >
            Turbocharged Speed.{" "}
            <span className="bg-gradient-to-r from-indigo-400 via-purple-400 to-pink-400 bg-clip-text text-transparent">
              Zero Compromises.
            </span>
          </h1>

          <p
            className="text-base sm:text-lg leading-relaxed max-w-2xl mx-auto"
            style={{ color: "var(--text-muted)" }}
          >
            Experience lightning-fast downloads powered by 16-stream parallel range socket balancing, automated browser integration, and memory-safe Rust cryptography. Free, open source, and lightweight.
          </p>

          {/* Cross-Platform Device-Aware CTA System */}
          <div className="pt-3 space-y-3">
            <div className="flex flex-wrap items-center justify-center gap-3">
              {/* Primary 1-Click Adaptive Download Button */}
              <a
                href="#download"
                className="inline-flex items-center gap-3 px-6 py-4 rounded-2xl bg-gradient-to-r from-indigo-600 via-indigo-500 to-violet-600 text-white font-bold text-sm sm:text-base shadow-xl shadow-indigo-500/30 hover:shadow-indigo-500/50 hover:scale-103 transition-all duration-200 cursor-pointer"
              >
                {detectedPlatform === "windows" && <WindowsIcon className="w-5 h-5" />}
                {detectedPlatform === "macos" && <Apple className="w-5 h-5" />}
                {detectedPlatform === "linux" && <Terminal className="w-5 h-5" />}
                <span>Download for {detectedPlatform === "windows" ? "Windows" : detectedPlatform === "macos" ? "macOS" : "Linux"}</span>
                <span className="text-xs opacity-80 font-normal px-2 py-0.5 rounded-full bg-white/20">
                  v1.0.0
                </span>
              </a>

              {/* GitHub Button */}
              <a
                href="https://github.com/promahbubul/rapid_download_manager"
                target="_blank"
                rel="noopener noreferrer"
                className="inline-flex items-center gap-2.5 px-5 py-4 rounded-2xl border text-sm font-semibold transition-all hover:scale-102 cursor-pointer"
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

            {/* Smart Detection & Platform Switcher Link */}
            <div className="flex items-center justify-center gap-2 text-xs text-slate-400">
              <span className="flex items-center gap-1.5 font-medium">
                <CheckCircle2 className="w-3.5 h-3.5 text-emerald-400" />
                <span>Detected for your system: <strong className="text-indigo-400">{detectedLabel}</strong></span>
              </span>
              <span>•</span>
              <a
                href="#download"
                className="text-indigo-400 hover:text-indigo-300 underline font-semibold flex items-center gap-0.5"
              >
                <span>Select other platforms & formats</span>
                <ChevronDown className="w-3 h-3" />
              </a>
            </div>
          </div>

          {/* 4 Trust Metrics */}
          <div className="grid grid-cols-2 sm:grid-cols-4 gap-3 sm:gap-4 pt-6 max-w-3xl mx-auto text-left">
            <div className="p-3.5 rounded-2xl border backdrop-blur-md" style={{ borderColor: "var(--border-subtle)", backgroundColor: "var(--bg-card)" }}>
              <div className="text-xl sm:text-2xl font-black bg-gradient-to-r from-indigo-400 to-purple-400 bg-clip-text text-transparent">Up to 16x</div>
              <div className="text-xs font-medium" style={{ color: "var(--text-muted)" }}>Chunk Socket Acceleration</div>
            </div>
            <div className="p-3.5 rounded-2xl border backdrop-blur-md" style={{ borderColor: "var(--border-subtle)", backgroundColor: "var(--bg-card)" }}>
              <div className="text-xl sm:text-2xl font-black text-emerald-400">&lt; 25 MB</div>
              <div className="text-xs font-medium" style={{ color: "var(--text-muted)" }}>Idle RAM Footprint</div>
            </div>
            <div className="p-3.5 rounded-2xl border backdrop-blur-md" style={{ borderColor: "var(--border-subtle)", backgroundColor: "var(--bg-card)" }}>
              <div className="text-xl sm:text-2xl font-black text-cyan-400">100% Rust</div>
              <div className="text-xs font-medium" style={{ color: "var(--text-muted)" }}>Memory-Safe Cryptography</div>
            </div>
            <div className="p-3.5 rounded-2xl border backdrop-blur-md" style={{ borderColor: "var(--border-subtle)", backgroundColor: "var(--bg-card)" }}>
              <div className="text-xl sm:text-2xl font-black text-purple-400">0 Logs</div>
              <div className="text-xs font-medium" style={{ color: "var(--text-muted)" }}>Zero Telemetry Privacy</div>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}

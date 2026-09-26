"use client";

import React, { useState, useRef } from "react";
import {
  Layers,
  Sparkles,
  Maximize2,
  X,
  Zap,
  Activity,
  ShieldCheck,
  Cpu,
  ArrowRight,
  Globe,
  Clock,
  Info,
} from "lucide-react";
import { useTheme } from "../context/ThemeContext";

interface ViewItem {
  id: string;
  name: string;
  badge: string;
  icon: React.ReactNode;
  img: string;
  title: string;
  desc: string;
  highlights: string[];
}

export default function VisualShowcase3D() {
  const { resolvedTheme } = useTheme();
  const isDark = resolvedTheme === "dark";

  const [activeTab, setActiveTab] = useState("dashboard");
  const [modalOpen, setModalOpen] = useState(false);

  // 3D Tilt Ref & state
  const frameRef = useRef<HTMLDivElement>(null);
  const [rotX, setRotX] = useState(6);
  const [rotY, setRotY] = useState(-8);
  const [glare, setGlare] = useState({ x: 50, y: 50, opacity: 0 });

  const views: ViewItem[] = [
    {
      id: "dashboard",
      name: "Live Dashboard",
      badge: "16 Sockets Active",
      icon: <Activity className="w-4 h-4 text-emerald-400" />,
      img: "/assets/screenshot_dashboard.png",
      title: "Real-Time 16-Segment Download Engine",
      desc: "Live visual tracking of parallel HTTP socket ranges, transfer rates, ETA, chunk completion integrity, and pause/resume states.",
      highlights: [
        "16 parallel HTTP Range sockets per file",
        "Visual segmented multi-color progress bars",
        "Zero memory thrashing with direct disk streaming",
      ],
    },
    {
      id: "add",
      name: "New Download Dialog",
      badge: "Batch & Headers",
      icon: <Zap className="w-4 h-4 text-indigo-400" />,
      img: "/assets/screenshot_add.png",
      title: "Intelligent URL & Header Interception",
      desc: "Paste links, configure custom authentication cookies, specify output directories, and customize stream parallelism on the fly.",
      highlights: [
        "Clipboard auto-detection & instant paste",
        "Custom User-Agent & Bearer token injection",
        "Configurable 1 to 32 parallel chunk threads",
      ],
    },
    {
      id: "browser",
      name: "Browser Bridge",
      badge: "Chrome & Edge",
      icon: <Globe className="w-4 h-4 text-cyan-400" />,
      img: "/assets/screenshot_browser.png",
      title: "Zero-Latency Browser Native Messaging",
      desc: "Automatically intercept downloads from Chrome, Brave, Edge, and Firefox via lightweight Manifest V3 JSON IPC host.",
      highlights: [
        "Native C/Rust binary stdio IPC protocol",
        "Context menu 'Download with Rapid Download Manager'",
        "One-click auto-registration for all browsers",
      ],
    },
    {
      id: "scheduler",
      name: "Task Scheduler",
      badge: "Bandwidth Control",
      icon: <Clock className="w-4 h-4 text-purple-400" />,
      img: "/assets/screenshot_scheduler.png",
      title: "Automated Queue & Speed Throttling",
      desc: "Set overnight download schedules, throttle peak bandwidth limits, and configure automatic system sleep or shutdown.",
      highlights: [
        "Peak-hour bandwidth limiter (e.g. 5 MB/s cap)",
        "Time-based auto start & stop queues",
        "Post-download system sleep / hibernate hooks",
      ],
    },
    {
      id: "about",
      name: "Rust Core Architecture",
      badge: "Tokio Engine",
      icon: <Cpu className="w-4 h-4 text-pink-400" />,
      img: "/assets/screenshot_about.png",
      title: "Pure Memory-Safe Rust Cryptography",
      desc: "Built with Tokio async I/O and Rustls 0.23, providing uncompromised security, zero garbage collection pauses, and tiny RAM usage.",
      highlights: [
        "100% Rust Tokio asynchronous core",
        "Rustls TLS 1.3 cryptographic backend",
        "Audited 0-telemetry privacy policy",
      ],
    },
  ];

  const currentView = views.find((v) => v.id === activeTab) || views[0];

  const handleMouseMove = (e: React.MouseEvent<HTMLDivElement>) => {
    if (!frameRef.current) return;
    const rect = frameRef.current.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;

    const centerX = rect.width / 2;
    const centerY = rect.height / 2;

    const rY = ((x - centerX) / centerX) * 14;
    const rX = -((y - centerY) / centerY) * 12;

    setRotX(rX);
    setRotY(rY);

    const glareX = (x / rect.width) * 100;
    const glareY = (y / rect.height) * 100;
    setGlare({ x: glareX, y: glareY, opacity: 0.16 });
  };

  const handleMouseLeave = () => {
    setRotX(4);
    setRotY(-6);
    setGlare((prev) => ({ ...prev, opacity: 0 }));
  };

  return (
    <section id="showcase" className="py-24 relative overflow-hidden">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        {/* Section Header */}
        <div className="text-center max-w-3xl mx-auto mb-12 space-y-4">
          <div className="inline-flex items-center gap-2 px-3.5 py-1.5 rounded-full text-xs font-semibold text-indigo-400 bg-indigo-500/10 border border-indigo-500/20">
            <Sparkles className="w-3.5 h-3.5" />
            <span>Interactive 3D Experience</span>
          </div>

          <h2 className="text-3xl sm:text-5xl font-extrabold tracking-tight" style={{ color: "var(--text-heading)" }}>
            Engineered for Precision & Speed
          </h2>

          <p className="text-sm sm:text-base leading-relaxed" style={{ color: "var(--text-muted)" }}>
            Explore the real, live native interface of Rapid Download Manager. Designed with sleek Cyber-Obsidian aesthetics and pixel-perfect ergonomics.
          </p>
        </div>

        {/* View Switcher Tabs */}
        <div className="flex flex-wrap items-center justify-center gap-2 sm:gap-3 mb-12">
          {views.map((tab) => {
            const isActive = tab.id === activeTab;
            return (
              <button
                key={tab.id}
                onClick={() => setActiveTab(tab.id)}
                className={`flex items-center gap-2 px-4 py-2.5 rounded-2xl text-xs sm:text-sm font-semibold transition-all duration-200 cursor-pointer ${
                  isActive
                    ? "bg-gradient-to-r from-indigo-600 to-violet-600 text-white shadow-lg shadow-indigo-500/25 scale-103"
                    : "border hover:scale-101"
                }`}
                style={{
                  borderColor: isActive ? "transparent" : "var(--border-subtle)",
                  backgroundColor: isActive ? undefined : "var(--bg-card)",
                  color: isActive ? "#ffffff" : "var(--text-heading)",
                }}
              >
                {tab.icon}
                <span>{tab.name}</span>
                <span
                  className={`text-[10px] px-1.5 py-0.5 rounded-full font-mono ${
                    isActive ? "bg-white/20 text-white" : "bg-slate-500/10 text-slate-400"
                  }`}
                >
                  {tab.badge}
                </span>
              </button>
            );
          })}
        </div>

        {/* 3D Interactive Showcase Frame */}
        <div className="relative max-w-5xl mx-auto perspective-1200">
          {/* Floating 3D Badge 1: Top-Left Speed Gauge */}
          <div
            className="hidden lg:flex absolute -top-6 -left-6 z-20 items-center gap-2.5 px-4 py-2.5 rounded-2xl border backdrop-blur-xl shadow-2xl transition-transform duration-300 hover:scale-105 pointer-events-none"
            style={{
              backgroundColor: isDark ? "rgba(17, 24, 39, 0.85)" : "rgba(255, 255, 255, 0.9)",
              borderColor: isDark ? "rgba(16, 185, 129, 0.4)" : "rgba(16, 185, 129, 0.3)",
            }}
          >
            <div className="w-8 h-8 rounded-xl bg-emerald-500/20 flex items-center justify-center text-emerald-400">
              <Activity className="w-4 h-4 animate-pulse" />
            </div>
            <div>
              <div className="text-[10px] uppercase font-bold tracking-wider text-slate-400">Throughput</div>
              <div className="text-xs sm:text-sm font-bold font-mono text-emerald-400">42.5 MB/s • 16 Sockets</div>
            </div>
          </div>

          {/* Floating 3D Badge 2: Top-Right Integrity Hash */}
          <div
            className="hidden lg:flex absolute -top-6 -right-6 z-20 items-center gap-2.5 px-4 py-2.5 rounded-2xl border backdrop-blur-xl shadow-2xl transition-transform duration-300 hover:scale-105 pointer-events-none"
            style={{
              backgroundColor: isDark ? "rgba(17, 24, 39, 0.85)" : "rgba(255, 255, 255, 0.9)",
              borderColor: isDark ? "rgba(99, 102, 241, 0.4)" : "rgba(99, 102, 241, 0.3)",
            }}
          >
            <div className="w-8 h-8 rounded-xl bg-indigo-500/20 flex items-center justify-center text-indigo-400">
              <ShieldCheck className="w-4 h-4" />
            </div>
            <div>
              <div className="text-[10px] uppercase font-bold tracking-wider text-slate-400">Security</div>
              <div className="text-xs sm:text-sm font-bold font-mono text-indigo-400">100% SHA-256 Valid</div>
            </div>
          </div>

          {/* Floating 3D Badge 3: Bottom-Left RAM Footprint */}
          <div
            className="hidden lg:flex absolute -bottom-6 -left-6 z-20 items-center gap-2.5 px-4 py-2.5 rounded-2xl border backdrop-blur-xl shadow-2xl transition-transform duration-300 hover:scale-105 pointer-events-none"
            style={{
              backgroundColor: isDark ? "rgba(17, 24, 39, 0.85)" : "rgba(255, 255, 255, 0.9)",
              borderColor: isDark ? "rgba(236, 72, 153, 0.4)" : "rgba(236, 72, 153, 0.3)",
            }}
          >
            <div className="w-8 h-8 rounded-xl bg-pink-500/20 flex items-center justify-center text-pink-400">
              <Cpu className="w-4 h-4" />
            </div>
            <div>
              <div className="text-[10px] uppercase font-bold tracking-wider text-slate-400">Resource Footprint</div>
              <div className="text-xs sm:text-sm font-bold font-mono text-pink-400">&lt; 25 MB RAM • 0% CPU Idle</div>
            </div>
          </div>

          {/* 3D Window Container */}
          <div
            ref={frameRef}
            onMouseMove={handleMouseMove}
            onMouseLeave={handleMouseLeave}
            className="relative rounded-2xl sm:rounded-3xl border shadow-2xl overflow-hidden cursor-pointer group transition-all duration-200"
            style={{
              transform: `perspective(1200px) rotateX(${rotX}deg) rotateY(${rotY}deg) scale3d(1, 1, 1)`,
              transformStyle: "preserve-3d",
              borderColor: isDark ? "rgba(99, 102, 241, 0.35)" : "rgba(99, 102, 241, 0.25)",
              backgroundColor: isDark ? "#0f172a" : "#ffffff",
              boxShadow: isDark
                ? "0 30px 60px -15px rgba(0, 0, 0, 0.7), 0 0 40px rgba(99, 102, 241, 0.15)"
                : "0 30px 60px -15px rgba(15, 23, 42, 0.15), 0 0 30px rgba(99, 102, 241, 0.1)",
            }}
            onClick={() => setModalOpen(true)}
          >
            {/* Gloss reflection layer */}
            <div
              className="absolute inset-0 pointer-events-none z-10 transition-opacity duration-300"
              style={{
                background: `radial-gradient(circle at ${glare.x}% ${glare.y}%, rgba(255, 255, 255, ${glare.opacity}) 0%, transparent 60%)`,
              }}
            />

            {/* Window Header Bar */}
            <div
              className="px-4 py-3 border-b flex items-center justify-between"
              style={{
                backgroundColor: isDark ? "rgba(15, 23, 42, 0.95)" : "rgba(248, 250, 252, 0.95)",
                borderColor: isDark ? "rgba(51, 65, 85, 0.6)" : "rgba(226, 232, 240, 0.8)",
              }}
            >
              <div className="flex items-center gap-2">
                <div className="w-3 h-3 rounded-full bg-rose-500/80" />
                <div className="w-3 h-3 rounded-full bg-amber-500/80" />
                <div className="w-3 h-3 rounded-full bg-emerald-500/80" />
                <span className="ml-2 text-xs font-semibold text-slate-400 truncate flex items-center gap-1.5">
                  <img src="/assets/app_icon.png" alt="icon" className="w-3.5 h-3.5" />
                  Rapid Download Manager — {currentView.name}
                </span>
              </div>

              <div className="flex items-center gap-2">
                <span className="hidden sm:inline-flex text-[10px] font-mono font-medium px-2 py-0.5 rounded bg-indigo-500/10 text-indigo-400 border border-indigo-500/20">
                  Click to inspect full resolution
                </span>
                <Maximize2 className="w-3.5 h-3.5 text-slate-400 group-hover:text-indigo-400 transition-colors" />
              </div>
            </div>

            {/* Active Screenshot Display */}
            <div className="relative aspect-[16/10] w-full overflow-hidden bg-slate-950 flex items-center justify-center">
              <img
                src={currentView.img}
                alt={currentView.title}
                className="w-full h-full object-cover sm:object-contain transition-transform duration-500 group-hover:scale-101"
              />

              {/* Hover Overlay Hint */}
              <div className="absolute inset-0 bg-slate-950/40 opacity-0 group-hover:opacity-100 transition-opacity duration-300 flex items-center justify-center backdrop-blur-xs">
                <div className="px-4 py-2 rounded-xl bg-slate-900/90 border border-indigo-500/40 text-white text-xs font-semibold flex items-center gap-2 shadow-2xl">
                  <Maximize2 className="w-4 h-4 text-indigo-400" />
                  <span>View High-Definition Screenshot</span>
                </div>
              </div>
            </div>

            {/* Bottom Details Strip */}
            <div
              className="p-4 sm:p-6 border-t space-y-3"
              style={{
                backgroundColor: isDark ? "rgba(15, 23, 42, 0.9)" : "rgba(248, 250, 252, 0.9)",
                borderColor: isDark ? "rgba(51, 65, 85, 0.5)" : "rgba(226, 232, 240, 0.8)",
              }}
            >
              <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-2">
                <div>
                  <h3 className="text-base sm:text-lg font-bold" style={{ color: "var(--text-heading)" }}>
                    {currentView.title}
                  </h3>
                  <p className="text-xs sm:text-sm mt-0.5" style={{ color: "var(--text-muted)" }}>
                    {currentView.desc}
                  </p>
                </div>
                <div className="shrink-0">
                  <a
                    href="#download"
                    className="inline-flex items-center gap-1.5 px-3.5 py-2 rounded-xl bg-indigo-600 hover:bg-indigo-500 text-white text-xs font-semibold shadow-md transition-all hover:scale-103"
                  >
                    <span>Get v1.0.0</span>
                    <ArrowRight className="w-3.5 h-3.5" />
                  </a>
                </div>
              </div>

              {/* 3 Key Highlights */}
              <div className="grid grid-cols-1 sm:grid-cols-3 gap-2 pt-2 border-t border-slate-700/20">
                {currentView.highlights.map((item, idx) => (
                  <div key={idx} className="flex items-center gap-2 text-[11px] sm:text-xs text-slate-400">
                    <div className="w-1.5 h-1.5 rounded-full bg-indigo-400 shrink-0" />
                    <span>{item}</span>
                  </div>
                ))}
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* High-Resolution Zoom Lightbox Modal */}
      {modalOpen && (
        <div
          className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/80 backdrop-blur-md animate-fade-in"
          onClick={() => setModalOpen(false)}
        >
          <div
            className="relative max-w-6xl w-full max-h-[92vh] rounded-2xl overflow-hidden border border-slate-700 bg-slate-900 shadow-2xl flex flex-col"
            onClick={(e) => e.stopPropagation()}
          >
            {/* Modal Bar */}
            <div className="px-5 py-3 border-b border-slate-800 bg-slate-950 flex items-center justify-between text-white">
              <div className="flex items-center gap-2 font-semibold text-sm">
                <img src="/assets/app_icon.png" alt="logo" className="w-4 h-4" />
                <span>{currentView.title}</span>
              </div>
              <button
                onClick={() => setModalOpen(false)}
                className="p-1.5 rounded-lg hover:bg-slate-800 text-slate-400 hover:text-white transition-colors cursor-pointer"
              >
                <X className="w-5 h-5" />
              </button>
            </div>

            {/* Modal Image */}
            <div className="p-2 sm:p-4 overflow-auto flex-1 flex items-center justify-center bg-slate-950">
              <img
                src={currentView.img}
                alt={currentView.title}
                className="max-h-[80vh] w-auto rounded-lg shadow-xl object-contain"
              />
            </div>
          </div>
        </div>
      )}
    </section>
  );
}

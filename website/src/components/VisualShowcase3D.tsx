"use client";

import React, { useState } from "react";
import {
  Activity,
  Zap,
  Globe,
  Clock,
  Cpu,
  Maximize2,
  X,
  CheckCircle2,
} from "lucide-react";
import { useTheme } from "../context/ThemeContext";
import { assetUrl } from "@/utils/assets";

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

  const views: ViewItem[] = [
    {
      id: "dashboard",
      name: "Live Dashboard",
      badge: "16 Sockets",
      icon: <Activity className="w-4 h-4 text-emerald-400" />,
      img: assetUrl("/assets/screenshot_dashboard.png"),
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
      img: assetUrl("/assets/screenshot_add.png"),
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
      img: assetUrl("/assets/screenshot_browser.png"),
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
      img: assetUrl("/assets/screenshot_scheduler.png"),
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
      img: assetUrl("/assets/screenshot_about.png"),
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

  return (
    <section id="showcase" className="py-16 relative">
      <div className="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
        {/* Section Header */}
        <div className="text-center max-w-2xl mx-auto mb-10 space-y-3">
          <h2 className="text-2xl sm:text-4xl font-extrabold tracking-tight" style={{ color: "var(--text-heading)" }}>
            Explore the Native Interface
          </h2>
          <p className="text-sm sm:text-base" style={{ color: "var(--text-muted)" }}>
            Crisp, clean, and distraction-free. Designed for keyboard navigation, multi-monitor setups, and high-DPI Windows displays.
          </p>
        </div>

        {/* View Switcher Tabs */}
        <div className="flex flex-wrap items-center justify-center gap-2 mb-8">
          {views.map((tab) => {
            const isActive = tab.id === activeTab;
            return (
              <button
                key={tab.id}
                onClick={() => setActiveTab(tab.id)}
                className={`flex items-center gap-2 px-4 py-2 rounded-xl text-xs sm:text-sm font-medium transition-all cursor-pointer ${
                  isActive
                    ? "bg-indigo-600 text-white shadow-sm font-semibold"
                    : "border hover:bg-slate-500/10"
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
                  className={`text-[10px] px-1.5 py-0.5 rounded font-mono ${
                    isActive ? "bg-white/20 text-white" : "bg-slate-500/10 text-slate-400"
                  }`}
                >
                  {tab.badge}
                </span>
              </button>
            );
          })}
        </div>

        {/* Standard, Flat, High-Resolution App Window Mockup */}
        <div
          className="rounded-2xl border shadow-2xl overflow-hidden transition-all duration-200"
          style={{
            borderColor: isDark ? "rgba(51, 65, 85, 0.8)" : "rgba(203, 213, 225, 0.8)",
            backgroundColor: isDark ? "#0f172a" : "#ffffff",
          }}
        >
          {/* Window Titlebar */}
          <div
            className="px-4 py-3 border-b flex items-center justify-between"
            style={{
              backgroundColor: isDark ? "#0b0f17" : "#f1f5f9",
              borderColor: isDark ? "rgba(51, 65, 85, 0.6)" : "rgba(226, 232, 240, 0.8)",
            }}
          >
            <div className="flex items-center gap-2.5">
              <div className="w-3 h-3 rounded-full bg-rose-500/80" />
              <div className="w-3 h-3 rounded-full bg-amber-500/80" />
              <div className="w-3 h-3 rounded-full bg-emerald-500/80" />
              <span className="ml-2 text-xs font-semibold text-slate-400 flex items-center gap-1.5">
                <img src={assetUrl("/assets/app_icon.png")} alt="icon" className="w-3.5 h-3.5" />
                Rapid Download Manager — {currentView.name}
              </span>
            </div>

            <button
              onClick={() => setModalOpen(true)}
              className="flex items-center gap-1.5 text-xs text-slate-400 hover:text-indigo-400 transition-colors cursor-pointer"
              title="Expand Screenshot"
            >
              <span className="hidden sm:inline text-[11px]">View Fullscreen</span>
              <Maximize2 className="w-3.5 h-3.5" />
            </button>
          </div>

          {/* Crisp, Sharp Screenshot */}
          <div
            className="relative w-full aspect-[16/10] bg-slate-950 flex items-center justify-center cursor-pointer group"
            onClick={() => setModalOpen(true)}
          >
            <img
              src={currentView.img}
              alt={currentView.title}
              className="w-full h-full object-contain"
            />

            {/* Click to zoom badge */}
            <div className="absolute bottom-4 right-4 px-3 py-1.5 rounded-lg bg-slate-900/90 border border-slate-700 text-white text-xs font-medium opacity-0 group-hover:opacity-100 transition-opacity flex items-center gap-1.5 shadow-lg">
              <Maximize2 className="w-3.5 h-3.5 text-indigo-400" />
              <span>Click to Zoom</span>
            </div>
          </div>

          {/* Window Footer Information */}
          <div
            className="p-4 sm:p-6 border-t space-y-3"
            style={{
              backgroundColor: isDark ? "#0f172a" : "#f8fafc",
              borderColor: isDark ? "rgba(51, 65, 85, 0.6)" : "rgba(226, 232, 240, 0.8)",
            }}
          >
            <div>
              <h3 className="text-base sm:text-lg font-bold" style={{ color: "var(--text-heading)" }}>
                {currentView.title}
              </h3>
              <p className="text-xs sm:text-sm mt-0.5" style={{ color: "var(--text-muted)" }}>
                {currentView.desc}
              </p>
            </div>

            <div className="grid grid-cols-1 sm:grid-cols-3 gap-2.5 pt-2 border-t border-slate-700/20">
              {currentView.highlights.map((item, idx) => (
                <div key={idx} className="flex items-center gap-2 text-xs" style={{ color: "var(--text-body)" }}>
                  <CheckCircle2 className="w-4 h-4 text-emerald-400 shrink-0" />
                  <span>{item}</span>
                </div>
              ))}
            </div>
          </div>
        </div>
      </div>

      {/* Fullscreen Lightbox Modal */}
      {modalOpen && (
        <div
          className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/85 backdrop-blur-sm"
          onClick={() => setModalOpen(false)}
        >
          <div
            className="relative max-w-6xl w-full max-h-[92vh] rounded-xl overflow-hidden border border-slate-700 bg-slate-900 shadow-2xl flex flex-col"
            onClick={(e) => e.stopPropagation()}
          >
            <div className="px-4 py-3 border-b border-slate-800 bg-slate-950 flex items-center justify-between text-white">
              <div className="flex items-center gap-2 font-semibold text-sm">
                <img src={assetUrl("/assets/app_icon.png")} alt="logo" className="w-4 h-4" />
                <span>{currentView.title}</span>
              </div>
              <button
                onClick={() => setModalOpen(false)}
                className="p-1 rounded-lg hover:bg-slate-800 text-slate-400 hover:text-white transition-colors cursor-pointer"
              >
                <X className="w-5 h-5" />
              </button>
            </div>
            <div className="p-2 overflow-auto flex-1 flex items-center justify-center bg-slate-950">
              <img
                src={currentView.img}
                alt={currentView.title}
                className="max-h-[82vh] w-auto rounded object-contain"
              />
            </div>
          </div>
        </div>
      )}
    </section>
  );
}

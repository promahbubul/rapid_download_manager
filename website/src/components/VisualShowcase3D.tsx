"use client";

import React, { useState } from "react";
import {
  Activity,
  PlusCircle,
  Globe,
  Clock,
  Maximize2,
  X,
} from "lucide-react";
import { useTheme } from "../context/ThemeContext";
import { assetUrl } from "@/utils/assets";

interface ShowcaseView {
  id: string;
  label: string;
  icon: React.ReactNode;
  img: string;
  title: string;
  description: string;
}

export default function VisualShowcase3D() {
  const { resolvedTheme } = useTheme();
  const isDark = resolvedTheme === "dark";

  const [activeTab, setActiveTab] = useState("dashboard");
  const [modalOpen, setModalOpen] = useState(false);

  const views: ShowcaseView[] = [
    {
      id: "dashboard",
      label: "Live Dashboard",
      icon: <Activity className="w-4 h-4 text-emerald-400" />,
      img: assetUrl("/assets/screenshot_dashboard.png"),
      title: "Real-Time Multi-Stream Visualizer",
      description:
        "Monitor active socket threads, per-chunk progress visualizers, instantaneous bandwidth gauges, and ETA calculations.",
    },
    {
      id: "add",
      label: "New Download Dialog",
      icon: <PlusCircle className="w-4 h-4 text-indigo-400" />,
      img: assetUrl("/assets/screenshot_add.png"),
      title: "Clipboard Detection & Thread Customization",
      description:
        "Instantly parse pasted links, configure parallel socket counts (1 to 32), and inject custom cookies or authorization tokens.",
    },
    {
      id: "browser",
      label: "Browser Extension",
      icon: <Globe className="w-4 h-4 text-cyan-400" />,
      img: assetUrl("/assets/screenshot_browser.png"),
      title: "Zero-Latency Browser Interception",
      description:
        "Automatically captures video streams, archives, and files from Chrome, Edge, Brave, Opera, and Firefox via native loopback RPC.",
    },
    {
      id: "scheduler",
      label: "Task Scheduler",
      icon: <Clock className="w-4 h-4 text-purple-400" />,
      img: assetUrl("/assets/screenshot_scheduler.png"),
      title: "Off-Peak Automation & Power Management",
      description:
        "Queue downloads to run overnight during off-peak internet hours with automatic PC sleep or shutdown upon queue completion.",
    },
  ];

  const currentView = views.find((v) => v.id === activeTab) || views[0];

  return (
    <section
      id="showcase"
      className="py-20 border-t"
      style={{
        backgroundColor: "var(--bg-page)",
        borderColor: "var(--border-subtle)",
      }}
    >
      <div className="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
        {/* Section Header */}
        <div className="text-center max-w-2xl mx-auto mb-10 space-y-3">
          <div className="inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-xs font-semibold uppercase tracking-wider text-emerald-400 bg-emerald-500/10 border border-emerald-500/20">
            Interface Preview
          </div>
          <h2
            className="text-3xl sm:text-4xl font-extrabold tracking-tight"
            style={{ color: "var(--text-heading)" }}
          >
            Designed for Modern Desktops
          </h2>
          <p className="text-sm sm:text-base leading-relaxed" style={{ color: "var(--text-muted)" }}>
            High-DPI responsive Cyber-Obsidian interface powered by GPU hardware acceleration.
          </p>
        </div>

        {/* Tab Switcher Pills */}
        <div className="flex flex-wrap items-center justify-center gap-2 mb-8">
          {views.map((v) => {
            const isSelected = activeTab === v.id;
            return (
              <button
                key={v.id}
                onClick={() => setActiveTab(v.id)}
                className={`flex items-center gap-2 px-4 py-2.5 rounded-xl text-xs sm:text-sm font-medium transition-all cursor-pointer ${
                  isSelected
                    ? "bg-indigo-600 text-white font-semibold shadow-md shadow-indigo-600/20 scale-102"
                    : "border hover:bg-slate-500/10"
                }`}
                style={{
                  borderColor: isSelected ? "transparent" : "var(--border-subtle)",
                  backgroundColor: isSelected ? undefined : "var(--bg-card)",
                  color: isSelected ? "#ffffff" : "var(--text-heading)",
                }}
              >
                {v.icon}
                <span>{v.label}</span>
              </button>
            );
          })}
        </div>

        {/* Screenshot Container with Caption */}
        <div className="space-y-4">
          <div
            className="rounded-2xl border overflow-hidden shadow-2xl transition-all duration-300 relative group cursor-pointer"
            onClick={() => setModalOpen(true)}
            style={{
              backgroundColor: isDark ? "#0f172a" : "#ffffff",
              borderColor: "var(--border-subtle)",
            }}
          >
            <div className="relative">
              <img
                src={currentView.img}
                alt={currentView.title}
                className="w-full h-auto object-cover"
              />
              <div className="absolute inset-0 bg-black/20 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center">
                <span className="px-4 py-2 rounded-xl bg-slate-900/90 text-white text-xs font-semibold backdrop-blur-md flex items-center gap-2 shadow-lg">
                  <Maximize2 className="w-4 h-4" />
                  <span>Click to expand image</span>
                </span>
              </div>
            </div>
          </div>

          {/* Description Caption */}
          <div className="text-center max-w-xl mx-auto space-y-1">
            <h3 className="text-base font-bold" style={{ color: "var(--text-heading)" }}>
              {currentView.title}
            </h3>
            <p className="text-xs sm:text-sm" style={{ color: "var(--text-muted)" }}>
              {currentView.description}
            </p>
          </div>
        </div>
      </div>

      {/* Lightbox Modal */}
      {modalOpen && (
        <div
          className="fixed inset-0 z-50 bg-black/85 backdrop-blur-md flex items-center justify-center p-4 sm:p-8 animate-in fade-in duration-200"
          onClick={() => setModalOpen(false)}
        >
          <div className="relative max-w-6xl w-full max-h-[90vh] flex flex-col items-center">
            <button
              onClick={() => setModalOpen(false)}
              className="absolute -top-12 right-0 p-2 text-white/80 hover:text-white transition-colors cursor-pointer"
              title="Close"
            >
              <X className="w-6 h-6" />
            </button>
            <img
              src={currentView.img}
              alt={currentView.title}
              className="w-full h-auto max-h-[85vh] object-contain rounded-xl shadow-2xl border border-white/10"
              onClick={(e) => e.stopPropagation()}
            />
          </div>
        </div>
      )}
    </section>
  );
}

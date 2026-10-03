"use client";

import React from "react";
import {
  Zap,
  Cloud,
  Globe,
  ShieldCheck,
  CheckCircle2,
  Lock,
  ArrowRight,
} from "lucide-react";
import { useTheme } from "../context/ThemeContext";

export default function Features() {
  const { resolvedTheme } = useTheme();
  const isDark = resolvedTheme === "dark";

  const corePillars = [
    {
      icon: <Zap className="w-5 h-5 text-amber-400" />,
      tag: "Speed",
      title: "Up to 32 Parallel Streams",
      description:
        "Saturate your full bandwidth by opening up to 32 concurrent HTTP range sockets with dynamic stream re-balancing.",
    },
    {
      icon: <Cloud className="w-5 h-5 text-cyan-400" />,
      tag: "Cloud",
      title: "Native MEGA & Drive Folders",
      description:
        "Download complete public folder trees from MEGA.nz and Google Drive with real-time decryption.",
    },
    {
      icon: <Globe className="w-5 h-5 text-indigo-400" />,
      tag: "Integration",
      title: "1-Click Browser Companion",
      description:
        "Automatically captures downloads from Chrome, Edge, Brave, and Firefox with full session cookie forwarding.",
    },
    {
      icon: <ShieldCheck className="w-5 h-5 text-emerald-400" />,
      tag: "Privacy",
      title: "Lightweight & 100% Private",
      description:
        "Zero Electron bloat. Uses under 25 MB RAM with memory-safe Rust cryptography and zero telemetry tracking.",
    },
  ];

  return (
    <section
      id="features"
      className="py-16 border-t"
      style={{
        backgroundColor: isDark ? "rgba(11, 15, 25, 0.4)" : "rgba(248, 250, 252, 0.6)",
        borderColor: "var(--border-subtle)",
      }}
    >
      <div className="max-w-5xl mx-auto px-4 sm:px-6 lg:px-8">
        {/* Section Header */}
        <div className="text-center max-w-xl mx-auto mb-10 space-y-2">
          <h2
            className="text-2xl sm:text-3xl font-extrabold tracking-tight"
            style={{ color: "var(--text-heading)" }}
          >
            Built for Pure Performance
          </h2>
          <p className="text-sm" style={{ color: "var(--text-muted)" }}>
            Engineered in Rust to deliver maximum transfer speeds with minimal resource footprint.
          </p>
        </div>

        {/* 4 Core Pillars Grid */}
        <div className="grid grid-cols-1 sm:grid-cols-2 gap-4 sm:gap-6">
          {corePillars.map((pillar, idx) => (
            <div
              key={idx}
              className="p-6 rounded-2xl border flex flex-col justify-between transition-all duration-300 hover:shadow-lg"
              style={{
                backgroundColor: "var(--bg-card)",
                borderColor: "var(--border-subtle)",
              }}
            >
              <div className="space-y-3">
                <div className="flex items-center justify-between">
                  <div className="w-10 h-10 rounded-xl bg-slate-500/10 flex items-center justify-center">
                    {pillar.icon}
                  </div>
                  <span className="text-[11px] font-semibold uppercase tracking-wider px-2 py-0.5 rounded bg-slate-500/10 text-indigo-400 font-mono">
                    {pillar.tag}
                  </span>
                </div>
                <h3 className="text-base sm:text-lg font-bold" style={{ color: "var(--text-heading)" }}>
                  {pillar.title}
                </h3>
                <p className="text-xs sm:text-sm leading-relaxed" style={{ color: "var(--text-muted)" }}>
                  {pillar.description}
                </p>
              </div>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}
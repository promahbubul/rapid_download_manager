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
      icon: <Zap className="w-6 h-6 text-amber-400" />,
      tag: "Parallel Speed",
      title: "Up to 32 Parallel Streams",
      description:
        "Partitions single files into up to 32 concurrent HTTP byte-range connections. Dynamically detects and re-balances lagging streams to saturate your full internet bandwidth.",
    },
    {
      icon: <Cloud className="w-6 h-6 text-cyan-400" />,
      tag: "Cloud Acceleration",
      title: "Native MEGA & Drive Folders",
      description:
        "Recursively crawls complete folder hierarchies from MEGA.nz and Google Drive. Streams encrypted blocks directly from CDN servers with real-time on-the-fly AES-128-CTR decryption.",
    },
    {
      icon: <Globe className="w-6 h-6 text-indigo-400" />,
      tag: "Browser Companion",
      title: "1-Click Browser Integration",
      description:
        "Hooks seamlessly into Google Chrome, Microsoft Edge, Brave, Opera, and Firefox via lightweight Manifest V3. Intercepts downloads with full cookie and session token preservation.",
    },
    {
      icon: <ShieldCheck className="w-6 h-6 text-emerald-400" />,
      tag: "Memory Safety",
      title: "100% Memory-Safe & Private",
      description:
        "Engineered in pure Rust with Tokio and Rustls TLS. No Electron bloat, sub-25 MB idle RAM footprint, zero telemetry tracking, and automated credential redaction in diagnostic logs.",
    },
  ];

  return (
    <section
      id="features"
      className="py-20 border-t"
      style={{
        backgroundColor: isDark ? "rgba(11, 15, 25, 0.4)" : "rgba(248, 250, 252, 0.6)",
        borderColor: "var(--border-subtle)",
      }}
    >
      <div className="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8">
        {/* Section Header */}
        <div className="text-center max-w-2xl mx-auto mb-14 space-y-3">
          <div className="inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-xs font-semibold uppercase tracking-wider text-indigo-400 bg-indigo-500/10 border border-indigo-500/20">
            Core Architecture
          </div>
          <h2
            className="text-3xl sm:text-4xl font-extrabold tracking-tight"
            style={{ color: "var(--text-heading)" }}
          >
            Engineered for Pure Performance
          </h2>
          <p className="text-sm sm:text-base leading-relaxed" style={{ color: "var(--text-muted)" }}>
            Four uncompromising engineering pillars designed to replace legacy 90s download utilities.
          </p>
        </div>

        {/* 4 Core Pillars Grid */}
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          {corePillars.map((pillar, idx) => (
            <div
              key={idx}
              className="p-7 rounded-2xl border flex flex-col justify-between transition-all duration-300 hover:scale-[1.01] hover:shadow-lg"
              style={{
                backgroundColor: "var(--bg-card)",
                borderColor: "var(--border-subtle)",
              }}
            >
              <div className="space-y-4">
                <div className="flex items-center justify-between">
                  <div className="w-12 h-12 rounded-xl bg-slate-500/10 flex items-center justify-center">
                    {pillar.icon}
                  </div>
                  <span className="text-xs font-semibold uppercase tracking-wider px-2.5 py-1 rounded-md bg-slate-500/10 text-indigo-400 font-mono">
                    {pillar.tag}
                  </span>
                </div>
                <h3 className="text-xl font-bold" style={{ color: "var(--text-heading)" }}>
                  {pillar.title}
                </h3>
                <p className="text-sm leading-relaxed" style={{ color: "var(--text-muted)" }}>
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
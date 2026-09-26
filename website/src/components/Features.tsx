"use client";


import React from "react";
import {
  Zap,
  Globe,
  ShieldCheck,
  Cpu,
  Layers,
  Sparkles,
  Repeat,
  Radio,
  FileCheck,
  Server,
  Lock,
} from "lucide-react";
import { useTheme } from "../context/ThemeContext";

export default function Features() {
  const { resolvedTheme } = useTheme();
  const isDark = resolvedTheme === "dark";

  const featuresList = [
    {
      icon: <Zap className="w-6 h-6 text-amber-400" />,
      title: "16x Parallel Multi-Chunk Engine",
      description:
        "Dynamically partitions single files into up to 16 concurrent HTTP byte-range connections, saturating your available bandwidth for peak wire-speed transfers.",
      tag: "Turbo Boost",
    },
    {
      icon: <Globe className="w-6 h-6 text-indigo-400" />,
      title: "1-Click Browser Integration",
      description:
        "Seamless native extensions for Chrome, Edge, Brave, Opera, and Firefox. Intercepts downloads with full cookie, auth-token, and referrer preservation.",
      tag: "Extension Included",
    },
    {
      icon: <ShieldCheck className="w-6 h-6 text-emerald-400" />,
      title: "Zero Telemetry & 100% Privacy",
      description:
        "No telemetry, zero user tracking, and no external analytic pings. Built-in log credential redactor automatically masks passwords and API tokens.",
      tag: "Privacy First",
    },
    {
      icon: <Cpu className="w-6 h-6 text-cyan-400" />,
      title: "Memory-Safe Rust Architecture",
      description:
        "Built from the ground up in Rust using Tokio and Rustls TLS. No heavy Electron bloat, zero garbage collection stutter, and sub-25MB idle memory.",
      tag: "Native Win32",
    },
    {
      icon: <Layers className="w-6 h-6 text-purple-400" />,
      title: "Dual-Layer Local Database",
      description:
        "Atomic disk persistence with automatic backup rotation prevents data loss. Your download history and queues remain safe even during sudden power failure.",
      tag: "Crash-Proof",
    },
    {
      icon: <Repeat className="w-6 h-6 text-rose-400" />,
      title: "Intelligent Auto-Resume",
      description:
        "Smart exponential backoff retry system transparently re-establishes broken TCP sockets and resumes downloads seamlessly from the exact byte offset.",
      tag: "Auto-Healing",
    },
  ];

  return (
    <section id="features" className="py-20 border-t"
      style={{
        backgroundColor: isDark ? "rgba(11, 15, 25, 0.4)" : "rgba(248, 250, 252, 0.6)",
        borderColor: "var(--border-subtle)",
      }}
    >
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="text-center max-w-3xl mx-auto mb-16 space-y-4">
          <div className="inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-xs font-semibold uppercase tracking-wider text-indigo-400 bg-indigo-500/10 border border-indigo-500/20">
            Engineered For Power
          </div>
          <h2 className="text-3xl sm:text-4xl font-extrabold tracking-tight" style={{ color: "var(--text-heading)" }}>
            Built for Extreme Speed & Total Reliability
          </h2>
          <p className="text-base" style={{ color: "var(--text-muted)" }}>
            Everything you need in a modern download accelerator, without the bloat, spyware, or subscription paywalls of legacy tools.
          </p>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {featuresList.map((item, idx) => (
            <div
              key={idx}
              className="p-6 rounded-2xl border transition-all duration-300 hover:scale-102 hover:shadow-xl group"
              style={{
                backgroundColor: "var(--bg-card)",
                borderColor: "var(--border-subtle)",
              }}
            >
              <div className="flex items-center justify-between mb-4">
                <div className="w-12 h-12 rounded-xl border flex items-center justify-center transition-transform group-hover:scale-110"
                  style={{
                    backgroundColor: isDark ? "rgba(30, 41, 59, 0.5)" : "rgba(241, 245, 249, 0.8)",
                    borderColor: "var(--border-subtle)",
                  }}
                >
                  {item.icon}
                </div>
                <span className="text-[11px] font-semibold px-2.5 py-1 rounded-full border border-indigo-500/20 text-indigo-400 bg-indigo-500/5">
                  {item.tag}
                </span>
              </div>
              <h3 className="text-lg font-bold mb-2 group-hover:text-indigo-400 transition-colors" style={{ color: "var(--text-heading)" }}>
                {item.title}
              </h3>
              <p className="text-sm leading-relaxed" style={{ color: "var(--text-muted)" }}>
                {item.description}
              </p>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}
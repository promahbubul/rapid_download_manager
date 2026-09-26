"use client";

import { ChromeIcon } from "./Icons";

import React from "react";
import { Download, Sparkles,  Zap, ArrowRight } from "lucide-react";
import { useTheme } from "../context/ThemeContext";

export default function QuickStart() {
  const { resolvedTheme } = useTheme();
  const isDark = resolvedTheme === "dark";

  const steps = [
    {
      num: "01",
      title: "Get Rapid Download Manager",
      desc: "Grab the official MSIX from Microsoft Store or run the lightweight portable standalone executable. No administrative rights needed.",
      icon: <Download className="w-5 h-5 text-indigo-400" />,
    },
    {
      num: "02",
      title: "Enable Browser Integration",
      desc: "Run install_extension.bat or load the provided Chrome/Edge extension. Links, videos, and multi-file downloads are automatically captured.",
      icon: <ChromeIcon className="w-5 h-5 text-purple-400" />,
    },
    {
      num: "03",
      title: "Download at Wire Speed",
      desc: "Watch the 16-chunk acceleration engine saturate your internet connection with real-time speed monitoring and instant auto-resumes.",
      icon: <Zap className="w-5 h-5 text-emerald-400" />,
    },
  ];

  return (
    <section className="py-20 border-t"
      style={{
        backgroundColor: "var(--bg-page)",
        borderColor: "var(--border-subtle)",
      }}
    >
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="text-center max-w-3xl mx-auto mb-16 space-y-4">
          <div className="inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-xs font-semibold uppercase tracking-wider text-purple-400 bg-purple-500/10 border border-purple-500/20">
            Quick Setup
          </div>
          <h2 className="text-3xl sm:text-4xl font-extrabold tracking-tight" style={{ color: "var(--text-heading)" }}>
            Get Up and Running in 30 Seconds
          </h2>
          <p className="text-base" style={{ color: "var(--text-muted)" }}>
            No bloated installers, no adware toolbars, and zero configuration headaches.
          </p>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-8 max-w-5xl mx-auto">
          {steps.map((s, idx) => (
            <div
              key={idx}
              className="relative p-6 rounded-2xl border flex flex-col justify-between transition-all duration-300 hover:scale-102"
              style={{
                backgroundColor: "var(--bg-card)",
                borderColor: "var(--border-subtle)",
              }}
            >
              <div className="space-y-4">
                <div className="flex items-center justify-between">
                  <div className="w-10 h-10 rounded-xl bg-indigo-500/15 flex items-center justify-center">
                    {s.icon}
                  </div>
                  <span className="text-2xl font-black font-mono" style={{ color: "var(--border-card)" }}>
                    {s.num}
                  </span>
                </div>
                <h3 className="text-lg font-bold" style={{ color: "var(--text-heading)" }}>
                  {s.title}
                </h3>
                <p className="text-sm leading-relaxed" style={{ color: "var(--text-muted)" }}>
                  {s.desc}
                </p>
              </div>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}
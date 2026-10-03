"use client";

import React from "react";
import { Check, X, Sparkles } from "lucide-react";
import { useTheme } from "../context/ThemeContext";

export default function Comparison() {
  const { resolvedTheme } = useTheme();
  const isDark = resolvedTheme === "dark";

  const rows = [
    {
      metric: "Up to 32 Parallel Streams",
      rdm: true,
      idm: true,
      fdm: true,
      browser: false,
      detail: "Saturates full bandwidth",
    },
    {
      metric: "Memory-Safe Rust Engine",
      rdm: true,
      idm: false,
      fdm: false,
      browser: false,
      detail: "Pure Tokio + Rustls TLS",
    },
    {
      metric: "Cloud Folder Acceleration",
      rdm: true,
      idm: false,
      fdm: false,
      browser: false,
      detail: "Native MEGA.nz & Drive crawler",
    },
    {
      metric: "100% Free & Open Source",
      rdm: true,
      idm: false,
      fdm: true,
      browser: true,
      detail: "MIT License, no paywalls",
    },
    {
      metric: "Zero Ads & Zero Telemetry",
      rdm: true,
      idm: true,
      fdm: false,
      browser: false,
      detail: "Audited local storage only",
    },
  ];

  return (
    <section
      id="comparison"
      className="py-20 border-t"
      style={{
        backgroundColor: isDark ? "rgba(11, 15, 25, 0.4)" : "rgba(248, 250, 252, 0.6)",
        borderColor: "var(--border-subtle)",
      }}
    >
      <div className="max-w-5xl mx-auto px-4 sm:px-6 lg:px-8">
        {/* Section Header */}
        <div className="text-center max-w-2xl mx-auto mb-12 space-y-3">
          <div className="inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-xs font-semibold uppercase tracking-wider text-indigo-400 bg-indigo-500/10 border border-indigo-500/20">
            Head-to-Head Comparison
          </div>
          <h2
            className="text-3xl sm:text-4xl font-extrabold tracking-tight"
            style={{ color: "var(--text-heading)" }}
          >
            Why Choose Rapid Download Manager?
          </h2>
          <p className="text-sm sm:text-base leading-relaxed" style={{ color: "var(--text-muted)" }}>
            A modern, transparent alternative to legacy 90s commercial download utilities.
          </p>
        </div>

        {/* Comparison Table */}
        <div
          className="rounded-2xl border overflow-x-auto shadow-xl"
          style={{
            backgroundColor: "var(--bg-card)",
            borderColor: "var(--border-subtle)",
          }}
        >
          <table className="w-full text-left border-collapse min-w-[550px]">
            <thead>
              <tr className="border-b text-xs uppercase tracking-wider" style={{ borderColor: "var(--border-subtle)" }}>
                <th className="p-4 sm:p-5 font-bold" style={{ color: "var(--text-muted)" }}>
                  Capability
                </th>
                <th className="p-4 sm:p-5 font-bold bg-indigo-500/10 text-indigo-400 border-x border-indigo-500/20 text-center">
                  <div className="flex items-center justify-center gap-1.5">
                    <Sparkles className="w-4 h-4 text-indigo-400" />
                    <span>Rapid DM</span>
                  </div>
                </th>
                <th className="p-4 sm:p-5 font-semibold text-center" style={{ color: "var(--text-muted)" }}>
                  Legacy IDM
                </th>
                <th className="p-4 sm:p-5 font-semibold text-center" style={{ color: "var(--text-muted)" }}>
                  Free DM
                </th>
                <th className="p-4 sm:p-5 font-semibold text-center" style={{ color: "var(--text-muted)" }}>
                  Browser
                </th>
              </tr>
            </thead>
            <tbody className="divide-y text-sm" style={{ borderColor: "var(--border-subtle)" }}>
              {rows.map((r, i) => (
                <tr key={i} className="hover:bg-slate-500/5 transition-colors">
                  <td className="p-4 sm:p-5">
                    <div className="font-semibold" style={{ color: "var(--text-heading)" }}>
                      {r.metric}
                    </div>
                    <div className="text-xs" style={{ color: "var(--text-muted)" }}>
                      {r.detail}
                    </div>
                  </td>
                  <td className="p-4 sm:p-5 text-center bg-indigo-500/5 border-x border-indigo-500/20">
                    <div className="inline-flex items-center justify-center w-6 h-6 rounded-full bg-emerald-500/20 text-emerald-400 font-bold">
                      <Check className="w-4 h-4" />
                    </div>
                  </td>
                  <td className="p-4 sm:p-5 text-center">
                    {r.idm ? (
                      <Check className="w-4 h-4 text-emerald-400 inline" />
                    ) : (
                      <X className="w-4 h-4 text-slate-500 inline" />
                    )}
                  </td>
                  <td className="p-4 sm:p-5 text-center">
                    {r.fdm ? (
                      <Check className="w-4 h-4 text-emerald-400 inline" />
                    ) : (
                      <X className="w-4 h-4 text-slate-500 inline" />
                    )}
                  </td>
                  <td className="p-4 sm:p-5 text-center">
                    {r.browser ? (
                      <Check className="w-4 h-4 text-emerald-400 inline" />
                    ) : (
                      <X className="w-4 h-4 text-slate-500 inline" />
                    )}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    </section>
  );
}
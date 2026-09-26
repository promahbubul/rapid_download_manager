"use client";


import React from "react";
import { Check, X, Sparkles } from "lucide-react";
import { useTheme } from "../context/ThemeContext";

export default function Comparison() {
  const { resolvedTheme } = useTheme();
  const isDark = resolvedTheme === "dark";

  const rows = [
    { metric: "16-Chunk Acceleration", rdm: true, idm: true, fdm: true, browser: false },
    { metric: "Memory-Safe Rust Engine", rdm: true, idm: false, fdm: false, browser: false },
    { metric: "Zero Ads / Zero Bundleware", rdm: true, idm: true, fdm: false, browser: true },
    { metric: "100% Free & Open Source", rdm: true, idm: false, fdm: true, browser: true },
    { metric: "Modern Obsidian UI & Dark Mode", rdm: true, idm: false, fdm: true, browser: false },
    { metric: "Pure Rustls TLS (No OpenSSL DLLs)", rdm: true, idm: false, fdm: false, browser: false },
    { metric: "Lightweight Idle RAM (<25 MB)", rdm: true, idm: true, fdm: false, browser: false },
    { metric: "Native Browser Ext (Chrome, Edge, Firefox)", rdm: true, idm: true, fdm: true, browser: false },
    { metric: "Automatic Crash Log Redaction", rdm: true, idm: false, fdm: false, browser: false },
  ];

  return (
    <section id="benchmarks" className="py-20 border-t"
      style={{
        backgroundColor: "var(--bg-page)",
        borderColor: "var(--border-subtle)",
      }}
    >
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="text-center max-w-3xl mx-auto mb-16 space-y-4">
          <div className="inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-xs font-semibold uppercase tracking-wider text-emerald-400 bg-emerald-500/10 border border-emerald-500/20">
            Head-to-Head Comparison
          </div>
          <h2 className="text-3xl sm:text-4xl font-extrabold tracking-tight" style={{ color: "var(--text-heading)" }}>
            Why Switch to Rapid Download Manager?
          </h2>
          <p className="text-base" style={{ color: "var(--text-muted)" }}>
            See how Rapid Download Manager outperforms legacy accelerators and basic browser download managers across performance, security, and ethics.
          </p>
        </div>

        <div className="max-w-4xl mx-auto rounded-2xl border overflow-x-auto shadow-xl"
          style={{
            backgroundColor: "var(--bg-card)",
            borderColor: "var(--border-subtle)",
          }}
        >
          <table className="w-full text-left border-collapse min-w-[600px]">
            <thead>
              <tr className="border-b" style={{ borderColor: "var(--border-subtle)" }}>
                <th className="p-4 sm:p-5 text-sm font-bold uppercase tracking-wider" style={{ color: "var(--text-muted)" }}>
                  Feature / Capability
                </th>
                <th className="p-4 sm:p-5 text-sm font-bold bg-indigo-500/10 text-indigo-400 border-x border-indigo-500/20 text-center">
                  <div className="flex items-center justify-center gap-1.5">
                    <Sparkles className="w-4 h-4 text-indigo-400" />
                    <span>Rapid DM</span>
                  </div>
                </th>
                <th className="p-4 sm:p-5 text-sm font-semibold text-center" style={{ color: "var(--text-muted)" }}>
                  IDM
                </th>
                <th className="p-4 sm:p-5 text-sm font-semibold text-center" style={{ color: "var(--text-muted)" }}>
                  Free DM
                </th>
                <th className="p-4 sm:p-5 text-sm font-semibold text-center" style={{ color: "var(--text-muted)" }}>
                  Browser
                </th>
              </tr>
            </thead>
            <tbody className="divide-y" style={{ borderColor: "var(--border-subtle)" }}>
              {rows.map((r, i) => (
                <tr key={i} className="hover:bg-slate-500/5 transition-colors">
                  <td className="p-4 sm:p-5 text-sm font-medium" style={{ color: "var(--text-heading)" }}>
                    {r.metric}
                  </td>
                  <td className="p-4 sm:p-5 text-center bg-indigo-500/5 border-x border-indigo-500/20">
                    <div className="inline-flex items-center justify-center w-6 h-6 rounded-full bg-emerald-500/20 text-emerald-400 font-bold">
                      <Check className="w-4 h-4" />
                    </div>
                  </td>
                  <td className="p-4 sm:p-5 text-center">
                    {r.idm ? (
                      <Check className="w-5 h-5 text-emerald-500 mx-auto" />
                    ) : (
                      <X className="w-5 h-5 text-rose-500/70 mx-auto" />
                    )}
                  </td>
                  <td className="p-4 sm:p-5 text-center">
                    {r.fdm ? (
                      <Check className="w-5 h-5 text-emerald-500 mx-auto" />
                    ) : (
                      <X className="w-5 h-5 text-rose-500/70 mx-auto" />
                    )}
                  </td>
                  <td className="p-4 sm:p-5 text-center">
                    {r.browser ? (
                      <Check className="w-5 h-5 text-emerald-500 mx-auto" />
                    ) : (
                      <X className="w-5 h-5 text-rose-500/70 mx-auto" />
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
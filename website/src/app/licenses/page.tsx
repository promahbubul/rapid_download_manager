"use client";


import React from "react";
import Link from "next/link";
import Navbar from "@/components/Navbar";
import Footer from "@/components/Footer";
import { FileText, ArrowLeft, CheckCircle2, Shield } from "lucide-react";
import { useTheme } from "@/context/ThemeContext";

export default function LicensesPage() {
  const { resolvedTheme } = useTheme();
  const isDark = resolvedTheme === "dark";

  const crates = [
    { name: "tokio", version: "1.53.1", license: "MIT", desc: "Asynchronous multi-threaded execution runtime" },
    { name: "reqwest", version: "0.12.28", license: "MIT / Apache-2.0", desc: "HTTP/2 and memory-safe Rustls TLS client" },
    { name: "eframe & egui", version: "0.28.1", license: "MIT / Apache-2.0", desc: "Immediate-mode desktop GUI framework" },
    { name: "serde & serde_json", version: "1.0", license: "MIT / Apache-2.0", desc: "Data serialization and storage migration framework" },
    { name: "chrono", version: "0.4.45", license: "MIT / Apache-2.0", desc: "Date, time, and scheduler synchronization engine" },
    { name: "indicatif", version: "0.17.11", license: "MIT", desc: "Terminal progress visualizer for CLI binary" },
    { name: "arboard", version: "3.6.1", license: "MIT / Apache-2.0", desc: "Native Windows clipboard monitoring and link capture" },
    { name: "rfd", version: "0.14.1", license: "MIT", desc: "Native Windows File Explorer picker dialogs" },
    { name: "anyhow & thiserror", version: "1.0 / 2.0", license: "MIT / Apache-2.0", desc: "Idiomatic domain error management" },
  ];

  return (
    <div className="min-h-screen flex flex-col">
      <Navbar />

      <main className="flex-1 max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-14">
        <Link
          href="/"
          className="inline-flex items-center gap-2 text-xs font-semibold text-indigo-400 hover:text-indigo-300 mb-8 transition-colors"
        >
          <ArrowLeft className="w-4 h-4" />
          <span>Back to Home</span>
        </Link>

        <div className="space-y-6">
          <div className="flex items-center gap-3">
            <div className="w-12 h-12 rounded-xl bg-indigo-500/15 text-indigo-400 flex items-center justify-center">
              <FileText className="w-6 h-6" />
            </div>
            <div>
              <h1 className="text-3xl font-extrabold tracking-tight" style={{ color: "var(--text-heading)" }}>
                Open Source Notices & Licenses
              </h1>
              <p className="text-xs" style={{ color: "var(--text-muted)" }}>
                Product: Rapid Download Manager • 100% Permissive Open Source Stack
              </p>
            </div>
          </div>

          <div className="p-4 rounded-xl border flex items-center gap-3"
            style={{
              backgroundColor: isDark ? "rgba(99, 102, 241, 0.08)" : "rgba(99, 102, 241, 0.12)",
              borderColor: "rgba(99, 102, 241, 0.3)",
              color: isDark ? "#a5b4fc" : "#3730a3",
            }}
          >
            <Shield className="w-5 h-5 shrink-0 text-indigo-400" />
            <div className="text-xs font-medium leading-relaxed">
              <strong>License Compliance Guarantee:</strong> 100% of all compiled direct and transitive dependencies utilize standard permissive licenses (MIT, Apache-2.0, BSD). Zero GPL or copyleft restrictions exist.
            </div>
          </div>

          <div className="rounded-2xl border overflow-x-auto shadow-sm"
            style={{
              backgroundColor: "var(--bg-card)",
              borderColor: "var(--border-subtle)",
            }}
          >
            <table className="w-full text-left border-collapse text-xs">
              <thead>
                <tr className="border-b" style={{ borderColor: "var(--border-subtle)" }}>
                  <th className="p-3.5 font-bold uppercase tracking-wider" style={{ color: "var(--text-muted)" }}>Component</th>
                  <th className="p-3.5 font-bold uppercase tracking-wider" style={{ color: "var(--text-muted)" }}>Version</th>
                  <th className="p-3.5 font-bold uppercase tracking-wider" style={{ color: "var(--text-muted)" }}>License</th>
                  <th className="p-3.5 font-bold uppercase tracking-wider" style={{ color: "var(--text-muted)" }}>Function</th>
                </tr>
              </thead>
              <tbody className="divide-y" style={{ borderColor: "var(--border-subtle)" }}>
                {crates.map((c, i) => (
                  <tr key={i} className="hover:bg-slate-500/5 transition-colors">
                    <td className="p-3.5 font-semibold text-indigo-400">{c.name}</td>
                    <td className="p-3.5 font-mono" style={{ color: "var(--text-muted)" }}>{c.version}</td>
                    <td className="p-3.5 font-semibold text-emerald-400">{c.license}</td>
                    <td className="p-3.5" style={{ color: "var(--text-muted)" }}>{c.desc}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>

          <div className="p-6 rounded-2xl border space-y-3"
            style={{
              backgroundColor: "var(--bg-card)",
              borderColor: "var(--border-subtle)",
            }}
          >
            <h3 className="text-sm font-bold" style={{ color: "var(--text-heading)" }}>
              Rapid Download Manager License (MIT)
            </h3>
            <pre className="p-4 rounded-xl text-xs font-mono overflow-x-auto leading-relaxed"
              style={{
                backgroundColor: isDark ? "#090D16" : "#F1F5F9",
                color: "var(--text-body)",
              }}
            >
{`MIT License

Copyright (c) 2026 Mahbubul Alam / Promahbubul

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.`}
            </pre>
          </div>
        </div>
      </main>

      <Footer />
    </div>
  );
}
"use client";

import React from "react";
import Link from "next/link";
import Navbar from "@/components/Navbar";
import Footer from "@/components/Footer";
import {
  Download,
  ShieldCheck,
  CheckCircle2,
  ArrowRight,
  ExternalLink,
  Laptop,
  ArrowLeft,
  Sparkles,
} from "lucide-react";
import { ChromeIcon } from "@/components/Icons";
import { useTheme } from "@/context/ThemeContext";

export default function ExtensionPage() {
  const { resolvedTheme } = useTheme();
  const isDark = resolvedTheme === "dark";

  return (
    <div className="min-h-screen flex flex-col">
      <Navbar />

      <main className="flex-1 max-w-5xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
        <Link
          href="/"
          className="inline-flex items-center gap-2 text-xs font-semibold text-indigo-400 hover:text-indigo-300 mb-6 transition-colors"
        >
          <ArrowLeft className="w-4 h-4" />
          <span>Back to Home</span>
        </Link>

        {/* Hero Section */}
        <div className="text-center max-w-3xl mx-auto space-y-4 mb-14">
          <div className="inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-xs font-semibold text-cyan-400 bg-cyan-500/10 border border-cyan-500/20">
            <Sparkles className="w-3.5 h-3.5 text-cyan-400" />
            <span>Companion Browser Extension • Manifest V3</span>
          </div>

          <h1 className="text-3xl sm:text-5xl font-extrabold tracking-tight" style={{ color: "var(--text-heading)" }}>
            Seamless 1-Click Browser Integration
          </h1>

          <p className="text-sm sm:text-base leading-relaxed" style={{ color: "var(--text-muted)" }}>
            Automatically intercept large file downloads in Google Chrome, Microsoft Edge, Brave, Opera, and Firefox. Seamlessly forwards authentication cookies, session tokens, and referrers to your local native engine.
          </p>

          <div className="pt-2">
            <a
              href="https://github.com/promahbubul/rapid_download_manager/releases/latest"
              target="_blank"
              rel="noopener noreferrer"
              className="inline-flex items-center gap-2 px-6 py-3 rounded-xl bg-gradient-to-r from-cyan-600 via-indigo-600 to-purple-600 text-white text-xs font-bold shadow-lg shadow-indigo-500/25 hover:scale-103 transition-all"
            >
              <Download className="w-4 h-4" />
              <span>Download Extension Package (.zip)</span>
              <span className="text-[11px] opacity-75 font-normal ml-1">206 KB</span>
            </a>
          </div>
        </div>

        {/* Browser Support Grid */}
        <div className="grid grid-cols-2 sm:grid-cols-5 gap-4 mb-14">
          {["Google Chrome", "Microsoft Edge", "Brave Browser", "Opera Browser", "Mozilla Firefox"].map((b, i) => (
            <div
              key={i}
              className="p-4 rounded-xl border text-center space-y-2"
              style={{
                backgroundColor: "var(--bg-card)",
                borderColor: "var(--border-subtle)",
              }}
            >
              <div className="w-10 h-10 rounded-xl bg-indigo-500/10 text-indigo-400 flex items-center justify-center mx-auto">
                <ChromeIcon className="w-5 h-5" />
              </div>
              <div className="text-xs font-bold" style={{ color: "var(--text-heading)" }}>{b}</div>
              <div className="text-[10px] text-emerald-400 font-semibold flex items-center justify-center gap-1">
                <CheckCircle2 className="w-3 h-3" />
                <span>Supported</span>
              </div>
            </div>
          ))}
        </div>

        {/* 3 Step Installation Flow */}
        <div className="p-8 rounded-2xl border space-y-6 mb-14"
          style={{
            backgroundColor: "var(--bg-card)",
            borderColor: "var(--border-subtle)",
          }}
        >
          <h2 className="text-xl font-bold" style={{ color: "var(--text-heading)" }}>
            How to Install the Extension in 3 Steps
          </h2>

          <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
            <div className="space-y-2 p-4 rounded-xl border" style={{ borderColor: "var(--border-subtle)" }}>
              <div className="text-2xl font-black font-mono text-indigo-400">01</div>
              <h3 className="text-sm font-bold" style={{ color: "var(--text-heading)" }}>Extract the Package</h3>
              <p className="text-xs leading-relaxed" style={{ color: "var(--text-muted)" }}>
                Extract <code>RapidExtension_StoreReady.zip</code> to a permanent folder on your PC (or use the <code>extension</code> folder in the app directory).
              </p>
            </div>

            <div className="space-y-2 p-4 rounded-xl border" style={{ borderColor: "var(--border-subtle)" }}>
              <div className="text-2xl font-black font-mono text-indigo-400">02</div>
              <h3 className="text-sm font-bold" style={{ color: "var(--text-heading)" }}>Open Extensions Page</h3>
              <p className="text-xs leading-relaxed" style={{ color: "var(--text-muted)" }}>
                Navigate to <code>chrome://extensions</code> or <code>edge://extensions</code> in your browser and turn on <strong>Developer Mode</strong>.
              </p>
            </div>

            <div className="space-y-2 p-4 rounded-xl border" style={{ borderColor: "var(--border-subtle)" }}>
              <div className="text-2xl font-black font-mono text-indigo-400">03</div>
              <h3 className="text-sm font-bold" style={{ color: "var(--text-heading)" }}>Load Unpacked</h3>
              <p className="text-xs leading-relaxed" style={{ color: "var(--text-muted)" }}>
                Click <strong>Load unpacked</strong>, choose the folder, and you are ready! Links will now automatically trigger Rapid Download Manager.
              </p>
            </div>
          </div>
        </div>

        {/* Security & Architecture Note */}
        <div className="p-6 rounded-2xl border flex items-start gap-4"
          style={{
            backgroundColor: isDark ? "rgba(16, 185, 129, 0.08)" : "rgba(16, 185, 129, 0.12)",
            borderColor: "rgba(16, 185, 129, 0.3)",
          }}
        >
          <ShieldCheck className="w-6 h-6 text-emerald-400 shrink-0 mt-0.5" />
          <div className="space-y-1">
            <h3 className="text-sm font-bold text-emerald-400">Privacy Guarantee (100% Local Loopback)</h3>
            <p className="text-xs leading-relaxed" style={{ color: isDark ? "#cbd5e1" : "#334155" }}>
              The Rapid Download Manager extension contains zero tracking pixels and sends no data to remote cloud servers. All communication is conducted strictly over local loopback (<code>127.0.0.1:18942</code>) between your browser and the native Win32 binary.
            </p>
          </div>
        </div>
      </main>

      <Footer />
    </div>
  );
}

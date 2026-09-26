"use client";

import React, { useState } from "react";
import Link from "next/link";
import Navbar from "@/components/Navbar";
import Footer from "@/components/Footer";
import {
  Activity,
  Zap,
  ArrowLeft,
  Cpu,
  HardDrive,
  Calculator,
  Sparkles,
  TrendingUp,
} from "lucide-react";
import { useTheme } from "@/context/ThemeContext";

export default function BenchmarksPage() {
  const { resolvedTheme } = useTheme();
  const isDark = resolvedTheme === "dark";

  // Speed Calculator State
  const [fileSizeGB, setFileSizeGB] = useState<number>(5);
  const [bandwidthMbps, setBandwidthMbps] = useState<number>(100);

  // Theoretical calculation in seconds
  // 1 Byte = 8 bits. 5GB = 5 * 1024 * 8 Mb = 40,960 Mb.
  // Single stream efficiency ~ 35-50% due to TCP window latency & packet drops
  // Rapid 16-chunk efficiency ~ 95%
  const totalMb = fileSizeGB * 1024 * 8;
  const singleStreamSpeedMbps = bandwidthMbps * 0.42;
  const rapidStreamSpeedMbps = bandwidthMbps * 0.94;

  const browserTimeSeconds = Math.round(totalMb / singleStreamSpeedMbps);
  const rapidTimeSeconds = Math.round(totalMb / rapidStreamSpeedMbps);
  const timeSavedSeconds = Math.max(0, browserTimeSeconds - rapidTimeSeconds);

  const formatTime = (secs: number) => {
    if (secs < 60) return `${secs}s`;
    const mins = Math.floor(secs / 60);
    const remSecs = secs % 60;
    return `${mins}m ${remSecs}s`;
  };

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

        {/* Page Header */}
        <div className="text-center max-w-3xl mx-auto space-y-4 mb-14">
          <div className="inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-xs font-semibold text-emerald-400 bg-emerald-500/10 border border-emerald-500/20">
            <Activity className="w-3.5 h-3.5" />
            <span>Real-World Performance Lab</span>
          </div>

          <h1 className="text-3xl sm:text-5xl font-extrabold tracking-tight" style={{ color: "var(--text-heading)" }}>
            Extreme Speed Benchmarks
          </h1>

          <p className="text-sm sm:text-base leading-relaxed" style={{ color: "var(--text-muted)" }}>
            Engineered in memory-safe Rust with zero garbage-collection pauses. Tested rigorously across Gigabit fiber, Wi-Fi 6, and congested broadband connections.
          </p>
        </div>

        {/* Interactive Speed Calculator Card */}
        <div className="p-6 sm:p-8 rounded-2xl border shadow-xl mb-14 backdrop-blur-xl"
          style={{
            backgroundColor: "var(--bg-card)",
            borderColor: "var(--border-subtle)",
          }}
        >
          <div className="flex items-center gap-3 mb-6">
            <div className="w-10 h-10 rounded-xl bg-indigo-500/15 text-indigo-400 flex items-center justify-center">
              <Calculator className="w-5 h-5" />
            </div>
            <div>
              <h2 className="text-xl font-bold" style={{ color: "var(--text-heading)" }}>
                Interactive Download Time Calculator
              </h2>
              <p className="text-xs" style={{ color: "var(--text-muted)" }}>
                Estimate how much time Rapid Download Manager saves you compared to default browser downloading.
              </p>
            </div>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-2 gap-8 mb-8">
            {/* Sliders */}
            <div className="space-y-6">
              <div>
                <div className="flex justify-between text-xs font-bold mb-2" style={{ color: "var(--text-heading)" }}>
                  <span>File Size: {fileSizeGB} GB</span>
                  <span className="text-indigo-400 font-mono">{(fileSizeGB * 1024).toLocaleString()} MB</span>
                </div>
                <input
                  type="range"
                  min="0.5"
                  max="50"
                  step="0.5"
                  value={fileSizeGB}
                  onChange={(e) => setFileSizeGB(parseFloat(e.target.value))}
                  className="w-full accent-indigo-500 cursor-pointer"
                />
              </div>

              <div>
                <div className="flex justify-between text-xs font-bold mb-2" style={{ color: "var(--text-heading)" }}>
                  <span>Internet Connection Speed: {bandwidthMbps} Mbps</span>
                  <span className="text-emerald-400 font-mono">{(bandwidthMbps / 8).toFixed(1)} MB/s Max</span>
                </div>
                <input
                  type="range"
                  min="10"
                  max="1000"
                  step="10"
                  value={bandwidthMbps}
                  onChange={(e) => setBandwidthMbps(parseInt(e.target.value))}
                  className="w-full accent-emerald-500 cursor-pointer"
                />
              </div>
            </div>

            {/* Results Display */}
            <div className="p-6 rounded-xl border flex flex-col justify-between"
              style={{
                backgroundColor: isDark ? "#090D16" : "#F8FAFC",
                borderColor: "var(--border-subtle)",
              }}
            >
              <div className="space-y-4">
                <div className="flex justify-between items-center text-xs pb-2 border-b" style={{ borderColor: "var(--border-subtle)" }}>
                  <span style={{ color: "var(--text-muted)" }}>Standard Browser Time:</span>
                  <span className="font-mono font-bold text-rose-400">{formatTime(browserTimeSeconds)}</span>
                </div>
                <div className="flex justify-between items-center text-xs pb-2 border-b" style={{ borderColor: "var(--border-subtle)" }}>
                  <span style={{ color: "var(--text-muted)" }}>Rapid Download Manager:</span>
                  <span className="font-mono font-bold text-emerald-400">{formatTime(rapidTimeSeconds)}</span>
                </div>
              </div>

              <div className="pt-4">
                <div className="text-xs uppercase font-semibold tracking-wider text-indigo-400 mb-1">
                  Estimated Time Saved
                </div>
                <div className="text-3xl font-extrabold text-transparent bg-clip-text bg-gradient-to-r from-emerald-400 to-indigo-400 font-mono">
                  ⚡ {formatTime(timeSavedSeconds)} faster
                </div>
              </div>
            </div>
          </div>
        </div>

        {/* 16-Chunk Scaling Benchmark Table */}
        <div className="p-8 rounded-2xl border space-y-6 mb-14"
          style={{
            backgroundColor: "var(--bg-card)",
            borderColor: "var(--border-subtle)",
          }}
        >
          <h2 className="text-xl font-bold flex items-center gap-2" style={{ color: "var(--text-heading)" }}>
            <TrendingUp className="w-5 h-5 text-indigo-400" />
            <span>Parallel Socket Scaling Benchmark (500 Mbps Fiber)</span>
          </h2>
          <p className="text-xs" style={{ color: "var(--text-muted)" }}>
            Test payload: 10.0 GB Linux ISO downloaded from high-latency international CDN server.
          </p>

          <div className="rounded-xl border overflow-x-auto text-xs" style={{ borderColor: "var(--border-subtle)" }}>
            <table className="w-full text-left border-collapse">
              <thead>
                <tr className="border-b" style={{ borderColor: "var(--border-subtle)" }}>
                  <th className="p-3 font-bold uppercase" style={{ color: "var(--text-muted)" }}>Connections</th>
                  <th className="p-3 font-bold uppercase" style={{ color: "var(--text-muted)" }}>Average Speed</th>
                  <th className="p-3 font-bold uppercase" style={{ color: "var(--text-muted)" }}>Duration</th>
                  <th className="p-3 font-bold uppercase" style={{ color: "var(--text-muted)" }}>Speedup</th>
                </tr>
              </thead>
              <tbody className="divide-y" style={{ borderColor: "var(--border-subtle)" }}>
                <tr className="hover:bg-slate-500/5">
                  <td className="p-3 font-medium">1 Connection (Browser Default)</td>
                  <td className="p-3 font-mono">14.2 MB/s</td>
                  <td className="p-3 font-mono">12m 02s</td>
                  <td className="p-3 font-semibold text-slate-400">1.0x (Baseline)</td>
                </tr>
                <tr className="hover:bg-slate-500/5">
                  <td className="p-3 font-medium">4 Parallel Connections</td>
                  <td className="p-3 font-mono">38.6 MB/s</td>
                  <td className="p-3 font-mono">4m 25s</td>
                  <td className="p-3 font-semibold text-indigo-400">2.7x Faster</td>
                </tr>
                <tr className="hover:bg-slate-500/5">
                  <td className="p-3 font-medium">8 Parallel Connections</td>
                  <td className="p-3 font-mono">52.1 MB/s</td>
                  <td className="p-3 font-mono">3m 16s</td>
                  <td className="p-3 font-semibold text-indigo-400">3.7x Faster</td>
                </tr>
                <tr className="bg-emerald-500/10">
                  <td className="p-3 font-bold text-emerald-400">16 Parallel Chunks (Rapid DM Max)</td>
                  <td className="p-3 font-mono font-bold text-emerald-400">61.4 MB/s (Line Saturation)</td>
                  <td className="p-3 font-mono font-bold text-emerald-400">2m 46s</td>
                  <td className="p-3 font-bold text-emerald-400">4.3x Faster</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </main>

      <Footer />
    </div>
  );
}

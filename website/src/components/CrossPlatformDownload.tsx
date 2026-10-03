"use client";

import React from "react";
import Link from "next/link";
import {
  Download,
  CheckCircle2,
  Package,
  Layers,
  Sparkles,
  ExternalLink,
  Laptop,
  FolderArchive,
  ArrowRight,
} from "lucide-react";
import { ChromeIcon, WindowsIcon } from "./Icons";
import { useTheme } from "../context/ThemeContext";

export default function CrossPlatformDownload() {
  const { resolvedTheme } = useTheme();
  const isDark = resolvedTheme === "dark";

  const repoBase = "https://github.com/promahbubul/rapid_download_manager";
  const releaseUrl = `${repoBase}/releases/latest`;
  const rawBase = `${repoBase}/raw/main/dist/installer`;

  const packages = [
    {
      title: "Microsoft Store (MSIX)",
      badge: "Recommended",
      badgeColor: "bg-indigo-600 text-white",
      description: "Official Windows MSIX container with sandboxed execution, zero admin requirements, and auto-updates.",
      specs: ["Windows 10 / 11 (x64)", "Size: ~60 MB (.msix)", "Store Certified"],
      primaryUrl: `${rawBase}/RapidDownloadManager_v1.0.5.msix`,
      primaryLabel: "Download MSIX Package",
      icon: <WindowsIcon className="w-6 h-6 text-indigo-400" />,
      featured: true,
    },
    {
      title: "Windows Setup (.exe)",
      badge: "Classic Installer",
      badgeColor: "bg-slate-500/15 text-slate-300",
      description: "Guided Windows setup wizard with Start Menu shortcuts, desktop icon, autostart toggle, and uninstaller.",
      specs: ["Windows 10 / 11 (x64)", "Size: ~49 MB (.exe)", "Inno Setup Wizard"],
      primaryUrl: `${rawBase}/RapidDownloadManager_Setup_v1.0.4.exe`,
      primaryLabel: "Download Setup (.exe)",
      icon: <Package className="w-6 h-6 text-emerald-400" />,
      featured: false,
    },
    {
      title: "Portable Standalone (.zip)",
      badge: "Zero Install",
      badgeColor: "bg-slate-500/15 text-slate-300",
      description: "No administrative rights or installation needed. Extract anywhere and run rapid-gui.exe immediately.",
      specs: ["Windows 10 / 11 (x64)", "Size: ~61 MB (.zip)", "Extract & Run"],
      primaryUrl: `${rawBase}/RapidDownloadManager_v1.0.5_Portable.zip`,
      primaryLabel: "Download Portable (.zip)",
      icon: <FolderArchive className="w-6 h-6 text-cyan-400" />,
      featured: false,
    },
  ];

  return (
    <section
      id="download"
      className="py-20 border-t"
      style={{
        backgroundColor: "var(--bg-page)",
        borderColor: "var(--border-subtle)",
      }}
    >
      <div className="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 space-y-12">
        {/* Section Header */}
        <div className="text-center max-w-2xl mx-auto space-y-3">
          <div className="inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-xs font-semibold uppercase tracking-wider text-indigo-400 bg-indigo-500/10 border border-indigo-500/20">
            Official Distribution
          </div>
          <h2
            className="text-3xl sm:text-4xl font-extrabold tracking-tight"
            style={{ color: "var(--text-heading)" }}
          >
            Download Rapid Download Manager
          </h2>
          <p className="text-sm sm:text-base leading-relaxed" style={{ color: "var(--text-muted)" }}>
            Current Production Release: <strong className="text-indigo-400">v1.0.5</strong>. Free, open source, and virus-scanned.
          </p>
        </div>

        {/* 3 Windows Distribution Packages */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
          {packages.map((pkg, idx) => (
            <div
              key={idx}
              className={`p-6 sm:p-7 rounded-2xl border flex flex-col justify-between transition-all duration-300 hover:scale-[1.02] hover:shadow-xl relative ${
                pkg.featured ? "ring-2 ring-indigo-500/40" : ""
              }`}
              style={{
                backgroundColor: "var(--bg-card)",
                borderColor: pkg.featured ? "rgba(99, 102, 241, 0.4)" : "var(--border-subtle)",
              }}
            >
              <div className="space-y-4">
                <div className="flex items-center justify-between">
                  <div className="w-12 h-12 rounded-xl bg-slate-500/10 flex items-center justify-center">
                    {pkg.icon}
                  </div>
                  <span className={`text-[11px] font-bold uppercase tracking-wider px-2.5 py-1 rounded-full ${pkg.badgeColor}`}>
                    {pkg.badge}
                  </span>
                </div>

                <div>
                  <h3 className="text-lg font-bold" style={{ color: "var(--text-heading)" }}>
                    {pkg.title}
                  </h3>
                  <p className="text-xs sm:text-sm mt-1 leading-relaxed" style={{ color: "var(--text-muted)" }}>
                    {pkg.description}
                  </p>
                </div>

                <div className="space-y-1.5 pt-2 border-t" style={{ borderColor: "var(--border-subtle)" }}>
                  {pkg.specs.map((spec, sIdx) => (
                    <div key={sIdx} className="flex items-center gap-2 text-xs" style={{ color: "var(--text-muted)" }}>
                      <CheckCircle2 className="w-3.5 h-3.5 text-emerald-400 flex-shrink-0" />
                      <span>{spec}</span>
                    </div>
                  ))}
                </div>
              </div>

              <div className="pt-6">
                <a
                  href={pkg.primaryUrl}
                  className={`w-full py-3 px-4 rounded-xl font-semibold text-xs flex items-center justify-center gap-2 transition-all cursor-pointer shadow-md ${
                    pkg.featured
                      ? "bg-indigo-600 hover:bg-indigo-500 text-white shadow-indigo-600/25"
                      : "border hover:bg-slate-500/10 text-indigo-400"
                  }`}
                  style={{
                    borderColor: pkg.featured ? "transparent" : "var(--border-subtle)",
                    backgroundColor: pkg.featured ? undefined : "var(--bg-card-hover)",
                  }}
                >
                  <Download className="w-4 h-4" />
                  <span>{pkg.primaryLabel}</span>
                </a>
              </div>
            </div>
          ))}
        </div>

        {/* Companion Browser Extension Banner */}
        <div
          className="p-6 sm:p-8 rounded-2xl border flex flex-col sm:flex-row items-center justify-between gap-6"
          style={{
            backgroundColor: isDark ? "rgba(15, 23, 42, 0.6)" : "rgba(241, 245, 249, 0.7)",
            borderColor: "var(--border-subtle)",
          }}
        >
          <div className="flex items-center gap-4">
            <div className="w-12 h-12 rounded-xl bg-indigo-500/15 text-indigo-400 flex items-center justify-center flex-shrink-0">
              <ChromeIcon className="w-6 h-6" />
            </div>
            <div>
              <h4 className="text-base font-bold" style={{ color: "var(--text-heading)" }}>
                Companion Browser Extension (Manifest V3)
              </h4>
              <p className="text-xs sm:text-sm mt-0.5" style={{ color: "var(--text-muted)" }}>
                Automatically capture downloads in Google Chrome, Microsoft Edge, Brave, and Firefox.
              </p>
            </div>
          </div>

          <div className="flex flex-wrap items-center gap-3 w-full sm:w-auto">
            <a
              href="https://addons.mozilla.org/en-US/firefox/addon/rapid-download-manager-integra/"
              target="_blank"
              rel="noopener noreferrer"
              className="flex-1 sm:flex-initial inline-flex items-center justify-center gap-2 px-5 py-2.5 rounded-xl bg-orange-600 hover:bg-orange-500 text-white text-xs font-semibold shadow-md shadow-orange-600/20 transition-all cursor-pointer"
            >
              <ExternalLink className="w-3.5 h-3.5" />
              <span>Firefox Add-on</span>
            </a>
            <Link
              href="/extension"
              className="flex-1 sm:flex-initial inline-flex items-center justify-center gap-2 px-5 py-2.5 rounded-xl border text-xs font-semibold transition-colors hover:bg-slate-500/10 cursor-pointer"
              style={{
                borderColor: "var(--border-subtle)",
                color: "var(--text-heading)",
              }}
            >
              <span>Chrome / Edge Guide</span>
              <ArrowRight className="w-3.5 h-3.5" />
            </Link>
          </div>
        </div>
      </div>
    </section>
  );
}

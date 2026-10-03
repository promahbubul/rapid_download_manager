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
  const releaseDownloadBase = `${repoBase}/releases/download/v1.0.5`;
  const rawBase = `${repoBase}/raw/main/dist/installer`;

  const packages = [
    {
      title: "Microsoft Store",
      badge: "Recommended",
      badgeColor: "bg-indigo-600 text-white",
      description: "Official sandboxed Windows package with automatic background updates and zero admin requirements.",
      meta: "Windows 10 / 11 (64-bit) • 63 MB",
      primaryUrl: `${rawBase}/RapidDownloadManager_v1.0.5.msix`,
      primaryLabel: "Download MSIX",
      icon: <WindowsIcon className="w-5 h-5 text-indigo-400" />,
      featured: true,
    },
    {
      title: "Windows Installer",
      badge: "Setup Wizard",
      badgeColor: "bg-slate-500/15 text-slate-300",
      description: "Traditional Windows installation wizard with Start Menu shortcuts and automatic uninstaller.",
      meta: "Windows 10 / 11 (64-bit) • 50 MB",
      primaryUrl: `${releaseDownloadBase}/RapidDownloadManager_Setup_v1.0.5.exe`,
      primaryLabel: "Download Setup (.exe)",
      icon: <Package className="w-5 h-5 text-emerald-400" />,
      featured: false,
    },
    {
      title: "Portable Archive",
      badge: "Zero-Install",
      badgeColor: "bg-slate-500/15 text-slate-300",
      description: "No installation or admin permissions needed. Extract anywhere and launch immediately.",
      meta: "Windows 10 / 11 (64-bit) • 61 MB",
      primaryUrl: `${releaseDownloadBase}/RapidDownloadManager_v1.0.5_Portable.zip`,
      primaryLabel: "Download Portable (.zip)",
      icon: <FolderArchive className="w-5 h-5 text-cyan-400" />,
      featured: false,
    },
  ];

  return (
    <section
      id="download"
      className="py-16 border-t"
      style={{
        backgroundColor: "var(--bg-page)",
        borderColor: "var(--border-subtle)",
      }}
    >
      <div className="max-w-5xl mx-auto px-4 sm:px-6 lg:px-8 space-y-10">
        {/* Section Header */}
        <div className="text-center max-w-xl mx-auto space-y-2">
          <h2
            className="text-2xl sm:text-3xl font-extrabold tracking-tight"
            style={{ color: "var(--text-heading)" }}
          >
            Download Rapid Download Manager
          </h2>
          <p className="text-sm" style={{ color: "var(--text-muted)" }}>
            Version <strong className="text-indigo-400">1.0.5</strong> • Free & Open Source for Windows 10 & 11
          </p>
        </div>

        {/* 3 Windows Distribution Packages */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-5">
          {packages.map((pkg, idx) => (
            <div
              key={idx}
              className={`p-6 rounded-2xl border flex flex-col justify-between transition-all duration-300 hover:shadow-xl relative ${
                pkg.featured ? "ring-2 ring-indigo-500/40" : ""
              }`}
              style={{
                backgroundColor: "var(--bg-card)",
                borderColor: pkg.featured ? "rgba(99, 102, 241, 0.4)" : "var(--border-subtle)",
              }}
            >
              <div className="space-y-3.5">
                <div className="flex items-center justify-between">
                  <div className="w-10 h-10 rounded-xl bg-slate-500/10 flex items-center justify-center">
                    {pkg.icon}
                  </div>
                  <span className={`text-[11px] font-bold uppercase tracking-wider px-2.5 py-0.5 rounded-full ${pkg.badgeColor}`}>
                    {pkg.badge}
                  </span>
                </div>

                <div>
                  <h3 className="text-base sm:text-lg font-bold" style={{ color: "var(--text-heading)" }}>
                    {pkg.title}
                  </h3>
                  <p className="text-xs sm:text-sm mt-1 leading-relaxed" style={{ color: "var(--text-muted)" }}>
                    {pkg.description}
                  </p>
                </div>

                <div className="text-[11px] font-mono pt-1 text-slate-400">
                  {pkg.meta}
                </div>
              </div>

              <div className="pt-5">
                <a
                  href={pkg.primaryUrl}
                  className={`w-full py-2.5 px-4 rounded-xl font-semibold text-xs flex items-center justify-center gap-2 transition-all cursor-pointer shadow-md ${
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

"use client";

import { GithubIcon } from "./Icons";

import React from "react";
import Link from "next/link";
import { Zap,  Shield, FileText, Heart } from "lucide-react";
import { useTheme } from "../context/ThemeContext";

export default function Footer() {
  const { resolvedTheme } = useTheme();
  const isDark = resolvedTheme === "dark";

  return (
    <footer className="border-t transition-colors duration-200"
      style={{
        backgroundColor: isDark ? "rgba(11, 15, 25, 0.9)" : "rgba(241, 245, 249, 0.9)",
        borderColor: "var(--border-subtle)",
      }}
    >
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
        <div className="grid grid-cols-1 md:grid-cols-4 gap-8 mb-8">
          {/* Brand Col */}
          <div className="md:col-span-2 space-y-3">
            <div className="flex items-center gap-2.5">
              <div className="w-8 h-8 rounded-lg bg-gradient-to-tr from-indigo-600 to-violet-500 flex items-center justify-center shadow-md shadow-indigo-500/25">
                <Zap className="w-4 h-4 text-white fill-white" />
              </div>
              <span className="font-bold text-lg" style={{ color: "var(--text-heading)" }}>
                Rapid Download Manager
              </span>
            </div>
            <p className="text-xs max-w-sm leading-relaxed" style={{ color: "var(--text-muted)" }}>
              High-performance, multi-segment download accelerator engineered in memory-safe Rust with automated browser integration for Windows 10/11.
            </p>
            <div className="text-xs font-medium" style={{ color: "var(--text-muted)" }}>
              Created by <span className="font-semibold text-indigo-400">Mahbubul Alam (promahbubul)</span>
            </div>
          </div>

          {/* Navigation Links */}
          <div className="space-y-3">
            <h4 className="text-xs font-bold uppercase tracking-wider text-indigo-400">
              Product
            </h4>
            <ul className="space-y-2 text-xs" style={{ color: "var(--text-muted)" }}>
              <li><a href="#features" className="hover:text-indigo-400 transition-colors">Key Features</a></li>
              <li><a href="#preview" className="hover:text-indigo-400 transition-colors">Desktop UI Preview</a></li>
              <li><a href="#benchmarks" className="hover:text-indigo-400 transition-colors">Benchmarks & Comparison</a></li>
              <li><a href="#download" className="hover:text-indigo-400 transition-colors">Download Center</a></li>
              <li><a href="#extension" className="hover:text-indigo-400 transition-colors">Browser Extension</a></li>
            </ul>
          </div>

          {/* Legal & Open Source */}
          <div className="space-y-3">
            <h4 className="text-xs font-bold uppercase tracking-wider text-indigo-400">
              Legal & Open Source
            </h4>
            <ul className="space-y-2 text-xs" style={{ color: "var(--text-muted)" }}>
              <li>
                <Link href="/privacy" className="hover:text-indigo-400 transition-colors flex items-center gap-1.5">
                  <Shield className="w-3.5 h-3.5 text-emerald-400" />
                  <span>Privacy Policy</span>
                </Link>
              </li>
              <li>
                <Link href="/licenses" className="hover:text-indigo-400 transition-colors flex items-center gap-1.5">
                  <FileText className="w-3.5 h-3.5 text-indigo-400" />
                  <span>Open Source Licenses</span>
                </Link>
              </li>
              <li>
                <a
                  href="https://github.com/promahbubul/rapid_download_manager"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="hover:text-indigo-400 transition-colors flex items-center gap-1.5"
                >
                  <GithubIcon className="w-3.5 h-3.5" />
                  <span>GitHub Repository</span>
                </a>
              </li>
            </ul>
          </div>
        </div>

        <div className="pt-8 border-t flex flex-col sm:flex-row items-center justify-between text-xs gap-4"
          style={{
            borderColor: "var(--border-subtle)",
            color: "var(--text-muted)",
          }}
        >
          <div>
            &copy; 2026 Rapid Download Manager. All rights reserved. Licensed under MIT.
          </div>
          <div className="flex items-center gap-1">
            <span>Built with precision in</span>
            <span className="font-semibold text-indigo-400">Rust & Next.js</span>
          </div>
        </div>
      </div>
    </footer>
  );
}
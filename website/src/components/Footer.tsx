"use client";

import React from "react";
import Link from "next/link";
import { Shield, FileText, Heart, HelpCircle, Terminal, Activity, Tag, Mail } from "lucide-react";
import { GithubIcon } from "./Icons";
import { useTheme } from "../context/ThemeContext";

export default function Footer() {
  const { resolvedTheme } = useTheme();
  const isDark = resolvedTheme === "dark";

  return (
    <footer
      className="border-t transition-colors duration-200 mt-auto"
      style={{
        backgroundColor: isDark ? "rgba(11, 15, 25, 0.95)" : "rgba(241, 245, 249, 0.95)",
        borderColor: "var(--border-subtle)",
      }}
    >
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
        <div className="grid grid-cols-1 md:grid-cols-5 gap-8 mb-8">
          {/* Brand Col */}
          <div className="md:col-span-2 space-y-3">
            <Link href="/" className="flex items-center gap-3">
              <img
                src="/assets/app_icon.png"
                alt="Rapid Download Manager Logo"
                className="w-8 h-8 object-contain drop-shadow"
              />
              <span className="font-bold text-base sm:text-lg" style={{ color: "var(--text-heading)" }}>
                Rapid Download Manager
              </span>
            </Link>
            <p className="text-xs max-w-sm leading-relaxed" style={{ color: "var(--text-muted)" }}>
              High-performance, multi-segment download accelerator engineered in memory-safe Rust. Built for speed, security, and uncompromised privacy.
            </p>
            <div className="flex items-center gap-3 pt-2 text-xs" style={{ color: "var(--text-muted)" }}>
              <span className="inline-flex items-center gap-1 font-mono text-[11px] text-emerald-400">
                <span className="w-2 h-2 rounded-full bg-emerald-500 animate-pulse" />
                v1.0.0 GA Live
              </span>
              <span>•</span>
              <span>Windows 10/11 Certified</span>
            </div>
          </div>

          {/* Col 1: Software */}
          <div className="space-y-2 text-xs">
            <div className="font-bold uppercase tracking-wider text-[11px]" style={{ color: "var(--text-heading)" }}>
              Product
            </div>
            <ul className="space-y-1.5" style={{ color: "var(--text-muted)" }}>
              <li><a href="/#download" className="hover:text-indigo-400 transition-colors">Download Matrix</a></li>
              <li><a href="/#features" className="hover:text-indigo-400 transition-colors">16x Parallel Engine</a></li>
              <li><a href="/#showcase" className="hover:text-indigo-400 transition-colors">3D Showcase</a></li>
              <li><Link href="/extension" className="hover:text-indigo-400 transition-colors">Browser Extension</Link></li>
              <li><Link href="/benchmarks" className="hover:text-indigo-400 transition-colors">Speed Benchmarks</Link></li>
            </ul>
          </div>

          {/* Col 2: Documentation */}
          <div className="space-y-2 text-xs">
            <div className="font-bold uppercase tracking-wider text-[11px]" style={{ color: "var(--text-heading)" }}>
              Resources
            </div>
            <ul className="space-y-1.5" style={{ color: "var(--text-muted)" }}>
              <li><Link href="/docs" className="hover:text-indigo-400 transition-colors">User Documentation</Link></li>
              <li><Link href="/changelog" className="hover:text-indigo-400 transition-colors">Changelog & Releases</Link></li>
              <li><Link href="/faq" className="hover:text-indigo-400 transition-colors">Frequently Asked Questions</Link></li>
              <li><Link href="/contact" className="hover:text-indigo-400 transition-colors">Support & Contact</Link></li>
              <li>
                <a
                  href="https://github.com/promahbubul/rapid_download_manager"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="hover:text-indigo-400 transition-colors inline-flex items-center gap-1"
                >
                  <span>GitHub Repository</span>
                </a>
              </li>
            </ul>
          </div>

          {/* Col 3: Legal & Trust */}
          <div className="space-y-2 text-xs">
            <div className="font-bold uppercase tracking-wider text-[11px]" style={{ color: "var(--text-heading)" }}>
              Legal & Privacy
            </div>
            <ul className="space-y-1.5" style={{ color: "var(--text-muted)" }}>
              <li><Link href="/privacy" className="hover:text-indigo-400 transition-colors">Zero-Telemetry Privacy</Link></li>
              <li><Link href="/terms" className="hover:text-indigo-400 transition-colors">Terms of Service</Link></li>
              <li><Link href="/licenses" className="hover:text-indigo-400 transition-colors">Open Source Licenses</Link></li>
              <li><a href="https://github.com/promahbubul/rapid_download_manager/security/policy" target="_blank" rel="noopener noreferrer" className="hover:text-indigo-400 transition-colors">Security Policy</a></li>
            </ul>
          </div>
        </div>

        {/* Bottom Bar */}
        <div
          className="pt-6 border-t flex flex-col sm:flex-row items-center justify-between gap-3 text-xs"
          style={{ borderColor: "var(--border-subtle)", color: "var(--text-muted)" }}
        >
          <div>
            &copy; {new Date().getFullYear()} Rapid Download Manager. All rights reserved. Open source under MIT/Apache 2.0.
          </div>
          <div className="flex items-center gap-4">
            <a
              href="https://github.com/promahbubul/rapid_download_manager"
              target="_blank"
              rel="noopener noreferrer"
              className="hover:text-indigo-400 transition-colors"
            >
              GitHub
            </a>
            <Link href="/privacy" className="hover:text-indigo-400 transition-colors">Privacy</Link>
            <Link href="/terms" className="hover:text-indigo-400 transition-colors">Terms</Link>
          </div>
        </div>
      </div>
    </footer>
  );
}

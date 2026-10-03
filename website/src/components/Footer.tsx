"use client";

import React from "react";
import Link from "next/link";
import { GithubIcon } from "./Icons";
import { useTheme } from "../context/ThemeContext";
import { assetUrl } from "@/utils/assets";

export default function Footer() {
  const { resolvedTheme } = useTheme();
  const isDark = resolvedTheme === "dark";

  const rawLogo = resolvedTheme === "light"
    ? "/assets/titlebar_logo_light.png"
    : "/assets/titlebar_logo.png";
  const logoSrc = assetUrl(rawLogo);

  return (
    <footer
      className="border-t transition-colors duration-200 mt-auto"
      style={{
        backgroundColor: isDark ? "rgba(11, 15, 23, 0.95)" : "rgba(241, 245, 249, 0.95)",
        borderColor: "var(--border-subtle)",
      }}
    >
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
        <div className="grid grid-cols-1 md:grid-cols-5 gap-8 mb-8">
          {/* Brand Col with Exact Software Logo */}
          <div className="md:col-span-2 space-y-3">
            <Link href="/" className="inline-block py-1">
              <img
                src={logoSrc}
                alt="Rapid Download Manager"
                className="h-7 w-auto object-contain"
              />
            </Link>
            <p className="text-xs max-w-sm leading-relaxed" style={{ color: "var(--text-muted)" }}>
              High-performance, multi-segment download accelerator engineered in memory-safe Rust. Built for speed, reliability, and zero telemetry privacy.
            </p>
            <div className="flex items-center gap-2 pt-1 text-xs" style={{ color: "var(--text-muted)" }}>
              <span className="w-2 h-2 rounded-full bg-emerald-500 animate-pulse" />
              <span className="font-mono text-[11px] text-emerald-400">v1.0.5 GA Official Release</span>
            </div>
          </div>

          {/* Col 1: Product */}
          <div className="space-y-2 text-xs">
            <div className="font-bold uppercase tracking-wider text-[11px]" style={{ color: "var(--text-heading)" }}>
              Product
            </div>
            <ul className="space-y-1.5" style={{ color: "var(--text-muted)" }}>
              <li><a href="/#download" className="hover:text-indigo-400 transition-colors">Download</a></li>
              <li><a href="/#features" className="hover:text-indigo-400 transition-colors">Features</a></li>
              <li><Link href="/extension" className="hover:text-indigo-400 transition-colors">Browser Extension</Link></li>
              <li><Link href="/changelog" className="hover:text-indigo-400 transition-colors">Changelog (v1.0.5)</Link></li>
            </ul>
          </div>

          {/* Col 2: Resources */}
          <div className="space-y-2 text-xs">
            <div className="font-bold uppercase tracking-wider text-[11px]" style={{ color: "var(--text-heading)" }}>
              Resources
            </div>
            <ul className="space-y-1.5" style={{ color: "var(--text-muted)" }}>
              <li><Link href="/docs" className="hover:text-indigo-400 transition-colors">Documentation</Link></li>
              <li><Link href="/changelog" className="hover:text-indigo-400 transition-colors">Release Notes</Link></li>
              <li><Link href="/faq" className="hover:text-indigo-400 transition-colors">FAQ</Link></li>
              <li><Link href="/contact" className="hover:text-indigo-400 transition-colors">Contact & Support</Link></li>
              <li>
                <a
                  href="https://github.com/promahbubul/rapid_download_manager"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="hover:text-indigo-400 transition-colors"
                >
                  GitHub Repository
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
            &copy; {new Date().getFullYear()} Rapid Download Manager. Open source under MIT/Apache 2.0.
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

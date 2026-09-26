"use client";

import React from "react";
import Link from "next/link";
import { Zap, Shield, FileText, Heart, HelpCircle, Terminal, Activity, Tag, Mail } from "lucide-react";
import { GithubIcon } from "./Icons";
import { useTheme } from "../context/ThemeContext";

export default function Footer() {
  const { resolvedTheme } = useTheme();
  const isDark = resolvedTheme === "dark";

  return (
    <footer className="border-t transition-colors duration-200 mt-auto"
      style={{
        backgroundColor: isDark ? "rgba(11, 15, 25, 0.95)" : "rgba(241, 245, 249, 0.95)",
        borderColor: "var(--border-subtle)",
      }}
    >
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
        <div className="grid grid-cols-1 md:grid-cols-5 gap-8 mb-8">
          {/* Brand Col */}
          <div className="md:col-span-2 space-y-3">
            <Link href="/" className="flex items-center gap-2.5">
              <div className="w-8 h-8 rounded-lg bg-gradient-to-tr from-indigo-600 to-violet-500 flex items-center justify-center shadow-md shadow-indigo-500/25">
                <Zap className="w-4 h-4 text-white fill-white" />
              </div>
              <span className="font-bold text-base sm:text-lg" style={{ color: "var(--text-heading)" }}>
                Rapid Download Manager
              </span>
            </Link>
            <p className="text-xs max-w-sm leading-relaxed" style={{ color: "var(--text-muted)" }}>
              High-performance, multi-segment download accelerator engineered in memory-safe Rust with automated browser integration for Windows 10/11.
            </p>
            <div className="text-xs font-medium" style={{ color: "var(--text-muted)" }}>
              Crafted by <span className="font-semibold text-indigo-400">Mahbubul Alam (promahbubul)</span>
            </div>
          </div>

          {/* Product Col */}
          <div className="space-y-3">
            <h4 className="text-xs font-bold uppercase tracking-wider text-indigo-400">
              Product
            </h4>
            <ul className="space-y-2 text-xs" style={{ color: "var(--text-muted)" }}>
              <li><Link href="/#features" className="hover:text-indigo-400 transition-colors">Key Features</Link></li>
              <li><Link href="/#preview" className="hover:text-indigo-400 transition-colors">Desktop UI Preview</Link></li>
              <li><Link href="/benchmarks" className="hover:text-indigo-400 transition-colors">Speed Lab & Benchmarks</Link></li>
              <li><Link href="/#download" className="hover:text-indigo-400 transition-colors">Download Center</Link></li>
              <li><Link href="/extension" className="hover:text-indigo-400 transition-colors">Browser Extension</Link></li>
            </ul>
          </div>

          {/* Resources & Support Col */}
          <div className="space-y-3">
            <h4 className="text-xs font-bold uppercase tracking-wider text-indigo-400">
              Resources & Support
            </h4>
            <ul className="space-y-2 text-xs" style={{ color: "var(--text-muted)" }}>
              <li><Link href="/docs" className="hover:text-indigo-400 transition-colors">Documentation Hub</Link></li>
              <li><Link href="/changelog" className="hover:text-indigo-400 transition-colors">Release Changelog</Link></li>
              <li><Link href="/faq" className="hover:text-indigo-400 transition-colors">Frequently Asked Questions</Link></li>
              <li><Link href="/contact" className="hover:text-indigo-400 transition-colors">Contact & Feedback</Link></li>
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
                <Link href="/terms" className="hover:text-indigo-400 transition-colors flex items-center gap-1.5">
                  <FileText className="w-3.5 h-3.5 text-indigo-400" />
                  <span>Terms of Service</span>
                </Link>
              </li>
              <li>
                <Link href="/licenses" className="hover:text-indigo-400 transition-colors flex items-center gap-1.5">
                  <FileText className="w-3.5 h-3.5 text-cyan-400" />
                  <span>Third-Party Licenses</span>
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
            &copy; 2026 Rapid Download Manager. Open-source under MIT License.
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

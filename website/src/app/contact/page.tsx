"use client";

import React from "react";
import Link from "next/link";
import Navbar from "@/components/Navbar";
import Footer from "@/components/Footer";
import { Mail, MessageSquare, ArrowLeft, Shield, ExternalLink, Bug, FileCode } from "lucide-react";
import { GithubIcon } from "@/components/Icons";
import { useTheme } from "@/context/ThemeContext";

export default function ContactPage() {
  const { resolvedTheme } = useTheme();
  const isDark = resolvedTheme === "dark";

  return (
    <div className="min-h-screen flex flex-col">
      <Navbar />

      <main className="flex-1 max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
        <Link
          href="/"
          className="inline-flex items-center gap-2 text-xs font-semibold text-indigo-400 hover:text-indigo-300 mb-6 transition-colors"
        >
          <ArrowLeft className="w-4 h-4" />
          <span>Back to Home</span>
        </Link>

        {/* Page Header */}
        <div className="text-center max-w-2xl mx-auto space-y-4 mb-12">
          <div className="inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-xs font-semibold text-indigo-400 bg-indigo-500/10 border border-indigo-500/20">
            <MessageSquare className="w-3.5 h-3.5" />
            <span>Developer Support & Feedback</span>
          </div>

          <h1 className="text-3xl sm:text-5xl font-extrabold tracking-tight" style={{ color: "var(--text-heading)" }}>
            Get in Touch
          </h1>

          <p className="text-sm sm:text-base leading-relaxed" style={{ color: "var(--text-muted)" }}>
            Rapid Download Manager is an open-source project. If you have an inquiry, discovered a bug, or want to contribute, we welcome your feedback.
          </p>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-12">
          {/* Card 1: Email */}
          <div
            className="p-6 rounded-2xl border flex flex-col justify-between"
            style={{
              backgroundColor: "var(--bg-card)",
              borderColor: "var(--border-subtle)",
            }}
          >
            <div className="space-y-3">
              <div className="w-10 h-10 rounded-xl bg-indigo-500/15 text-indigo-400 flex items-center justify-center">
                <Mail className="w-5 h-5" />
              </div>
              <h3 className="text-base font-bold" style={{ color: "var(--text-heading)" }}>
                Direct Email
              </h3>
              <p className="text-xs leading-relaxed" style={{ color: "var(--text-muted)" }}>
                Contact lead developer Mahbubul Alam for security disclosures, partnerships, or direct questions.
              </p>
            </div>
            <div className="pt-6">
              <a
                href="mailto:mahbublalam500@gmail.com"
                className="w-full inline-flex items-center justify-center gap-2 px-4 py-2.5 rounded-xl bg-indigo-600 hover:bg-indigo-500 text-white font-semibold text-xs transition-colors"
              >
                <Mail className="w-4 h-4" />
                <span>mahbublalam500@gmail.com</span>
              </a>
            </div>
          </div>

          {/* Card 2: GitHub Issues */}
          <div
            className="p-6 rounded-2xl border flex flex-col justify-between"
            style={{
              backgroundColor: "var(--bg-card)",
              borderColor: "var(--border-subtle)",
            }}
          >
            <div className="space-y-3">
              <div className="w-10 h-10 rounded-xl bg-purple-500/15 text-purple-400 flex items-center justify-center">
                <GithubIcon className="w-5 h-5" />
              </div>
              <h3 className="text-base font-bold" style={{ color: "var(--text-heading)" }}>
                GitHub Tracker
              </h3>
              <p className="text-xs leading-relaxed" style={{ color: "var(--text-muted)" }}>
                Submit bug reports, feature requests, or browse ongoing development roadmaps on GitHub.
              </p>
            </div>
            <div className="pt-6">
              <a
                href="https://github.com/promahbubul/rapid_download_manager/issues"
                target="_blank"
                rel="noopener noreferrer"
                className="w-full inline-flex items-center justify-center gap-2 px-4 py-2.5 rounded-xl border font-semibold text-xs transition-colors hover:bg-slate-500/10"
                style={{
                  borderColor: "var(--border-subtle)",
                  backgroundColor: "var(--bg-card)",
                  color: "var(--text-heading)",
                }}
              >
                <Bug className="w-4 h-4" />
                <span>Open an Issue &rarr;</span>
              </a>
            </div>
          </div>

          {/* Card 3: Microsoft Publisher */}
          <div
            className="p-6 rounded-2xl border flex flex-col justify-between"
            style={{
              backgroundColor: "var(--bg-card)",
              borderColor: "var(--border-subtle)",
            }}
          >
            <div className="space-y-3">
              <div className="w-10 h-10 rounded-xl bg-emerald-500/15 text-emerald-400 flex items-center justify-center">
                <Shield className="w-5 h-5" />
              </div>
              <h3 className="text-base font-bold" style={{ color: "var(--text-heading)" }}>
                Verified Publisher
              </h3>
              <p className="text-xs leading-relaxed" style={{ color: "var(--text-muted)" }}>
                Published under verified Microsoft Partner Center developer account: <strong>promahbubul</strong>.
              </p>
            </div>
            <div className="pt-6">
              <div
                className="w-full px-3 py-2 rounded-xl border text-center font-mono text-xs"
                style={{
                  borderColor: "var(--border-subtle)",
                  color: "var(--text-muted)",
                  backgroundColor: isDark ? "rgba(15, 23, 42, 0.6)" : "rgba(241, 245, 249, 0.8)",
                }}
              >
                Seller ID: 96219170
              </div>
            </div>
          </div>
        </div>

        {/* Diagnostic Logging Guidance */}
        <div
          className="p-6 sm:p-8 rounded-2xl border space-y-3"
          style={{
            backgroundColor: "var(--bg-card)",
            borderColor: "var(--border-subtle)",
          }}
        >
          <div className="flex items-center gap-2 font-bold text-sm" style={{ color: "var(--text-heading)" }}>
            <FileCode className="w-4 h-4 text-indigo-400" />
            <span>Reporting a Bug? Help Us Fix It Faster</span>
          </div>
          <p className="text-xs leading-relaxed" style={{ color: "var(--text-muted)" }}>
            When filing an issue, including the diagnostic log output helps us resolve connection issues quickly.
            All diagnostic logs are stored locally in:
          </p>
          <div className="p-3 rounded-xl bg-slate-950 border border-slate-800 font-mono text-xs text-indigo-300 overflow-x-auto">
            <code>%APPDATA%\RapidDownloadManager\gui_log.txt</code>
          </div>
          <p className="text-xs text-emerald-400 font-medium pt-1">
            Note: Rapid Download Manager features an automated credential redactor. All passwords, session cookies, and authentication tokens are automatically masked before any diagnostic lines are written to disk.
          </p>
        </div>
      </main>

      <Footer />
    </div>
  );
}

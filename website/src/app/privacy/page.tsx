"use client";


import React from "react";
import Link from "next/link";
import Navbar from "@/components/Navbar";
import Footer from "@/components/Footer";
import { ShieldCheck, ArrowLeft, Lock, HardDrive, Globe, EyeOff } from "lucide-react";
import { useTheme } from "@/context/ThemeContext";

export default function PrivacyPage() {
  const { resolvedTheme } = useTheme();
  const isDark = resolvedTheme === "dark";

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
            <div className="w-12 h-12 rounded-xl bg-emerald-500/15 text-emerald-400 flex items-center justify-center">
              <ShieldCheck className="w-6 h-6" />
            </div>
            <div>
              <h1 className="text-3xl font-extrabold tracking-tight" style={{ color: "var(--text-heading)" }}>
                Privacy Policy
              </h1>
              <p className="text-xs" style={{ color: "var(--text-muted)" }}>
                Effective Date: September 26, 2026 • Product: Rapid Download Manager
              </p>
            </div>
          </div>

          <div className="p-4 rounded-xl border flex items-center gap-3"
            style={{
              backgroundColor: isDark ? "rgba(16, 185, 129, 0.08)" : "rgba(16, 185, 129, 0.12)",
              borderColor: "rgba(16, 185, 129, 0.3)",
              color: isDark ? "#6ee7b7" : "#065f46",
            }}
          >
            <Lock className="w-5 h-5 shrink-0" />
            <div className="text-xs font-medium leading-relaxed">
              <strong>Core Privacy Pledge:</strong> Rapid Download Manager collects zero personal data, transmits zero telemetry, and performs zero user surveillance. All configuration, logs, and downloads reside strictly on your local PC.
            </div>
          </div>

          <div className="space-y-6 text-sm leading-relaxed" style={{ color: "var(--text-body)" }}>
            <section className="space-y-2">
              <h2 className="text-lg font-bold" style={{ color: "var(--text-heading)" }}>
                1. Data Collection & Telemetry (None)
              </h2>
              <p>
                Rapid Download Manager does not collect, harvest, store, or transmit any personal information, browsing history, downloaded files, or device hardware telemetry to any external server or third party. There are no tracking scripts, analytics SDKs, or advertising networks embedded in the software.
              </p>
            </section>

            <section className="space-y-2">
              <h2 className="text-lg font-bold" style={{ color: "var(--text-heading)" }}>
                2. Local Storage & Application State
              </h2>
              <p>
                All data created during your use of the application is saved locally on your personal storage device:
              </p>
              <ul className="list-disc pl-5 space-y-1 text-xs" style={{ color: "var(--text-muted)" }}>
                <li><strong>Downloaded Content:</strong> Saved to your configured local Downloads directory.</li>
                <li><strong>Configuration & State:</strong> Saved to <code>%APPDATA%\RapidDownloadManager\</code>.</li>
                <li><strong>Log Files:</strong> Saved to <code>%LOCALAPPDATA%\RapidDownloadManager\logs\</code>.</li>
              </ul>
            </section>

            <section className="space-y-2">
              <h2 className="text-lg font-bold" style={{ color: "var(--text-heading)" }}>
                3. Browser Extension Integration
              </h2>
              <p>
                The optional companion browser extension functions strictly as a local bridge between your web browser and the local desktop engine. The extension communicates via local loopback sockets (<code>127.0.0.1</code>) and does not transmit URLs or cookies to any cloud services.
              </p>
            </section>

            <section className="space-y-2">
              <h2 className="text-lg font-bold" style={{ color: "var(--text-heading)" }}>
                4. Automated Credential Redaction
              </h2>
              <p>
                The built-in logging subsystem features an automated credential scrubber. Any sensitive authentication parameters (such as tokens, passwords, API keys, or session identifiers) detected in URLs or headers are automatically redacted prior to being written to local log files.
              </p>
            </section>

            <section className="space-y-2">
              <h2 className="text-lg font-bold" style={{ color: "var(--text-heading)" }}>
                5. Developer Contact
              </h2>
              <p>
                If you have inquiries regarding this privacy policy or our open-source software, contact developer Mahbubul Alam at <strong>mahbublalam500@gmail.com</strong> or file an issue on GitHub.
              </p>
            </section>
          </div>
        </div>
      </main>

      <Footer />
    </div>
  );
}
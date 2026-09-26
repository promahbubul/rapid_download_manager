"use client";

import React from "react";
import Link from "next/link";
import Navbar from "@/components/Navbar";
import Footer from "@/components/Footer";
import { FileText, ArrowLeft, ShieldCheck } from "lucide-react";
import { useTheme } from "@/context/ThemeContext";

export default function TermsPage() {
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
        <div className="flex items-center gap-3 mb-8">
          <div className="w-10 h-10 rounded-xl bg-indigo-500/15 text-indigo-400 flex items-center justify-center">
            <FileText className="w-5 h-5" />
          </div>
          <div>
            <h1 className="text-3xl font-extrabold tracking-tight" style={{ color: "var(--text-heading)" }}>
              Terms of Service
            </h1>
            <p className="text-xs" style={{ color: "var(--text-muted)" }}>
              Effective Date: September 26, 2026 • Product: Rapid Download Manager
            </p>
          </div>
        </div>

        <div className="p-6 rounded-2xl border space-y-6 text-xs sm:text-sm leading-relaxed"
          style={{
            backgroundColor: "var(--bg-card)",
            borderColor: "var(--border-subtle)",
            color: "var(--text-body)",
          }}
        >
          <section className="space-y-2">
            <h2 className="text-base font-bold" style={{ color: "var(--text-heading)" }}>
              1. Acceptance of Terms
            </h2>
            <p>
              By installing, downloading, or using Rapid Download Manager, you agree to comply with and be bound by these Terms of Service and our open-source MIT License terms.
            </p>
          </section>

          <section className="space-y-2">
            <h2 className="text-base font-bold" style={{ color: "var(--text-heading)" }}>
              2. Open Source License (MIT)
            </h2>
            <p>
              Rapid Download Manager is provided under the terms of the MIT License. You are free to inspect the source code, run the software for personal and commercial purposes, and contribute enhancements back to the community via GitHub.
            </p>
          </section>

          <section className="space-y-2">
            <h2 className="text-base font-bold" style={{ color: "var(--text-heading)" }}>
              3. Permissible & Lawful Use
            </h2>
            <p>
              You agree to use Rapid Download Manager solely for lawful purposes. You must not use the software to download, intercept, or distribute materials that violate copyright laws, intellectual property rights, or the terms of service of third-party content providers.
            </p>
          </section>

          <section className="space-y-2">
            <h2 className="text-base font-bold" style={{ color: "var(--text-heading)" }}>
              4. Disclaimer of Warranty
            </h2>
            <p>
              The software is provided &ldquo;as is&rdquo;, without warranty of any kind, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose, and non-infringement. In no event shall the authors or copyright holders be liable for any claim, damages, or other liability.
            </p>
          </section>

          <section className="space-y-2">
            <h2 className="text-base font-bold" style={{ color: "var(--text-heading)" }}>
              5. Governing Contact
            </h2>
            <p>
              If you have any questions concerning these terms, contact Mahbubul Alam (promahbubul) at <strong>mahbublalam500@gmail.com</strong>.
            </p>
          </section>
        </div>
      </main>

      <Footer />
    </div>
  );
}

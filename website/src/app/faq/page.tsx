"use client";

import React, { useState } from "react";
import Link from "next/link";
import Navbar from "@/components/Navbar";
import Footer from "@/components/Footer";
import { HelpCircle, ChevronDown, Search, ArrowLeft, Shield, Zap, Sparkles } from "lucide-react";
import { useTheme } from "@/context/ThemeContext";

export default function FAQPage() {
  const { resolvedTheme } = useTheme();
  const isDark = resolvedTheme === "dark";

  const [openIndex, setOpenIndex] = useState<number | null>(0);
  const [searchQuery, setSearchQuery] = useState("");
  const [selectedCategory, setSelectedCategory] = useState<string>("all");

  const faqs = [
    {
      category: "general",
      q: "Is Rapid Download Manager completely free?",
      a: "Yes! Rapid Download Manager is 100% free and open-source software licensed under the MIT License. There are no subscriptions, no paywalled features, and no annoying advertisements.",
    },
    {
      category: "general",
      q: "Why doesn't the app ask for Administrator privileges during installation?",
      a: "Rapid Download Manager is built following certified Windows 10/11 modern desktop security standards. It runs as standard user (asInvoker), storing its configuration in %APPDATA% and downloads in your user folder. It never modifies protected system files or kernel drivers.",
    },
    {
      category: "speed",
      q: "How does 16-chunk parallel acceleration work?",
      a: "When a server supports HTTP byte-ranges (Accept-Ranges: bytes), Rapid Download Manager splits the download into up to 16 equal segments. Each segment is fetched simultaneously through a separate TCP socket, preventing single-connection ISP throttling and saturating your full connection bandwidth.",
    },
    {
      category: "speed",
      q: "Will Rapid Download Manager work on slow or unstable connections?",
      a: "Absolutely. In fact, that is where it shines the most. If a connection drops, Rapid DM uses an exponential-backoff auto-retry system to resume the broken socket right from the exact byte where it paused, avoiding any need to redownload the whole file.",
    },
    {
      category: "extension",
      q: "Which browsers are supported by the companion extension?",
      a: "The extension is built on modern Manifest V3 and works seamlessly on Google Chrome, Microsoft Edge, Brave, Opera, Vivaldi, and Mozilla Firefox.",
    },
    {
      category: "extension",
      q: "How does the extension communicate with the desktop application?",
      a: "It communicates purely through an encrypted local loopback socket (127.0.0.1:18942) residing solely inside your PC. No data, URLs, or cookies are ever relayed to the cloud or third-party servers.",
    },
    {
      category: "privacy",
      q: "Does Rapid Download Manager collect any browsing history or analytics?",
      a: "None whatsoever. There is zero telemetry, zero tracking, and zero analytics scripts embedded in the software. All download queues, speeds, and history remain exclusively on your local storage.",
    },
    {
      category: "privacy",
      q: "What is the automated log credential redactor?",
      a: "If a URL or HTTP header contains sensitive authentication tokens, API keys, or passwords (e.g., token= or Bearer), our logging subsystem automatically sanitizes and redacts them before writing diagnostic reports to disk.",
    },
  ];

  const filteredFaqs = faqs.filter((item) => {
    const matchesCategory = selectedCategory === "all" || item.category === selectedCategory;
    const matchesQuery =
      item.q.toLowerCase().includes(searchQuery.toLowerCase()) ||
      item.a.toLowerCase().includes(searchQuery.toLowerCase());
    return matchesCategory && matchesQuery;
  });

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
        <div className="text-center max-w-3xl mx-auto space-y-4 mb-10">
          <div className="inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-xs font-semibold text-indigo-400 bg-indigo-500/10 border border-indigo-500/20">
            <HelpCircle className="w-3.5 h-3.5" />
            <span>Support & Knowledgebase</span>
          </div>

          <h1 className="text-3xl sm:text-5xl font-extrabold tracking-tight" style={{ color: "var(--text-heading)" }}>
            Frequently Asked Questions
          </h1>

          <p className="text-sm sm:text-base" style={{ color: "var(--text-muted)" }}>
            Have questions about acceleration, browser integration, or privacy? Find direct answers below.
          </p>

          {/* Search Box */}
          <div className="relative max-w-md mx-auto pt-4">
            <Search className="w-4 h-4 absolute left-3.5 top-7 text-slate-400" />
            <input
              type="text"
              placeholder="Search questions (e.g. speed, extension, admin)..."
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-full pl-10 pr-4 py-2.5 rounded-xl border text-xs sm:text-sm focus:outline-none focus:border-indigo-500 transition-colors"
              style={{
                backgroundColor: "var(--bg-card)",
                borderColor: "var(--border-subtle)",
                color: "var(--text-heading)",
              }}
            />
          </div>
        </div>

        {/* Category Filter Pills */}
        <div className="flex flex-wrap items-center justify-center gap-2 mb-10">
          {[
            { id: "all", label: "All Questions" },
            { id: "general", label: "General" },
            { id: "speed", label: "Speed & Sockets" },
            { id: "extension", label: "Browser Extension" },
            { id: "privacy", label: "Privacy & Security" },
          ].map((cat) => (
            <button
              key={cat.id}
              onClick={() => setSelectedCategory(cat.id)}
              className={`px-3.5 py-1.5 rounded-xl text-xs font-semibold transition-all ${
                selectedCategory === cat.id
                  ? "bg-indigo-600 text-white shadow-sm shadow-indigo-500/20"
                  : "hover:bg-slate-500/10 text-slate-400 border border-slate-700/20"
              }`}
            >
              {cat.label}
            </button>
          ))}
        </div>

        {/* Accordion FAQ Items */}
        <div className="space-y-4 mb-14">
          {filteredFaqs.length === 0 ? (
            <div className="text-center py-12 text-sm" style={{ color: "var(--text-muted)" }}>
              No matching questions found for &ldquo;{searchQuery}&rdquo;. Try another term!
            </div>
          ) : (
            filteredFaqs.map((f, idx) => (
              <div
                key={idx}
                className="rounded-2xl border transition-all duration-200 overflow-hidden"
                style={{
                  backgroundColor: "var(--bg-card)",
                  borderColor: "var(--border-subtle)",
                }}
              >
                <button
                  onClick={() => setOpenIndex(openIndex === idx ? null : idx)}
                  className="w-full p-5 text-left flex items-center justify-between gap-4 font-bold text-sm sm:text-base transition-colors hover:text-indigo-400"
                  style={{ color: "var(--text-heading)" }}
                >
                  <span>{f.q}</span>
                  <ChevronDown
                    className={`w-4 h-4 shrink-0 transition-transform duration-200 ${
                      openIndex === idx ? "rotate-180 text-indigo-400" : "text-slate-400"
                    }`}
                  />
                </button>
                {openIndex === idx && (
                  <div className="px-5 pb-5 pt-1 text-xs sm:text-sm leading-relaxed border-t"
                    style={{
                      borderColor: "var(--border-subtle)",
                      color: "var(--text-muted)",
                    }}
                  >
                    {f.a}
                  </div>
                )}
              </div>
            ))
          )}
        </div>
      </main>

      <Footer />
    </div>
  );
}

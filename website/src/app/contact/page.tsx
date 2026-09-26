"use client";

import React, { useState } from "react";
import Link from "next/link";
import Navbar from "@/components/Navbar";
import Footer from "@/components/Footer";
import { Mail, MessageSquare, Send, CheckCircle2, ArrowLeft, Sparkles } from "lucide-react";
import { GithubIcon } from "@/components/Icons";
import { useTheme } from "@/context/ThemeContext";

export default function ContactPage() {
  const { resolvedTheme } = useTheme();
  const isDark = resolvedTheme === "dark";
  const [submitted, setSubmitted] = useState(false);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    setSubmitted(true);
  };

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
            Have a question, encountered an unexpected bug, or want to suggest a new feature? We are always glad to help!
          </p>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-8 mb-14">
          {/* Contact Card 1: Email */}
          <div className="p-6 rounded-2xl border space-y-3"
            style={{
              backgroundColor: "var(--bg-card)",
              borderColor: "var(--border-subtle)",
            }}
          >
            <div className="w-10 h-10 rounded-xl bg-indigo-500/15 text-indigo-400 flex items-center justify-center">
              <Mail className="w-5 h-5" />
            </div>
            <h3 className="text-sm font-bold" style={{ color: "var(--text-heading)" }}>Direct Developer Email</h3>
            <p className="text-xs" style={{ color: "var(--text-muted)" }}>
              Reach out directly to lead architect Mahbubul Alam for urgent inquiries.
            </p>
            <a
              href="mailto:mahbublalam500@gmail.com"
              className="text-xs font-mono font-bold text-indigo-400 hover:underline block pt-1"
            >
              mahbublalam500@gmail.com
            </a>
          </div>

          {/* Contact Card 2: GitHub Issues */}
          <div className="p-6 rounded-2xl border space-y-3"
            style={{
              backgroundColor: "var(--bg-card)",
              borderColor: "var(--border-subtle)",
            }}
          >
            <div className="w-10 h-10 rounded-xl bg-purple-500/15 text-purple-400 flex items-center justify-center">
              <GithubIcon className="w-5 h-5" />
            </div>
            <h3 className="text-sm font-bold" style={{ color: "var(--text-heading)" }}>GitHub Issue Tracker</h3>
            <p className="text-xs" style={{ color: "var(--text-muted)" }}>
              Report a bug, submit diagnostic logs, or discuss roadmap feature requests.
            </p>
            <a
              href="https://github.com/promahbubul/rapid_download_manager/issues"
              target="_blank"
              rel="noopener noreferrer"
              className="text-xs font-bold text-purple-400 hover:underline block pt-1"
            >
              Open a GitHub Issue &rarr;
            </a>
          </div>

          {/* Contact Card 3: Microsoft Store */}
          <div className="p-6 rounded-2xl border space-y-3"
            style={{
              backgroundColor: "var(--bg-card)",
              borderColor: "var(--border-subtle)",
            }}
          >
            <div className="w-10 h-10 rounded-xl bg-emerald-500/15 text-emerald-400 flex items-center justify-center">
              <Sparkles className="w-5 h-5" />
            </div>
            <h3 className="text-sm font-bold" style={{ color: "var(--text-heading)" }}>Microsoft Partner Center</h3>
            <p className="text-xs" style={{ color: "var(--text-muted)" }}>
              Published under verified developer account: <strong>promahbubul</strong>.
            </p>
            <div className="text-xs font-mono text-emerald-400 pt-1">
              Seller ID: 96219170
            </div>
          </div>
        </div>

        {/* Interactive Feedback Form */}
        <div className="p-8 rounded-2xl border shadow-xl"
          style={{
            backgroundColor: "var(--bg-card)",
            borderColor: "var(--border-subtle)",
          }}
        >
          <h2 className="text-xl font-bold mb-2" style={{ color: "var(--text-heading)" }}>
            Send Us a Quick Message
          </h2>
          <p className="text-xs mb-6" style={{ color: "var(--text-muted)" }}>
            Feel free to send feedback or questions regarding Rapid Download Manager.
          </p>

          {submitted ? (
            <div className="p-6 rounded-xl border bg-emerald-500/10 border-emerald-500/30 text-emerald-400 text-center space-y-2">
              <CheckCircle2 className="w-8 h-8 mx-auto" />
              <div className="font-bold text-sm">Thank You for Your Feedback!</div>
              <div className="text-xs opacity-90">We will review your message promptly.</div>
            </div>
          ) : (
            <form onSubmit={handleSubmit} className="space-y-4">
              <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
                <div>
                  <label className="block text-xs font-bold mb-1.5" style={{ color: "var(--text-heading)" }}>
                    Your Name
                  </label>
                  <input
                    type="text"
                    required
                    placeholder="e.g. Mahbubul Alam"
                    className="w-full px-3.5 py-2.5 rounded-xl border text-xs focus:outline-none focus:border-indigo-500"
                    style={{
                      backgroundColor: "var(--bg-page)",
                      borderColor: "var(--border-subtle)",
                      color: "var(--text-heading)",
                    }}
                  />
                </div>
                <div>
                  <label className="block text-xs font-bold mb-1.5" style={{ color: "var(--text-heading)" }}>
                    Email Address
                  </label>
                  <input
                    type="email"
                    required
                    placeholder="e.g. you@example.com"
                    className="w-full px-3.5 py-2.5 rounded-xl border text-xs focus:outline-none focus:border-indigo-500"
                    style={{
                      backgroundColor: "var(--bg-page)",
                      borderColor: "var(--border-subtle)",
                      color: "var(--text-heading)",
                    }}
                  />
                </div>
              </div>

              <div>
                <label className="block text-xs font-bold mb-1.5" style={{ color: "var(--text-heading)" }}>
                  Subject / Topic
                </label>
                <input
                  type="text"
                  required
                  placeholder="e.g. Feature request for custom proxy authentication"
                  className="w-full px-3.5 py-2.5 rounded-xl border text-xs focus:outline-none focus:border-indigo-500"
                  style={{
                    backgroundColor: "var(--bg-page)",
                    borderColor: "var(--border-subtle)",
                    color: "var(--text-heading)",
                  }}
                />
              </div>

              <div>
                <label className="block text-xs font-bold mb-1.5" style={{ color: "var(--text-heading)" }}>
                  Message
                </label>
                <textarea
                  rows={4}
                  required
                  placeholder="Type your message or details here..."
                  className="w-full px-3.5 py-2.5 rounded-xl border text-xs focus:outline-none focus:border-indigo-500"
                  style={{
                    backgroundColor: "var(--bg-page)",
                    borderColor: "var(--border-subtle)",
                    color: "var(--text-heading)",
                  }}
                />
              </div>

              <button
                type="submit"
                className="px-6 py-2.5 rounded-xl bg-gradient-to-r from-indigo-600 to-purple-600 text-white font-semibold text-xs flex items-center gap-2 shadow-md shadow-indigo-500/25 hover:shadow-indigo-500/40 hover:scale-102 transition-all"
              >
                <Send className="w-3.5 h-3.5" />
                <span>Send Message</span>
              </button>
            </form>
          )}
        </div>
      </main>

      <Footer />
    </div>
  );
}

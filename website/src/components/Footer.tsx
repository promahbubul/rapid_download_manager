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
      className="border-t transition-colors duration-200 py-6 mt-auto"
      style={{
        backgroundColor: isDark ? "rgba(11, 15, 23, 0.95)" : "rgba(241, 245, 249, 0.95)",
        borderColor: "var(--border-subtle)",
      }}
    >
      <div className="max-w-5xl mx-auto px-4 sm:px-6 lg:px-8 flex flex-col sm:flex-row items-center justify-between gap-4 text-xs" style={{ color: "var(--text-muted)" }}>
        <div className="flex items-center gap-3">
          <Link href="/">
            <img src={logoSrc} alt="Rapid Download Manager" className="h-6 w-auto object-contain" />
          </Link>
          <span className="opacity-40">•</span>
          <span>v1.0.5 • Free & Open Source</span>
        </div>

        <div className="flex flex-wrap items-center gap-4 sm:gap-6">
          <Link href="/docs" className="hover:text-indigo-400 transition-colors">Documentation</Link>
          <Link href="/extension" className="hover:text-indigo-400 transition-colors">Extension</Link>
          <Link href="/changelog" className="hover:text-indigo-400 transition-colors">Changelog</Link>
          <Link href="/privacy" className="hover:text-indigo-400 transition-colors">Privacy</Link>
          <a
            href="https://github.com/promahbubul/rapid_download_manager"
            target="_blank"
            rel="noopener noreferrer"
            className="flex items-center gap-1 hover:text-indigo-400 transition-colors"
          >
            <GithubIcon className="w-3.5 h-3.5" />
            <span>GitHub</span>
          </a>
        </div>
      </div>
    </footer>
  );
}

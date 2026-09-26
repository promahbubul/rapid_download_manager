"use client";

import React, { useState, useEffect, useRef } from "react";
import Link from "next/link";
import { useTheme } from "../context/ThemeContext";
import {
  Zap,
  Moon,
  Sun,
  Laptop,
  Check,
  ChevronDown,
  Menu,
  X,
  Download,
} from "lucide-react";
import { GithubIcon } from "./Icons";

export default function Navbar() {
  const { theme, resolvedTheme, setTheme } = useTheme();
  const [dropdownOpen, setDropdownOpen] = useState(false);
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);
  const dropdownRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    function handleClickOutside(event: MouseEvent) {
      if (dropdownRef.current && !dropdownRef.current.contains(event.target as Node)) {
        setDropdownOpen(false);
      }
    }
    document.addEventListener("mousedown", handleClickOutside);
    return () => document.removeEventListener("mousedown", handleClickOutside);
  }, []);

  const navLinks = [
    { name: "Features", href: "/#features" },
    { name: "Docs", href: "/docs" },
    { name: "Extension", href: "/extension" },
    { name: "Benchmarks", href: "/benchmarks" },
    { name: "Changelog", href: "/changelog" },
    { name: "FAQ", href: "/faq" },
  ];

  return (
    <header className="sticky top-0 z-50 w-full backdrop-blur-xl border-b transition-colors duration-200"
      style={{
        backgroundColor: resolvedTheme === "dark" ? "rgba(11, 15, 25, 0.88)" : "rgba(255, 255, 255, 0.88)",
        borderColor: resolvedTheme === "dark" ? "rgba(30, 41, 59, 0.8)" : "rgba(226, 232, 240, 0.8)",
      }}
    >
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 h-16 flex items-center justify-between">
        {/* Brand Logo & Title */}
        <Link href="/" className="flex items-center gap-2.5 group">
          <div className="w-9 h-9 rounded-xl bg-gradient-to-tr from-indigo-600 to-violet-500 flex items-center justify-center shadow-md shadow-indigo-500/25 group-hover:scale-105 transition-transform duration-200">
            <Zap className="w-5 h-5 text-white fill-white" />
          </div>
          <div className="flex flex-col">
            <div className="flex items-center gap-1.5">
              <span className="font-bold text-base sm:text-lg tracking-tight" style={{ color: "var(--text-heading)" }}>
                Rapid
              </span>
              <span className="font-light text-base sm:text-lg tracking-tight" style={{ color: "var(--text-heading)" }}>
                Download Manager
              </span>
              <span className="hidden lg:inline-flex text-[10px] font-semibold px-1.5 py-0.5 rounded-full border border-indigo-500/30 text-indigo-400 bg-indigo-500/10">
                v1.0.0
              </span>
            </div>
            <span className="hidden sm:inline text-[9px] tracking-wider uppercase font-semibold" style={{ color: "var(--text-muted)" }}>
              Rust Windows Accelerator
            </span>
          </div>
        </Link>

        {/* Desktop Navigation Links */}
        <nav className="hidden md:flex items-center gap-5 lg:gap-6">
          {navLinks.map((link) => (
            <Link
              key={link.name}
              href={link.href}
              className="text-xs sm:text-sm font-medium transition-colors hover:text-indigo-400"
              style={{ color: "var(--text-muted)" }}
            >
              {link.name}
            </Link>
          ))}
        </nav>

        {/* Action Controls & Theme Toggle */}
        <div className="flex items-center gap-2.5">
          {/* GitHub Star Button */}
          <a
            href="https://github.com/promahbubul/rapid_download_manager"
            target="_blank"
            rel="noopener noreferrer"
            className="hidden sm:flex items-center gap-1.5 px-3 py-1.5 rounded-xl border text-xs font-semibold transition-all hover:border-indigo-500 hover:scale-102"
            style={{
              borderColor: "var(--border-subtle)",
              backgroundColor: "var(--bg-card)",
              color: "var(--text-heading)",
            }}
          >
            <GithubIcon className="w-3.5 h-3.5" />
            <span>GitHub</span>
          </a>

          {/* Theme Dropdown Toggle */}
          <div className="relative" ref={dropdownRef}>
            <button
              onClick={() => setDropdownOpen(!dropdownOpen)}
              className="w-9 h-9 rounded-xl border flex items-center justify-center transition-all duration-200 hover:border-indigo-500 hover:scale-105 focus:outline-none"
              style={{
                borderColor: "var(--border-subtle)",
                backgroundColor: "var(--bg-card)",
                color: "var(--text-heading)",
              }}
              title="Switch Appearance Theme"
              aria-label="Toggle theme dropdown"
            >
              {theme === "light" && <Sun className="w-4 h-4 text-amber-500" />}
              {theme === "dark" && <Moon className="w-4 h-4 text-indigo-400" />}
              {theme === "system" && <Laptop className="w-4 h-4 text-emerald-400" />}
            </button>

            {/* Floating Dropdown Menu */}
            {dropdownOpen && (
              <div
                className="absolute right-0 mt-2 w-44 rounded-2xl border shadow-2xl p-1.5 backdrop-blur-2xl transition-all duration-150 animate-in fade-in zoom-in-95 z-50"
                style={{
                  backgroundColor: resolvedTheme === "dark" ? "rgba(17, 24, 39, 0.95)" : "rgba(255, 255, 255, 0.95)",
                  borderColor: "var(--border-subtle)",
                }}
              >
                <div className="px-2.5 py-1 text-[10px] font-bold uppercase tracking-wider text-indigo-400">
                  Select Theme
                </div>

                <button
                  onClick={() => {
                    setTheme("light");
                    setDropdownOpen(false);
                  }}
                  className={`w-full flex items-center justify-between px-3 py-2 rounded-xl text-xs font-medium transition-all ${
                    theme === "light" ? "bg-indigo-500/15 text-indigo-400 font-semibold" : "hover:bg-slate-500/10"
                  }`}
                  style={{ color: theme === "light" ? "#818cf8" : "var(--text-heading)" }}
                >
                  <div className="flex items-center gap-2">
                    <Sun className="w-3.5 h-3.5 text-amber-500" />
                    <span>Light Mode</span>
                  </div>
                  {theme === "light" && <Check className="w-3.5 h-3.5 text-indigo-400" />}
                </button>

                <button
                  onClick={() => {
                    setTheme("dark");
                    setDropdownOpen(false);
                  }}
                  className={`w-full flex items-center justify-between px-3 py-2 rounded-xl text-xs font-medium transition-all ${
                    theme === "dark" ? "bg-indigo-500/15 text-indigo-400 font-semibold" : "hover:bg-slate-500/10"
                  }`}
                  style={{ color: theme === "dark" ? "#818cf8" : "var(--text-heading)" }}
                >
                  <div className="flex items-center gap-2">
                    <Moon className="w-3.5 h-3.5 text-indigo-400" />
                    <span>Dark Mode</span>
                  </div>
                  {theme === "dark" && <Check className="w-3.5 h-3.5 text-indigo-400" />}
                </button>

                <button
                  onClick={() => {
                    setTheme("system");
                    setDropdownOpen(false);
                  }}
                  className={`w-full flex items-center justify-between px-3 py-2 rounded-xl text-xs font-medium transition-all ${
                    theme === "system" ? "bg-indigo-500/15 text-indigo-400 font-semibold" : "hover:bg-slate-500/10"
                  }`}
                  style={{ color: theme === "system" ? "#818cf8" : "var(--text-heading)" }}
                >
                  <div className="flex items-center gap-2">
                    <Laptop className="w-3.5 h-3.5 text-emerald-400" />
                    <span>System Default</span>
                  </div>
                  {theme === "system" && <Check className="w-3.5 h-3.5 text-indigo-400" />}
                </button>
              </div>
            )}
          </div>

          {/* Download CTA Button */}
          <Link
            href="/#download"
            className="flex items-center gap-1.5 px-3.5 py-2 rounded-xl bg-gradient-to-r from-indigo-600 via-indigo-500 to-violet-600 text-white text-xs font-semibold shadow-md shadow-indigo-500/25 hover:shadow-indigo-500/40 hover:scale-102 transition-all duration-200"
          >
            <Download className="w-3.5 h-3.5" />
            <span className="hidden sm:inline">Get Free</span>
          </Link>

          {/* Mobile Menu Hamburger */}
          <button
            onClick={() => setMobileMenuOpen(!mobileMenuOpen)}
            className="md:hidden w-9 h-9 rounded-xl border flex items-center justify-center"
            style={{
              borderColor: "var(--border-subtle)",
              backgroundColor: "var(--bg-card)",
              color: "var(--text-heading)",
            }}
            aria-label="Toggle mobile menu"
          >
            {mobileMenuOpen ? <X className="w-4 h-4" /> : <Menu className="w-4 h-4" />}
          </button>
        </div>
      </div>

      {/* Mobile Drawer Menu */}
      {mobileMenuOpen && (
        <div
          className="md:hidden border-b px-4 py-4 space-y-2.5"
          style={{
            backgroundColor: "var(--bg-page)",
            borderColor: "var(--border-subtle)",
          }}
        >
          {navLinks.map((link) => (
            <Link
              key={link.name}
              href={link.href}
              onClick={() => setMobileMenuOpen(false)}
              className="block px-3 py-2 rounded-lg text-xs font-semibold hover:bg-indigo-500/10 hover:text-indigo-400"
              style={{ color: "var(--text-heading)" }}
            >
              {link.name}
            </Link>
          ))}
          <div className="pt-2 border-t border-slate-700/20 flex items-center justify-between">
            <a
              href="https://github.com/promahbubul/rapid_download_manager"
              target="_blank"
              rel="noopener noreferrer"
              className="flex items-center gap-2 text-xs font-medium"
              style={{ color: "var(--text-muted)" }}
            >
              <GithubIcon className="w-3.5 h-3.5" />
              <span>GitHub Repo</span>
            </a>
            <span className="text-[10px] font-bold text-indigo-400">v1.0.0</span>
          </div>
        </div>
      )}
    </header>
  );
}

"use client";

import React, { useState, useEffect, useRef } from "react";
import Link from "next/link";
import { useTheme } from "../context/ThemeContext";
import {
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
    { name: "3D Showcase", href: "/#showcase" },
    { name: "Download", href: "/#download" },
    { name: "Docs", href: "/docs" },
    { name: "Extension", href: "/extension" },
    { name: "Benchmarks", href: "/benchmarks" },
    { name: "Changelog", href: "/changelog" },
    { name: "FAQ", href: "/faq" },
  ];

  return (
    <header
      className="sticky top-0 z-50 w-full backdrop-blur-xl border-b transition-colors duration-200"
      style={{
        backgroundColor: resolvedTheme === "dark" ? "rgba(11, 15, 25, 0.88)" : "rgba(255, 255, 255, 0.88)",
        borderColor: resolvedTheme === "dark" ? "rgba(30, 41, 59, 0.8)" : "rgba(226, 232, 240, 0.8)",
      }}
    >
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 h-16 flex items-center justify-between">
        {/* Brand Logo with Official Software App Icon */}
        <Link href="/" className="flex items-center gap-3 group">
          <div className="relative w-9 h-9 flex items-center justify-center group-hover:scale-108 transition-transform duration-300">
            <img
              src="/assets/app_icon.png"
              alt="Rapid Download Manager Icon"
              className="w-9 h-9 object-contain drop-shadow-md"
            />
          </div>
          <div className="flex flex-col">
            <div className="flex items-center gap-1.5">
              <span className="font-extrabold text-base sm:text-lg tracking-tight" style={{ color: "var(--text-heading)" }}>
                Rapid
              </span>
              <span className="font-light text-base sm:text-lg tracking-tight" style={{ color: "var(--text-heading)" }}>
                Download Manager
              </span>
              <span className="hidden lg:inline-flex text-[10px] font-semibold px-1.5 py-0.5 rounded-full border border-indigo-500/30 text-indigo-400 bg-indigo-500/10">
                v1.0.0 GA
              </span>
            </div>
            <span className="hidden sm:inline text-[9px] tracking-wider uppercase font-semibold text-slate-400">
              Rust Acceleration • Multi-Platform
            </span>
          </div>
        </Link>

        {/* Desktop Navigation Links */}
        <nav className="hidden md:flex items-center gap-1 lg:gap-2">
          {navLinks.map((link) => (
            <Link
              key={link.name}
              href={link.href}
              className="px-2.5 py-1.5 rounded-lg text-xs lg:text-sm font-medium transition-colors hover:text-indigo-400"
              style={{ color: "var(--text-muted)" }}
            >
              {link.name}
            </Link>
          ))}
        </nav>

        {/* Right Action Icons & Theme Switcher */}
        <div className="flex items-center gap-2 sm:gap-3">
          {/* GitHub Star Button */}
          <a
            href="https://github.com/promahbubul/rapid_download_manager"
            target="_blank"
            rel="noopener noreferrer"
            className="p-2 rounded-xl border transition-all hover:scale-105 hidden sm:flex items-center justify-center"
            style={{
              borderColor: "var(--border-subtle)",
              backgroundColor: "var(--bg-card)",
              color: "var(--text-heading)",
            }}
            title="View on GitHub"
          >
            <GithubIcon className="w-4 h-4" />
          </a>

          {/* 3-Mode Theme Dropdown */}
          <div className="relative" ref={dropdownRef}>
            <button
              onClick={() => setDropdownOpen(!dropdownOpen)}
              className="p-2 rounded-xl border flex items-center gap-1 text-xs font-medium transition-all hover:scale-105 cursor-pointer"
              style={{
                borderColor: "var(--border-subtle)",
                backgroundColor: "var(--bg-card)",
                color: "var(--text-heading)",
              }}
              title="Toggle Theme (Light, Dark, System)"
              aria-label="Theme menu"
            >
              {resolvedTheme === "dark" ? (
                <Moon className="w-4 h-4 text-indigo-400" />
              ) : (
                <Sun className="w-4 h-4 text-amber-500" />
              )}
              <ChevronDown className="w-3 h-3 opacity-60" />
            </button>

            {dropdownOpen && (
              <div
                className="absolute right-0 mt-2 w-36 rounded-xl border shadow-xl p-1.5 z-50 text-xs backdrop-blur-xl animate-in fade-in zoom-in-95 duration-100"
                style={{
                  backgroundColor: resolvedTheme === "dark" ? "#111827" : "#FFFFFF",
                  borderColor: "var(--border-subtle)",
                }}
              >
                <button
                  onClick={() => {
                    setTheme("light");
                    setDropdownOpen(false);
                  }}
                  className="w-full flex items-center justify-between px-3 py-2 rounded-lg font-medium transition-colors hover:bg-slate-500/10 cursor-pointer"
                  style={{ color: theme === "light" ? "var(--accent-primary)" : "var(--text-heading)" }}
                >
                  <div className="flex items-center gap-2">
                    <Sun className="w-3.5 h-3.5 text-amber-500" />
                    <span>Light</span>
                  </div>
                  {theme === "light" && <Check className="w-3.5 h-3.5 text-indigo-500" />}
                </button>

                <button
                  onClick={() => {
                    setTheme("dark");
                    setDropdownOpen(false);
                  }}
                  className="w-full flex items-center justify-between px-3 py-2 rounded-lg font-medium transition-colors hover:bg-slate-500/10 cursor-pointer"
                  style={{ color: theme === "dark" ? "var(--accent-primary)" : "var(--text-heading)" }}
                >
                  <div className="flex items-center gap-2">
                    <Moon className="w-3.5 h-3.5 text-indigo-400" />
                    <span>Dark</span>
                  </div>
                  {theme === "dark" && <Check className="w-3.5 h-3.5 text-indigo-500" />}
                </button>

                <button
                  onClick={() => {
                    setTheme("system");
                    setDropdownOpen(false);
                  }}
                  className="w-full flex items-center justify-between px-3 py-2 rounded-lg font-medium transition-colors hover:bg-slate-500/10 cursor-pointer"
                  style={{ color: theme === "system" ? "var(--accent-primary)" : "var(--text-heading)" }}
                >
                  <div className="flex items-center gap-2">
                    <Laptop className="w-3.5 h-3.5 text-cyan-400" />
                    <span>System</span>
                  </div>
                  {theme === "system" && <Check className="w-3.5 h-3.5 text-indigo-500" />}
                </button>
              </div>
            )}
          </div>

          {/* Direct CTA */}
          <a
            href="#download"
            className="hidden sm:inline-flex items-center gap-1.5 px-3.5 py-2 rounded-xl bg-gradient-to-r from-indigo-600 to-violet-600 text-white text-xs font-semibold shadow-md shadow-indigo-500/20 hover:scale-104 transition-all duration-200"
          >
            <Download className="w-3.5 h-3.5" />
            <span>Download</span>
          </a>

          {/* Mobile Menu Hamburger */}
          <button
            onClick={() => setMobileMenuOpen(!mobileMenuOpen)}
            className="p-2 rounded-xl border md:hidden"
            style={{
              borderColor: "var(--border-subtle)",
              backgroundColor: "var(--bg-card)",
              color: "var(--text-heading)",
            }}
          >
            {mobileMenuOpen ? <X className="w-4 h-4" /> : <Menu className="w-4 h-4" />}
          </button>
        </div>
      </div>

      {/* Mobile Drawer */}
      {mobileMenuOpen && (
        <div
          className="md:hidden border-b px-4 py-4 space-y-2 backdrop-blur-xl"
          style={{
            backgroundColor: resolvedTheme === "dark" ? "#0b0f19" : "#FFFFFF",
            borderColor: "var(--border-subtle)",
          }}
        >
          {navLinks.map((link) => (
            <Link
              key={link.name}
              href={link.href}
              onClick={() => setMobileMenuOpen(false)}
              className="block px-3 py-2 rounded-lg text-sm font-medium hover:bg-slate-500/10"
              style={{ color: "var(--text-heading)" }}
            >
              {link.name}
            </Link>
          ))}
          <div className="pt-2">
            <a
              href="#download"
              onClick={() => setMobileMenuOpen(false)}
              className="w-full flex items-center justify-center gap-2 px-4 py-2.5 rounded-xl bg-indigo-600 text-white text-sm font-semibold shadow-md"
            >
              <Download className="w-4 h-4" />
              <span>Download v1.0.0</span>
            </a>
          </div>
        </div>
      )}
    </header>
  );
}

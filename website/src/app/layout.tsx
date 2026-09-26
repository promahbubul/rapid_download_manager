import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";
import { ThemeProvider } from "@/context/ThemeContext";

const inter = Inter({
  subsets: ["latin"],
  variable: "--font-inter",
});

export const metadata: Metadata = {
  title: "Rapid Download Manager — High-Speed Download Accelerator for Windows",
  description:
    "Turbocharged multi-segment download accelerator engineered in memory-safe Rust with automated browser integration for Windows 10/11.",
  keywords: [
    "download manager",
    "download accelerator",
    "windows download manager",
    "rust download manager",
    "idm alternative",
    "rapid download manager",
  ],
  authors: [{ name: "Mahbubul Alam (promahbubul)" }],
  openGraph: {
    title: "Rapid Download Manager — High-Speed Download Accelerator",
    description: "16x chunk acceleration, zero telemetry, memory-safe Rust for Windows.",
    type: "website",
  },
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en" className={`${inter.variable} antialiased`} suppressHydrationWarning>
      <body className="min-h-screen flex flex-col font-sans transition-colors duration-200">
        <ThemeProvider>{children}</ThemeProvider>
      </body>
    </html>
  );
}

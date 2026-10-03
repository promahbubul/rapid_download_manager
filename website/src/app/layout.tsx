import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";
import { ThemeProvider } from "@/context/ThemeContext";

const inter = Inter({
  subsets: ["latin"],
  variable: "--font-inter",
});

export const metadata: Metadata = {
  metadataBase: new URL("https://promahbubul.github.io/rapid_download_manager"),
  title: "Rapid Download Manager — High-Speed Download Accelerator",
  description:
    "Turbocharged 16-socket multi-segment download accelerator engineered in memory-safe Rust with automated browser integration for Windows.",
  icons: {
    icon: "/assets/app_icon.png",
    apple: "/assets/app_icon.png",
  },
  keywords: [
    "download manager",
    "download accelerator",
    "windows download manager",
    "rust download manager",
    "idm alternative",
    "rapid download manager",
    "open source download manager",
  ],
  authors: [{ name: "Mahbubul Alam (promahbubul)" }],
  openGraph: {
    title: "Rapid Download Manager — High-Speed Download Accelerator",
    description: "16x chunk acceleration, zero telemetry, memory-safe Rust for Windows.",
    images: ["/assets/master_logo.png"],
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

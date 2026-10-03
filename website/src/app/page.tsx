import React from "react";
import Navbar from "@/components/Navbar";
import Hero from "@/components/Hero";
import Features from "@/components/Features";
import VisualShowcase3D from "@/components/VisualShowcase3D";
import Comparison from "@/components/Comparison";
import CrossPlatformDownload from "@/components/CrossPlatformDownload";
import Footer from "@/components/Footer";

export default function Home() {
  return (
    <div className="min-h-screen flex flex-col bg-grid-pattern">
      <Navbar />
      <main className="flex-1">
        <Hero />
        <Features />
        <VisualShowcase3D />
        <Comparison />
        <CrossPlatformDownload />
      </main>
      <Footer />
    </div>
  );
}

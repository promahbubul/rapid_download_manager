import React from "react";
import Navbar from "@/components/Navbar";
import Hero from "@/components/Hero";
import VisualShowcase3D from "@/components/VisualShowcase3D";
import Features from "@/components/Features";
import Comparison from "@/components/Comparison";
import CrossPlatformDownload from "@/components/CrossPlatformDownload";
import QuickStart from "@/components/QuickStart";
import Footer from "@/components/Footer";
import Background3DCanvas from "@/components/Background3DCanvas";

export default function Home() {
  return (
    <div className="min-h-screen flex flex-col relative">
      {/* Interactive 3D Three.js Particle Background */}
      <Background3DCanvas />

      <Navbar />
      <main className="flex-1">
        <Hero />
        <VisualShowcase3D />
        <Features />
        <Comparison />
        <CrossPlatformDownload />
        <QuickStart />
      </main>
      <Footer />
    </div>
  );
}

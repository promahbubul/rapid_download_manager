import React from "react";
import Navbar from "@/components/Navbar";
import Hero from "@/components/Hero";
import Features from "@/components/Features";
import CrossPlatformDownload from "@/components/CrossPlatformDownload";
import Footer from "@/components/Footer";

export default function Home() {
  return (
    <div className="min-h-screen flex flex-col bg-grid-pattern">
      <Navbar />
      <main className="flex-1">
        <Hero />
        <Features />
        <CrossPlatformDownload />
      </main>
      <Footer />
    </div>
  );
}

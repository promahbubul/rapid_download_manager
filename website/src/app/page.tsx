import React from "react";
import Navbar from "@/components/Navbar";
import Hero from "@/components/Hero";
import Footer from "@/components/Footer";

export default function Home() {
  return (
    <div className="min-h-screen flex flex-col bg-grid-pattern">
      <Navbar />
      <main className="flex-1 flex flex-col justify-center">
        <Hero />
      </main>
      <Footer />
    </div>
  );
}

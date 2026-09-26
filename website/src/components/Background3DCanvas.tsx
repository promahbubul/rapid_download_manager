"use client";

import React, { useEffect, useRef } from "react";
import * as THREE from "three";
import { useTheme } from "../context/ThemeContext";

export default function Background3DCanvas() {
  const containerRef = useRef<HTMLDivElement>(null);
  const { resolvedTheme } = useTheme();
  const themeRef = useRef(resolvedTheme);

  useEffect(() => {
    themeRef.current = resolvedTheme;
  }, [resolvedTheme]);

  useEffect(() => {
    const container = containerRef.current;
    if (!container) return;

    // Scene, Camera, Renderer
    const scene = new THREE.Scene();
    const camera = new THREE.PerspectiveCamera(
      60,
      window.innerWidth / window.innerHeight,
      0.1,
      1000
    );
    camera.position.z = 80;

    let renderer: THREE.WebGLRenderer | null = null;
    try {
      renderer = new THREE.WebGLRenderer({
        alpha: true,
        antialias: true,
        powerPreference: "high-performance",
      });
      renderer.setSize(window.innerWidth, window.innerHeight);
      renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
      container.appendChild(renderer.domElement);
    } catch (e) {
      console.warn("WebGL not supported for 3D background:", e);
      return;
    }

    // 3D Particles
    const particleCount = 260;
    const geometry = new THREE.BufferGeometry();
    const positions = new Float32Array(particleCount * 3);
    const colors = new Float32Array(particleCount * 3);
    const velocities: { x: number; y: number; z: number }[] = [];

    const isDark = themeRef.current === "dark";
    const primaryColor = new THREE.Color(isDark ? 0x6366f1 : 0x4f46e5); // Indigo
    const accentColor = new THREE.Color(isDark ? 0x06b6d4 : 0x0284c7);  // Cyan
    const secondaryColor = new THREE.Color(isDark ? 0xa855f7 : 0x7c3aed); // Purple

    for (let i = 0; i < particleCount; i++) {
      positions[i * 3] = (Math.random() - 0.5) * 220;
      positions[i * 3 + 1] = (Math.random() - 0.5) * 140;
      positions[i * 3 + 2] = (Math.random() - 0.5) * 160;

      const choice = Math.random();
      const c = choice < 0.45 ? primaryColor : choice < 0.75 ? accentColor : secondaryColor;
      colors[i * 3] = c.r;
      colors[i * 3 + 1] = c.g;
      colors[i * 3 + 2] = c.b;

      velocities.push({
        x: (Math.random() - 0.5) * 0.12,
        y: (Math.random() - 0.5) * 0.12,
        z: (Math.random() - 0.5) * 0.08,
      });
    }

    geometry.setAttribute("position", new THREE.BufferAttribute(positions, 3));
    geometry.setAttribute("color", new THREE.BufferAttribute(colors, 3));

    // Round particle sprite
    const canvas = document.createElement("canvas");
    canvas.width = 32;
    canvas.height = 32;
    const ctx = canvas.getContext("2d");
    if (ctx) {
      const gradient = ctx.createRadialGradient(16, 16, 0, 16, 16, 16);
      gradient.addColorStop(0, "rgba(255,255,255,1)");
      gradient.addColorStop(0.3, "rgba(255,255,255,0.85)");
      gradient.addColorStop(0.8, "rgba(255,255,255,0.2)");
      gradient.addColorStop(1, "rgba(255,255,255,0)");
      ctx.fillStyle = gradient;
      ctx.beginPath();
      ctx.arc(16, 16, 16, 0, Math.PI * 2);
      ctx.fill();
    }
    const texture = new THREE.CanvasTexture(canvas);

    const pointsMaterial = new THREE.PointsMaterial({
      size: 3.2,
      map: texture,
      vertexColors: true,
      transparent: true,
      opacity: isDark ? 0.75 : 0.45,
      blending: THREE.AdditiveBlending,
      depthWrite: false,
    });

    const particles = new THREE.Points(geometry, pointsMaterial);
    scene.add(particles);

    // Floating 3D Geometric Cyber Rings
    const ringGeo = new THREE.TorusGeometry(32, 0.4, 16, 100);
    const ringMat = new THREE.MeshBasicMaterial({
      color: isDark ? 0x6366f1 : 0x818cf8,
      wireframe: true,
      transparent: true,
      opacity: isDark ? 0.18 : 0.08,
    });
    const torus = new THREE.Mesh(ringGeo, ringMat);
    torus.position.set(45, -15, -20);
    torus.rotation.x = Math.PI / 3;
    scene.add(torus);

    const ringGeo2 = new THREE.TorusGeometry(22, 0.3, 16, 80);
    const ringMat2 = new THREE.MeshBasicMaterial({
      color: isDark ? 0x06b6d4 : 0x38bdf8,
      wireframe: true,
      transparent: true,
      opacity: isDark ? 0.22 : 0.1,
    });
    const torus2 = new THREE.Mesh(ringGeo2, ringMat2);
    torus2.position.set(-50, 20, -35);
    torus2.rotation.y = Math.PI / 4;
    scene.add(torus2);

    // Mouse movement parallax
    let mouseX = 0;
    let mouseY = 0;
    let targetX = 0;
    let targetY = 0;

    const handleMouseMove = (e: MouseEvent) => {
      const windowHalfX = window.innerWidth / 2;
      const windowHalfY = window.innerHeight / 2;
      mouseX = (e.clientX - windowHalfX) * 0.04;
      mouseY = (e.clientY - windowHalfY) * 0.04;
    };

    window.addEventListener("mousemove", handleMouseMove, { passive: true });

    const handleResize = () => {
      if (!renderer) return;
      camera.aspect = window.innerWidth / window.innerHeight;
      camera.updateProjectionMatrix();
      renderer.setSize(window.innerWidth, window.innerHeight);
    };
    window.addEventListener("resize", handleResize);

    let animationId: number;
    const animate = () => {
      animationId = requestAnimationFrame(animate);

      // Camera lerping
      targetX += (mouseX - targetX) * 0.04;
      targetY += (mouseY - targetY) * 0.04;
      camera.position.x = targetX;
      camera.position.y = -targetY;
      camera.lookAt(scene.position);

      // Rings spin
      torus.rotation.z += 0.002;
      torus.rotation.y += 0.0015;
      torus2.rotation.x += 0.003;
      torus2.rotation.z -= 0.002;

      // Particle drift
      const posAttr = geometry.attributes.position as THREE.BufferAttribute;
      const posArray = posAttr.array as Float32Array;

      for (let i = 0; i < particleCount; i++) {
        posArray[i * 3] += velocities[i].x;
        posArray[i * 3 + 1] += velocities[i].y;
        posArray[i * 3 + 2] += velocities[i].z;

        if (Math.abs(posArray[i * 3]) > 110) velocities[i].x *= -1;
        if (Math.abs(posArray[i * 3 + 1]) > 70) velocities[i].y *= -1;
        if (Math.abs(posArray[i * 3 + 2]) > 80) velocities[i].z *= -1;
      }
      posAttr.needsUpdate = true;

      // React to theme changes
      const currentDark = themeRef.current === "dark";
      pointsMaterial.opacity = currentDark ? 0.75 : 0.45;
      ringMat.opacity = currentDark ? 0.18 : 0.08;
      ringMat2.opacity = currentDark ? 0.22 : 0.1;

      if (renderer) {
        renderer.render(scene, camera);
      }
    };

    animate();

    return () => {
      window.removeEventListener("mousemove", handleMouseMove);
      window.removeEventListener("resize", handleResize);
      cancelAnimationFrame(animationId);
      if (renderer && renderer.domElement && container.contains(renderer.domElement)) {
        container.removeChild(renderer.domElement);
      }
      geometry.dispose();
      pointsMaterial.dispose();
      ringGeo.dispose();
      ringMat.dispose();
      ringGeo2.dispose();
      ringMat2.dispose();
      texture.dispose();
      if (renderer) renderer.dispose();
    };
  }, []);

  return (
    <div
      ref={containerRef}
      aria-hidden="true"
      className="fixed inset-0 pointer-events-none -z-10 overflow-hidden transition-opacity duration-500"
    />
  );
}

use eframe::egui::Color32;

// =========================================================================
// GLASSMORPHISM MASTER PALETTE (Futuristic • Elegant • Premium)
// =========================================================================
pub const GLASS_PRIMARY: Color32 = Color32::from_rgb(139, 92, 246);        // Electric Violet (#8B5CF6)
pub const GLASS_PRIMARY_HOVER: Color32 = Color32::from_rgb(167, 139, 250);  // Light Violet Glow (#A78BFA)
pub const GLASS_SECONDARY: Color32 = Color32::from_rgb(6, 182, 212);       // Neon Cyan / Electric Aqua (#06B6D4)
pub const GLASS_SECONDARY_HOVER: Color32 = Color32::from_rgb(34, 211, 238); // Light Cyan Glow (#22D3EE)
pub const GLASS_BG: Color32 = Color32::from_rgb(2, 6, 23);                 // Deep Midnight Slate-950 (#020617)
pub const GLASS_SURFACE: Color32 = Color32::from_rgb(11, 18, 36);          // Translucent Dark Glass Surface
pub const GLASS_CARD: Color32 = Color32::from_rgb(14, 23, 46);             // Frosted Glass Card Fill
pub const GLASS_BORDER: Color32 = Color32::from_rgb(32, 45, 74);           // Glass Rim Border
pub const GLASS_TEXT: Color32 = Color32::from_rgb(249, 250, 251);          // Crisp Bright White (#F9FAFB)
pub const GLASS_MUTED: Color32 = Color32::from_rgb(156, 163, 175);         // Cool Slate Muted (#9CA3AF)
pub const _GLASS_ACCENT_BG: Color32 = Color32::from_rgb(28, 20, 58);       // Violet Aura Pill Fill
pub const _GLASS_ACCENT_BORDER: Color32 = Color32::from_rgb(124, 58, 237); // Glowing Violet Rim

// Backward-compatible aliases for seamless rendering
pub const PINK_NEON: Color32 = GLASS_PRIMARY;                              // Electric Violet (#8B5CF6)
pub const PINK_ROSE: Color32 = GLASS_SECONDARY;                            // Neon Cyan (#06B6D4)
pub const PINK_PASTEL: Color32 = GLASS_TEXT;                               // Crisp White (#F9FAFB)
pub const PINK_MUTED: Color32 = GLASS_MUTED;                               // Cool Slate Muted (#9CA3AF)
pub const _PINK_ACCENT_BG: Color32 = _GLASS_ACCENT_BG;                     // Violet Aura Pill Fill
pub const _PINK_ACCENT_BORDER: Color32 = _GLASS_ACCENT_BORDER;             // Glowing Violet Rim

pub const VELVET_BLACK: Color32 = GLASS_BG;                                // Deep Midnight (#020617)
pub const VELVET_SURFACE: Color32 = GLASS_SURFACE;                         // Glass Surface (#0B1224)
pub const _VELVET_CARD: Color32 = GLASS_CARD;                              // Frosted Card (#0E172E)
pub const VELVET_BORDER: Color32 = GLASS_BORDER;                           // Glass Rim Border (#202D4A)

pub const STATUS_DOWNLOADING: Color32 = GLASS_SECONDARY;                   // Neon Cyan (#06B6D4)
pub const STATUS_COMPLETED: Color32 = Color32::from_rgb(16, 185, 129);     // Emerald Mint (#10B981)
pub const STATUS_PAUSED: Color32 = Color32::from_rgb(245, 158, 11);        // Warm Amber (#F59E0B)
pub const STATUS_FAILED: Color32 = Color32::from_rgb(244, 63, 94);         // Crimson Rose (#F43F5E)

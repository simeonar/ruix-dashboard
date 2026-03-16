// Color palette and threshold logic for the dark dashboard theme.

// ── Background & Surface ─────────────────────────────────────────────────────
pub const BG: &str = "#0F172A";
pub const SURFACE: &str = "#1E293B";
pub const SURFACE_ALT: &str = "#334155";

// ── Accent ───────────────────────────────────────────────────────────────────
pub const PRIMARY: &str = "#3B82F6";

// ── Severity ─────────────────────────────────────────────────────────────────
pub const SUCCESS: &str = "#22C55E";
pub const WARNING: &str = "#F59E0B";
pub const DANGER: &str = "#EF4444";

// ── Text ─────────────────────────────────────────────────────────────────────
pub const TEXT_PRIMARY: &str = "#F8FAFC";
pub const TEXT_SECONDARY: &str = "#94A3B8";
pub const TEXT_TERTIARY: &str = "#64748B";

// ── Borders ──────────────────────────────────────────────────────────────────
pub const BORDER: &str = "#475569";

// ── Layout constants ─────────────────────────────────────────────────────────
pub const W: f32 = 1280.0;
pub const H: f32 = 800.0;
pub const HEADER_H: f32 = 48.0;
pub const STATUS_H: f32 = 24.0;
pub const SIDEBAR_W: f32 = 200.0;
pub const CARD_PAD: f32 = 16.0;
pub const CARD_GAP: f32 = 16.0;
pub const CARD_H: f32 = 120.0;
pub const SECTION_GAP: f32 = 16.0;
pub const BAR_H: f32 = 6.0;

// ── Content area derived constants ───────────────────────────────────────────
pub const CONTENT_X: f32 = SIDEBAR_W;
pub const CONTENT_Y: f32 = HEADER_H;
pub const CONTENT_W: f32 = W - SIDEBAR_W;
pub const CONTENT_H: f32 = H - HEADER_H - STATUS_H;

// ── Threshold → color ────────────────────────────────────────────────────────

/// CPU load color: 0–50% green, 50–80% yellow, 80%+ red.
#[must_use]
pub fn cpu_color(percent: f32) -> &'static str {
    if percent >= 80.0 {
        DANGER
    } else if percent >= 50.0 {
        WARNING
    } else {
        SUCCESS
    }
}

/// Memory load color: 0–60% green, 60–85% yellow, 85%+ red.
#[must_use]
pub fn memory_color(percent: f32) -> &'static str {
    if percent >= 85.0 {
        DANGER
    } else if percent >= 60.0 {
        WARNING
    } else {
        SUCCESS
    }
}

/// Disk usage color: 0–70% green, 70–90% yellow, 90%+ red.
#[must_use]
pub fn disk_color(percent: f32) -> &'static str {
    if percent >= 90.0 {
        DANGER
    } else if percent >= 70.0 {
        WARNING
    } else {
        SUCCESS
    }
}

/// Generic load color (same thresholds as CPU).
#[must_use]
pub fn load_color(percent: f32) -> &'static str {
    cpu_color(percent)
}

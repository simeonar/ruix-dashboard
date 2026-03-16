/// Deterministic NodeId scheme for stable delta updates.
use ui_core::types::NodeId;

/// Shorthand constructor.
#[must_use]
pub fn n(v: u64) -> NodeId {
    NodeId::new(v)
}

// ── Well-known structural IDs ────────────────────────────────────────────────
pub const ROOT: u64 = 1;
pub const HEADER: u64 = 2;
pub const BODY: u64 = 3;
pub const STATUS_BAR: u64 = 4;

// ── Sidebar (10–19) ─────────────────────────────────────────────────────────
pub const SIDEBAR: u64 = 10;
/// Nav item background: 11, 12, 13, 14, 15, 16
pub const NAV_ITEM_BASE: u64 = 11;
/// Nav item text labels: 17, 18, 19 (reuses 17–19 range, but we'll offset by 60)
pub const NAV_TEXT_BASE: u64 = 60;

// ── Header children (20–29) ─────────────────────────────────────────────────
pub const HEADER_TITLE: u64 = 20;
pub const HEADER_CLOCK: u64 = 21;
pub const HEADER_HOSTNAME: u64 = 22;
pub const HEADER_LOGO: u64 = 23;

// ── Status bar children (40–49) ─────────────────────────────────────────────
pub const STATUS_REFRESH: u64 = 40;
pub const STATUS_LAST_UPDATE: u64 = 41;
pub const STATUS_CONNECTED: u64 = 42;

// ── Content area (50) ───────────────────────────────────────────────────────
pub const CONTENT: u64 = 50;

// ── Overview page — metric cards (100–199) ──────────────────────────────────
/// Each card occupies a block of 10 IDs: card_base + 0..9
pub const CARD_BASE: u64 = 100;

/// Get the base ID for card at given index (0-based).
#[must_use]
pub fn card_base(index: u64) -> u64 {
    CARD_BASE + index * 10
}

// Sub-offsets within each card block
pub const CARD_BG: u64 = 0;
pub const CARD_ACCENT_BAR: u64 = 1;
pub const CARD_STATUS_DOT: u64 = 2;
pub const CARD_LABEL: u64 = 3;
pub const CARD_VALUE: u64 = 4;
pub const CARD_BAR_TRACK: u64 = 5;
pub const CARD_BAR_FILL: u64 = 6;
pub const CARD_FOOTER_LEFT: u64 = 7;
pub const CARD_FOOTER_RIGHT: u64 = 8;

// ── Overview page — per-core grid (300–399) ─────────────────────────────────
pub const CORE_SECTION: u64 = 300;
pub const CORE_TITLE: u64 = 301;
/// Each core occupies 3 IDs: label, track, fill
pub const CORE_ITEM_BASE: u64 = 310;

#[must_use]
pub fn core_label(core_idx: u64) -> u64 {
    CORE_ITEM_BASE + core_idx * 3
}

#[must_use]
pub fn core_track(core_idx: u64) -> u64 {
    CORE_ITEM_BASE + core_idx * 3 + 1
}

#[must_use]
pub fn core_fill(core_idx: u64) -> u64 {
    CORE_ITEM_BASE + core_idx * 3 + 2
}

// ── Overview page — top processes (400–499) ─────────────────────────────────
pub const PROC_SECTION: u64 = 400;
pub const PROC_TITLE: u64 = 401;
pub const PROC_HEADER_ROW: u64 = 402;
/// Each process row occupies 4 IDs: name, cpu, mem, separator
pub const PROC_ROW_BASE: u64 = 410;

#[must_use]
pub fn proc_name(row: u64) -> u64 {
    PROC_ROW_BASE + row * 4
}

#[must_use]
pub fn proc_cpu(row: u64) -> u64 {
    PROC_ROW_BASE + row * 4 + 1
}

#[must_use]
pub fn proc_mem(row: u64) -> u64 {
    PROC_ROW_BASE + row * 4 + 2
}

// ── Sparklines (9000–9199) ───────────────────────────────────────────────────
pub const SPARK_CPU_SECTION: u64 = 9000;
pub const SPARK_CPU_BASE: u64 = 9010; // 60 bars: 9010–9069
pub const SPARK_MEM_SECTION: u64 = 9100;
pub const SPARK_MEM_BASE: u64 = 9110; // 60 bars: 9110–9169

// ── Overview page — system info (500–599) ───────────────────────────────────
pub const SYSINFO_SECTION: u64 = 500;
pub const SYSINFO_TITLE: u64 = 501;
pub const SYSINFO_LINE1: u64 = 502;
pub const SYSINFO_LINE2: u64 = 503;

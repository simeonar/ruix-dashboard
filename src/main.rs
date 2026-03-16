mod card;
mod data;
mod history;
mod ids;
mod layout_helpers;
mod nav;
mod pages;
mod shell;
mod sparkline;
mod theme;
mod updater;

use std::time::{Duration, Instant};

use ui_core::core::UiCoreImpl;
use ui_core::prelude::*;
use ui_core::render::softbuffer::softbuffer_backend;

use crate::data::{DataSources, SystemMetrics};
use crate::history::MetricsHistory;
use crate::nav::NavState;
use crate::pages::Page;

fn main() {
    // ── Initialize data sources ──────────────────────────────────────────
    let mut sources = DataSources::new();
    let mut history = MetricsHistory::new();
    let metrics = SystemMetrics::collect(&sources);
    history.push(&metrics);

    // ── Build initial tree ───────────────────────────────────────────────
    let snapshot = updater::build_snapshot(&metrics, &history);

    let config = UiCoreConfig {
        target_fps: Some(60),
        enable_inspector: false,
        max_frame_time_ms: 16,
        debug_layout: false,
        ..Default::default()
    };

    let mut core = UiCoreImpl::new(config, softbuffer_backend());
    core.apply_snapshot(snapshot).expect("apply_snapshot");

    // ── Navigation state ─────────────────────────────────────────────────
    let nav_state = NavState::new();

    // ── Register sidebar click handlers ──────────────────────────────────
    for i in 0..Page::COUNT as u64 {
        let state = nav_state.clone();
        core.event_dispatcher_mut().registry.register_bubble(
            ids::n(ids::NAV_ITEM_BASE + i),
            move |event| {
                if matches!(event, InputEvent::PointerDown { .. }) {
                    state.navigate(Page::from_index(i as u8));
                }
                EventResult::Propagate
            },
        );
        // Also handle clicks on the text labels (children of sidebar, not nav bg)
        let state2 = nav_state.clone();
        core.event_dispatcher_mut().registry.register_bubble(
            ids::n(ids::NAV_TEXT_BASE + i),
            move |event| {
                if matches!(event, InputEvent::PointerDown { .. }) {
                    state2.navigate(Page::from_index(i as u8));
                }
                EventResult::Propagate
            },
        );
    }

    // ── Headless CI mode ─────────────────────────────────────────────────
    if std::env::var("HEADLESS").is_ok() {
        core.tick(Some(100)).expect("headless tick");
        return;
    }

    // ── Periodic data refresh via frame hook ─────────────────────────────
    let refresh_interval = Duration::from_secs(1);
    let mut last_refresh = Instant::now();
    let hook_nav = nav_state.clone();

    // Keep a cached snapshot to avoid re-collecting on nav-only changes.
    let mut cached_metrics = SystemMetrics::collect(&sources);

    core.set_frame_hook(Box::new(move |tree: &mut NodeTree| {
        let nav_dirty = hook_nav.is_dirty();
        let data_stale = last_refresh.elapsed() >= refresh_interval;

        if !nav_dirty && !data_stale {
            return;
        }

        if data_stale {
            sources.refresh();
            cached_metrics = SystemMetrics::collect(&sources);
            history.push(&cached_metrics);
            last_refresh = Instant::now();
        }

        updater::update_in_place(tree, &cached_metrics, &history, &hook_nav);
    }));

    let _win = core.register_window(
        WindowDescriptor::builder("RUIX System Monitor")
            .size(theme::W, theme::H)
            .icon(generate_app_icon())
            .build(),
    );

    core.run().expect("event loop error");
}

/// Generate a simple 32x32 RGBA app icon programmatically.
/// Blue (#3B82F6) rounded square with a white "R" silhouette.
fn generate_app_icon() -> WindowIcon {
    const S: u32 = 32;
    let mut rgba = vec![0u8; (S * S * 4) as usize];
    let bg = [0x3Bu8, 0x82, 0xF6, 0xFF]; // PRIMARY blue
    let fg = [0xF8u8, 0xFA, 0xFC, 0xFF]; // TEXT_PRIMARY white

    for y in 0..S {
        for x in 0..S {
            let i = ((y * S + x) * 4) as usize;
            // Rounded corners: skip pixels in 3px corner radius
            let in_corner = (x < 3 && y < 3 && (x + y) < 3)
                || (x >= S - 3 && y < 3 && (S - 1 - x + y) < 3)
                || (x < 3 && y >= S - 3 && (x + S - 1 - y) < 3)
                || (x >= S - 3 && y >= S - 3 && (S - 1 - x + S - 1 - y) < 3);
            if in_corner {
                rgba[i..i + 4].copy_from_slice(&[0, 0, 0, 0]);
                continue;
            }
            // "R" letter: simple pixel pattern in center region
            let is_r = (x >= 8 && x <= 10 && y >= 7 && y <= 24)      // vertical stroke
                || (x >= 11 && x <= 20 && y >= 7 && y <= 9)           // top horizontal
                || (x >= 11 && x <= 20 && y >= 14 && y <= 16)         // middle horizontal
                || (x >= 21 && x <= 23 && y >= 10 && y <= 13)         // top-right vertical
                || (x >= 14 && x <= 16 && y >= 17 && y <= 18)         // leg start
                || (x >= 17 && x <= 19 && y >= 19 && y <= 21)         // leg middle
                || (x >= 20 && x <= 23 && y >= 22 && y <= 24);        // leg end

            let color = if is_r { &fg } else { &bg };
            rgba[i..i + 4].copy_from_slice(color);
        }
    }
    WindowIcon { rgba, width: S, height: S }
}

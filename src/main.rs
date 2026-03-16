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

    core.set_frame_hook(Box::new(move |tree: &mut NodeTree| {
        let nav_dirty = hook_nav.is_dirty();
        let data_stale = last_refresh.elapsed() >= refresh_interval;

        if !nav_dirty && !data_stale {
            return;
        }

        if data_stale {
            sources.refresh();
            last_refresh = Instant::now();
        }

        let metrics = SystemMetrics::collect(&sources);
        history.push(&metrics);
        updater::update_in_place(tree, &metrics, &history, &hook_nav);
    }));

    let _win = core.register_window(
        WindowDescriptor::builder("RUIX System Monitor")
            .size(theme::W, theme::H)
            .build(),
    );

    core.run().expect("event loop error");
}

mod card;
mod data;
mod history;
mod ids;
mod layout_helpers;
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

    // ── Headless CI mode ─────────────────────────────────────────────────
    if std::env::var("HEADLESS").is_ok() {
        core.tick(Some(100)).expect("headless tick");
        return;
    }

    // ── Periodic data refresh via frame hook ─────────────────────────────
    let refresh_interval = Duration::from_secs(1);
    let mut last_refresh = Instant::now();

    core.set_frame_hook(Box::new(move |tree: &mut NodeTree| {
        if last_refresh.elapsed() < refresh_interval {
            return;
        }
        sources.refresh();
        let metrics = SystemMetrics::collect(&sources);
        history.push(&metrics);
        updater::update_in_place(tree, &metrics, &history);
        last_refresh = Instant::now();
    }));

    let _win = core.register_window(
        WindowDescriptor::builder("RUIX System Monitor")
            .size(theme::W, theme::H)
            .build(),
    );

    core.run().expect("event loop error");
}

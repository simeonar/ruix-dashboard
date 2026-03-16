// Tree rebuilder + in-place updater for live metrics.
use std::collections::HashMap;

use ui_core::inspector::AssetsMeta;
use ui_core::prelude::*;

use crate::data::{self, SystemMetrics};
use crate::history::MetricsHistory;
use crate::ids::{self, n};
use crate::layout_helpers::lb;
use crate::pages;
use crate::shell;
use crate::sparkline;
use crate::theme;

/// Build a complete dashboard snapshot from current metrics.
#[must_use]
pub fn build_snapshot(metrics: &SystemMetrics, history: &MetricsHistory) -> Snapshot {
    let mut tree = NodeTree::new(n(ids::ROOT));
    shell::build_shell(&mut tree, metrics);
    pages::overview::build_overview(&mut tree, shell::content_id(), metrics, history);
    Snapshot {
        version: 1,
        tree,
        view_states: HashMap::new(),
        assets_meta: AssetsMeta::default(),
    }
}

/// Update existing tree in-place with new metrics (called from frame_hook).
/// Only modifies prop values and bar widths — no structural changes.
pub fn update_in_place(tree: &mut NodeTree, metrics: &SystemMetrics, history: &MetricsHistory) {
    // ── Header ───────────────────────────────────────────────────────────
    set_text(tree, ids::HEADER_HOSTNAME, &metrics.hostname);
    set_text(
        tree,
        ids::HEADER_CLOCK,
        &metrics.timestamp.format("%H:%M:%S").to_string(),
    );

    // ── Status bar ───────────────────────────────────────────────────────
    set_text(
        tree,
        ids::STATUS_LAST_UPDATE,
        &format!("Last update: {}", metrics.timestamp.format("%H:%M:%S")),
    );

    // ── CPU card ─────────────────────────────────────────────────────────
    update_card(
        tree,
        0,
        &format!("{:.0}%", metrics.cpu_total_percent),
        metrics.cpu_total_percent,
        theme::cpu_color,
    );

    // ── Memory card ──────────────────────────────────────────────────────
    update_card(
        tree,
        1,
        &data::format_bytes(metrics.mem_used_bytes),
        metrics.mem_percent(),
        theme::memory_color,
    );

    // ── Disk card ────────────────────────────────────────────────────────
    update_card(
        tree,
        2,
        &format!("{:.0}%", metrics.disk_percent()),
        metrics.disk_percent(),
        theme::disk_color,
    );

    // ── Network card ─────────────────────────────────────────────────────
    let net_total = history.net_rx_rate + history.net_tx_rate;
    let net_pct = (net_total as f32 / 1_048_576.0 * 10.0).min(100.0);
    update_card(
        tree,
        3,
        &data::format_throughput(net_total),
        net_pct,
        theme::load_color,
    );

    // ── Per-core grid ────────────────────────────────────────────────────
    for (i, &pct) in metrics.cpu_per_core.iter().enumerate() {
        if i >= 16 {
            break;
        }
        let idx = i as u64;
        set_text(tree, ids::core_label(idx), &format!("Core {i}: {pct:.0}%"));

        // Read track layout first, then mutate fill node
        let track_layout = tree.get(n(ids::core_track(idx))).and_then(|t| t.layout);
        if let Some(node) = tree.get_mut(n(ids::core_fill(idx))) {
            let color = theme::cpu_color(pct);
            node.props.insert("fill_color".into(), color.into());
            if let Some(tl) = track_layout {
                let fill_w = tl.size.width * (pct / 100.0).clamp(0.0, 1.0);
                node.layout = Some(lb(tl.origin.x, tl.origin.y, fill_w, theme::BAR_H));
            }
        }
    }

    // ── Top processes ────────────────────────────────────────────────────
    for (i, proc) in metrics.top_cpu_processes.iter().take(5).enumerate() {
        let row = i as u64;
        set_text(tree, ids::proc_name(row), &proc.name);
        set_text(
            tree,
            ids::proc_cpu(row),
            &format!("{:.1}%", proc.cpu_percent),
        );
        set_text(
            tree,
            ids::proc_mem(row),
            &data::format_bytes(proc.mem_bytes),
        );
    }

    // ── System info ──────────────────────────────────────────────────────
    set_text(
        tree,
        ids::SYSINFO_LINE2,
        &format!(
            "Uptime: {}  |  Processes: {}",
            data::format_uptime(metrics.uptime_seconds),
            metrics.process_count
        ),
    );

    // ── Sparklines ──────────────────────────────────────────────────────
    let cx = theme::CONTENT_X;
    let cy = theme::CONTENT_Y;
    let cw = theme::CONTENT_W;
    let spark_y = cy + theme::CARD_PAD + theme::CARD_H + theme::SECTION_GAP;
    let spark_h = 50.0;
    let spark_half_w = (cw - theme::CARD_PAD * 2.0 - theme::CARD_GAP) / 2.0;

    sparkline::update_sparkline(
        tree,
        ids::SPARK_CPU_SECTION,
        ids::SPARK_CPU_BASE,
        &history.cpu_total,
        cx + theme::CARD_PAD,
        spark_y,
        spark_half_w,
        spark_h,
        theme::PRIMARY,
    );
    sparkline::update_sparkline(
        tree,
        ids::SPARK_MEM_SECTION,
        ids::SPARK_MEM_BASE,
        &history.mem_percent,
        cx + theme::CARD_PAD + spark_half_w + theme::CARD_GAP,
        spark_y,
        spark_half_w,
        spark_h,
        theme::SUCCESS,
    );
}

// ── Helpers ──────────────────────────────────────────────────────────────────

fn set_text(tree: &mut NodeTree, id: u64, text: &str) {
    if let Some(node) = tree.get_mut(n(id)) {
        node.props.insert("text".into(), text.into());
    }
}

fn update_card(
    tree: &mut NodeTree,
    card_index: u64,
    value_text: &str,
    percent: f32,
    color_fn: fn(f32) -> &'static str,
) {
    let base = ids::card_base(card_index);
    let color = color_fn(percent);

    // Value text
    set_text(tree, base + ids::CARD_VALUE, value_text);

    // Status dot color
    if let Some(node) = tree.get_mut(n(base + ids::CARD_STATUS_DOT)) {
        node.props.insert("fill_color".into(), color.into());
    }

    // Bar fill width + color
    if let Some(track_layout) = tree
        .get(n(base + ids::CARD_BAR_TRACK))
        .and_then(|t| t.layout)
    {
        let fill_w = track_layout.size.width * (percent / 100.0).clamp(0.0, 1.0);
        if let Some(node) = tree.get_mut(n(base + ids::CARD_BAR_FILL)) {
            node.props.insert("fill_color".into(), color.into());
            node.layout = Some(lb(
                track_layout.origin.x,
                track_layout.origin.y,
                fill_w,
                theme::BAR_H,
            ));
        }
    }
}

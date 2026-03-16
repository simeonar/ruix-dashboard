// Tree rebuilder + in-place updater for live metrics.
use std::collections::HashMap;

use ui_core::inspector::AssetsMeta;
use ui_core::prelude::*;

use crate::data::{self, SystemMetrics};
use crate::history::MetricsHistory;
use crate::ids::{self, n};
use crate::layout_helpers::lb;
use crate::nav::NavState;
use crate::pages::{self, Page};
use crate::shell;
use crate::sparkline;
use crate::theme;

/// Build a complete dashboard snapshot from current metrics.
/// Only the Overview page is built initially; other pages are built on demand.
#[must_use]
pub fn build_snapshot(metrics: &SystemMetrics, history: &MetricsHistory) -> Snapshot {
    let mut tree = NodeTree::new(n(ids::ROOT));
    shell::build_shell(&mut tree, metrics);
    build_page_content(&mut tree, Page::Overview, metrics, history);
    Snapshot {
        version: 1,
        tree,
        view_states: HashMap::new(),
        assets_meta: AssetsMeta::default(),
    }
}

/// Build the content for `page` as children of the CONTENT node.
fn build_page_content(
    tree: &mut NodeTree,
    page: Page,
    metrics: &SystemMetrics,
    history: &MetricsHistory,
) {
    let content_id = shell::content_id();
    match page {
        Page::Overview => pages::overview::build_overview(tree, content_id, metrics, history),
        Page::Cpu => pages::cpu::build_cpu_page(tree, content_id, metrics),
        Page::Memory => pages::memory::build_memory_page(tree, content_id, metrics),
        Page::Processes => pages::processes::build_processes_page(tree, content_id, metrics),
        Page::Network => pages::network::build_network_page(tree, content_id, history),
        Page::Disks => pages::disks::build_disks_page(tree, content_id, metrics),
    }
}

/// Remove all children of CONTENT, then build the new page.
fn switch_page(tree: &mut NodeTree, page: Page, metrics: &SystemMetrics, history: &MetricsHistory) {
    let content_id = shell::content_id();
    let children: Vec<_> = tree.children(content_id).to_vec();
    for child_id in children {
        let _ = tree.remove(child_id);
    }
    build_page_content(tree, page, metrics, history);

    // Update sidebar highlight
    for i in 0..Page::COUNT as u64 {
        let is_active = i == page as u64;
        let bg_color = if is_active {
            theme::PRIMARY
        } else {
            theme::SURFACE_ALT
        };
        let text_color = if is_active {
            theme::TEXT_PRIMARY
        } else {
            theme::TEXT_SECONDARY
        };
        if let Some(bg) = tree.get_mut(n(ids::NAV_ITEM_BASE + i)) {
            bg.props.insert("fill_color".into(), bg_color.into());
        }
        if let Some(txt) = tree.get_mut(n(ids::NAV_TEXT_BASE + i)) {
            txt.props.insert("color".into(), text_color.into());
        }
    }
}

/// Update existing tree in-place with new metrics (called from frame_hook).
///
/// If a page switch occurred (nav dirty flag), destroys old page subtree
/// and builds the new one. Otherwise, updates active page props in-place.
pub fn update_in_place(
    tree: &mut NodeTree,
    metrics: &SystemMetrics,
    history: &MetricsHistory,
    nav: &NavState,
) {
    let page = nav.current();
    let nav_changed = nav.take_dirty();

    // ── Page switch: remove old content, build new ────────────────────────
    if nav_changed {
        switch_page(tree, page, metrics, history);
    }

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

    // ── Update only the active page ──────────────────────────────────────
    match page {
        Page::Overview => update_overview(tree, metrics, history),
        Page::Cpu => update_cpu(tree, metrics),
        Page::Memory => update_memory(tree, metrics),
        Page::Processes => update_processes(tree, metrics),
        Page::Network => update_network(tree, history),
        Page::Disks => update_disks(tree, metrics),
    }
}

// ── Overview update ──────────────────────────────────────────────────────────

fn update_overview(tree: &mut NodeTree, metrics: &SystemMetrics, history: &MetricsHistory) {
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
        ids::SPARK_MEM_BASE,
        &history.mem_percent,
        cx + theme::CARD_PAD + spark_half_w + theme::CARD_GAP,
        spark_y,
        spark_half_w,
        spark_h,
        theme::SUCCESS,
    );
}

// ── CPU page update ──────────────────────────────────────────────────────────

fn update_cpu(tree: &mut NodeTree, metrics: &SystemMetrics) {
    set_text(
        tree,
        ids::CPU_TOTAL_VALUE,
        &format!("{:.1}%", metrics.cpu_total_percent),
    );

    // Total bar
    let color = theme::cpu_color(metrics.cpu_total_percent);
    let track_layout = tree.get(n(ids::CPU_TOTAL_BAR_TRACK)).and_then(|t| t.layout);
    if let Some(node) = tree.get_mut(n(ids::CPU_TOTAL_BAR_FILL)) {
        node.props.insert("fill_color".into(), color.into());
        if let Some(tl) = track_layout {
            let fill_w = tl.size.width * (metrics.cpu_total_percent / 100.0).clamp(0.0, 1.0);
            node.layout = Some(lb(tl.origin.x, tl.origin.y, fill_w, 8.0));
        }
    }

    // Frequency
    set_text(
        tree,
        ids::CPU_FREQ_TEXT,
        &format!(
            "{} cores  |  {} MHz",
            metrics.cpu_core_count, metrics.cpu_frequency_mhz
        ),
    );

    // Per-core
    for (i, &pct) in metrics.cpu_per_core.iter().enumerate() {
        if i >= 16 {
            break;
        }
        let idx = i as u64;
        set_text(
            tree,
            ids::cpu_detail_label(idx),
            &format!("Core {i}: {pct:.0}%"),
        );

        let core_color = theme::cpu_color(pct);
        let track_l = tree
            .get(n(ids::cpu_detail_track(idx)))
            .and_then(|t| t.layout);
        if let Some(node) = tree.get_mut(n(ids::cpu_detail_fill(idx))) {
            node.props.insert("fill_color".into(), core_color.into());
            if let Some(tl) = track_l {
                let fill_w = tl.size.width * (pct / 100.0).clamp(0.0, 1.0);
                node.layout = Some(lb(tl.origin.x, tl.origin.y, fill_w, theme::BAR_H));
            }
        }
    }
}

// ── Memory page update ───────────────────────────────────────────────────────

fn update_memory(tree: &mut NodeTree, metrics: &SystemMetrics) {
    let mem_pct = metrics.mem_percent();
    let mem_color = theme::memory_color(mem_pct);

    set_text(
        tree,
        ids::MEM_PHYSICAL_VALUE,
        &format!(
            "{} / {}",
            data::format_bytes(metrics.mem_used_bytes),
            data::format_bytes(metrics.mem_total_bytes)
        ),
    );

    // Physical bar
    update_bar(
        tree,
        ids::MEM_PHYSICAL_BAR_TRACK,
        ids::MEM_PHYSICAL_BAR_FILL,
        mem_pct,
        mem_color,
    );

    // Swap
    let swap_pct = if metrics.swap_total_bytes > 0 {
        (metrics.swap_used_bytes as f64 / metrics.swap_total_bytes as f64 * 100.0) as f32
    } else {
        0.0
    };
    let swap_color = theme::memory_color(swap_pct);

    set_text(
        tree,
        ids::MEM_SWAP_VALUE,
        &format!(
            "{} / {}",
            data::format_bytes(metrics.swap_used_bytes),
            data::format_bytes(metrics.swap_total_bytes)
        ),
    );

    update_bar(
        tree,
        ids::MEM_SWAP_BAR_TRACK,
        ids::MEM_SWAP_BAR_FILL,
        swap_pct,
        swap_color,
    );

    // Details
    let avail = metrics
        .mem_total_bytes
        .saturating_sub(metrics.mem_used_bytes);
    set_text(
        tree,
        ids::MEM_DETAILS_TEXT,
        &format!(
            "Available: {}  |  Used: {:.1}%",
            data::format_bytes(avail),
            mem_pct
        ),
    );

    // Top mem processes
    for (i, proc) in metrics.top_mem_processes.iter().take(10).enumerate() {
        let row = i as u64;
        set_text(tree, ids::mem_proc_name(row), &proc.name);
        set_text(
            tree,
            ids::mem_proc_mem(row),
            &data::format_bytes(proc.mem_bytes),
        );
        let pct_str = if metrics.mem_total_bytes > 0 {
            format!(
                "{:.1}%",
                proc.mem_bytes as f64 / metrics.mem_total_bytes as f64 * 100.0
            )
        } else {
            "0.0%".into()
        };
        set_text(tree, ids::mem_proc_pct(row), &pct_str);
    }
}

// ── Processes page update ────────────────────────────────────────────────────

fn update_processes(tree: &mut NodeTree, metrics: &SystemMetrics) {
    for (i, proc) in metrics.top_cpu_processes.iter().take(20).enumerate() {
        let row = i as u64;
        set_text(tree, ids::proc_page_pid(row), &format!("{}", proc.pid));
        set_text(tree, ids::proc_page_name(row), &proc.name);
        set_text(
            tree,
            ids::proc_page_cpu(row),
            &format!("{:.1}%", proc.cpu_percent),
        );
        set_text(
            tree,
            ids::proc_page_mem(row),
            &data::format_bytes(proc.mem_bytes),
        );
    }
    // Clear leftover rows if fewer processes than before
    for i in metrics.top_cpu_processes.len()..20 {
        let row = i as u64;
        set_text(tree, ids::proc_page_pid(row), "");
        set_text(tree, ids::proc_page_name(row), "-");
        set_text(tree, ids::proc_page_cpu(row), "");
        set_text(tree, ids::proc_page_mem(row), "");
    }
}

// ── Network page update ──────────────────────────────────────────────────────

fn update_network(tree: &mut NodeTree, history: &MetricsHistory) {
    set_text(
        tree,
        ids::NET_RX_VALUE,
        &data::format_throughput(history.net_rx_rate),
    );
    set_text(
        tree,
        ids::NET_TX_VALUE,
        &data::format_throughput(history.net_tx_rate),
    );
    let total = history.net_rx_rate + history.net_tx_rate;
    set_text(tree, ids::NET_TOTAL_VALUE, &data::format_throughput(total));
}

// ── Disks page update ────────────────────────────────────────────────────────

fn update_disks(tree: &mut NodeTree, metrics: &SystemMetrics) {
    for (i, disk) in metrics.disks.iter().take(8).enumerate() {
        let idx = i as u64;
        let d_pct = if disk.total_bytes > 0 {
            (disk.used_bytes as f64 / disk.total_bytes as f64 * 100.0) as f32
        } else {
            0.0
        };
        let name = if disk.name.is_empty() {
            &disk.mount_point
        } else {
            &disk.name
        };
        let color = theme::disk_color(d_pct);

        set_text(tree, ids::disk_name(idx), name);
        set_text(
            tree,
            ids::disk_usage_text(idx),
            &format!(
                "{} / {} ({:.1}%)",
                data::format_bytes(disk.used_bytes),
                data::format_bytes(disk.total_bytes),
                d_pct
            ),
        );
        set_text(tree, ids::disk_mount_text(idx), &disk.mount_point);

        update_bar(
            tree,
            ids::disk_bar_track(idx),
            ids::disk_bar_fill(idx),
            d_pct,
            color,
        );
    }
    // Clear unused disk slots
    for i in metrics.disks.len()..8 {
        let idx = i as u64;
        set_text(tree, ids::disk_name(idx), "-");
        set_text(tree, ids::disk_usage_text(idx), "");
        set_text(tree, ids::disk_mount_text(idx), "");
    }
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

fn update_bar(tree: &mut NodeTree, track_id: u64, fill_id: u64, pct: f32, color: &str) {
    let track_layout = tree.get(n(track_id)).and_then(|t| t.layout);
    if let Some(node) = tree.get_mut(n(fill_id)) {
        node.props.insert("fill_color".into(), color.into());
        if let Some(tl) = track_layout {
            let fill_w = tl.size.width * (pct / 100.0).clamp(0.0, 1.0);
            node.layout = Some(lb(tl.origin.x, tl.origin.y, fill_w, 8.0));
        }
    }
}

// Overview page: metric cards, per-core grid, top processes, system info.
use ui_core::prelude::*;
use ui_core::types::NodeId;

use crate::card::{self, CardData};
use crate::data::{self, SystemMetrics};
use crate::history::MetricsHistory;
use crate::ids::{self, n};
use crate::layout_helpers::{self, lb, GridArea};
use crate::sparkline;
use crate::theme;

/// Build the entire overview page inside the content container.
pub fn build_overview(
    tree: &mut NodeTree,
    content_id: NodeId,
    metrics: &SystemMetrics,
    history: &MetricsHistory,
) {
    let cx = theme::CONTENT_X;
    let cy = theme::CONTENT_Y;
    let cw = theme::CONTENT_W;

    build_metric_cards(tree, content_id, cx, cy, cw, metrics, history);

    // ── Sparklines (below cards) ─────────────────────────────────────────
    let spark_y = cy + theme::CARD_PAD + theme::CARD_H + theme::SECTION_GAP;
    let spark_h = 50.0;
    let spark_half_w = (cw - theme::CARD_PAD * 2.0 - theme::CARD_GAP) / 2.0;
    sparkline::build_sparkline(
        tree,
        content_id,
        ids::SPARK_CPU_SECTION,
        ids::SPARK_CPU_BASE,
        &history.cpu_total,
        cx + theme::CARD_PAD,
        spark_y,
        spark_half_w,
        spark_h,
        theme::PRIMARY,
    );
    sparkline::build_sparkline(
        tree,
        content_id,
        ids::SPARK_MEM_SECTION,
        ids::SPARK_MEM_BASE,
        &history.mem_percent,
        cx + theme::CARD_PAD + spark_half_w + theme::CARD_GAP,
        spark_y,
        spark_half_w,
        spark_h,
        theme::SUCCESS,
    );

    let row2_y = spark_y + spark_h + theme::SECTION_GAP;
    let half_w = (cw - theme::CARD_PAD * 2.0 - theme::CARD_GAP) / 2.0;
    build_core_grid(
        tree,
        content_id,
        cx + theme::CARD_PAD,
        row2_y,
        half_w,
        metrics,
    );

    let proc_x = cx + theme::CARD_PAD + half_w + theme::CARD_GAP;
    build_top_processes(tree, content_id, proc_x, row2_y, half_w, metrics);

    let row3_y = row2_y + 240.0 + theme::SECTION_GAP;
    build_system_info(
        tree,
        content_id,
        cx + theme::CARD_PAD,
        row3_y,
        cw - theme::CARD_PAD * 2.0,
        metrics,
    );
}

// ── Metric Cards ─────────────────────────────────────────────────────────────

#[allow(clippy::too_many_arguments)]
fn build_metric_cards(
    tree: &mut NodeTree,
    content_id: NodeId,
    cx: f32,
    cy: f32,
    cw: f32,
    metrics: &SystemMetrics,
    history: &MetricsHistory,
) {
    let grid = GridArea {
        x: cx,
        y: cy,
        w: cw,
        columns: 4,
        pad: theme::CARD_PAD,
        gap: theme::CARD_GAP,
        card_h: theme::CARD_H,
    };

    let net_total = history.net_rx_rate + history.net_tx_rate;
    let net_str = data::format_throughput(net_total);

    let cards = [
        CardData {
            label: "CPU",
            value: format!("{:.0}%", metrics.cpu_total_percent),
            percent: metrics.cpu_total_percent,
            footer_left: format!("{} cores", metrics.cpu_core_count),
            footer_right: format!("{} MHz", metrics.cpu_frequency_mhz),
            color_fn: theme::cpu_color,
        },
        CardData {
            label: "Memory",
            value: data::format_bytes(metrics.mem_used_bytes),
            percent: metrics.mem_percent(),
            footer_left: format!("{} total", data::format_bytes(metrics.mem_total_bytes)),
            footer_right: format!("Swap: {}", data::format_bytes(metrics.swap_used_bytes)),
            color_fn: theme::memory_color,
        },
        CardData {
            label: "Disk",
            value: format!("{:.0}%", metrics.disk_percent()),
            percent: metrics.disk_percent(),
            footer_left: format!(
                "{} / {}",
                data::format_bytes(metrics.disk_used_bytes),
                data::format_bytes(metrics.disk_total_bytes)
            ),
            footer_right: String::new(),
            color_fn: theme::disk_color,
        },
        CardData {
            label: "Network",
            value: net_str,
            percent: (net_total as f32 / 1_048_576.0 * 10.0).min(100.0), // scale for bar
            footer_left: format!("RX: {}", data::format_throughput(history.net_rx_rate)),
            footer_right: format!("TX: {}", data::format_throughput(history.net_tx_rate)),
            color_fn: theme::load_color,
        },
    ];

    for (i, d) in cards.iter().enumerate() {
        let (x, y, w, h) = layout_helpers::card_rect(i, &grid);
        card::build_card(tree, content_id, i as u64, d, x, y, w, h);
    }
}

// ── Per-Core CPU Grid ────────────────────────────────────────────────────────

fn build_core_grid(
    tree: &mut NodeTree,
    content_id: NodeId,
    x: f32,
    y: f32,
    w: f32,
    metrics: &SystemMetrics,
) {
    let section_h = 240.0;

    let mut section = Node::new(n(ids::CORE_SECTION), "Rectangle");
    section
        .props
        .insert("fill_color".into(), theme::SURFACE.into());
    section.layout = Some(lb(x, y, w, section_h));
    tree.insert(content_id, section)
        .expect("insert core section");

    let mut title = Node::new(n(ids::CORE_TITLE), "Text");
    title.props.insert("text".into(), "CPU Per-Core".into());
    title
        .props
        .insert("color".into(), theme::TEXT_PRIMARY.into());
    title.props.insert("fontSize".into(), 13.0.into());
    title.layout = Some(lb(x + 12.0, y + 10.0, w - 24.0, 16.0));
    tree.insert(n(ids::CORE_SECTION), title)
        .expect("insert core title");

    let cols = 2;
    let row_h = 24.0;
    let col_w = (w - 24.0) / cols as f32;
    let bar_w = col_w - 80.0;
    let start_y = y + 34.0;

    for (i, &pct) in metrics.cpu_per_core.iter().enumerate() {
        if i >= 16 {
            break; // cap at 16 cores for layout
        }
        let col = i % cols;
        let row = i / cols;
        let ix = x + 12.0 + col as f32 * col_w;
        let iy = start_y + row as f32 * row_h;
        let color = theme::cpu_color(pct);

        let mut label = Node::new(n(ids::core_label(i as u64)), "Text");
        label
            .props
            .insert("text".into(), format!("Core {i}: {pct:.0}%").into());
        label
            .props
            .insert("color".into(), theme::TEXT_SECONDARY.into());
        label.props.insert("fontSize".into(), 11.0.into());
        label.layout = Some(lb(ix, iy + 2.0, 70.0, 14.0));
        tree.insert(n(ids::CORE_SECTION), label)
            .expect("insert core label");

        let mut track = Node::new(n(ids::core_track(i as u64)), "Rectangle");
        track
            .props
            .insert("fill_color".into(), theme::SURFACE_ALT.into());
        track.layout = Some(lb(ix + 72.0, iy + 4.0, bar_w, theme::BAR_H));
        tree.insert(n(ids::CORE_SECTION), track)
            .expect("insert core track");

        let fill_w = bar_w * (pct / 100.0).clamp(0.0, 1.0);
        let mut fill = Node::new(n(ids::core_fill(i as u64)), "Rectangle");
        fill.props.insert("fill_color".into(), color.into());
        fill.layout = Some(lb(ix + 72.0, iy + 4.0, fill_w, theme::BAR_H));
        tree.insert(n(ids::CORE_SECTION), fill)
            .expect("insert core fill");
    }
}

// ── Top Processes ────────────────────────────────────────────────────────────

fn build_top_processes(
    tree: &mut NodeTree,
    content_id: NodeId,
    x: f32,
    y: f32,
    w: f32,
    metrics: &SystemMetrics,
) {
    let section_h = 240.0;

    let mut section = Node::new(n(ids::PROC_SECTION), "Rectangle");
    section
        .props
        .insert("fill_color".into(), theme::SURFACE.into());
    section.layout = Some(lb(x, y, w, section_h));
    tree.insert(content_id, section)
        .expect("insert proc section");

    let section_id = n(ids::PROC_SECTION);

    let mut title = Node::new(n(ids::PROC_TITLE), "Text");
    title.props.insert("text".into(), "Top Processes".into());
    title
        .props
        .insert("color".into(), theme::TEXT_PRIMARY.into());
    title.props.insert("fontSize".into(), 13.0.into());
    title.layout = Some(lb(x + 12.0, y + 10.0, w - 24.0, 16.0));
    tree.insert(section_id, title).expect("insert proc title");

    let hdr_y = y + 34.0;
    let mut hdr = Node::new(n(ids::PROC_HEADER_ROW), "Text");
    hdr.props
        .insert("text".into(), "Name                  CPU%   Memory".into());
    hdr.props
        .insert("color".into(), theme::TEXT_TERTIARY.into());
    hdr.props.insert("fontSize".into(), 11.0.into());
    hdr.layout = Some(lb(x + 12.0, hdr_y, w - 24.0, 14.0));
    tree.insert(section_id, hdr).expect("insert proc header");

    let row_h = 22.0;
    let start_y = hdr_y + 20.0;
    let name_w = w * 0.45;
    let cpu_w = w * 0.2;

    for (i, proc) in metrics.top_cpu_processes.iter().take(5).enumerate() {
        let ry = start_y + i as f32 * row_h;
        let row = i as u64;

        let mut name_node = Node::new(n(ids::proc_name(row)), "Text");
        name_node
            .props
            .insert("text".into(), proc.name.as_str().into());
        name_node
            .props
            .insert("color".into(), theme::TEXT_SECONDARY.into());
        name_node.props.insert("fontSize".into(), 11.0.into());
        name_node.layout = Some(lb(x + 12.0, ry, name_w, 14.0));
        tree.insert(section_id, name_node)
            .expect("insert proc name");

        let mut cpu_node = Node::new(n(ids::proc_cpu(row)), "Text");
        cpu_node
            .props
            .insert("text".into(), format!("{:.1}%", proc.cpu_percent).into());
        cpu_node
            .props
            .insert("color".into(), theme::TEXT_SECONDARY.into());
        cpu_node.props.insert("fontSize".into(), 11.0.into());
        cpu_node.layout = Some(lb(x + 12.0 + name_w, ry, cpu_w, 14.0));
        tree.insert(section_id, cpu_node).expect("insert proc cpu");

        let mut mem_node = Node::new(n(ids::proc_mem(row)), "Text");
        mem_node
            .props
            .insert("text".into(), data::format_bytes(proc.mem_bytes).into());
        mem_node
            .props
            .insert("color".into(), theme::TEXT_SECONDARY.into());
        mem_node.props.insert("fontSize".into(), 11.0.into());
        mem_node.layout = Some(lb(
            x + 12.0 + name_w + cpu_w,
            ry,
            w - name_w - cpu_w - 24.0,
            14.0,
        ));
        tree.insert(section_id, mem_node).expect("insert proc mem");
    }
}

// ── System Info ──────────────────────────────────────────────────────────────

fn build_system_info(
    tree: &mut NodeTree,
    content_id: NodeId,
    x: f32,
    y: f32,
    w: f32,
    metrics: &SystemMetrics,
) {
    let section_h = 70.0;

    let mut section = Node::new(n(ids::SYSINFO_SECTION), "Rectangle");
    section
        .props
        .insert("fill_color".into(), theme::SURFACE.into());
    section.layout = Some(lb(x, y, w, section_h));
    tree.insert(content_id, section)
        .expect("insert sysinfo section");

    let section_id = n(ids::SYSINFO_SECTION);

    let mut title = Node::new(n(ids::SYSINFO_TITLE), "Text");
    title
        .props
        .insert("text".into(), "System Information".into());
    title
        .props
        .insert("color".into(), theme::TEXT_PRIMARY.into());
    title.props.insert("fontSize".into(), 13.0.into());
    title.layout = Some(lb(x + 12.0, y + 10.0, w - 24.0, 16.0));
    tree.insert(section_id, title)
        .expect("insert sysinfo title");

    let line1_text = format!(
        "OS: {}  |  CPU: {}  |  RAM: {}",
        metrics.os_version,
        metrics.cpu_model,
        data::format_bytes(metrics.mem_total_bytes)
    );
    let mut line1 = Node::new(n(ids::SYSINFO_LINE1), "Text");
    line1.props.insert("text".into(), line1_text.into());
    line1
        .props
        .insert("color".into(), theme::TEXT_SECONDARY.into());
    line1.props.insert("fontSize".into(), 11.0.into());
    line1.layout = Some(lb(x + 12.0, y + 30.0, w - 24.0, 14.0));
    tree.insert(section_id, line1)
        .expect("insert sysinfo line1");

    let line2_text = format!(
        "Uptime: {}  |  Processes: {}",
        data::format_uptime(metrics.uptime_seconds),
        metrics.process_count
    );
    let mut line2 = Node::new(n(ids::SYSINFO_LINE2), "Text");
    line2.props.insert("text".into(), line2_text.into());
    line2
        .props
        .insert("color".into(), theme::TEXT_SECONDARY.into());
    line2.props.insert("fontSize".into(), 11.0.into());
    line2.layout = Some(lb(x + 12.0, y + 48.0, w - 24.0, 14.0));
    tree.insert(section_id, line2)
        .expect("insert sysinfo line2");
}

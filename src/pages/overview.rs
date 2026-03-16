// Overview page: metric cards, per-core grid, top processes, system info.
use ui_core::prelude::*;
use ui_core::types::NodeId;

use crate::card::{self, CardData};
use crate::ids::{self, n};
use crate::layout_helpers::{self, GridArea, lb};
use crate::theme;

/// Build the entire overview page inside the content container.
pub fn build_overview(tree: &mut NodeTree, content_id: NodeId) {
    let cx = theme::CONTENT_X;
    let cy = theme::CONTENT_Y;
    let cw = theme::CONTENT_W;

    // ── Metric cards (row 1) ─────────────────────────────────────────────
    build_metric_cards(tree, content_id, cx, cy, cw);

    // ── Per-core CPU grid (row 2, left half) ─────────────────────────────
    let row2_y = cy + theme::CARD_PAD + theme::CARD_H + theme::SECTION_GAP;
    let half_w = (cw - theme::CARD_PAD * 2.0 - theme::CARD_GAP) / 2.0;
    build_core_grid(tree, content_id, cx + theme::CARD_PAD, row2_y, half_w);

    // ── Top processes (row 2, right half) ────────────────────────────────
    let proc_x = cx + theme::CARD_PAD + half_w + theme::CARD_GAP;
    build_top_processes(tree, content_id, proc_x, row2_y, half_w);

    // ── System info (row 3) ──────────────────────────────────────────────
    let row3_y = row2_y + 240.0 + theme::SECTION_GAP;
    build_system_info(tree, content_id, cx + theme::CARD_PAD, row3_y, cw - theme::CARD_PAD * 2.0);

    // Update content's children list
    let content = tree.get_mut(content_id).expect("content exists");
    let mut children = Vec::new();
    // Cards
    for i in 0..4u64 {
        children.push(n(ids::card_base(i) + ids::CARD_BG));
    }
    // Core section
    children.push(n(ids::CORE_SECTION));
    // Process section
    children.push(n(ids::PROC_SECTION));
    // System info section
    children.push(n(ids::SYSINFO_SECTION));
    content.children = children;
}

// ── Metric Cards ─────────────────────────────────────────────────────────────

fn build_metric_cards(tree: &mut NodeTree, content_id: NodeId, cx: f32, cy: f32, cw: f32) {
    let grid = GridArea {
        x: cx,
        y: cy,
        w: cw,
        columns: 4,
        pad: theme::CARD_PAD,
        gap: theme::CARD_GAP,
        card_h: theme::CARD_H,
    };

    let cards = [
        CardData {
            label: "CPU",
            value: "34%".into(),
            percent: 34.0,
            footer_left: "8 cores / 16 threads".into(),
            footer_right: "Peak: 87%".into(),
            color_fn: theme::cpu_color,
        },
        CardData {
            label: "Memory",
            value: "8.2 GB".into(),
            percent: 64.0,
            footer_left: "12.8 GB total".into(),
            footer_right: "Swap: 1.2 GB".into(),
            color_fn: theme::memory_color,
        },
        CardData {
            label: "Disk",
            value: "45%".into(),
            percent: 45.0,
            footer_left: "238 GB / 512 GB".into(),
            footer_right: "R: 12 MB/s".into(),
            color_fn: theme::disk_color,
        },
        CardData {
            label: "Network",
            value: "1.2 MB/s".into(),
            percent: 12.0,
            footer_left: "Ethernet".into(),
            footer_right: "TX: 0.3 MB/s".into(),
            color_fn: theme::load_color,
        },
    ];

    for (i, data) in cards.iter().enumerate() {
        let (x, y, w, h) = layout_helpers::card_rect(i, &grid);
        card::build_card(tree, content_id, i as u64, data, x, y, w, h);
    }
}

// ── Per-Core CPU Grid ────────────────────────────────────────────────────────

fn build_core_grid(tree: &mut NodeTree, content_id: NodeId, x: f32, y: f32, w: f32) {
    let section_h = 240.0;

    // Section background
    let mut section = Node::new(n(ids::CORE_SECTION), "Rectangle");
    section.props.insert("fill_color".into(), theme::SURFACE.into());
    section.layout = Some(lb(x, y, w, section_h));

    // Build children list
    let mock_cores: &[f32] = &[45.0, 78.0, 23.0, 52.0, 67.0, 31.0, 89.0, 15.0];
    let mut children = vec![n(ids::CORE_TITLE)];
    for i in 0..mock_cores.len() as u64 {
        children.push(n(ids::core_label(i)));
        children.push(n(ids::core_track(i)));
        children.push(n(ids::core_fill(i)));
    }
    section.children = children;
    tree.insert(content_id, section).expect("insert core section");

    // Title
    let mut title = Node::new(n(ids::CORE_TITLE), "Text");
    title.props.insert("text".into(), "CPU Per-Core".into());
    title.props.insert("fontSize".into(), 13.0.into());
    title.layout = Some(lb(x + 12.0, y + 10.0, w - 24.0, 16.0));
    tree.insert(n(ids::CORE_SECTION), title).expect("insert core title");

    // Core bars — 2 columns
    let cols = 2;
    let row_h = 24.0;
    let col_w = (w - 24.0) / cols as f32;
    let bar_w = col_w - 80.0;
    let start_y = y + 34.0;

    for (i, &pct) in mock_cores.iter().enumerate() {
        let col = i % cols;
        let row = i / cols;
        let ix = x + 12.0 + col as f32 * col_w;
        let iy = start_y + row as f32 * row_h;
        let color = theme::cpu_color(pct);

        // Label: "Core N:"
        let mut label = Node::new(n(ids::core_label(i as u64)), "Text");
        label.props.insert("text".into(), format!("Core {i}: {pct:.0}%").into());
        label.props.insert("fontSize".into(), 11.0.into());
        label.layout = Some(lb(ix, iy + 2.0, 70.0, 14.0));
        tree.insert(n(ids::CORE_SECTION), label).expect("insert core label");

        // Bar track
        let mut track = Node::new(n(ids::core_track(i as u64)), "Rectangle");
        track.props.insert("fill_color".into(), theme::SURFACE_ALT.into());
        track.layout = Some(lb(ix + 72.0, iy + 4.0, bar_w, theme::BAR_H));
        tree.insert(n(ids::CORE_SECTION), track).expect("insert core track");

        // Bar fill
        let fill_w = bar_w * (pct / 100.0);
        let mut fill = Node::new(n(ids::core_fill(i as u64)), "Rectangle");
        fill.props.insert("fill_color".into(), color.into());
        fill.layout = Some(lb(ix + 72.0, iy + 4.0, fill_w, theme::BAR_H));
        tree.insert(n(ids::CORE_SECTION), fill).expect("insert core fill");
    }
}

// ── Top Processes ────────────────────────────────────────────────────────────

fn build_top_processes(tree: &mut NodeTree, content_id: NodeId, x: f32, y: f32, w: f32) {
    let section_h = 240.0;

    let mut section = Node::new(n(ids::PROC_SECTION), "Rectangle");
    section.props.insert("fill_color".into(), theme::SURFACE.into());
    section.layout = Some(lb(x, y, w, section_h));

    // Build children
    let mut children = vec![n(ids::PROC_TITLE), n(ids::PROC_HEADER_ROW)];
    for i in 0..5u64 {
        children.push(n(ids::proc_name(i)));
        children.push(n(ids::proc_cpu(i)));
        children.push(n(ids::proc_mem(i)));
    }
    section.children = children;
    tree.insert(content_id, section).expect("insert proc section");

    let section_id = n(ids::PROC_SECTION);

    // Title
    let mut title = Node::new(n(ids::PROC_TITLE), "Text");
    title.props.insert("text".into(), "Top Processes".into());
    title.props.insert("fontSize".into(), 13.0.into());
    title.layout = Some(lb(x + 12.0, y + 10.0, w - 24.0, 16.0));
    tree.insert(section_id, title).expect("insert proc title");

    // Header row
    let hdr_y = y + 34.0;
    let mut hdr = Node::new(n(ids::PROC_HEADER_ROW), "Text");
    hdr.props.insert("text".into(), "Name                  CPU%   Memory".into());
    hdr.props.insert("fontSize".into(), 11.0.into());
    hdr.layout = Some(lb(x + 12.0, hdr_y, w - 24.0, 14.0));
    tree.insert(section_id, hdr).expect("insert proc header");

    // Mock process rows
    let procs = [
        ("chrome.exe", "12.3%", "340 MB"),
        ("code.exe", "8.1%", "280 MB"),
        ("rust-analyzer", "6.4%", "190 MB"),
        ("explorer.exe", "2.1%", "80 MB"),
        ("svchost.exe", "1.8%", "65 MB"),
    ];

    let row_h = 22.0;
    let start_y = hdr_y + 20.0;
    let name_w = w * 0.45;
    let cpu_w = w * 0.2;

    for (i, (pname, cpu, mem)) in procs.iter().enumerate() {
        let ry = start_y + i as f32 * row_h;
        let row = i as u64;

        let mut name_node = Node::new(n(ids::proc_name(row)), "Text");
        name_node.props.insert("text".into(), (*pname).into());
        name_node.props.insert("fontSize".into(), 11.0.into());
        name_node.layout = Some(lb(x + 12.0, ry, name_w, 14.0));
        tree.insert(section_id, name_node).expect("insert proc name");

        let mut cpu_node = Node::new(n(ids::proc_cpu(row)), "Text");
        cpu_node.props.insert("text".into(), (*cpu).into());
        cpu_node.props.insert("fontSize".into(), 11.0.into());
        cpu_node.layout = Some(lb(x + 12.0 + name_w, ry, cpu_w, 14.0));
        tree.insert(section_id, cpu_node).expect("insert proc cpu");

        let mut mem_node = Node::new(n(ids::proc_mem(row)), "Text");
        mem_node.props.insert("text".into(), (*mem).into());
        mem_node.props.insert("fontSize".into(), 11.0.into());
        mem_node.layout = Some(lb(x + 12.0 + name_w + cpu_w, ry, w - name_w - cpu_w - 24.0, 14.0));
        tree.insert(section_id, mem_node).expect("insert proc mem");
    }
}

// ── System Info ──────────────────────────────────────────────────────────────

fn build_system_info(tree: &mut NodeTree, content_id: NodeId, x: f32, y: f32, w: f32) {
    let section_h = 70.0;

    let mut section = Node::new(n(ids::SYSINFO_SECTION), "Rectangle");
    section.props.insert("fill_color".into(), theme::SURFACE.into());
    section.layout = Some(lb(x, y, w, section_h));
    section.children = vec![
        n(ids::SYSINFO_TITLE),
        n(ids::SYSINFO_LINE1),
        n(ids::SYSINFO_LINE2),
    ];
    tree.insert(content_id, section).expect("insert sysinfo section");

    let section_id = n(ids::SYSINFO_SECTION);

    let mut title = Node::new(n(ids::SYSINFO_TITLE), "Text");
    title.props.insert("text".into(), "System Information".into());
    title.props.insert("fontSize".into(), 13.0.into());
    title.layout = Some(lb(x + 12.0, y + 10.0, w - 24.0, 16.0));
    tree.insert(section_id, title).expect("insert sysinfo title");

    let mut line1 = Node::new(n(ids::SYSINFO_LINE1), "Text");
    line1.props.insert("text".into(), "OS: Windows 11 23H2  |  CPU: AMD Ryzen 9 7950X  |  RAM: 12.8 GB".into());
    line1.props.insert("fontSize".into(), 11.0.into());
    line1.layout = Some(lb(x + 12.0, y + 30.0, w - 24.0, 14.0));
    tree.insert(section_id, line1).expect("insert sysinfo line1");

    let mut line2 = Node::new(n(ids::SYSINFO_LINE2), "Text");
    line2.props.insert("text".into(), "Uptime: 3d 14h 22m  |  Processes: 312  |  Threads: 4218".into());
    line2.props.insert("fontSize".into(), 11.0.into());
    line2.layout = Some(lb(x + 12.0, y + 48.0, w - 24.0, 14.0));
    tree.insert(section_id, line2).expect("insert sysinfo line2");
}

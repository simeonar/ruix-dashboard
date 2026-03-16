// Processes detail page — Phase 3.
use ui_core::prelude::*;
use ui_core::types::NodeId;

use crate::data::{self, SystemMetrics};
use crate::ids::{self, n};
use crate::layout_helpers::lb;
use crate::theme;

const MAX_PROC_ROWS: usize = 20;

/// Build the Processes detail page inside its container.
pub fn build_processes_page(tree: &mut NodeTree, parent_id: NodeId, metrics: &SystemMetrics) {
    let cx = theme::CONTENT_X;
    let cy = theme::CONTENT_Y;
    let cw = theme::CONTENT_W;

    // ── Section background ────────────────────────────────────────────────
    let mut section = Node::new(n(ids::PROC_PAGE_SECTION), "Rectangle");
    section
        .props
        .insert("fill_color".into(), theme::SURFACE.into());
    section.layout = Some(lb(cx + 16.0, cy + 16.0, cw - 32.0, 700.0));
    tree.insert(parent_id, section)
        .expect("insert proc page section");

    let section_id = n(ids::PROC_PAGE_SECTION);
    let sx = cx + 16.0;
    let sy = cy + 16.0;
    let sw = cw - 32.0;

    // ── Title ─────────────────────────────────────────────────────────────
    let mut title = Node::new(n(ids::PROC_PAGE_TITLE), "Text");
    title
        .props
        .insert("text".into(), "Processes (by CPU)".into());
    title
        .props
        .insert("color".into(), theme::TEXT_PRIMARY.into());
    title.props.insert("fontSize".into(), 16.0.into());
    title.layout = Some(lb(sx + 16.0, sy + 12.0, sw - 32.0, 20.0));
    tree.insert(section_id, title)
        .expect("insert proc page title");

    // ── Header row ────────────────────────────────────────────────────────
    let hdr_y = sy + 42.0;
    let mut hdr = Node::new(n(ids::PROC_PAGE_HDR), "Text");
    hdr.props.insert(
        "text".into(),
        "PID         Name                          CPU%       Memory".into(),
    );
    hdr.props
        .insert("color".into(), theme::TEXT_TERTIARY.into());
    hdr.props.insert("fontSize".into(), 11.0.into());
    hdr.layout = Some(lb(sx + 16.0, hdr_y, sw - 32.0, 14.0));
    tree.insert(section_id, hdr)
        .expect("insert proc page header");

    // ── Rows ──────────────────────────────────────────────────────────────
    let row_h = 22.0;
    let start_y = hdr_y + 22.0;
    let pid_w = 70.0;
    let name_w = sw * 0.4;
    let cpu_w = 80.0;

    for i in 0..MAX_PROC_ROWS {
        let ry = start_y + i as f32 * row_h;
        let row = i as u64;
        let proc = metrics.top_cpu_processes.get(i);

        let p_name = proc.map_or("-", |p| p.name.as_str());
        let p_cpu = proc.map_or_else(String::new, |p| format!("{:.1}%", p.cpu_percent));
        let p_mem = proc.map_or_else(String::new, |p| data::format_bytes(p.mem_bytes));
        let p_pid = proc.map_or_else(String::new, |p| format!("{}", p.pid));

        let mut pid_node = Node::new(n(ids::proc_page_pid(row)), "Text");
        pid_node.props.insert("text".into(), p_pid.into());
        pid_node
            .props
            .insert("color".into(), theme::TEXT_TERTIARY.into());
        pid_node.props.insert("fontSize".into(), 11.0.into());
        pid_node.layout = Some(lb(sx + 16.0, ry, pid_w, 14.0));
        tree.insert(section_id, pid_node)
            .expect("insert proc page pid");

        let mut name_node = Node::new(n(ids::proc_page_name(row)), "Text");
        name_node.props.insert("text".into(), p_name.into());
        name_node
            .props
            .insert("color".into(), theme::TEXT_SECONDARY.into());
        name_node.props.insert("fontSize".into(), 11.0.into());
        name_node.layout = Some(lb(sx + 16.0 + pid_w, ry, name_w, 14.0));
        tree.insert(section_id, name_node)
            .expect("insert proc page name");

        let mut cpu_node = Node::new(n(ids::proc_page_cpu(row)), "Text");
        cpu_node.props.insert("text".into(), p_cpu.into());
        cpu_node
            .props
            .insert("color".into(), theme::TEXT_SECONDARY.into());
        cpu_node.props.insert("fontSize".into(), 11.0.into());
        cpu_node.layout = Some(lb(sx + 16.0 + pid_w + name_w, ry, cpu_w, 14.0));
        tree.insert(section_id, cpu_node)
            .expect("insert proc page cpu");

        let mut mem_node = Node::new(n(ids::proc_page_mem(row)), "Text");
        mem_node.props.insert("text".into(), p_mem.into());
        mem_node
            .props
            .insert("color".into(), theme::TEXT_SECONDARY.into());
        mem_node.props.insert("fontSize".into(), 11.0.into());
        mem_node.layout = Some(lb(
            sx + 16.0 + pid_w + name_w + cpu_w,
            ry,
            sw - pid_w - name_w - cpu_w - 32.0,
            14.0,
        ));
        tree.insert(section_id, mem_node)
            .expect("insert proc page mem");
    }
}

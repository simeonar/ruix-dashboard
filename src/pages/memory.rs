// Memory detail page — Phase 3.
use ui_core::prelude::*;
use ui_core::types::NodeId;

use crate::data::{self, SystemMetrics};
use crate::ids::{self, n};
use crate::layout_helpers::lb;
use crate::theme;

const MAX_MEM_PROCS: usize = 10;

/// Build the Memory detail page inside its container.
pub fn build_memory_page(tree: &mut NodeTree, parent_id: NodeId, metrics: &SystemMetrics) {
    let cx = 0.0;
    let cy = 0.0;
    let cw = theme::CONTENT_W;

    // ── Section background ────────────────────────────────────────────────
    let mut section = Node::new(n(ids::MEM_SECTION), "Rectangle");
    section
        .props
        .insert("fill_color".into(), theme::SURFACE.into());
    section.layout = Some(lb(cx + 16.0, cy + 16.0, cw - 32.0, 700.0));
    tree.insert(parent_id, section)
        .expect("insert mem section");

    let section_id = n(ids::MEM_SECTION);
    let sx = cx + 16.0;
    let sy = cy + 16.0;
    let sw = cw - 32.0;

    // ── Title ─────────────────────────────────────────────────────────────
    let mut title = Node::new(n(ids::MEM_TITLE), "Text");
    title.props.insert("text".into(), "Memory Details".into());
    title
        .props
        .insert("color".into(), theme::TEXT_PRIMARY.into());
    title.props.insert("fontSize".into(), 16.0.into());
    title.layout = Some(lb(sx + 16.0, sy + 12.0, sw - 32.0, 20.0));
    tree.insert(section_id, title).expect("insert mem title");

    // ── Physical RAM ──────────────────────────────────────────────────────
    let ram_y = sy + 44.0;

    let mut phys_label = Node::new(n(ids::MEM_PHYSICAL_LABEL), "Text");
    phys_label
        .props
        .insert("text".into(), "Physical Memory".into());
    phys_label
        .props
        .insert("color".into(), theme::TEXT_SECONDARY.into());
    phys_label.props.insert("fontSize".into(), 13.0.into());
    phys_label.layout = Some(lb(sx + 16.0, ram_y, 200.0, 16.0));
    tree.insert(section_id, phys_label)
        .expect("insert mem phys label");

    let mut phys_val = Node::new(n(ids::MEM_PHYSICAL_VALUE), "Text");
    phys_val.props.insert(
        "text".into(),
        format!(
            "{} / {}",
            data::format_bytes(metrics.mem_used_bytes),
            data::format_bytes(metrics.mem_total_bytes)
        )
        .into(),
    );
    phys_val
        .props
        .insert("color".into(), theme::TEXT_PRIMARY.into());
    phys_val.props.insert("fontSize".into(), 14.0.into());
    phys_val.layout = Some(lb(sx + 16.0, ram_y + 20.0, sw - 32.0, 18.0));
    tree.insert(section_id, phys_val)
        .expect("insert mem phys value");

    let bar_y = ram_y + 44.0;
    let bar_w = sw - 32.0;
    let mem_pct = metrics.mem_percent();
    let mem_color = theme::memory_color(mem_pct);

    let mut pt = Node::new(n(ids::MEM_PHYSICAL_BAR_TRACK), "Rectangle");
    pt.props
        .insert("fill_color".into(), theme::SURFACE_ALT.into());
    pt.layout = Some(lb(sx + 16.0, bar_y, bar_w, 8.0));
    tree.insert(section_id, pt)
        .expect("insert mem phys bar track");

    let fill_w = bar_w * (mem_pct / 100.0).clamp(0.0, 1.0);
    let mut pf = Node::new(n(ids::MEM_PHYSICAL_BAR_FILL), "Rectangle");
    pf.props.insert("fill_color".into(), mem_color.into());
    pf.layout = Some(lb(sx + 16.0, bar_y, fill_w, 8.0));
    tree.insert(section_id, pf)
        .expect("insert mem phys bar fill");

    // ── Swap ──────────────────────────────────────────────────────────────
    let swap_y = bar_y + 24.0;

    let mut swap_label = Node::new(n(ids::MEM_SWAP_LABEL), "Text");
    swap_label.props.insert("text".into(), "Swap".into());
    swap_label
        .props
        .insert("color".into(), theme::TEXT_SECONDARY.into());
    swap_label.props.insert("fontSize".into(), 13.0.into());
    swap_label.layout = Some(lb(sx + 16.0, swap_y, 200.0, 16.0));
    tree.insert(section_id, swap_label)
        .expect("insert mem swap label");

    let swap_pct = if metrics.swap_total_bytes > 0 {
        (metrics.swap_used_bytes as f64 / metrics.swap_total_bytes as f64 * 100.0) as f32
    } else {
        0.0
    };
    let swap_color = theme::memory_color(swap_pct);

    let mut swap_val = Node::new(n(ids::MEM_SWAP_VALUE), "Text");
    swap_val.props.insert(
        "text".into(),
        format!(
            "{} / {}",
            data::format_bytes(metrics.swap_used_bytes),
            data::format_bytes(metrics.swap_total_bytes)
        )
        .into(),
    );
    swap_val
        .props
        .insert("color".into(), theme::TEXT_PRIMARY.into());
    swap_val.props.insert("fontSize".into(), 14.0.into());
    swap_val.layout = Some(lb(sx + 16.0, swap_y + 20.0, sw - 32.0, 18.0));
    tree.insert(section_id, swap_val)
        .expect("insert mem swap value");

    let swap_bar_y = swap_y + 44.0;

    let mut st = Node::new(n(ids::MEM_SWAP_BAR_TRACK), "Rectangle");
    st.props
        .insert("fill_color".into(), theme::SURFACE_ALT.into());
    st.layout = Some(lb(sx + 16.0, swap_bar_y, bar_w, 8.0));
    tree.insert(section_id, st)
        .expect("insert mem swap bar track");

    let sfill_w = bar_w * (swap_pct / 100.0).clamp(0.0, 1.0);
    let mut sf = Node::new(n(ids::MEM_SWAP_BAR_FILL), "Rectangle");
    sf.props.insert("fill_color".into(), swap_color.into());
    sf.layout = Some(lb(sx + 16.0, swap_bar_y, sfill_w, 8.0));
    tree.insert(section_id, sf)
        .expect("insert mem swap bar fill");

    // ── Details text ──────────────────────────────────────────────────────
    let detail_y = swap_bar_y + 20.0;
    let avail = metrics.mem_total_bytes.saturating_sub(metrics.mem_used_bytes);
    let mut detail = Node::new(n(ids::MEM_DETAILS_TEXT), "Text");
    detail.props.insert(
        "text".into(),
        format!(
            "Available: {}  |  Used: {:.1}%",
            data::format_bytes(avail),
            mem_pct
        )
        .into(),
    );
    detail
        .props
        .insert("color".into(), theme::TEXT_SECONDARY.into());
    detail.props.insert("fontSize".into(), 12.0.into());
    detail.layout = Some(lb(sx + 16.0, detail_y, sw - 32.0, 16.0));
    tree.insert(section_id, detail)
        .expect("insert mem details text");

    // ── Top memory processes ──────────────────────────────────────────────
    let procs_y = detail_y + 32.0;
    let mut ptitle = Node::new(n(ids::MEM_TOP_PROCS_TITLE), "Text");
    ptitle
        .props
        .insert("text".into(), "Top Memory Consumers".into());
    ptitle
        .props
        .insert("color".into(), theme::TEXT_PRIMARY.into());
    ptitle.props.insert("fontSize".into(), 13.0.into());
    ptitle.layout = Some(lb(sx + 16.0, procs_y, sw - 32.0, 16.0));
    tree.insert(section_id, ptitle)
        .expect("insert mem procs title");

    let row_h = 22.0;
    let start_y = procs_y + 24.0;
    let name_w = sw * 0.45;
    let mem_w = sw * 0.25;

    for i in 0..MAX_MEM_PROCS {
        let ry = start_y + i as f32 * row_h;
        let row = i as u64;
        let proc = metrics.top_mem_processes.get(i);

        let pname = proc.map_or("-", |p| p.name.as_str());
        let pmem = proc.map_or_else(String::new, |p| data::format_bytes(p.mem_bytes));
        let ppct = proc.map_or_else(String::new, |p| {
            if metrics.mem_total_bytes > 0 {
                format!(
                    "{:.1}%",
                    p.mem_bytes as f64 / metrics.mem_total_bytes as f64 * 100.0
                )
            } else {
                "0.0%".into()
            }
        });

        let mut name_node = Node::new(n(ids::mem_proc_name(row)), "Text");
        name_node.props.insert("text".into(), pname.into());
        name_node
            .props
            .insert("color".into(), theme::TEXT_SECONDARY.into());
        name_node.props.insert("fontSize".into(), 11.0.into());
        name_node.layout = Some(lb(sx + 16.0, ry, name_w, 14.0));
        tree.insert(section_id, name_node)
            .expect("insert mem proc name");

        let mut mem_node = Node::new(n(ids::mem_proc_mem(row)), "Text");
        mem_node.props.insert("text".into(), pmem.into());
        mem_node
            .props
            .insert("color".into(), theme::TEXT_SECONDARY.into());
        mem_node.props.insert("fontSize".into(), 11.0.into());
        mem_node.layout = Some(lb(sx + 16.0 + name_w, ry, mem_w, 14.0));
        tree.insert(section_id, mem_node)
            .expect("insert mem proc mem");

        let mut pct_node = Node::new(n(ids::mem_proc_pct(row)), "Text");
        pct_node.props.insert("text".into(), ppct.into());
        pct_node
            .props
            .insert("color".into(), theme::TEXT_SECONDARY.into());
        pct_node.props.insert("fontSize".into(), 11.0.into());
        pct_node.layout = Some(lb(sx + 16.0 + name_w + mem_w, ry, 80.0, 14.0));
        tree.insert(section_id, pct_node)
            .expect("insert mem proc pct");
    }
}

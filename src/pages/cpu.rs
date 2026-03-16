// CPU detail page — Phase 3.
use ui_core::prelude::*;
use ui_core::types::NodeId;

use crate::data::SystemMetrics;
use crate::ids::{self, n};
use crate::layout_helpers::lb;
use crate::theme;

const MAX_CORES: usize = 16;

/// Build the CPU detail page inside its container.
pub fn build_cpu_page(tree: &mut NodeTree, parent_id: NodeId, metrics: &SystemMetrics) {
    let cx = theme::CONTENT_X;
    let cy = theme::CONTENT_Y;
    let cw = theme::CONTENT_W;

    // ── Main section background ───────────────────────────────────────────
    let mut section = Node::new(n(ids::CPU_SECTION_MAIN), "Rectangle");
    section
        .props
        .insert("fill_color".into(), theme::SURFACE.into());
    section.layout = Some(lb(cx + 16.0, cy + 16.0, cw - 32.0, 700.0));
    tree.insert(parent_id, section).expect("insert cpu section");

    let section_id = n(ids::CPU_SECTION_MAIN);
    let sx = cx + 16.0;
    let sy = cy + 16.0;
    let sw = cw - 32.0;

    // ── Title ─────────────────────────────────────────────────────────────
    let mut title = Node::new(n(ids::CPU_TITLE), "Text");
    title.props.insert("text".into(), "CPU Details".into());
    title
        .props
        .insert("color".into(), theme::TEXT_PRIMARY.into());
    title.props.insert("fontSize".into(), 16.0.into());
    title.layout = Some(lb(sx + 16.0, sy + 12.0, sw - 32.0, 20.0));
    tree.insert(section_id, title).expect("insert cpu title");

    // ── Total CPU percentage (large text) ─────────────────────────────────
    let mut total_val = Node::new(n(ids::CPU_TOTAL_VALUE), "Text");
    total_val.props.insert(
        "text".into(),
        format!("{:.1}%", metrics.cpu_total_percent).into(),
    );
    total_val
        .props
        .insert("color".into(), theme::TEXT_PRIMARY.into());
    total_val.props.insert("fontSize".into(), 36.0.into());
    total_val.layout = Some(lb(sx + 16.0, sy + 40.0, 200.0, 40.0));
    tree.insert(section_id, total_val)
        .expect("insert cpu total value");

    // ── Full-width total bar ──────────────────────────────────────────────
    let bar_y = sy + 88.0;
    let bar_w = sw - 32.0;

    let mut track = Node::new(n(ids::CPU_TOTAL_BAR_TRACK), "Rectangle");
    track
        .props
        .insert("fill_color".into(), theme::SURFACE_ALT.into());
    track.layout = Some(lb(sx + 16.0, bar_y, bar_w, 8.0));
    tree.insert(section_id, track)
        .expect("insert cpu total bar track");

    let fill_w = bar_w * (metrics.cpu_total_percent / 100.0).clamp(0.0, 1.0);
    let color = theme::cpu_color(metrics.cpu_total_percent);
    let mut fill = Node::new(n(ids::CPU_TOTAL_BAR_FILL), "Rectangle");
    fill.props.insert("fill_color".into(), color.into());
    fill.layout = Some(lb(sx + 16.0, bar_y, fill_w, 8.0));
    tree.insert(section_id, fill)
        .expect("insert cpu total bar fill");

    // ── CPU model / freq info ─────────────────────────────────────────────
    let info_y = bar_y + 20.0;
    let mut model = Node::new(n(ids::CPU_MODEL_TEXT), "Text");
    model
        .props
        .insert("text".into(), metrics.cpu_model.as_str().into());
    model
        .props
        .insert("color".into(), theme::TEXT_SECONDARY.into());
    model.props.insert("fontSize".into(), 12.0.into());
    model.layout = Some(lb(sx + 16.0, info_y, sw - 32.0, 16.0));
    tree.insert(section_id, model)
        .expect("insert cpu model text");

    let mut freq = Node::new(n(ids::CPU_FREQ_TEXT), "Text");
    freq.props.insert(
        "text".into(),
        format!(
            "{} cores  |  {} MHz",
            metrics.cpu_core_count, metrics.cpu_frequency_mhz
        )
        .into(),
    );
    freq.props
        .insert("color".into(), theme::TEXT_SECONDARY.into());
    freq.props.insert("fontSize".into(), 12.0.into());
    freq.layout = Some(lb(sx + 16.0, info_y + 18.0, sw - 32.0, 16.0));
    tree.insert(section_id, freq).expect("insert cpu freq text");

    // ── Per-core grid (4 columns) ─────────────────────────────────────────
    let grid_y = info_y + 50.0;
    let cols = 4;
    let col_w = (sw - 32.0) / cols as f32;
    let row_h = 28.0;

    for i in 0..MAX_CORES {
        let pct = metrics.cpu_per_core.get(i).copied().unwrap_or(0.0);
        let col = i % cols;
        let row = i / cols;
        let ix = sx + 16.0 + col as f32 * col_w;
        let iy = grid_y + row as f32 * row_h;
        let core_color = theme::cpu_color(pct);

        let mut label = Node::new(n(ids::cpu_detail_label(i as u64)), "Text");
        label
            .props
            .insert("text".into(), format!("Core {i}: {pct:.0}%").into());
        label
            .props
            .insert("color".into(), theme::TEXT_SECONDARY.into());
        label.props.insert("fontSize".into(), 11.0.into());
        label.layout = Some(lb(ix, iy + 2.0, 80.0, 14.0));
        tree.insert(section_id, label)
            .expect("insert cpu core label");

        let core_bar_w = col_w - 100.0;
        let mut ctrack = Node::new(n(ids::cpu_detail_track(i as u64)), "Rectangle");
        ctrack
            .props
            .insert("fill_color".into(), theme::SURFACE_ALT.into());
        ctrack.layout = Some(lb(ix + 82.0, iy + 4.0, core_bar_w, theme::BAR_H));
        tree.insert(section_id, ctrack)
            .expect("insert cpu core track");

        let cfill_w = core_bar_w * (pct / 100.0).clamp(0.0, 1.0);
        let mut cfill = Node::new(n(ids::cpu_detail_fill(i as u64)), "Rectangle");
        cfill.props.insert("fill_color".into(), core_color.into());
        cfill.layout = Some(lb(ix + 82.0, iy + 4.0, cfill_w, theme::BAR_H));
        tree.insert(section_id, cfill)
            .expect("insert cpu core fill");
    }
}

// Disks detail page — Phase 3.
use ui_core::prelude::*;
use ui_core::types::NodeId;

use crate::data::{self, SystemMetrics};
use crate::ids::{self, n};
use crate::layout_helpers::lb;
use crate::theme;

const MAX_DISKS: usize = 8;

/// Build the Disks detail page inside its container.
pub fn build_disks_page(tree: &mut NodeTree, parent_id: NodeId, metrics: &SystemMetrics) {
    let cx = theme::CONTENT_X;
    let cy = theme::CONTENT_Y;
    let cw = theme::CONTENT_W;

    // ── Section background ────────────────────────────────────────────────
    let mut section = Node::new(n(ids::DISK_SECTION), "Rectangle");
    section
        .props
        .insert("fill_color".into(), theme::SURFACE.into());
    section.layout = Some(lb(cx + 16.0, cy + 16.0, cw - 32.0, 700.0));
    tree.insert(parent_id, section)
        .expect("insert disk section");

    let section_id = n(ids::DISK_SECTION);
    let sx = cx + 16.0;
    let sy = cy + 16.0;
    let sw = cw - 32.0;

    // ── Title ─────────────────────────────────────────────────────────────
    let mut title = Node::new(n(ids::DISK_TITLE), "Text");
    title.props.insert("text".into(), "Disk Usage".into());
    title
        .props
        .insert("color".into(), theme::TEXT_PRIMARY.into());
    title.props.insert("fontSize".into(), 16.0.into());
    title.layout = Some(lb(sx + 16.0, sy + 12.0, sw - 32.0, 20.0));
    tree.insert(section_id, title).expect("insert disk title");

    // ── Per-disk rows ─────────────────────────────────────────────────────
    let row_h = 72.0;
    let start_y = sy + 44.0;
    let bar_w = sw - 32.0;

    for i in 0..MAX_DISKS {
        let ry = start_y + i as f32 * row_h;
        let idx = i as u64;
        let disk = metrics.disks.get(i);

        let d_name = disk.map_or_else(
            || "-".to_string(),
            |d| {
                if d.name.is_empty() {
                    d.mount_point.clone()
                } else {
                    d.name.clone()
                }
            },
        );
        let d_pct = disk.map_or(0.0f32, |d| {
            if d.total_bytes > 0 {
                (d.used_bytes as f64 / d.total_bytes as f64 * 100.0) as f32
            } else {
                0.0
            }
        });
        let d_usage = disk.map_or_else(String::new, |d| {
            format!(
                "{} / {} ({:.1}%)",
                data::format_bytes(d.used_bytes),
                data::format_bytes(d.total_bytes),
                d_pct
            )
        });
        let d_mount = disk.map_or_else(String::new, |d| d.mount_point.clone());
        let color = theme::disk_color(d_pct);

        // Disk name
        let mut name_node = Node::new(n(ids::disk_name(idx)), "Text");
        name_node.props.insert("text".into(), d_name.into());
        name_node
            .props
            .insert("color".into(), theme::TEXT_PRIMARY.into());
        name_node.props.insert("fontSize".into(), 13.0.into());
        name_node.layout = Some(lb(sx + 16.0, ry, bar_w, 16.0));
        tree.insert(section_id, name_node)
            .expect("insert disk name");

        // Bar track
        let bary = ry + 20.0;
        let mut bt = Node::new(n(ids::disk_bar_track(idx)), "Rectangle");
        bt.props
            .insert("fill_color".into(), theme::SURFACE_ALT.into());
        bt.layout = Some(lb(sx + 16.0, bary, bar_w, 8.0));
        tree.insert(section_id, bt).expect("insert disk bar track");

        // Bar fill
        let fill_w = bar_w * (d_pct / 100.0).clamp(0.0, 1.0);
        let mut bf = Node::new(n(ids::disk_bar_fill(idx)), "Rectangle");
        bf.props.insert("fill_color".into(), color.into());
        bf.layout = Some(lb(sx + 16.0, bary, fill_w, 8.0));
        tree.insert(section_id, bf).expect("insert disk bar fill");

        // Usage text
        let mut usage_node = Node::new(n(ids::disk_usage_text(idx)), "Text");
        usage_node.props.insert("text".into(), d_usage.into());
        usage_node
            .props
            .insert("color".into(), theme::TEXT_SECONDARY.into());
        usage_node.props.insert("fontSize".into(), 11.0.into());
        usage_node.layout = Some(lb(sx + 16.0, bary + 14.0, bar_w * 0.6, 14.0));
        tree.insert(section_id, usage_node)
            .expect("insert disk usage text");

        // Mount point
        let mut mount_node = Node::new(n(ids::disk_mount_text(idx)), "Text");
        mount_node.props.insert("text".into(), d_mount.into());
        mount_node
            .props
            .insert("color".into(), theme::TEXT_TERTIARY.into());
        mount_node.props.insert("fontSize".into(), 11.0.into());
        mount_node.layout = Some(lb(sx + 16.0 + bar_w * 0.6, bary + 14.0, bar_w * 0.4, 14.0));
        tree.insert(section_id, mount_node)
            .expect("insert disk mount text");
    }
}

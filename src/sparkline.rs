// Sparkline bar chart from a ring buffer of values.
#![allow(clippy::too_many_arguments)]
use std::collections::VecDeque;

use ui_core::prelude::*;
use ui_core::types::NodeId;

use crate::ids::n;
use crate::layout_helpers::lb;
use crate::theme;

/// Build a sparkline section (background + up to 60 thin bars).
///
/// `section_id` and `bar_base_id` are the NodeId bases.
/// Bars are rendered right-aligned: newest value on the right.
pub fn build_sparkline(
    tree: &mut NodeTree,
    parent_id: NodeId,
    section_id: u64,
    bar_base_id: u64,
    values: &VecDeque<f32>,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    color: &str,
) {
    // Section background
    let mut section = Node::new(n(section_id), "Rectangle");
    section
        .props
        .insert("fill_color".into(), theme::SURFACE.into());
    section.layout = Some(lb(x, y, w, h));
    tree.insert(parent_id, section)
        .expect("insert spark section");

    let max_bars = 60usize;
    let bar_w = w / max_bars as f32;
    let max_val = values.iter().copied().fold(1.0f32, f32::max).max(1.0);

    for (i, &val) in values.iter().enumerate() {
        let bar_h = (val / max_val) * (h - 4.0); // 2px top/bottom padding
        let bx = x + i as f32 * bar_w;
        let by = y + h - 2.0 - bar_h;

        let mut bar = Node::new(n(bar_base_id + i as u64), "Rectangle");
        bar.props.insert("fill_color".into(), color.into());
        bar.layout = Some(lb(bx, by, (bar_w - 1.0).max(1.0), bar_h.max(1.0)));
        tree.insert(n(section_id), bar).expect("insert spark bar");
    }
}

/// Update sparkline bars in-place (called from frame_hook).
pub fn update_sparkline(
    tree: &mut NodeTree,
    section_id: u64,
    bar_base_id: u64,
    values: &VecDeque<f32>,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    color: &str,
) {
    let max_bars = 60usize;
    let bar_w = w / max_bars as f32;
    let max_val = values.iter().copied().fold(1.0f32, f32::max).max(1.0);

    // Update existing bars or create new ones
    for (i, &val) in values.iter().enumerate() {
        let bar_h = (val / max_val) * (h - 4.0);
        let bx = x + i as f32 * bar_w;
        let by = y + h - 2.0 - bar_h;
        let id = n(bar_base_id + i as u64);

        if let Some(node) = tree.get_mut(id) {
            node.props.insert("fill_color".into(), color.into());
            node.layout = Some(lb(bx, by, (bar_w - 1.0).max(1.0), bar_h.max(1.0)));
        } else {
            // Bar doesn't exist yet — create it
            let mut bar = Node::new(id, "Rectangle");
            bar.props.insert("fill_color".into(), color.into());
            bar.layout = Some(lb(bx, by, (bar_w - 1.0).max(1.0), bar_h.max(1.0)));
            // Ignore error if section doesn't exist
            let _ = tree.insert(n(section_id), bar);
        }
    }
}

// Sparkline bar chart from a ring buffer of values.
#![allow(clippy::too_many_arguments)]
use std::collections::VecDeque;

use ui_core::prelude::*;
use ui_core::types::NodeId;

use crate::ids::n;
use crate::layout_helpers::lb;
use crate::theme;

const MAX_BARS: usize = 60;

/// Build a sparkline section with background + all 60 pre-allocated bars.
/// Bars without data are zero-height (invisible).
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

    let bar_w = w / MAX_BARS as f32;
    let max_val = values.iter().copied().fold(1.0f32, f32::max).max(1.0);

    // Pre-create all 60 bars — unused ones are zero-height
    for i in 0..MAX_BARS {
        let (bx, by, bh) = if i < values.len() {
            let val = values[i];
            let bar_h = (val / max_val) * (h - 4.0);
            (x + i as f32 * bar_w, y + h - 2.0 - bar_h, bar_h)
        } else {
            (x + i as f32 * bar_w, y + h - 2.0, 0.0)
        };

        let mut bar = Node::new(n(bar_base_id + i as u64), "Rectangle");
        bar.props.insert("fill_color".into(), color.into());
        bar.layout = Some(lb(bx, by, (bar_w - 1.0).max(1.0), bh.max(0.0)));
        tree.insert(n(section_id), bar).expect("insert spark bar");
    }
}

/// Update sparkline bars in-place (called from frame_hook).
/// Only modifies existing nodes — no structural changes.
pub fn update_sparkline(
    tree: &mut NodeTree,
    bar_base_id: u64,
    values: &VecDeque<f32>,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    color: &str,
) {
    let bar_w = w / MAX_BARS as f32;
    let max_val = values.iter().copied().fold(1.0f32, f32::max).max(1.0);

    for i in 0..MAX_BARS {
        let id = n(bar_base_id + i as u64);
        if let Some(node) = tree.get_mut(id) {
            let (bx, by, bh) = if i < values.len() {
                let val = values[i];
                let bar_h = (val / max_val) * (h - 4.0);
                (x + i as f32 * bar_w, y + h - 2.0 - bar_h, bar_h)
            } else {
                (x + i as f32 * bar_w, y + h - 2.0, 0.0)
            };
            node.props.insert("fill_color".into(), color.into());
            node.layout = Some(lb(bx, by, (bar_w - 1.0).max(1.0), bh.max(0.0)));
        }
    }
}

// Reusable metric card builder.
use ui_core::prelude::*;
use ui_core::types::NodeId;

use crate::ids::{self, n};
use crate::layout_helpers::lb;
use crate::theme;

/// Data needed to render a single metric card.
pub struct CardData {
    pub label: &'static str,
    pub value: String,
    pub percent: f32,
    pub footer_left: String,
    pub footer_right: String,
    pub color_fn: fn(f32) -> &'static str,
}

/// Build a single metric card subtree and attach it to `parent_id`.
///
/// `card_index` determines the ID block (0-based).
/// `(x, y, w, h)` is the card's absolute position.
#[allow(clippy::too_many_arguments)]
pub fn build_card(
    tree: &mut NodeTree,
    parent_id: NodeId,
    card_index: u64,
    data: &CardData,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
) {
    let base = ids::card_base(card_index);
    let color = (data.color_fn)(data.percent);

    // ── Card background ──────────────────────────────────────────────────
    let mut bg = Node::new(n(base + ids::CARD_BG), "Rectangle");
    bg.props.insert("fill_color".into(), theme::SURFACE.into());
    bg.layout = Some(lb(x, y, w, h));
    tree.insert(parent_id, bg).expect("insert card bg");

    let card_id = n(base + ids::CARD_BG);

    // ── Top accent bar (3px) ─────────────────────────────────────────────
    let mut accent = Node::new(n(base + ids::CARD_ACCENT_BAR), "Rectangle");
    accent
        .props
        .insert("fill_color".into(), theme::PRIMARY.into());
    accent.layout = Some(lb(x, y, w, 3.0));
    tree.insert(card_id, accent).expect("insert accent");

    // ── Status dot ───────────────────────────────────────────────────────
    let dot_size = 8.0;
    let mut dot = Node::new(n(base + ids::CARD_STATUS_DOT), "Rectangle");
    dot.props.insert("fill_color".into(), color.into());
    dot.layout = Some(lb(x + 12.0, y + 16.0, dot_size, dot_size));
    tree.insert(card_id, dot).expect("insert dot");

    // ── Label ────────────────────────────────────────────────────────────
    let mut label = Node::new(n(base + ids::CARD_LABEL), "Text");
    label.props.insert("text".into(), data.label.into());
    label
        .props
        .insert("color".into(), theme::TEXT_SECONDARY.into());
    label.props.insert("fontSize".into(), 12.0.into());
    label.layout = Some(lb(x + 26.0, y + 14.0, w - 40.0, 16.0));
    tree.insert(card_id, label).expect("insert label");

    // ── Value (large) ────────────────────────────────────────────────────
    let mut value = Node::new(n(base + ids::CARD_VALUE), "Text");
    value.props.insert("text".into(), data.value.clone().into());
    value
        .props
        .insert("color".into(), theme::TEXT_PRIMARY.into());
    value.props.insert("fontSize".into(), 28.0.into());
    value.layout = Some(lb(x + 12.0, y + 36.0, w - 24.0, 32.0));
    tree.insert(card_id, value).expect("insert value");

    // ── Progress bar track ───────────────────────────────────────────────
    let bar_y = y + 76.0;
    let bar_w = w - 24.0;
    let mut track = Node::new(n(base + ids::CARD_BAR_TRACK), "Rectangle");
    track
        .props
        .insert("fill_color".into(), theme::SURFACE_ALT.into());
    track.layout = Some(lb(x + 12.0, bar_y, bar_w, theme::BAR_H));
    tree.insert(card_id, track).expect("insert bar track");

    // ── Progress bar fill ────────────────────────────────────────────────
    let fill_w = bar_w * (data.percent / 100.0).clamp(0.0, 1.0);
    let mut fill = Node::new(n(base + ids::CARD_BAR_FILL), "Rectangle");
    fill.props.insert("fill_color".into(), color.into());
    fill.layout = Some(lb(x + 12.0, bar_y, fill_w, theme::BAR_H));
    tree.insert(card_id, fill).expect("insert bar fill");

    // ── Footer left ──────────────────────────────────────────────────────
    let footer_y = y + h - 22.0;
    let mut fl = Node::new(n(base + ids::CARD_FOOTER_LEFT), "Text");
    fl.props
        .insert("text".into(), data.footer_left.clone().into());
    fl.props.insert("color".into(), theme::TEXT_TERTIARY.into());
    fl.props.insert("fontSize".into(), 11.0.into());
    fl.layout = Some(lb(x + 12.0, footer_y, w / 2.0 - 16.0, 14.0));
    tree.insert(card_id, fl).expect("insert footer left");

    // ── Footer right ─────────────────────────────────────────────────────
    let mut fr = Node::new(n(base + ids::CARD_FOOTER_RIGHT), "Text");
    fr.props
        .insert("text".into(), data.footer_right.clone().into());
    fr.props.insert("color".into(), theme::TEXT_TERTIARY.into());
    fr.props.insert("fontSize".into(), 11.0.into());
    fr.layout = Some(lb(x + w / 2.0, footer_y, w / 2.0 - 12.0, 14.0));
    tree.insert(card_id, fr).expect("insert footer right");
}

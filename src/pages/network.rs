// Network detail page — Phase 3.
use ui_core::prelude::*;
use ui_core::types::NodeId;

use crate::data;
use crate::history::MetricsHistory;
use crate::ids::{self, n};
use crate::layout_helpers::lb;
use crate::theme;

/// Build the Network detail page inside its container.
pub fn build_network_page(tree: &mut NodeTree, parent_id: NodeId, history: &MetricsHistory) {
    let cx = theme::CONTENT_X;
    let cy = theme::CONTENT_Y;
    let cw = theme::CONTENT_W;

    // ── Section background ────────────────────────────────────────────────
    let mut section = Node::new(n(ids::NET_SECTION), "Rectangle");
    section
        .props
        .insert("fill_color".into(), theme::SURFACE.into());
    section.layout = Some(lb(cx + 16.0, cy + 16.0, cw - 32.0, 300.0));
    tree.insert(parent_id, section).expect("insert net section");

    let section_id = n(ids::NET_SECTION);
    let sx = cx + 16.0;
    let sy = cy + 16.0;
    let sw = cw - 32.0;

    // ── Title ─────────────────────────────────────────────────────────────
    let mut title = Node::new(n(ids::NET_TITLE), "Text");
    title.props.insert("text".into(), "Network".into());
    title
        .props
        .insert("color".into(), theme::TEXT_PRIMARY.into());
    title.props.insert("fontSize".into(), 16.0.into());
    title.layout = Some(lb(sx + 16.0, sy + 12.0, sw - 32.0, 20.0));
    tree.insert(section_id, title).expect("insert net title");

    // ── RX rate ───────────────────────────────────────────────────────────
    let row_y = sy + 50.0;
    let col_w = (sw - 32.0) / 3.0;

    let mut rx_label = Node::new(n(ids::NET_RX_LABEL), "Text");
    rx_label.props.insert("text".into(), "Download".into());
    rx_label
        .props
        .insert("color".into(), theme::TEXT_SECONDARY.into());
    rx_label.props.insert("fontSize".into(), 12.0.into());
    rx_label.layout = Some(lb(sx + 16.0, row_y, col_w, 16.0));
    tree.insert(section_id, rx_label)
        .expect("insert net rx label");

    let mut rx_val = Node::new(n(ids::NET_RX_VALUE), "Text");
    rx_val.props.insert(
        "text".into(),
        data::format_throughput(history.net_rx_rate).into(),
    );
    rx_val
        .props
        .insert("color".into(), theme::TEXT_PRIMARY.into());
    rx_val.props.insert("fontSize".into(), 28.0.into());
    rx_val.layout = Some(lb(sx + 16.0, row_y + 20.0, col_w, 32.0));
    tree.insert(section_id, rx_val)
        .expect("insert net rx value");

    // ── TX rate ───────────────────────────────────────────────────────────
    let mut tx_label = Node::new(n(ids::NET_TX_LABEL), "Text");
    tx_label.props.insert("text".into(), "Upload".into());
    tx_label
        .props
        .insert("color".into(), theme::TEXT_SECONDARY.into());
    tx_label.props.insert("fontSize".into(), 12.0.into());
    tx_label.layout = Some(lb(sx + 16.0 + col_w, row_y, col_w, 16.0));
    tree.insert(section_id, tx_label)
        .expect("insert net tx label");

    let mut tx_val = Node::new(n(ids::NET_TX_VALUE), "Text");
    tx_val.props.insert(
        "text".into(),
        data::format_throughput(history.net_tx_rate).into(),
    );
    tx_val
        .props
        .insert("color".into(), theme::TEXT_PRIMARY.into());
    tx_val.props.insert("fontSize".into(), 28.0.into());
    tx_val.layout = Some(lb(sx + 16.0 + col_w, row_y + 20.0, col_w, 32.0));
    tree.insert(section_id, tx_val)
        .expect("insert net tx value");

    // ── Total rate ────────────────────────────────────────────────────────
    let mut total_label = Node::new(n(ids::NET_TOTAL_LABEL), "Text");
    total_label.props.insert("text".into(), "Total".into());
    total_label
        .props
        .insert("color".into(), theme::TEXT_SECONDARY.into());
    total_label.props.insert("fontSize".into(), 12.0.into());
    total_label.layout = Some(lb(sx + 16.0 + col_w * 2.0, row_y, col_w, 16.0));
    tree.insert(section_id, total_label)
        .expect("insert net total label");

    let total = history.net_rx_rate + history.net_tx_rate;
    let mut total_val = Node::new(n(ids::NET_TOTAL_VALUE), "Text");
    total_val
        .props
        .insert("text".into(), data::format_throughput(total).into());
    total_val
        .props
        .insert("color".into(), theme::TEXT_PRIMARY.into());
    total_val.props.insert("fontSize".into(), 28.0.into());
    total_val.layout = Some(lb(sx + 16.0 + col_w * 2.0, row_y + 20.0, col_w, 32.0));
    tree.insert(section_id, total_val)
        .expect("insert net total value");
}

// Shell layout: header, sidebar, content area, status bar.
use ui_core::prelude::*;
use ui_core::types::NodeId;

use crate::ids::{self, n};
use crate::layout_helpers::lb;
use crate::theme;

/// Build the dashboard chrome (header, sidebar, content area, status bar).
/// Populates the tree starting from the root node.
pub fn build_shell(tree: &mut NodeTree) {
    // ── Root ─────────────────────────────────────────────────────────────────
    {
        let root = tree.get_mut(n(ids::ROOT)).expect("root exists");
        root.children = vec![
            n(ids::HEADER),
            n(ids::BODY),
            n(ids::STATUS_BAR),
        ];
        root.layout = Some(lb(0.0, 0.0, theme::W, theme::H));
    }

    build_header(tree);
    build_body(tree);
    build_status_bar(tree);
}

/// Returns the NodeId of the content area container.
#[must_use]
pub fn content_id() -> NodeId {
    n(ids::CONTENT)
}

// ── Header ───────────────────────────────────────────────────────────────────

fn build_header(tree: &mut NodeTree) {
    let mut header = Node::new(n(ids::HEADER), "Rectangle");
    header.props.insert("fill_color".into(), theme::SURFACE.into());
    header.layout = Some(lb(0.0, 0.0, theme::W, theme::HEADER_H));
    header.children = vec![
        n(ids::HEADER_LOGO),
        n(ids::HEADER_TITLE),
        n(ids::HEADER_HOSTNAME),
        n(ids::HEADER_CLOCK),
    ];
    tree.insert(n(ids::ROOT), header).expect("insert header");

    // Logo accent square
    let mut logo = Node::new(n(ids::HEADER_LOGO), "Rectangle");
    logo.props.insert("fill_color".into(), theme::PRIMARY.into());
    logo.layout = Some(lb(16.0, 10.0, 28.0, 28.0));
    tree.insert(n(ids::HEADER), logo).expect("insert logo");

    // Title
    let mut title = Node::new(n(ids::HEADER_TITLE), "Text");
    title.props.insert("text".into(), "RUIX System Monitor".into());
    title.props.insert("fontSize".into(), 18.0.into());
    title.layout = Some(lb(56.0, 14.0, 250.0, 22.0));
    tree.insert(n(ids::HEADER), title).expect("insert title");

    // Hostname (right side)
    let mut hostname = Node::new(n(ids::HEADER_HOSTNAME), "Text");
    hostname.props.insert("text".into(), "localhost".into());
    hostname.props.insert("fontSize".into(), 12.0.into());
    hostname.layout = Some(lb(theme::W - 260.0, 17.0, 120.0, 16.0));
    tree.insert(n(ids::HEADER), hostname).expect("insert hostname");

    // Clock (right side)
    let mut clock = Node::new(n(ids::HEADER_CLOCK), "Text");
    clock.props.insert("text".into(), "00:00:00".into());
    clock.props.insert("fontSize".into(), 14.0.into());
    clock.layout = Some(lb(theme::W - 120.0, 16.0, 100.0, 18.0));
    tree.insert(n(ids::HEADER), clock).expect("insert clock");
}

// ── Body (sidebar + content) ─────────────────────────────────────────────────

fn build_body(tree: &mut NodeTree) {
    let body_y = theme::HEADER_H;
    let body_h = theme::CONTENT_H;

    let mut body = Node::new(n(ids::BODY), "Container");
    body.children = vec![n(ids::SIDEBAR), n(ids::CONTENT)];
    body.layout = Some(lb(0.0, body_y, theme::W, body_h));
    tree.insert(n(ids::ROOT), body).expect("insert body");

    build_sidebar(tree, body_y, body_h);
    build_content_area(tree, body_y, body_h);
}

fn build_sidebar(tree: &mut NodeTree, body_y: f32, body_h: f32) {
    let mut sidebar = Node::new(n(ids::SIDEBAR), "Rectangle");
    sidebar.props.insert("fill_color".into(), theme::SURFACE.into());
    sidebar.layout = Some(lb(0.0, body_y, theme::SIDEBAR_W, body_h));

    // Collect nav item IDs for children list
    let nav_labels = ["Overview", "CPU", "Memory", "Processes", "Network", "Disks"];
    let mut children = Vec::new();
    for i in 0..nav_labels.len() as u64 {
        children.push(n(ids::NAV_ITEM_BASE + i));
        children.push(n(ids::NAV_TEXT_BASE + i));
    }
    sidebar.children = children;
    tree.insert(n(ids::BODY), sidebar).expect("insert sidebar");

    // Nav items
    let item_h = 36.0;
    let item_pad = 8.0;
    let start_y = body_y + 12.0;

    for (i, label) in nav_labels.iter().enumerate() {
        let iy = start_y + i as f32 * (item_h + item_pad);
        let is_active = i == 0; // "Overview" is active

        // Background rectangle
        let bg_color = if is_active { theme::PRIMARY } else { theme::SURFACE_ALT };
        let mut bg = Node::new(n(ids::NAV_ITEM_BASE + i as u64), "Rectangle");
        bg.props.insert("fill_color".into(), bg_color.into());
        bg.layout = Some(lb(8.0, iy, theme::SIDEBAR_W - 16.0, item_h));
        tree.insert(n(ids::SIDEBAR), bg).expect("insert nav bg");

        // Text label
        let mut text = Node::new(n(ids::NAV_TEXT_BASE + i as u64), "Text");
        text.props.insert("text".into(), (*label).into());
        text.props.insert("fontSize".into(), 13.0.into());
        text.layout = Some(lb(20.0, iy + 10.0, theme::SIDEBAR_W - 40.0, 16.0));
        tree.insert(n(ids::SIDEBAR), text).expect("insert nav text");
    }
}

fn build_content_area(tree: &mut NodeTree, body_y: f32, body_h: f32) {
    let mut content = Node::new(n(ids::CONTENT), "Rectangle");
    content.props.insert("fill_color".into(), theme::BG.into());
    content.layout = Some(lb(theme::SIDEBAR_W, body_y, theme::CONTENT_W, body_h));
    // Children will be added by page builders
    tree.insert(n(ids::BODY), content).expect("insert content");
}

// ── Status Bar ───────────────────────────────────────────────────────────────

fn build_status_bar(tree: &mut NodeTree) {
    let sy = theme::H - theme::STATUS_H;

    let mut bar = Node::new(n(ids::STATUS_BAR), "Rectangle");
    bar.props.insert("fill_color".into(), theme::BG.into());
    bar.layout = Some(lb(0.0, sy, theme::W, theme::STATUS_H));
    bar.children = vec![
        n(ids::STATUS_REFRESH),
        n(ids::STATUS_LAST_UPDATE),
        n(ids::STATUS_CONNECTED),
    ];
    tree.insert(n(ids::ROOT), bar).expect("insert status bar");

    // Refresh rate
    let mut refresh = Node::new(n(ids::STATUS_REFRESH), "Text");
    refresh.props.insert("text".into(), "Refresh: 1.0s".into());
    refresh.props.insert("fontSize".into(), 11.0.into());
    refresh.layout = Some(lb(12.0, sy + 5.0, 120.0, 14.0));
    tree.insert(n(ids::STATUS_BAR), refresh).expect("insert status refresh");

    // Last update
    let mut last_update = Node::new(n(ids::STATUS_LAST_UPDATE), "Text");
    last_update.props.insert("text".into(), "Last update: --:--:--".into());
    last_update.props.insert("fontSize".into(), 11.0.into());
    last_update.layout = Some(lb(theme::W / 2.0 - 80.0, sy + 5.0, 180.0, 14.0));
    tree.insert(n(ids::STATUS_BAR), last_update).expect("insert status update");

    // Connected indicator
    let mut connected = Node::new(n(ids::STATUS_CONNECTED), "Text");
    connected.props.insert("text".into(), "Connected".into());
    connected.props.insert("fontSize".into(), 11.0.into());
    connected.layout = Some(lb(theme::W - 120.0, sy + 5.0, 100.0, 14.0));
    tree.insert(n(ids::STATUS_BAR), connected).expect("insert status connected");
}

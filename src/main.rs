#[allow(dead_code)]
mod ids;
#[allow(dead_code)]
mod layout_helpers;
#[allow(dead_code)]
mod theme;

use std::collections::HashMap;

use ui_core::core::UiCoreImpl;
use ui_core::inspector::AssetsMeta;
use ui_core::prelude::*;
use ui_core::render::softbuffer::softbuffer_backend;

use crate::ids::n;
use crate::layout_helpers::lb;

fn main() {
    let mut tree = NodeTree::new(n(ids::ROOT));

    // Root container
    {
        let root = tree.get_mut(n(ids::ROOT)).expect("root exists");
        root.children = vec![n(2)];
        root.layout = Some(lb(0.0, 0.0, theme::W, theme::H));
    }

    // Smoke-test rectangle
    let mut rect = Node::new(n(2), "Rectangle");
    rect.props.insert("fill_color".into(), theme::SURFACE.into());
    rect.layout = Some(lb(40.0, 40.0, theme::W - 80.0, theme::H - 80.0));
    tree.insert(n(ids::ROOT), rect).expect("insert rect");

    let snapshot = Snapshot {
        version: 1,
        tree,
        view_states: HashMap::new(),
        assets_meta: AssetsMeta::default(),
    };

    let config = UiCoreConfig {
        target_fps: Some(60),
        enable_inspector: false,
        max_frame_time_ms: 16,
        debug_layout: false,
        ..Default::default()
    };

    let mut core = UiCoreImpl::new(config, softbuffer_backend());
    core.apply_snapshot(snapshot).expect("apply_snapshot");

    // Headless CI mode
    if std::env::var("HEADLESS").is_ok() {
        core.tick(Some(100)).expect("headless tick");
        return;
    }

    let _win = core.register_window(
        WindowDescriptor::builder("RUIX System Monitor")
            .size(theme::W, theme::H)
            .build(),
    );

    core.run().expect("event loop error");
}

use std::collections::HashMap;

use ui_core::core::UiCoreImpl;
use ui_core::inspector::AssetsMeta;
use ui_core::prelude::*;
use ui_core::render::softbuffer::softbuffer_backend;
use ui_core::types::{LayoutBox, NodeId, Point, Rect, Size};

const W: f32 = 1280.0;
const H: f32 = 800.0;

fn main() {
    let mut tree = NodeTree::new(NodeId::new(1));

    // Root container
    {
        let root = tree.get_mut(NodeId::new(1)).expect("root exists");
        root.children = vec![NodeId::new(2)];
        root.layout = Some(lb(0.0, 0.0, W, H));
    }

    // Smoke-test rectangle
    let mut rect = Node::new(NodeId::new(2), "Rectangle");
    rect.props.insert("fill_color".into(), "#1E293B".into());
    rect.layout = Some(lb(40.0, 40.0, W - 80.0, H - 80.0));
    tree.insert(NodeId::new(1), rect).expect("insert rect");

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
            .size(W, H)
            .build(),
    );

    core.run().expect("event loop error");
}

fn lb(x: f32, y: f32, w: f32, h: f32) -> LayoutBox {
    LayoutBox {
        origin: Point::new(x, y),
        size: Size::new(w.max(0.0), h.max(0.0)),
        clip: Rect::new(x, y, w.max(0.0), h.max(0.0)),
        z_order: 0,
    }
}

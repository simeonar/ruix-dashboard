// Layout helper functions for manual LayoutBox positioning.
use ui_core::types::{LayoutBox, Point, Rect, Size};

/// Create a LayoutBox from (x, y, w, h).
#[must_use]
pub fn lb(x: f32, y: f32, w: f32, h: f32) -> LayoutBox {
    let w = w.max(0.0);
    let h = h.max(0.0);
    LayoutBox {
        origin: Point::new(x, y),
        size: Size::new(w, h),
        clip: Rect::new(x, y, w, h),
        z_order: 0,
    }
}

/// Parameters for a card grid layout.
pub struct GridArea {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub columns: usize,
    pub pad: f32,
    pub gap: f32,
    pub card_h: f32,
}

/// Calculate position and size for a card in a grid layout.
///
/// Returns (x, y, card_width, card_height).
#[must_use]
pub fn card_rect(index: usize, grid: &GridArea) -> (f32, f32, f32, f32) {
    let col = index % grid.columns;
    let row = index / grid.columns;
    let total_gap = grid.gap * (grid.columns as f32 - 1.0);
    let card_w = (grid.w - grid.pad * 2.0 - total_gap) / grid.columns as f32;
    let x = grid.x + grid.pad + col as f32 * (card_w + grid.gap);
    let y = grid.y + grid.pad + row as f32 * (grid.card_h + grid.gap);
    (x, y, card_w, grid.card_h)
}

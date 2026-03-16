// Navigation state: lock-free shared state for event handler ↔ frame hook.
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::Arc;

use crate::pages::Page;

/// Shared lock-free navigation state.
///
/// Cloned into event handlers and the frame hook closure.
#[derive(Clone)]
pub struct NavState {
    page: Arc<AtomicU8>,
    dirty: Arc<AtomicBool>,
}

impl NavState {
    #[must_use]
    pub fn new() -> Self {
        Self {
            page: Arc::new(AtomicU8::new(Page::Overview as u8)),
            dirty: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Current page.
    #[must_use]
    pub fn current(&self) -> Page {
        Page::from_index(self.page.load(Ordering::Relaxed))
    }

    /// Switch to `page`. Sets dirty flag if page actually changed.
    pub fn navigate(&self, page: Page) {
        let prev = self.page.swap(page as u8, Ordering::Relaxed);
        if prev != page as u8 {
            self.dirty.store(true, Ordering::Relaxed);
        }
    }

    /// Check if the dirty flag is set (peek without clearing).
    #[must_use]
    pub fn is_dirty(&self) -> bool {
        self.dirty.load(Ordering::Relaxed)
    }

    /// Check and clear the dirty flag (returns true once per change).
    pub fn take_dirty(&self) -> bool {
        self.dirty.swap(false, Ordering::Relaxed)
    }
}

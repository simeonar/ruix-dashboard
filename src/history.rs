// Metrics history ring buffer for sparkline rendering.
use std::collections::VecDeque;

use crate::data::SystemMetrics;

const HISTORY_LEN: usize = 60;

pub struct MetricsHistory {
    pub cpu_total: VecDeque<f32>,
    pub mem_percent: VecDeque<f32>,
    /// Network bytes received since last refresh (already a rate from sysinfo).
    pub net_rx_rate: u64,
    /// Network bytes transmitted since last refresh.
    pub net_tx_rate: u64,
}

impl MetricsHistory {
    #[must_use]
    pub fn new() -> Self {
        Self {
            cpu_total: VecDeque::with_capacity(HISTORY_LEN + 1),
            mem_percent: VecDeque::with_capacity(HISTORY_LEN + 1),
            net_rx_rate: 0,
            net_tx_rate: 0,
        }
    }

    pub fn push(&mut self, metrics: &SystemMetrics) {
        push_capped(&mut self.cpu_total, metrics.cpu_total_percent);
        push_capped(&mut self.mem_percent, metrics.mem_percent());
        // sysinfo::received()/transmitted() already return bytes-since-last-refresh
        self.net_rx_rate = metrics.net_rx_bytes;
        self.net_tx_rate = metrics.net_tx_bytes;
    }
}

fn push_capped(buf: &mut VecDeque<f32>, value: f32) {
    if buf.len() >= HISTORY_LEN {
        buf.pop_front();
    }
    buf.push_back(value);
}

// Metrics history ring buffer for sparkline rendering.
use std::collections::VecDeque;

use crate::data::SystemMetrics;

const HISTORY_LEN: usize = 60;

pub struct MetricsHistory {
    pub cpu_total: VecDeque<f32>,
    pub mem_percent: VecDeque<f32>,
    prev_net_rx: u64,
    prev_net_tx: u64,
    pub net_rx_rate: u64,
    pub net_tx_rate: u64,
}

impl MetricsHistory {
    #[must_use]
    pub fn new() -> Self {
        Self {
            cpu_total: VecDeque::with_capacity(HISTORY_LEN + 1),
            mem_percent: VecDeque::with_capacity(HISTORY_LEN + 1),
            prev_net_rx: 0,
            prev_net_tx: 0,
            net_rx_rate: 0,
            net_tx_rate: 0,
        }
    }

    pub fn push(&mut self, metrics: &SystemMetrics) {
        push_capped(&mut self.cpu_total, metrics.cpu_total_percent);
        push_capped(&mut self.mem_percent, metrics.mem_percent());

        // Network rate as delta between consecutive raw totals
        if self.prev_net_rx > 0 {
            self.net_rx_rate = metrics.net_rx_total.saturating_sub(self.prev_net_rx);
            self.net_tx_rate = metrics.net_tx_total.saturating_sub(self.prev_net_tx);
        }
        self.prev_net_rx = metrics.net_rx_total;
        self.prev_net_tx = metrics.net_tx_total;
    }
}

fn push_capped(buf: &mut VecDeque<f32>, value: f32) {
    if buf.len() >= HISTORY_LEN {
        buf.pop_front();
    }
    buf.push_back(value);
}

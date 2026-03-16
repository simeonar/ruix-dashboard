// System metrics collection via sysinfo.
use sysinfo::{Disks, Networks, System};

/// Snapshot of all system metrics at a point in time.
#[allow(dead_code)]
pub struct SystemMetrics {
    pub timestamp: chrono::DateTime<chrono::Local>,

    // CPU
    pub cpu_total_percent: f32,
    pub cpu_per_core: Vec<f32>,
    pub cpu_frequency_mhz: u64,
    pub cpu_model: String,
    pub cpu_core_count: usize,

    // Memory
    pub mem_total_bytes: u64,
    pub mem_used_bytes: u64,
    pub swap_total_bytes: u64,
    pub swap_used_bytes: u64,

    // Disk (aggregate)
    pub disk_total_bytes: u64,
    pub disk_used_bytes: u64,

    // Network (bytes since last refresh — already a rate)
    pub net_rx_bytes: u64,
    pub net_tx_bytes: u64,

    // Processes
    pub process_count: usize,
    pub top_cpu_processes: Vec<ProcessMetric>,
    pub top_mem_processes: Vec<ProcessMetric>,

    // Per-disk
    pub disks: Vec<DiskMetric>,

    // System
    pub hostname: String,
    pub os_version: String,
    pub uptime_seconds: u64,
}

pub struct ProcessMetric {
    pub name: String,
    pub pid: u32,
    pub cpu_percent: f32,
    pub mem_bytes: u64,
}

/// Per-disk snapshot.
pub struct DiskMetric {
    pub name: String,
    pub mount_point: String,
    pub total_bytes: u64,
    pub used_bytes: u64,
}

/// Sources that need separate refreshing in sysinfo 0.32.
pub struct DataSources {
    pub sys: System,
    pub disks: Disks,
    pub networks: Networks,
}

impl DataSources {
    #[must_use]
    pub fn new() -> Self {
        let mut sys = System::new_all();
        // First CPU sample is always 0; sleep briefly then refresh again.
        std::thread::sleep(std::time::Duration::from_millis(200));
        sys.refresh_all();
        Self {
            sys,
            disks: Disks::new_with_refreshed_list(),
            networks: Networks::new_with_refreshed_list(),
        }
    }

    pub fn refresh(&mut self) {
        self.sys.refresh_all();
        self.disks.refresh();
        self.networks.refresh();
    }
}

impl SystemMetrics {
    /// Collect all metrics from the data sources.
    pub fn collect(src: &DataSources) -> Self {
        let sys = &src.sys;
        let cpus = sys.cpus();
        let cpu_per_core: Vec<f32> = cpus.iter().map(|c| c.cpu_usage()).collect();
        let cpu_freq = cpus.first().map_or(0, |c| c.frequency());
        let cpu_model = cpus
            .first()
            .map_or_else(|| "Unknown".into(), |c| c.brand().to_string());

        // Disk — aggregate all + per-disk
        let mut disk_total = 0u64;
        let mut disk_used = 0u64;
        let mut disks_vec = Vec::new();
        for d in src.disks.iter() {
            let total = d.total_space();
            let used = total.saturating_sub(d.available_space());
            disk_total += total;
            disk_used += used;
            disks_vec.push(DiskMetric {
                name: d.name().to_string_lossy().into_owned(),
                mount_point: d.mount_point().to_string_lossy().into_owned(),
                total_bytes: total,
                used_bytes: used,
            });
        }
        disks_vec.truncate(8);

        // Network — aggregate raw totals
        let mut net_rx = 0u64;
        let mut net_tx = 0u64;
        for (_name, data) in src.networks.iter() {
            net_rx += data.received();
            net_tx += data.transmitted();
        }

        // Top processes by CPU
        let mut procs: Vec<ProcessMetric> = sys
            .processes()
            .iter()
            .map(|(pid, p)| ProcessMetric {
                name: p.name().to_string_lossy().into_owned(),
                pid: pid.as_u32(),
                cpu_percent: p.cpu_usage(),
                mem_bytes: p.memory(),
            })
            .collect();
        procs.sort_by(|a, b| {
            b.cpu_percent
                .partial_cmp(&a.cpu_percent)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        procs.truncate(20);

        // Top processes by memory (separate list)
        let mut mem_procs: Vec<ProcessMetric> = sys
            .processes()
            .iter()
            .map(|(pid, p)| ProcessMetric {
                name: p.name().to_string_lossy().into_owned(),
                pid: pid.as_u32(),
                cpu_percent: p.cpu_usage(),
                mem_bytes: p.memory(),
            })
            .collect();
        mem_procs.sort_by(|a, b| b.mem_bytes.cmp(&a.mem_bytes));
        mem_procs.truncate(10);

        Self {
            timestamp: chrono::Local::now(),
            cpu_total_percent: sys.global_cpu_usage(),
            cpu_per_core,
            cpu_frequency_mhz: cpu_freq,
            cpu_model,
            cpu_core_count: cpus.len(),
            mem_total_bytes: sys.total_memory(),
            mem_used_bytes: sys.used_memory(),
            swap_total_bytes: sys.total_swap(),
            swap_used_bytes: sys.used_swap(),
            disk_total_bytes: disk_total,
            disk_used_bytes: disk_used,
            net_rx_bytes: net_rx,
            net_tx_bytes: net_tx,
            process_count: sys.processes().len(),
            top_cpu_processes: procs,
            top_mem_processes: mem_procs,
            disks: disks_vec,
            hostname: System::host_name().unwrap_or_else(|| "unknown".into()),
            os_version: format!(
                "{} {}",
                System::name().unwrap_or_default(),
                System::os_version().unwrap_or_default()
            ),
            uptime_seconds: System::uptime(),
        }
    }

    #[must_use]
    pub fn mem_percent(&self) -> f32 {
        if self.mem_total_bytes == 0 {
            return 0.0;
        }
        (self.mem_used_bytes as f64 / self.mem_total_bytes as f64 * 100.0) as f32
    }

    #[must_use]
    pub fn disk_percent(&self) -> f32 {
        if self.disk_total_bytes == 0 {
            return 0.0;
        }
        (self.disk_used_bytes as f64 / self.disk_total_bytes as f64 * 100.0) as f32
    }
}

// ── Formatting helpers ───────────────────────────────────────────────────────

#[must_use]
pub fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * KB;
    const GB: u64 = 1024 * MB;
    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.0} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.0} KB", bytes as f64 / KB as f64)
    } else {
        format!("{bytes} B")
    }
}

#[must_use]
pub fn format_throughput(bps: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * KB;
    if bps >= MB {
        format!("{:.1} MB/s", bps as f64 / MB as f64)
    } else if bps >= KB {
        format!("{:.0} KB/s", bps as f64 / KB as f64)
    } else {
        format!("{bps} B/s")
    }
}

#[must_use]
pub fn format_uptime(seconds: u64) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let mins = (seconds % 3600) / 60;
    if days > 0 {
        format!("{days}d {hours}h {mins}m")
    } else if hours > 0 {
        format!("{hours}h {mins}m")
    } else {
        format!("{mins}m")
    }
}

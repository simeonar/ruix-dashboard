# RUIX System Monitor Dashboard

Real-time system resources dashboard built on the [RUIX engine](https://github.com/simeonar/ruix-engine). Single portable executable, no installer required.

![Status](https://img.shields.io/badge/status-functional-brightgreen)
![Binary](https://img.shields.io/badge/binary-2.3%20MB-blue)
![Platform](https://img.shields.io/badge/platform-Windows-lightgrey)

## Features

- **6 pages:** Overview, CPU, Memory, Processes, Network, Disks
- **Live data:** 1-second refresh via `sysinfo` crate
- **Navigation:** Sidebar click + keyboard shortcuts (1-6)
- **Sparklines:** CPU and memory 60-second history charts
- **Portable:** Single static .exe, no external dependencies

## Screenshot

Overview page with metric cards, per-core CPU grid, top processes, sparklines, and system info.

## Download

Pre-built Windows x64 binary available on the [Releases](https://github.com/simeonar/ruix-dashboard/releases) page. No installer — just download and run.

## Building from source

> **Note:** Requires the RUIX `ui-core` crate as a sibling directory (`../ui-core`). The engine is not yet published on crates.io.

```bash
# Debug
cargo run

# Release (2.3 MB, static CRT, LTO)
cargo build --release
```

## Architecture

- `src/main.rs` — Entry point, event loop, frame hook, nav event handlers
- `src/data.rs` — `SystemMetrics` collection via `sysinfo`
- `src/history.rs` — Ring buffer for sparkline data (60 samples)
- `src/updater.rs` — Tree build/rebuild + per-page in-place updates
- `src/nav.rs` — Lock-free `NavState` (AtomicU8 + AtomicBool)
- `src/shell.rs` — Header, sidebar, status bar chrome
- `src/pages/` — Overview, CPU, Memory, Processes, Network, Disks
- `src/card.rs` — Reusable metric card builder
- `src/sparkline.rs` — Bar-chart sparkline renderer
- `src/theme.rs` — Dark theme colors, thresholds, layout constants
- `src/ids.rs` — Deterministic NodeId scheme

## License

MIT

// Dashboard pages.
pub mod cpu;
pub mod disks;
pub mod memory;
pub mod network;
pub mod overview;
pub mod processes;

/// Page identifiers matching sidebar nav order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Page {
    Overview = 0,
    Cpu = 1,
    Memory = 2,
    Processes = 3,
    Network = 4,
    Disks = 5,
}

impl Page {
    #[must_use]
    pub fn from_index(i: u8) -> Self {
        match i {
            0 => Self::Overview,
            1 => Self::Cpu,
            2 => Self::Memory,
            3 => Self::Processes,
            4 => Self::Network,
            5 => Self::Disks,
            _ => Self::Overview,
        }
    }

    pub const COUNT: usize = 6;
}

use std::collections::VecDeque;

use clap::Parser;
use ratatui::{
    layout::Constraint,
    widgets::{Cell, TableState},
};
use stomata_core::collectors::structs::{ProcessData, SingleProcessData};
use sysinfo::DiskUsage;

use crate::constants::MAX_HISTORY;

#[derive(Parser, Debug)]
#[command(name = "stomata")]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, default_value_t = 1000)]
    pub interval: u64,
    #[arg(short, long, default_value_t = false)]
    pub store: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Page {
    System,
    Metrics,
    Processes,
    SingleProcess(ProcessData), // pid
    Network,
}

impl Page {
    pub fn titles() -> Vec<&'static str> {
        vec!["System", "Metrics", "Processes", "Network"]
    }

    pub fn from_index(index: usize) -> Self {
        match index {
            0 => Page::System,
            1 => Page::Metrics,
            2 => Page::Processes,
            3 => Page::Network,
            _ => Page::System,
        }
    }
}

// Trait that any type must implement to be displayable in a table
pub trait TableRow {
    fn to_cells(&self) -> Vec<Cell<'_>>;
    fn column_widths() -> Vec<Constraint>;
}

#[derive(Debug)]
pub struct UIState {
    pub process_list: TableState,
    pub single_process_disk_usage: SingleProcessDiskUsage,
}

impl Default for UIState {
    fn default() -> Self {
        Self {
            process_list: TableState::default().with_selected(0),
            single_process_disk_usage: SingleProcessDiskUsage::default(),
        }
    }
}

pub struct SingleProcessUI<'a> {
    pub data: SingleProcessData<'a>,
}

#[derive(Debug)]
pub struct SingleProcessDiskUsage {
    pub pid: u32,
    pub disk_read_usage: VecDeque<u64>,
    pub disk_write_usage: VecDeque<u64>,
}

impl Default for SingleProcessDiskUsage {
    fn default() -> Self {
        Self {
            pid: 0,
            disk_read_usage: VecDeque::<u64>::with_capacity(MAX_HISTORY),
            disk_write_usage: VecDeque::<u64>::with_capacity(MAX_HISTORY),
        }
    }
}

impl SingleProcessDiskUsage {
    pub fn update_disk_history(&mut self, pid: u32, disk_usage: &DiskUsage) {
        // reset the UI state data for disk write/read when changed at current displaying pid
        if pid != self.pid {
            self.disk_read_usage.clear();
            self.disk_write_usage.clear();
            self.pid = pid;
        }

        if self.disk_read_usage.len() > 60 {
            self.disk_read_usage.pop_front();
        }
        self.disk_read_usage.push_back(disk_usage.read_bytes);

        if self.disk_write_usage.len() > 60 {
            self.disk_write_usage.pop_front();
        }
        self.disk_write_usage.push_back(disk_usage.written_bytes);
    }
}

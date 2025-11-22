use chrono::{DateTime, Utc};
use std::collections::VecDeque;
use sysinfo::{DiskUsage, Pid, Process, System};

use crate::collectors::system::metrics::SystemMetrics;

pub enum MetricsCategory {
    ProcessesWithoutTasks, // refreshes processes but not tasks
    Processes,             // refreshes all processes with tasks
    ProcessWithPid(u32),
    Memory,
    CPU,
    AllResources, // refreshes everything
    Basic,        // refreshes CPU + Memory usage
}

#[derive(Debug)]
pub struct SystemCollector {
    pub system: System,
    pub system_metrics: MetricsHistory,
}

#[derive(Debug)]
pub enum MetricsHistory {
    Single(SystemMetrics),
    History(VecDeque<SystemMetrics>),
}

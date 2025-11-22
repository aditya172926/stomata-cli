use chrono::{DateTime, Utc};
use sysinfo::System;

use crate::{collectors::structs::{MetricsCategory, MetricsHistory}, constants::MAX_HISTORY};

#[derive(Debug, Default, Clone)]
pub struct SystemMetrics {
    pub timestamp: DateTime<Utc>,
    pub cpu_count: usize,
    pub cpu_usage: f32,
    pub memory_used: u64,
    pub memory_total: u64,
    pub swap_used: u64,
    pub swap_total: u64
}
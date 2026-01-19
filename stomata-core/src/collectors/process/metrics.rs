use std::collections::HashMap;

use sysinfo::{DiskUsage, Pid, Process};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct ProcessData {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory: u64,
    pub status: String,
    pub cgroup_path: String,
    pub cgroup_controllers: HashMap<String, String> // mapping controller to path
}

#[derive(Debug)]
pub struct CgroupTree {
    pub path: String,
    pub processes: Vec<ProcessData>,
    pub children: HashMap<String, CgroupTree>,
    pub total_cpu: f32,
    total_memory: u64
}

#[derive(Default, Clone)]
pub struct SingleProcessData<'a> {
    pub basic_process_data: ProcessData,
    pub tasks: Vec<&'a Process>,
    pub disk_usage: DiskUsage,
    pub start_time: u64,
    pub running_time: u64,
    pub current_working_dir: Option<String>,
    pub parent_pid: Option<Pid>,
}

#[derive(Debug, Clone, Default)]
pub struct CgroupInfo {
    pub hierarchy_id: u32,
    pub controllers: Vec<String>,
    pub path: String
}
use sysinfo::{DiskUsage, Pid, Process};

#[derive(Debug, Clone, PartialEq)]
pub struct ProcessData {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory: u64,
    pub status: String,
}

pub struct SingleProcessData<'a> {
    pub basic_process_data: ProcessData,
    pub tasks: Vec<&'a Process>,
    pub disk_usage: DiskUsage,
    pub start_time: u64,
    pub running_time: u64,
    pub current_working_dir: Option<String>,
    pub parent_pid: Option<Pid>,
}
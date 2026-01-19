use std::{collections::HashMap, fs};

use anyhow::Result;
use sysinfo::{Pid, Process, System};

use crate::collectors::process::metrics::{CgroupInfo, ProcessData, SingleProcessData};

impl From<&Process> for ProcessData {
    fn from(process: &Process) -> Self {
        let pid = process.pid().as_u32();
        let cgroups = match Self::read_cgroups(pid) {
            Ok(cg) => cg,
            Err(_err) => {
                vec![CgroupInfo::default()]
            }
        };
        
        // controller to path mapping
        let mut controller_map = HashMap::new();
        let mut primary_path = String::from("/");

        for cgroup in &cgroups {
            for controller in &cgroup.controllers {
                controller_map.insert(controller.clone(), cgroup.path.clone());
            }

            // using unified hierarchy (hierarchy_id == 0) or systemd as primary
            if cgroup.hierarchy_id == 0 || cgroup.controllers.contains(&"systemd".to_string()) {
                primary_path = cgroup.path.clone();
            }
        }

        ProcessData {
            pid,
            name: process.name().to_string_lossy().to_string(),
            cpu_usage: process.cpu_usage(),
            memory: process.memory(),
            status: process.status().to_string(),
            cgroup_path: primary_path,
            cgroup_controllers: controller_map
        }
    }
}

impl ProcessData {
    pub fn fetch(system: &System) -> Vec<Self> {
        let processes: Vec<ProcessData> =
            system.processes().values().map(ProcessData::from).collect();
        return processes;
    }

    // read group information for a specific pid
    pub fn read_cgroups(pid: u32) -> Result<Vec<CgroupInfo>> {
        let cgroup_path = format!("proc/{}/cgroup", pid);
        let contents = fs::read_to_string(cgroup_path)?;
        let cgroups: Vec<CgroupInfo> = contents.lines().filter_map(|line| {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() == 3 {
                let controllers: Vec<String> = if parts[1].is_empty() {
                    vec!["unified".to_string()]
                } else {
                    parts[1].split(',').map(String::from).collect()
                };

                Some(CgroupInfo {
                    hierarchy_id: parts[0].parse().ok()?,
                    controllers,
                    path: parts[2].to_string()
                })
            } else {
                None
            }
        }).collect();

        Ok(cgroups)
    }
}

// Single Process
impl<'a> From<(&'a Process, Vec<&'a Process>)> for SingleProcessData<'a> {
    fn from((process, tasks): (&'a Process, Vec<&'a Process>)) -> Self {
        let disk_usage = process.disk_usage();
        let current_working_dir = if let Some(cwd) = process.cwd() {
            Some(cwd.to_string_lossy().to_string())
        } else {
            None
        };
        let start_time = process.start_time();
        let running_time = process.run_time();
        let parent_pid = process.parent();

        SingleProcessData {
            basic_process_data: ProcessData::from(process),
            tasks: tasks,
            disk_usage,
            start_time,
            running_time,
            current_working_dir,
            parent_pid,
        }
    }
}

impl SingleProcessData<'_> {
    pub fn fetch(system: &mut System, pid: u32) -> Option<SingleProcessData<'_>> {
        if let Some(process) = system.process(Pid::from_u32(pid)) {
            let tasks = if let Some(task_pids) = process.tasks() {
                task_pids
                    .iter()
                    .filter_map(|p| system.process(*p))
                    .collect()
            } else {
                Vec::new()
            };

            let single_process_data = SingleProcessData::from((process, tasks));
            Some(single_process_data)
        } else {
            None
        }
    }
}

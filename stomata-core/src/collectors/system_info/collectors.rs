use sysinfo::System;

use crate::collectors::system_info::metrics::SystemInfo;

impl SystemInfo {
    pub fn new(system: &System) -> Self {
        Self {
            os_name: System::name().unwrap_or_else(|| "Unknown".to_string()),
            os_version: System::os_version().unwrap_or_else(|| "Unknown".to_string()),
            kernel_version: System::kernel_version().unwrap_or_else(|| "Unknown".to_string()),
            hostname: System::host_name().unwrap_or_else(|| "Unknown".to_string()),
        }
    }
}
use crate::errors::{DeviceMetricsError, DeviceMetricsErrorCodes};
use anyhow::{bail, Result};
use sysinfo::{CpuExt, DiskExt, System, SystemExt};
use tracing::{error as trace_error, info, trace, warn};

#[derive(Debug, Default)]
pub struct DeviceMetrics {
    system: System,
}

impl DeviceMetrics {
    pub fn new() -> Self {
        trace!(task = "device_metrics instance", "init");
        let mut system = System::new_all();
        system.refresh_all();
        DeviceMetrics { system }
    }

    pub fn get_cpu_usage(&self) -> Result<f32> {
        trace!(task = "get_cpu_usage", "init");
        match self.system.global_cpu_info().cpu_usage() {
            cpu_usage => {
                info!(task = "get_cpu_usage", "cpu usage: {}", cpu_usage);
                Ok(cpu_usage)
            }

            _ => {
                trace_error!(task = "get_cpu_usage", "failed to get CPU usage");
                bail!(DeviceMetricsError::new(
                    DeviceMetricsErrorCodes::FailedToGetCpuUsage,
                    "failed to get CPU usage".to_string(),
                ))
            }
        }
    }

    pub fn get_memory_usage(&self) -> Result<u64> {
        trace!(task = "get_memory_usage", "init");
        match self.system.used_memory() {
            memory_usage => {
                info!(task = "get_memory_usage", "memory usage: {}", memory_usage);
                Ok(memory_usage)
            }
            _ => {
                trace_error!(task = "get_memory_usage", "failed to get memory usage");
                bail!(DeviceMetricsError::new(
                    DeviceMetricsErrorCodes::FailedToGetMemoryUsage,
                    "failed to get memory usage".to_string(),
                ))
            }
        }
    }

    pub fn get_disk_usage(&self) -> Result<u64> {
        trace!(task = "get_disk_usage", "init");
        //take primary disk
        match self.system.disks().iter().take(1).next() {
            Some(primary_disk) => {
                info!(
                    task = "get_disk_usage",
                    "disk usage: {}",
                    primary_disk.total_space() - primary_disk.available_space()
                );
                let disk_usage = primary_disk.total_space() - primary_disk.available_space();
                Ok(disk_usage)
            }
            None => {
                warn!(task = "get_disk_usage", "failed to get disk usage");
                bail!(DeviceMetricsError::new(
                    DeviceMetricsErrorCodes::FailedToGetDiskUsage,
                    "failed to get disk usage".to_string(),
                ))
            }
        }
    }
}

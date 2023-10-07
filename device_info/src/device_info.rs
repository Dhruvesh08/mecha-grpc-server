use anyhow::{bail, Result};
use sysinfo::{CpuExt, DiskExt, System, SystemExt};
use tracing::{error as trace_error, info, trace, warn};

use crate::{DeviceInfoError, DeviceInfoErrorCodes};

#[derive(Debug, Default)]
pub struct MemoryInfo {
    pub total_memory: u64,
    pub available_memory: u64,
    pub free_memory: u64,
}

#[derive(Debug, Default)]
pub struct CpuInfo {
    pub cpu_name: String,
    pub cpu_frequency: u64,
    pub number_of_cores: usize,
}

#[derive(Debug, Default)]
pub struct DiskInfo {
    pub name: String,
    pub fs: String,
    pub removable: bool,
    pub mount_point: String,
    pub used_space: u64,
    pub total_space: u64,
}

#[derive(Debug, Default)]
pub struct DeviceInfo {}

impl DeviceInfo {
    pub fn new() -> Self {
        DeviceInfo {}
    }

    pub fn get_memory_info(&self) -> Result<MemoryInfo> {
        trace!(task = "get_memory_info", "init");
        let mut system = System::new_all();
        system.refresh_all();

        let total_memory = system.total_memory();
        let free_memory = system.free_memory();
        let available_memory = system.available_memory();

        match (total_memory, free_memory, available_memory) {
            (total_memory, free_memory, available_memory) => {
                let memory_info = MemoryInfo {
                    total_memory,
                    free_memory,
                    available_memory,
                };
                info!(task = "get_memory_info", "memory info: {:?}", memory_info);
                return Ok(memory_info);
            }
            _ => {
                trace_error!(task = "get_memory_info", "failed to get memory info");
                bail!(DeviceInfoError::new(
                    DeviceInfoErrorCodes::FailedToGetMemoryUsage,
                    "failed to get memory info".to_string(),
                ))
            }
        }
    }

    pub fn get_cpu_info(&self) -> Result<CpuInfo> {
        trace!(task = "get_cpu_info", "init");
        let mut system = System::new_all();
        system.refresh_all();

        //use some or none to get the first cpu data and peroperly handle the error
        match system.cpus().iter().take(1).next() {
            Some(cpu_data) => {
                // use some or none to get the number of cores and properly handle the error
                let numer_of_cores = match system.physical_core_count() {
                    Some(processor_cores) => {
                        info!(
                            task = "get_cpu_info",
                            "number of cores: {}", processor_cores
                        );
                        processor_cores
                    }
                    None => {
                        warn!(task = "get_cpu_info", "failed to get number of cores");
                        bail!(DeviceInfoError::new(
                            DeviceInfoErrorCodes::FailedToGetCpuUsage,
                            "failed to get CPU info".to_string(),
                        ))
                    }
                };

                let cpu_info = CpuInfo {
                    cpu_name: cpu_data.brand().to_string(),
                    cpu_frequency: cpu_data.frequency(),
                    number_of_cores: numer_of_cores,
                };
                info!(task = "get_cpu_info", "cpu info: {:?}", cpu_info);
                return Ok(cpu_info);
            }
            None => bail!(DeviceInfoError::new(
                DeviceInfoErrorCodes::FailedToGetCpuUsage,
                "failed to get CPU info".to_string(),
            )),
        }

        //if there is cpu data and number of cores then return cpu info else return error with bail and DeviceInfoError
    }

    pub fn get_disk_info(&self) -> Result<Vec<DiskInfo>> {
        trace!(task = "get_disk_info", "init");
        let mut system = System::new_all();
        system.refresh_all();

        let mut disks = Vec::new();

        for disk in system.disks() {
            let disk_info = DiskInfo {
                name: disk.name().to_string_lossy().into_owned(),
                fs: String::from_utf8_lossy(disk.file_system()).into_owned(),
                removable: disk.is_removable(),
                mount_point: disk.mount_point().to_string_lossy().into_owned(),
                used_space: disk.total_space() - disk.available_space(),
                total_space: disk.total_space(),
            };
            info!(task = "get_disk_info", "disk info: {:?}", disk_info);
            disks.push(disk_info);
        }

        if disks.is_empty() {
            warn!("failed to get disk info");
            bail!(DeviceInfoError::new(
                DeviceInfoErrorCodes::FailedToGetDiskUsage,
                "failed to get disk info".to_string(),
            ));
        }

        Ok(disks)
    }
}

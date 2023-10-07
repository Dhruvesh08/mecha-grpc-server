use anyhow::{bail, Result};
use std::fs::{read_to_string, File};
use std::io::Write;
use tracing::{error as trace_error, info, trace, warn};

use crate::{CpuCtrlError, CpuCtrlErrorCodes};

#[derive(Debug)]
pub enum CpuFrequency {
    Freq1200000,
    Freq1600000,
    Freq1800000,
}

#[derive(Debug)]
pub struct CpuCtrl {
    pub cpu_frequency_path: String,
}

impl CpuCtrl {
    pub fn new() -> Self {
        trace!(tack = "cpuctrl instace", "init");
        CpuCtrl {
            cpu_frequency_path: String::from("/sys/devices/system/cpu/cpu0/cpufreq"),
        }
    }

    pub fn set_cpu_governor(&self) -> Result<()> {
        trace!(task = "set_cpu_governor", "init");
        let mut file = match File::create(format!("{}/scaling_governor", self.cpu_frequency_path)) {
            Ok(file) => {
                info!(task = "set_cpu_governor", "set cpu governor to userspace");
                file
            }
            Err(e) => bail!(CpuCtrlError::new(
                CpuCtrlErrorCodes::FailedToSetCpuGovernorPath,
                format!("failed to set CPU governor: {}", e)
            )),
        };
        match file.write_all(b"userspace") {
            Ok(_) => {
                info!(task = "set_cpu_governor", "set cpu governor to userspace");
                Ok(())
            }
            Err(e) => bail!(CpuCtrlError::new(
                CpuCtrlErrorCodes::FailedToSetCpuGovernor,
                format!("failed to set CPU governor: {}", e)
            )),
        }
    }

    pub fn get_cpu_governor(&self) -> Result<String> {
        match read_to_string(format!("{}/scaling_governor", self.cpu_frequency_path)) {
            Ok(content) => {
                info!(task = "get_cpu_governor", "get cpu governor: {}", content);
                Ok(content)
            }
            Err(e) => {
                trace_error!(
                    task = "get_cpu_governor",
                    "failed to get CPU governor: {}",
                    e
                );
                bail!(CpuCtrlError::new(
                    CpuCtrlErrorCodes::FailedToGetCpuGovernor,
                    format!("failed to get CPU governor: {}", e)
                ))
            }
        }
    }

    pub fn get_cpu_frequency(&self) -> Result<String> {
        match read_to_string(format!("{}/scaling_cur_freq", self.cpu_frequency_path)) {
            Ok(content) => Ok(content),
            Err(e) => {
                trace_error!(
                    task = "get_cpu_frequency",
                    "failed to get CPU frequency: {}",
                    e
                );
                bail!(CpuCtrlError::new(
                    CpuCtrlErrorCodes::FailedToGetCpuFrequency,
                    format!("failed to get CPU frequency: {}", e)
                ))
            }
        }
    }

    pub fn set_cpu_frequency(&self, frequency: CpuFrequency) -> Result<()> {
        let freq_str = match frequency {
            CpuFrequency::Freq1200000 => "1200000",
            CpuFrequency::Freq1600000 => "1600000",
            CpuFrequency::Freq1800000 => "1800000",
        };

        let mut file = match File::create(format!("{}/scaling_setspeed", self.cpu_frequency_path)) {
            Ok(file) => {
                info!(
                    task = "set_cpu_frequency",
                    "set cpu frequency to {}", freq_str
                );
                file
            }
            Err(e) => {
                trace_error!(
                    task = "set_cpu_frequency",
                    "failed to set CPU frequency: {}",
                    e
                );
                bail!(CpuCtrlError::new(
                    CpuCtrlErrorCodes::FailedToSetCpuFrequencyPath,
                    format!("failed to set CPU frequency: {}", e)
                ))
            }
        };
        match file.write_all(freq_str.as_bytes()) {
            Ok(_) => {
                info!(
                    task = "set_cpu_frequency",
                    "set cpu frequency to {}", freq_str
                );
                Ok(())
            }
            Err(e) => {
                warn!(
                    task = "set_cpu_frequency",
                    "failed to set CPU frequency: {}", e
                );
                bail!(CpuCtrlError::new(
                    CpuCtrlErrorCodes::FailedToSetCpuFrequency,
                    format!("failed to set CPU frequency: {}", e)
                ))
            }
        }
    }
}

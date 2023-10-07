use crate::{PowerSupplyError, PowerSupplyErrorCodes};
use anyhow::{bail, Result};
use std::fs::{self, File};
use std::io::Read;
use tracing::{error as trace_error, info, trace, warn};

#[derive(Debug)]
pub struct PowerSupply {
    pub name: String,
    pub r#type: String,
    pub status: String,
    pub present: bool,
    pub voltage_now: u32,
    pub current_now: i32,
    pub capacity: u8,
    pub capacity_level: String,
    pub temp: i32,
    pub technology: String,
    pub charge_full: u32,
    pub charge_now: u32,
    pub charge_full_design: u32,
    pub manufacturer: String,
}

pub trait PowerSupplyInfo {
    fn info(&self) -> Result<PowerSupply>;
    fn set_device(&mut self, device: &str) -> Result<()>;
    fn get_device(&self) -> Result<&str>;
    fn get_current(&self) -> Result<i64>;
}

#[derive(Debug, Default)]
pub struct Battery {
    pub path: String,
    pub currnet_now: String,
}

impl PowerSupplyInfo for Battery {
    fn info(&self) -> Result<PowerSupply> {
        trace!(task = "battery_info", "init");
        info!("Battery info");
        let mut file = File::open(&self.path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut power_supply = PowerSupply {
            name: String::new(),
            r#type: String::new(),
            status: String::new(),
            present: false,
            voltage_now: 0,
            current_now: 0,
            capacity: 0,
            capacity_level: String::new(),
            temp: 0,
            technology: String::new(),
            charge_full: 0,
            charge_now: 0,
            charge_full_design: 0,
            manufacturer: String::new(),
        };

        for line in contents.lines() {
            let mut parts = line.splitn(2, '=');
            let key = parts.next().unwrap_or("").trim();
            let value = parts.next().unwrap_or("").trim();

            match key {
                "POWER_SUPPLY_NAME" => power_supply.name = value.to_string(),
                "POWER_SUPPLY_TYPE" => power_supply.r#type = value.to_string(),
                "POWER_SUPPLY_STATUS" => power_supply.status = value.to_string(),
                "POWER_SUPPLY_PRESENT" => power_supply.present = value == "1",
                "POWER_SUPPLY_VOLTAGE_NOW" => power_supply.voltage_now = value.parse().unwrap_or(0),
                "POWER_SUPPLY_CURRENT_NOW" => power_supply.current_now = value.parse().unwrap_or(0),
                "POWER_SUPPLY_CAPACITY" => power_supply.capacity = value.parse().unwrap_or(0),
                "POWER_SUPPLY_CAPACITY_LEVEL" => power_supply.capacity_level = value.to_string(),
                "POWER_SUPPLY_TEMP" => power_supply.temp = value.parse().unwrap_or(0),
                "POWER_SUPPLY_TECHNOLOGY" => power_supply.technology = value.to_string(),
                "POWER_SUPPLY_CHARGE_FULL" => power_supply.charge_full = value.parse().unwrap_or(0),
                "POWER_SUPPLY_CHARGE_NOW" => power_supply.charge_now = value.parse().unwrap_or(0),
                "POWER_SUPPLY_CHARGE_FULL_DESIGN" => {
                    power_supply.charge_full_design = value.parse().unwrap_or(0)
                }
                "POWER_SUPPLY_MANUFACTURER" => power_supply.manufacturer = value.to_string(),
                _ => {}
            }
        }

        Ok(power_supply)
    }

    fn set_device(&mut self, device: &str) -> Result<()> {
        //try to set path or return error
        trace!(task = "set_device", "init");
        info!(task = "set_devide", "set power device path");
        if device.is_empty() {
            trace_error!("Device path is empty");
            bail!(PowerSupplyError::new(
                PowerSupplyErrorCodes::FailedToOpenFile,
                "failed to set device".to_string(),
            ));
        }
        self.path = device.to_owned();
        Ok(())
    }

    fn get_device(&self) -> Result<&str> {
        if self.path.is_empty() {
            trace_error!(task = "get_device_path", "Device path is empty");
            bail!(PowerSupplyError::new(
                PowerSupplyErrorCodes::FailedToOpenFile,
                "Device path is empty".to_string(),
            ));
        }
        Ok(&self.path)
    }

    //to get current_now value read file from current_now path
    fn get_current(&self) -> Result<i64> {
        let mut file = fs::File::open(&self.currnet_now)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let current_now = match contents.trim().parse::<i64>() {
            Ok(value) => value,
            Err(_) => {
                trace_error!(task = "get_current", "Failed to parse current_now value");
                bail!(PowerSupplyError::new(
                PowerSupplyErrorCodes::InvalidDataFormat,
                "Failed to parse current_now value".to_string()
            ))},
        };
        Ok(current_now)
    }
}

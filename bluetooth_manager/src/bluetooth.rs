use crate::errors::{BluetoothError, BluetoothErrorCodes};
use anyhow::{bail, Result};
use bluer::Session;
use tracing::{error as trace_error, info, trace};
//allow dead_code
#[allow(dead_code)]

pub struct BluetoothController {
    session: Session,
}

impl BluetoothController {
    pub async fn new() -> Result<Self> {
        let session = Session::new().await?;
        Ok(Self { session })
    }

    pub async fn bluetooth_status(&self) -> Result<bool> {
        trace!(task = "bluetooth_status", "init");
        let adapter = self.session.default_adapter().await?;
        let powered = match adapter.is_powered().await {
            Ok(powered) => powered,
            Err(e) => {
                trace_error!(
                    task = "bluetooth_status",
                    "unable to get bluetooth status: {}",
                    e
                );
                bail!(BluetoothError::new(
                    BluetoothErrorCodes::UnableToGetBluetoothDeviceStatus,
                    format!("unable to get bluetooth status: {}", e),
                ))
            }
        };
        Ok(powered)
    }

    pub async fn enable_bluetooth(&self) -> Result<()> {
        trace!(task = "enable_bluetooth", "init");
        let adapter = self.session.default_adapter().await?;
        match adapter.set_powered(true).await {
            Ok(_) => {
                info!(task = "enable_bluetooth", "bluetooth turned on");
                Ok(())
            }
            Err(e) => {
                trace_error!(
                    task = "enable_bluetooth",
                    "unable to turn on bluetooth: {}",
                    e
                );
                bail!(BluetoothError::new(
                    BluetoothErrorCodes::UnableToTurnOnBluetooth,
                    format!("unable to turn on bluetooth: {}", e),
                ))
            }
        }
    }

    pub async fn disable_bluetooth(&self) -> Result<()> {
        trace!(task = "disable_bluetooth", "init");
        let adapter = self.session.default_adapter().await?;
        match adapter.set_powered(false).await {
            Ok(_) => {
                info!(task = "disable_bluetooth", "bluetooth turned off");
                Ok(())
            }
            Err(e) => {
                trace_error!(
                    task = "disable_bluetooth",
                    "unable to turn off bluetooth: {}",
                    e
                );
                bail!(BluetoothError::new(
                    BluetoothErrorCodes::UnableToTurnOffBluetooth,
                    format!("unable to turn off bluetooth: {}", e),
                ))
            }
        }
    }
}

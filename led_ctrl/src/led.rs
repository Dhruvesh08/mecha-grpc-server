use crate::errors::{LedCtrlError, LedCtrlErrorCodes};
use anyhow::{bail, Result};
use std::fs::File;
use std::io::{self, Write};
use tracing::{error as trace_error, info, trace};

#[derive(Debug, PartialEq, Eq)]
pub enum LedColor {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Default)]
pub struct LedCtrl {
    red_led_path: String,
    green_led_path: String,
    blue_led_path: String,
}

impl LedCtrl {
    // Constructor for LedCtrl
    pub fn new(red_led_path: &str, green_led_path: &str, blue_led_path: &str) -> Self {
        trace!(task = "led_ctrl instance", "init");
        LedCtrl {
            red_led_path: String::from(red_led_path),
            green_led_path: String::from(green_led_path),
            blue_led_path: String::from(blue_led_path),
        }
    }

    // Function to set the LED based on the specified color
    pub fn set_led(&self, color: LedColor) -> Result<()> {
        trace!(task = "set_led", "init");
        //check if color is valid or not if not return error
        let path = match color {
            LedColor::Red => &self.red_led_path,
            LedColor::Green => &self.green_led_path,
            LedColor::Blue => &self.blue_led_path,
        };

        //try to write the brightness value to the file or return an error
        if let Err(e) = self.write_brightness(path, "1") {
            trace_error!(task = "set_led", "unable to write brightness value: {}", e);
            bail!(LedCtrlError::new(
                LedCtrlErrorCodes::InvalidLedPathValueError,
                format!("unable to write brightness value: {}", e),
            ));
        }
        info!(task = "set_led", "set led to {:?}", color);

        Ok(())
    }

    // Function to clear the LED (set brightness to 0) based on the specified color
    pub fn clear_led(&self, color: LedColor) -> Result<()> {
        trace!(task = "clear_led", "init");
        let path = match color {
            LedColor::Red => &self.red_led_path,
            LedColor::Green => &self.green_led_path,
            LedColor::Blue => &self.blue_led_path,
        };

        //try to write the brightness value to the file or return an error
        if let Err(e) = self.write_brightness(path, "0") {
            trace_error!(
                task = "clear_led",
                "unable to write brightness value: {}",
                e
            );
            bail!(LedCtrlError::new(
                LedCtrlErrorCodes::InvalidLedPathValueError,
                format!("unable to read brightness value: {}", e),
            ));
        }
        info!(task = "clear_led", "clear led {:?}", color);
        Ok(())
    }

    // Private function to write to the brightness file with error handling
    fn write_brightness(&self, path: &str, value: &str) -> Result<(), io::Error> {
        trace!(task = "write_brightness", "init");
        let mut file = File::create(path)?;
        file.write_all(value.as_bytes())?;
        Ok(())
    }
}

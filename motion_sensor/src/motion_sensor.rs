use crate::errors::{MotionSensorError, MotionSensorErrorCodes};
use anyhow::{bail, Result};
use std::default::Default;
use std::fs::File;
use tracing::{error as trace_error, info, trace};
//allow unused import
#[allow(unused_imports)]
use std::io::{self, BufRead, BufReader, Read, Write};

#[derive(Debug, Default)]
pub struct MotionSensor {
    x_axis_path: String,
    y_axis_path: String,
    z_axis_path: String,
}

impl MotionSensor {
    pub fn new(x_path: &str, y_path: &str, z_path: &str) -> Self {
        trace!(task = "motion_sensor instance", "init");
        MotionSensor {
            x_axis_path: String::from(x_path),
            y_axis_path: String::from(y_path),
            z_axis_path: String::from(z_path),
        }
    }

    pub fn read_motion_sensor_value(&self) -> Result<(f64, f64, f64)> {
        trace!(task = "read_motion_sensor_value", "init");
        //read x,y,z values from the motion sensor or error using match and anyhow error
        let (x_value, y_value, z_value) = match (
            self.read_value_from_file(&self.x_axis_path),
            self.read_value_from_file(&self.y_axis_path),
            self.read_value_from_file(&self.z_axis_path),
        ) {
            (Ok(x), Ok(y), Ok(z)) => {
                info!(
                    task = "read_motion_sensor_value",
                    "x: {}, y: {}, z: {}", x, y, z
                );
                (x, y, z)
            }
            (Err(e), _, _) => {
                trace_error!(
                    task = "read_motion_sensor_value",
                    "unable to read x axis value: {}",
                    e
                );
                bail!(MotionSensorError::new(
                    MotionSensorErrorCodes::UnableToReadMotionSensor,
                    format!("unable to read x axis value: {}", e),
                ))
            }
            (_, Err(e), _) => {
                trace_error!(
                    task = "read_motion_sensor_value",
                    "unable to read y axis value: {}",
                    e
                );
                bail!(MotionSensorError::new(
                    MotionSensorErrorCodes::UnableToReadMotionSensor,
                    format!("unable to read y axis value: {}", e),
                ))
            }
            (_, _, Err(e)) => {
                trace_error!(
                    task = "read_motion_sensor_value",
                    "unable to read z axis value: {}",
                    e
                );
                bail!(MotionSensorError::new(
                    MotionSensorErrorCodes::UnableToReadMotionSensor,
                    format!("unable to read z axis value: {}", e),
                ))
            }
        };

        Ok((x_value, y_value, z_value))
    }

    fn read_value_from_file(&self, path: &str) -> Result<f64> {
        trace!(task = "read_value_from_file", "init");
        let file = match File::open(path) {
            Ok(file) => {
                info!(
                    task = "read_value_from_file",
                    "read value from file: {}", path
                );
                file
            }
            Err(e) => {
                trace_error!(
                    task = "read_value_from_file",
                    "unable to open file value: {}",
                    e
                );
                bail!(MotionSensorError::new(
                    MotionSensorErrorCodes::UnableToOpenFile,
                    format!("unable to open file value: {}", e),
                ))
            }
        };

        let buffer = BufReader::new(file);
        let buffer_value = match buffer.lines().next() {
            Some(value) => {
                info!(
                    task = "read_value_from_file",
                    "read value from buffer : {}", path
                );
                value
            }
            None => {
                trace_error!(
                    task = "read_value_from_file",
                    "unable to open file value: {}",
                    path
                );
                bail!(MotionSensorError::new(
                    MotionSensorErrorCodes::UnableToOpenFile,
                    "unable to open file".to_string(),
                ))
            }
        };
        let sensor_value_string = match buffer_value {
            Ok(value) => {
                info!(
                    task = "read_value_from_file",
                    "read sensor_value_string from buffer : {}", path
                );
                value
            }
            Err(e) => {
                trace_error!(
                    task = "read_value_from_file",
                    "unable to parce sensor_value_string from buffer: {}",
                    e
                );
                bail!(MotionSensorError::new(
                    MotionSensorErrorCodes::UnableToParseValue,
                    format!("unable to parse value: {}", e),
                ))
            }
        };

        let sansor_value = match sensor_value_string.trim().parse::<f64>() {
            Ok(value) => {
                info!(
                    task = "read_value_from_file",
                    "read sansor_value from  {}", path
                );
                value
            }
            Err(e) => {
                trace_error!(
                    task = "read_value_from_file",
                    "unable to parse sansor_value : {}",
                    e
                );
                bail!(MotionSensorError::new(
                    MotionSensorErrorCodes::UnableToParseValue,
                    format!("unable to parse value: {}", e),
                ))
            }
        };

        Ok(sansor_value)
    }

    //allow unused function
    //todo: will be used in the next update
    #[allow(dead_code)]
    fn write_value_to_file(&self, path: &str, value: f64) -> Result<()> {
        trace!(task = "write_value_to_file", "init");
        let mut file = match File::create(path) {
            Ok(file) => {
                info!(task = "write_value_to_file", "create file: {}", path);
                file
            }
            Err(e) => {
                trace_error!(
                    task = "write_value_to_file",
                    "unable to open file value: {}",
                    e
                );
                bail!(MotionSensorError::new(
                    MotionSensorErrorCodes::UnableToOpenFile,
                    format!("unable to open file value: {}", e),
                ))
            }
        };

        //try to write the brightness value to the file or return an error
        let _ = match write!(file, "{}", value) {
            Ok(file) => file,
            Err(e) => {
                bail!(MotionSensorError::new(
                    MotionSensorErrorCodes::UnableToParseValue,
                    format!("unable to write data to sensor {}", e),
                ))
            }
        };

        info!(
            task = "write_value_to_file",
            "write value to set brightness: {}", value
        );

        Ok(())
    }

    pub fn detect_motion_sensor_event(&self) -> Result<bool> {
        trace!(task = "detect_motion_sensor_event", "init");
        let (x_value, y_value, z_value) = match self.read_motion_sensor_value() {
            Ok((x, y, z)) => {
                info!(
                    task = "detect_motion_sensor_event",
                    "x: {}, y: {}, z: {}", x, y, z
                );
                (x, y, z)
            }
            Err(e) => {
                trace_error!(
                    task = "detect_motion_sensor_event",
                    "unable to read motion sensor value: {}",
                    e
                );
                bail!(MotionSensorError::new(
                    MotionSensorErrorCodes::UnableToReadMotionSensor,
                    format!("unable to read motion sensor value: {}", e),
                ))
            }
        };
        let mut is_motion_detected = false;
        if x_value != 0.0 || y_value != 0.0 || z_value != 0.0 {
            is_motion_detected = true;
        }
        info!(
            task = "detect_motion_sensor_event",
            "is motion detected: {}", is_motion_detected
        );
        Ok(is_motion_detected)
    }
}

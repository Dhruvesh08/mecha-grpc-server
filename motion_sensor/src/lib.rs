#![deny(clippy::all)]
mod motion_sensor;
pub use motion_sensor::MotionSensor;

mod errors;
pub use errors::{MotionSensorError, MotionSensorErrorCodes};

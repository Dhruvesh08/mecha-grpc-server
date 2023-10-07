#![deny(clippy::all)]

mod cpu_ctrl;
pub use cpu_ctrl::{CpuCtrl,CpuFrequency};

mod errors;
pub use errors::{CpuCtrlError, CpuCtrlErrorCodes};
#![deny(clippy::all)]
mod led;
pub use led::{LedCtrl, LedColor};

mod errors;
pub use errors::{LedCtrlError, LedCtrlErrorCodes};
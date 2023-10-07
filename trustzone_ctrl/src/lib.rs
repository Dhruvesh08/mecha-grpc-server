#![deny(clippy::all)]

mod trust_ctrl;
pub use trust_ctrl::{TrustZoneCtrl,KeySize,KeyType};

mod errors;
pub use errors::{TrustZoneCtrlError, TrustZoneCtrlErrorCodes};

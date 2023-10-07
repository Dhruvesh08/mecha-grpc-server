#![deny(clippy::all)]
mod power_supply;
pub use power_supply::{Battery, PowerSupply, PowerSupplyInfo};

mod errors;
pub use errors::{PowerSupplyError, PowerSupplyErrorCodes};

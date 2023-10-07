//make a sturct for WifiError and implement the Error trait for it as we did for all the other errors try to includ all the possible error code that you can think of while working with linux and wifi

#[derive(Debug, Default, Clone, Copy)]
pub enum WifiErrorCodes {
    #[default]
    NoWifiDeviceFound,
    UnableToTurnOnWifi,
    UnableToTurnOffWifi,
    UnableToConnectToWifiDevice,
    UnableToDisconnectFromWifiDevice,
    UnableToGetWifiDeviceStatus,
    UnableToRemoveWifiDevice,
    Unknown,
}

impl std::fmt::Display for WifiErrorCodes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            WifiErrorCodes::NoWifiDeviceFound => write!(f, "NoWifiDeviceFound"),
            WifiErrorCodes::UnableToTurnOnWifi => write!(f, "UnableToTurnOnWifi"),
            WifiErrorCodes::UnableToTurnOffWifi => write!(f, "UnableToTurnOffWifi"),
            WifiErrorCodes::UnableToConnectToWifiDevice => {
                write!(f, "UnableToConnectToWifiDevice")
            }
            WifiErrorCodes::UnableToDisconnectFromWifiDevice => {
                write!(f, "UnableToDisconnectFromWifiDevice")
            }
            WifiErrorCodes::UnableToGetWifiDeviceStatus => {
                write!(f, "UnableToGetWifiDeviceStatus")
            }
            WifiErrorCodes::UnableToRemoveWifiDevice => {
                write!(f, "UnableToRemoveWifiDevice")
            }
            WifiErrorCodes::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug)]
pub struct WifiError {
    pub code: WifiErrorCodes,
    pub message: String,
}

impl std::fmt::Display for WifiError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}

impl WifiError {
    pub fn new(code: WifiErrorCodes, message: String) -> Self {
        WifiError { code, message }
    }
}

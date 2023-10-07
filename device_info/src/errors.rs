
#[derive(Debug)]
pub enum DeviceInfoErrorCodes {
    UnknownError,
    FailedToGetCpuUsage,
    FailedToGetMemoryUsage,
    FailedToGetDiskUsage,
}

// imple Display for DeviceInfoErrorCodes
impl std::fmt::Display for DeviceInfoErrorCodes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            DeviceInfoErrorCodes::UnknownError => write!(f, "UnknownError"),
            DeviceInfoErrorCodes::FailedToGetCpuUsage => write!(f, "FailedToGetCpuUsage"),
            DeviceInfoErrorCodes::FailedToGetMemoryUsage => write!(f, "FailedToGetMemoryUsage"),
            DeviceInfoErrorCodes::FailedToGetDiskUsage => write!(f, "FailedToGetDiskUsage"),
        }
    }
}

#[derive(Debug)]
pub struct DeviceInfoError {
    pub code: DeviceInfoErrorCodes,
    pub message: String,
}

// impl Display for DeviceInfoError
impl std::fmt::Display for DeviceInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}

impl DeviceInfoError {
    pub fn new(code: DeviceInfoErrorCodes, message: String) -> Self {
        DeviceInfoError { code, message }
    }
}

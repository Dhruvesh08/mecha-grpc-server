
#[derive(Debug)]
pub enum DeviceMetricsErrorCodes {
    UnknownError,
    FailedToGetCpuUsage,
    FailedToGetMemoryUsage,
    FailedToGetDiskUsage,
}

// imple Display for DeviceMetricsErrorCodes
impl std::fmt::Display for DeviceMetricsErrorCodes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            DeviceMetricsErrorCodes::UnknownError => write!(f, "UnknownError"),
            DeviceMetricsErrorCodes::FailedToGetCpuUsage => write!(f, "FailedToGetCpuUsage"),
            DeviceMetricsErrorCodes::FailedToGetMemoryUsage => write!(f, "FailedToGetMemoryUsage"),
            DeviceMetricsErrorCodes::FailedToGetDiskUsage => write!(f, "FailedToGetDiskUsage"),
        }
    }
}

#[derive(Debug)]
pub struct DeviceMetricsError {
    pub code: DeviceMetricsErrorCodes,
    pub message: String,
}

// impl Display for DeviceMetricsError
impl std::fmt::Display for DeviceMetricsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}

impl DeviceMetricsError {
    pub fn new(code: DeviceMetricsErrorCodes, message: String) -> Self {
        DeviceMetricsError { code, message }
    }
}

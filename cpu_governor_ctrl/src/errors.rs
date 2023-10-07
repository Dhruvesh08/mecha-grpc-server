#[derive(Debug)]
pub enum CpuCtrlErrorCodes {
    FailedToSetCpuGovernor,
    FailedToSetCpuGovernorPath,
    FailedToGetCpuGovernor,
    FailedToGetCpuFrequency,
    FailedToSetCpuFrequency,
    FailedToSetCpuFrequencyPath,
    FailedToOpenFile,
    FailedToWriteToFile,
    FailedToReadFile,
    UnknownError,
}

//impl std::fmt::Display  for CpuCtrlErrorCodes
impl std::fmt::Display for CpuCtrlErrorCodes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            CpuCtrlErrorCodes::FailedToSetCpuGovernor => write!(f, "FailedToSetCpuGovernor"),
            CpuCtrlErrorCodes::FailedToSetCpuGovernorPath => {
                write!(f, "FailedToSetCpuGovernorPath")
            }
            CpuCtrlErrorCodes::FailedToGetCpuGovernor => write!(f, "FailedToGetCpuGovernor"),
            CpuCtrlErrorCodes::FailedToGetCpuFrequency => write!(f, "FailedToGetCpuFrequency"),
            CpuCtrlErrorCodes::FailedToSetCpuFrequency => write!(f, "FailedToSetCpuFrequency"),
            CpuCtrlErrorCodes::FailedToSetCpuFrequencyPath => {
                write!(f, "FailedToSetCpuFrequencyPath")
            }
            CpuCtrlErrorCodes::FailedToOpenFile => write!(f, "FailedToOpenFile"),
            CpuCtrlErrorCodes::FailedToWriteToFile => write!(f, "FailedToWriteToFile"),
            CpuCtrlErrorCodes::FailedToReadFile => write!(f, "FailedToReadFile"),
            CpuCtrlErrorCodes::UnknownError => write!(f, "UnknownError"),
        }
    }
}

#[derive(Debug)]
pub struct CpuCtrlError {
    pub code: CpuCtrlErrorCodes,
    pub message: String,
}

//impl std::fmt::Display for CpuCtrlError
impl std::fmt::Display for CpuCtrlError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}

impl CpuCtrlError {
    pub fn new(code: CpuCtrlErrorCodes, message: String) -> Self {
        CpuCtrlError { code, message }
    }
}

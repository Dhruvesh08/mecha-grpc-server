#[derive(Debug)]
pub enum LedCtrlErrorCodes {
    InvalidLedColorError,
    InvalidLedPathValueError,
}

#[derive(Debug)]
pub struct LedCtrlError {
    pub code: LedCtrlErrorCodes,
    pub message: String,
}

impl std::fmt::Display for LedCtrlErrorCodes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            LedCtrlErrorCodes::InvalidLedPathValueError => {
                write!(f, "InvalidBrightnessValueError")
            }
            LedCtrlErrorCodes::InvalidLedColorError => {
                write!(f, "InvalidBrightnessPathError")
            }
        }
    }
}

impl std::fmt::Display for LedCtrlError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}

impl LedCtrlError {
    pub fn new(code: LedCtrlErrorCodes, message: String) -> Self {
        LedCtrlError { code, message }
    }
}

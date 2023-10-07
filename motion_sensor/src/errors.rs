#[derive(Debug)]
pub enum MotionSensorErrorCodes {
    NoMotionDetected,
    UnableToReadMotionSensor,
    Unknown,
    UnableToOpenFile,
    UnableToParseValue,
}

impl std::fmt::Display for MotionSensorErrorCodes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            MotionSensorErrorCodes::NoMotionDetected => write!(f, "NoMotionDetected"),
            MotionSensorErrorCodes::UnableToReadMotionSensor => write!(f, "UnableToReadMotionSensor"),
            MotionSensorErrorCodes::Unknown => write!(f, "Unknown"),
            MotionSensorErrorCodes::UnableToOpenFile => write!(f, "UnableToOpenFile"),
            MotionSensorErrorCodes::UnableToParseValue => write!(f, "UnableToParseValue"),
        }
    }
}

#[derive(Debug)]
pub struct MotionSensorError {
    pub code: MotionSensorErrorCodes,
    pub message: String,
}

impl std::fmt::Display for MotionSensorError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}

impl MotionSensorError {
    pub fn new(code: MotionSensorErrorCodes, message: String) -> Self {
        MotionSensorError { code, message }
    }
}

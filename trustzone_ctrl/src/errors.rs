#[derive(Debug)]
pub enum TrustZoneCtrlErrorCodes {
    // define your error codes here
    UnableToReadTrustZoneCert,
    FileReadError,
    UnableToWriteTrustZoneCert,
    UnableToRemoveTrustZoneCert,
    UnableToGenrateToken,
    UnableToSignTrust,
    UnableToSignTrustZoneData,
    UnableToVerifyToken,
    UnableToVerifyTrustZoneData,
    UnableToGenerateTrustZoneKey,
    UnableToReadTrustDeviceKey,
    UnableToGenerateTrustZoneHMAC,
    UnableToDeriveTrustZoneKey,
    UnableToDecryptTrust,
    UnableToEncryptTrust,
}

impl std::fmt::Display for TrustZoneCtrlErrorCodes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            TrustZoneCtrlErrorCodes::UnableToReadTrustZoneCert => {
                write!(f, "UnableToReadTrustZoneCert")
            }
            TrustZoneCtrlErrorCodes::FileReadError => {
                write!(f, "FileReadError")
            }
            TrustZoneCtrlErrorCodes::UnableToWriteTrustZoneCert => {
                write!(f, "UnableToWriteTrustZoneCert")
            }
            TrustZoneCtrlErrorCodes::UnableToRemoveTrustZoneCert => {
                write!(f, "UnableToRemoveTrustZoneCert")
            }
            TrustZoneCtrlErrorCodes::UnableToGenrateToken => {
                write!(f, "UnableToGenrateToken")
            }
            TrustZoneCtrlErrorCodes::UnableToSignTrust => {
                write!(f, "UnableToSignTrust")
            }
            TrustZoneCtrlErrorCodes::UnableToSignTrustZoneData => {
                write!(f, "UnableToSignTrustZoneData")
            }
            TrustZoneCtrlErrorCodes::UnableToVerifyToken => {
                write!(f, "UnableToVerifyToken")
            }
            TrustZoneCtrlErrorCodes::UnableToVerifyTrustZoneData => {
                write!(f, "UnableToVerifyTrustZoneData")
            }
            TrustZoneCtrlErrorCodes::UnableToGenerateTrustZoneKey => {
                write!(f, "UnableToGenerateTrustZoneKey")
            }
            TrustZoneCtrlErrorCodes::UnableToGenerateTrustZoneHMAC => {
                write!(f, "UnableToGenerateTrustZoneHMAC")
            }
            TrustZoneCtrlErrorCodes::UnableToReadTrustDeviceKey => {
                write!(f, "UnableToReadTrustDeviceKey")
            }
            TrustZoneCtrlErrorCodes::UnableToDeriveTrustZoneKey => {
                write!(f, "UnableToDeriveTrustZoneKey")
            }
            TrustZoneCtrlErrorCodes::UnableToDecryptTrust => {
                write!(f, "UnableToDecryptTrust")
            }
            TrustZoneCtrlErrorCodes::UnableToEncryptTrust => {
                write!(f, "UnableToEncryptTrust")
            }
        }
    }
}

#[derive(Debug)]
pub struct TrustZoneCtrlError {
    pub code: TrustZoneCtrlErrorCodes,
    pub message: String,
}

impl std::fmt::Display for TrustZoneCtrlError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(code: {:?}, message: {})", self.code, self.message)
    }
}

impl TrustZoneCtrlError {
    pub fn new(code: TrustZoneCtrlErrorCodes, message: String) -> Self {
        TrustZoneCtrlError { code, message }
    }
}

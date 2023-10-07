use std::{fs, process::Command};

use anyhow::{bail, Result};
use tracing::{error as trace_error, info, trace};

use crate::{TrustZoneCtrlError, TrustZoneCtrlErrorCodes};

#[derive(Debug, Default)]
pub struct TrustZoneCtrl {
    pub path: String,
}

// Key type enum
#[derive(Debug)]
pub enum KeyType {
    Auth,
    Enc,
    HFWU,
    DevM,
    Sign,
    Agmt,
}

// Key size enum
#[derive(Debug)]
pub enum KeySize {
    ECC256,
    ECC384,
    ECC521,
    BRAINPOOL256,
    BRAINPOOL384,
    BRAINPOOL512,
}

impl TrustZoneCtrl {
    pub fn new() -> Self {
        TrustZoneCtrl {
            path: String::from("/MECHA_TEST/optiga_trust_m"),
        }
    }

    fn read_value_from_file(&self, path: &str) -> Result<String, TrustZoneCtrlError> {
        fs::read_to_string(path).map_err(|e| {
            TrustZoneCtrlError::new(TrustZoneCtrlErrorCodes::FileReadError, e.to_string())
        })
    }

    //write a function to write a file to the trustzone ic and return ok or error

    //read_trustzone_cert we need to read the cert from the trustzone ic and return it that will be type of
    pub fn read_trustzone_cert(&self, output_file: &str, region: &str) -> Result<String> {
        trace!(task = "read_trustzone_cert", "init");

        let command_args = [
            "/MECHA_TEST/optiga_trust_m/trustm_cert",
            "-r",
            region,
            "-o",
            output_file,
        ];
        let command_output = Command::new(&command_args[0])
            .args(&command_args[1..])
            .output()
            .map_err(|e| {
                TrustZoneCtrlError::new(
                    TrustZoneCtrlErrorCodes::UnableToReadTrustZoneCert,
                    format!("Error executing trustm_cert: {}", e),
                )
            });

        match command_output {
            Ok(output) => {
                let output = String::from_utf8(output.stdout).unwrap();
                info!(task = "read_trustzone_cert", "output: {}", output);

                //read the file and return it
                let output = match fs::read_to_string(output_file) {
                    Ok(x) => {
                        info!(task = "read_trustzone_cert", "output: {}", x);
                        x
                    }
                    Err(e) => {
                        trace_error!(task = "read_trustzone_cert", "unable to read cert: {}", e);
                        bail!(TrustZoneCtrlError::new(
                            TrustZoneCtrlErrorCodes::UnableToReadTrustZoneCert,
                            format!("unable to read cert: {}", e),
                        ))
                    }
                };

                Ok(output)
            }
            Err(e) => {
                trace_error!(task = "read_trustzone_cert", "unable to read cert: {}", e);
                bail!(TrustZoneCtrlError::new(
                    TrustZoneCtrlErrorCodes::UnableToReadTrustZoneCert,
                    format!("unable to read cert: {}", e),
                ))
            }
        }
    }

    //write_trustzone_cert we need to write the cert to the trustzone ic and return ok or error using match and anyhow error
    pub fn write_trustzone_cert(&self, cert_file: &str, oid: &str) -> Result<()> {
        trace!(task = "write_trustzone_cert", "init");

        let command_args = [
            "/MECHA_TEST/optiga_trust_m/trustm_cert",
            "-w",
            oid,
            "-i",
            cert_file,
        ];

        let command_output = Command::new(&command_args[0])
            .args(&command_args[1..])
            .output()
            .map_err(|e| {
                TrustZoneCtrlError::new(
                    TrustZoneCtrlErrorCodes::UnableToWriteTrustZoneCert,
                    format!("Error executing trustm_cert: {}", e),
                )
            });

        match command_output {
            Ok(output) => {
                let output_str = String::from_utf8(output.stdout).unwrap();
                info!(task = "write_trustzone_cert", "output: {}", output_str);

                if output.status.success() {
                    Ok(())
                } else {
                    trace_error!(
                        task = "write_trustzone_cert",
                        "unable to write cert: {}",
                        output.status
                    );
                    bail!(TrustZoneCtrlError::new(
                        TrustZoneCtrlErrorCodes::UnableToWriteTrustZoneCert,
                        format!("unable to write cert: {}", output.status)
                    ))
                }
            }
            Err(e) => {
                trace_error!(task = "write_trustzone_cert", "unable to write cert: {}", e);
                bail!(TrustZoneCtrlError::new(
                    TrustZoneCtrlErrorCodes::UnableToWriteTrustZoneCert,
                    format!("unable to write cert: {}", e),
                ))
            }
        }
    }

    //remove_trustzone_cert form the trustzone ic and return ok or error using match and anyhow error
    pub fn remove_trustzone_cert(&self, oid: &str) -> Result<()> {
        trace!(task = "remove_trustzone_cert", "init");

        let command_args = ["/MECHA_TEST/optiga_trust_m/trustm_cert", "-c", oid];

        let command_output = Command::new(&command_args[0])
            .args(&command_args[1..])
            .output()
            .map_err(|e| {
                TrustZoneCtrlError::new(
                    TrustZoneCtrlErrorCodes::UnableToRemoveTrustZoneCert,
                    format!("Error executing trustm_cert: {}", e),
                )
            });

        match command_output {
            Ok(output) => {
                let output_str = String::from_utf8(output.stdout).unwrap();
                info!(task = "remove_trustzone_cert", "output: {}", output_str);

                if output.status.success() {
                    Ok(())
                } else {
                    trace_error!(
                        task = "remove_trustzone_cert",
                        "unable to remove cert: {}",
                        output.status
                    );
                    bail!(TrustZoneCtrlError::new(
                        TrustZoneCtrlErrorCodes::UnableToRemoveTrustZoneCert,
                        format!("unable to remove cert: {}", output.status)
                    ))
                }
            }
            Err(e) => {
                trace_error!(
                    task = "remove_trustzone_cert",
                    "unable to remove cert: {}",
                    e
                );
                bail!(TrustZoneCtrlError::new(
                    TrustZoneCtrlErrorCodes::UnableToRemoveTrustZoneCert,
                    format!("unable to remove cert: {}", e),
                ))
            }
        }
    }

    //generate_trustzone_key we need to generate a key uisng trust ic and return it that will be type of string or error using match and anyhow error
    pub fn generate_trustzone_key(
        &self,
        oid: &str,
        key_type: KeyType,
        key_size: KeySize,
        output_file: &str,
    ) -> Result<String> {
        trace!(task = "generate_trustzone_key", "init");

        let key_type_flag = match key_type {
            KeyType::Auth => "0x01",
            KeyType::Enc => "0x02",
            KeyType::HFWU => "0x04",
            KeyType::DevM => "0x08",
            KeyType::Sign => "0x10",
            KeyType::Agmt => "0x20",
        };

        let key_size_flag = match key_size {
            KeySize::ECC256 => "0x03",
            KeySize::ECC384 => "0x04",
            KeySize::ECC521 => "0x05",
            KeySize::BRAINPOOL256 => "0x13",
            KeySize::BRAINPOOL384 => "0x15",
            KeySize::BRAINPOOL512 => "0x16",
        };

        let command_args = [
            "/MECHA_TEST/optiga_trust_m/trustm_ecc_keygen",
            "-g",
            oid,
            "-t",
            key_type_flag,
            "-k",
            key_size_flag,
            "-o",
            output_file,
        ];

        let command_output = Command::new(&command_args[0])
            .args(&command_args[1..])
            .output()
            .map_err(|e| {
                TrustZoneCtrlError::new(
                    TrustZoneCtrlErrorCodes::UnableToGenerateTrustZoneKey,
                    format!("Error executing trustm_ecc_keygen: {}", e),
                )
            });

        match command_output {
            Ok(output) => {
                let output_str = String::from_utf8(output.stdout).unwrap();
                info!(task = "generate_trustzone_key", "output: {}", output_str);

                if output.status.success() {
                    // Read the public key from the output file and return it
                    match fs::read_to_string(output_file) {
                        Ok(public_key) => {
                            info!(
                                task = "generate_trustzone_key",
                                "public key: {}", public_key
                            );
                            Ok(public_key)
                        }
                        Err(e) => {
                            trace_error!(
                                task = "generate_trustzone_key",
                                "unable to read public key: {}",
                                e
                            );
                            bail!(TrustZoneCtrlError::new(
                                TrustZoneCtrlErrorCodes::UnableToGenerateTrustZoneKey,
                                format!("unable to read public key: {}", e),
                            ))
                        }
                    }
                } else {
                    trace_error!(
                        task = "generate_trustzone_key",
                        "unable to generate key: {}",
                        output.status
                    );
                    bail!(TrustZoneCtrlError::new(
                        TrustZoneCtrlErrorCodes::UnableToGenerateTrustZoneKey,
                        format!("unable to generate key: {}", output.status)
                    ))
                }
            }
            Err(e) => {
                trace_error!(
                    task = "generate_trustzone_key",
                    "unable to generate key: {}",
                    e
                );
                bail!(TrustZoneCtrlError::new(
                    TrustZoneCtrlErrorCodes::UnableToGenerateTrustZoneKey,
                    format!("unable to generate key: {}", e),
                ))
            }
        }
    }

    //sign_trustzone_data uisng trust ic and return it that will be type of string or error using match and anyhow error
    pub fn sign_trustzone_data(
        &self,
        key_oid: &str,
        input_file: &str,
        output_file: &str,
        hash_before_sign: bool,
    ) -> Result<String> {
        trace!(task = "sign_trustzone_data", "init");

        let mut command_args = vec![
            "/MECHA_TEST/optiga_trust_m/trustm_ecc_sign",
            "-k",
            key_oid,
            "-o",
            output_file,
            "-i",
            input_file,
        ];

        if hash_before_sign {
            command_args.push("-H");
        }

        let command_output = Command::new(&command_args[0])
            .args(&command_args[1..])
            .output()
            .map_err(|e| {
                TrustZoneCtrlError::new(
                    TrustZoneCtrlErrorCodes::UnableToSignTrustZoneData,
                    format!("Error executing trustm_ecc_sign: {}", e),
                )
            });

        match command_output {
            Ok(output) => {
                let output_str = String::from_utf8(output.stdout).unwrap();
                info!(task = "sign_trustzone_data", "output: {}", output_str);

                if output.status.success() {
                    // Read the signed data from the output file and return it
                    match fs::read_to_string(output_file) {
                        Ok(signed_data) => {
                            info!(task = "sign_trustzone_data", "signed data: {}", signed_data);
                            Ok(signed_data)
                        }
                        Err(e) => {
                            trace_error!(
                                task = "sign_trustzone_data",
                                "unable to read signed data: {}",
                                e
                            );
                            bail!(TrustZoneCtrlError::new(
                                TrustZoneCtrlErrorCodes::UnableToSignTrustZoneData,
                                format!("unable to read signed data: {}", e),
                            ))
                        }
                    }
                } else {
                    trace_error!(
                        task = "sign_trustzone_data",
                        "unable to sign data: {}",
                        output.status
                    );
                    bail!(TrustZoneCtrlError::new(
                        TrustZoneCtrlErrorCodes::UnableToSignTrustZoneData,
                        format!("unable to sign data: {}", output.status)
                    ))
                }
            }
            Err(e) => {
                trace_error!(task = "sign_trustzone_data", "unable to sign data: {}", e);
                bail!(TrustZoneCtrlError::new(
                    TrustZoneCtrlErrorCodes::UnableToSignTrustZoneData,
                    format!("unable to sign data: {}", e),
                ))
            }
        }
    }

    //verify_trustzone_data that accept path of file using trust ic and return it that will be type of string or error using match and anyhow error
    pub fn verify_trustzone_data(
        &self,
        pubkey_file: &str,
        input_file: &str,
        signature_file: &str,
        hash_before_verify: bool,
    ) -> Result<String> {
        trace!(task = "verify_trustzone_data", "init");

        let mut command_args = vec![
            "/MECHA_TEST/optiga_trust_m/trustm_ecc_verify",
            "-p",
            pubkey_file,
            "-i",
            input_file,
            "-s",
            signature_file,
        ];

        if hash_before_verify {
            command_args.push("-H");
        }

        let command_output = Command::new(&command_args[0])
            .args(&command_args[1..])
            .output()
            .map_err(|e| {
                TrustZoneCtrlError::new(
                    TrustZoneCtrlErrorCodes::UnableToVerifyTrustZoneData,
                    format!("Error executing trustm_ecc_verify: {}", e),
                )
            });

        match command_output {
            Ok(output) => {
                let output_str = String::from_utf8(output.stdout).unwrap();
                info!(task = "verify_trustzone_data", "output: {}", output_str);

                if output.status.success() {
                    Ok("Verify Success.".to_string())
                } else {
                    trace_error!(
                        task = "verify_trustzone_data",
                        "unable to verify data: {}",
                        output.status
                    );
                    bail!(TrustZoneCtrlError::new(
                        TrustZoneCtrlErrorCodes::UnableToVerifyTrustZoneData,
                        format!("unable to verify data: {}", output.status)
                    ))
                }
            }
            Err(e) => {
                trace_error!(
                    task = "verify_trustzone_data",
                    "unable to verify data: {}",
                    e
                );
                bail!(TrustZoneCtrlError::new(
                    TrustZoneCtrlErrorCodes::UnableToVerifyTrustZoneData,
                    format!("unable to verify data: {}", e),
                ))
            }
        }
    }
    //encrypt_trustzone_data that accept path of file using trust ic and return it that will be type of string or error using match and anyhow error
    pub fn encrypt_trustzone_data(&self, _data: &str) -> Result<String> {
        trace!(task = "encrypt_trustzone_data", "init");
        //read x,y,z values from the motion sensor or error using match and anyhow error
        let encrypted_data = match self.read_value_from_file("/dev/trustzone_encrypt") {
            Ok(x) => {
                info!(task = "encrypt_trustzone_data", "encrypted_data: {}", x);
                x
            }
            Err(e) => {
                trace_error!(
                    task = "encrypt_trustzone_data",
                    "unable to read encrypted_data: {}",
                    e
                );
                bail!(TrustZoneCtrlError::new(
                    TrustZoneCtrlErrorCodes::UnableToReadTrustZoneCert,
                    format!("unable to read encrypted_data: {}", e),
                ))
            }
        };
        Ok(encrypted_data)
    }

    //decrypt_trustzone_data that accept path of file using trust ic and return it that will be type of string or error using match and anyhow error
    pub fn decrypt_trustzone_data(&self, _data: &str) -> Result<String> {
        trace!(task = "decrypt_trustzone_data", "init");
        //read x,y,z values from the motion sensor or error using match and anyhow error
        let decrypted_data = match self.read_value_from_file("/dev/trustzone_decrypt") {
            Ok(x) => {
                info!(task = "decrypt_trustzone_data", "decrypted_data: {}", x);
                x
            }
            Err(e) => {
                trace_error!(
                    task = "decrypt_trustzone_data",
                    "unable to read decrypted_data: {}",
                    e
                );
                bail!(TrustZoneCtrlError::new(
                    TrustZoneCtrlErrorCodes::UnableToReadTrustZoneCert,
                    format!("unable to read decrypted_data: {}", e),
                ))
            }
        };
        Ok(decrypted_data)
    }

    //encrypt_trustzone_data that accept path of file using trust ic and return it that will be type of string or error using match and anyhow error
    pub fn encrypt_trustzone_data_with_key(&self, _data: &str, _key: &str) -> Result<String> {
        trace!(task = "encrypt_trustzone_data_with_key", "init");
        //read x,y,z values from the motion sensor or error using match and anyhow error
        let encrypted_data = match self.read_value_from_file("/dev/trustzone_encrypt") {
            Ok(x) => {
                info!(
                    task = "encrypt_trustzone_data_with_key",
                    "encrypted_data: {}", x
                );
                x
            }
            Err(e) => {
                trace_error!(
                    task = "encrypt_trustzone_data_with_key",
                    "unable to read encrypted_data: {}",
                    e
                );
                bail!(TrustZoneCtrlError::new(
                    TrustZoneCtrlErrorCodes::UnableToReadTrustZoneCert,
                    format!("unable to read encrypted_data: {}", e),
                ))
            }
        };
        Ok(encrypted_data)
    }

    pub fn derive_trustzone_key(
        &self,
        secret_oid: &str,
        hkdf_type: u16,
        info_file: &str,
        salt_file: &str,
        output_file: &str,
    ) -> Result<String> {
        trace!(task = "derive_trustzone_key", "init");

        let command_args = [
            "/MECHA_TEST/optiga_trust_m/trustm_hkdf",
            "-i",
            secret_oid,
            "-H",
            &format!("{:#06X}", hkdf_type),
            "-f",
            info_file,
            "-s",
            salt_file,
            "-o",
            output_file,
        ];

        let command_output = Command::new(&command_args[0])
            .args(&command_args[1..])
            .output()
            .map_err(|e| {
                TrustZoneCtrlError::new(
                    TrustZoneCtrlErrorCodes::UnableToDeriveTrustZoneKey,
                    format!("Error executing trustm_hkdf: {}", e),
                )
            });

        match command_output {
            Ok(output) => {
                let output_str = String::from_utf8(output.stdout).unwrap();
                info!(task = "derive_trustzone_key", "output: {}", output_str);

                if output.status.success() {
                    // Read the derived key from the output file and return it
                    match fs::read_to_string(output_file) {
                        Ok(derived_key) => {
                            info!(
                                task = "derive_trustzone_key",
                                "derived key: {}", derived_key
                            );
                            Ok(derived_key)
                        }
                        Err(e) => {
                            trace_error!(
                                task = "derive_trustzone_key",
                                "unable to read derived key: {}",
                                e
                            );
                            bail!(TrustZoneCtrlError::new(
                                TrustZoneCtrlErrorCodes::UnableToDeriveTrustZoneKey,
                                format!("unable to read derived key: {}", e),
                            ))
                        }
                    }
                } else {
                    trace_error!(
                        task = "derive_trustzone_key",
                        "unable to derive key: {}",
                        output.status
                    );
                    bail!(TrustZoneCtrlError::new(
                        TrustZoneCtrlErrorCodes::UnableToDeriveTrustZoneKey,
                        format!("unable to derive key: {}", output.status)
                    ))
                }
            }
            Err(e) => {
                trace_error!(task = "derive_trustzone_key", "unable to derive key: {}", e);
                bail!(TrustZoneCtrlError::new(
                    TrustZoneCtrlErrorCodes::UnableToDeriveTrustZoneKey,
                    format!("unable to derive key: {}", e),
                ))
            }
        }
    }

    //generate_trustzone_hmac we need to generate a hmac uisng trust ic and return it that will be type of string or error using match and anyhow error
    pub fn generate_trustzone_hmac(
        &self,
        secret_oid: &str,
        hmac_type: u16,
        input_data: &str,
        output_file: &str,
    ) -> Result<String> {
        trace!(task = "generate_trustzone_hmac", "init");

        let command_args = [
            "/MECHA_TEST/optiga_trust_m/trustm_hmac",
            "-I",
            secret_oid,
            "-H",
            &format!("{:#06X}", hmac_type),
            "-i",
            input_data,
            "-o",
            output_file,
        ];

        let command_output = Command::new(&command_args[0])
            .args(&command_args[1..])
            .output()
            .map_err(|e| {
                TrustZoneCtrlError::new(
                    TrustZoneCtrlErrorCodes::UnableToGenerateTrustZoneHMAC,
                    format!("Error executing trustm_hmac: {}", e),
                )
            });

        match command_output {
            Ok(output) => {
                let output_str = String::from_utf8(output.stdout).unwrap();
                info!(task = "generate_trustzone_hmac", "output: {}", output_str);

                if output.status.success() {
                    // Read the generated HMAC from the output file and return it
                    match fs::read_to_string(output_file) {
                        Ok(generated_hmac) => {
                            info!(
                                task = "generate_trustzone_hmac",
                                "generated HMAC: {}", generated_hmac
                            );
                            Ok(generated_hmac)
                        }
                        Err(e) => {
                            trace_error!(
                                task = "generate_trustzone_hmac",
                                "unable to read generated HMAC: {}",
                                e
                            );
                            bail!(TrustZoneCtrlError::new(
                                TrustZoneCtrlErrorCodes::UnableToGenerateTrustZoneHMAC,
                                format!("unable to read generated HMAC: {}", e),
                            ))
                        }
                    }
                } else {
                    trace_error!(
                        task = "generate_trustzone_hmac",
                        "unable to generate HMAC: {}",
                        output.status
                    );
                    bail!(TrustZoneCtrlError::new(
                        TrustZoneCtrlErrorCodes::UnableToGenerateTrustZoneHMAC,
                        format!("unable to generate HMAC: {}", output.status)
                    ))
                }
            }
            Err(e) => {
                trace_error!(
                    task = "generate_trustzone_hmac",
                    "unable to generate HMAC: {}",
                    e
                );
                bail!(TrustZoneCtrlError::new(
                    TrustZoneCtrlErrorCodes::UnableToGenerateTrustZoneHMAC,
                    format!("unable to generate HMAC: {}", e),
                ))
            }
        }
    }
}

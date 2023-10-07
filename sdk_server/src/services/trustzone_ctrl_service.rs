pub use mecha_trustzone_ctrl::{KeySize, KeyType, TrustZoneCtrl};
use tonic::{Request, Response, Status};
pub mod trustzone {
    tonic::include_proto!("trustzonectrl");
}

pub use trustzone::{
    trust_zone_ctrl_service_server::{TrustZoneCtrlService, TrustZoneCtrlServiceServer},
    DeriveKeyRequest, DeriveKeyResponse, GenerateHmacRequest, GenerateHmacResponse,
    GenerateKeyRequest, GenerateKeyResponse, ReadCertificationRequest, ReadCertificationResponse,
    RemoveCertificateRequest, RemoveCertificateResponse, SignDataRequest, SignDataResponse,
    VerifyDataRequest, VerifyDataResponse, WriteCertificateRequest, WriteCertificateResponse,
};

#[derive(Debug, Default)]
pub struct TrustZoneCtrlServiceManager {
    pub trustzone_ctrl: TrustZoneCtrl,
}

#[tonic::async_trait]
impl TrustZoneCtrlService for TrustZoneCtrlServiceManager {
    async fn read_certification(
        &self,
        request: Request<ReadCertificationRequest>,
    ) -> Result<Response<ReadCertificationResponse>, Status> {
        //extract the request data
        let request = request.into_inner();
        let output_file = request.output_file;
        let region = request.region;
        //read the certification from the trustzone
        match self
            .trustzone_ctrl
            .read_trustzone_cert(&output_file, &region)
        {
            Ok(certificate) => {
                // Construct a successful response with the certification.
                Ok(Response::new(ReadCertificationResponse { certificate }))
            }
            Err(err) => Err(Status::from_error(err.into())),
        }
    }

    // Write the certification to the trustzone
    async fn write_certificate(
        &self,
        request: Request<WriteCertificateRequest>,
    ) -> Result<Response<WriteCertificateResponse>, Status> {
        //extract the request data
        let request = request.into_inner();
        let cert_file = request.cert_file;
        let region = request.oid;

        //write the certification to the trustzone
        match self
            .trustzone_ctrl
            .write_trustzone_cert(&cert_file, &region)
        {
            Ok(()) => {
                // Return success as true.
                Ok(Response::new(WriteCertificateResponse { success: true }))
            }
            Err(_) => {
                // Return success as false and include an error message if needed.
                Ok(Response::new(WriteCertificateResponse { success: false }))
            }
        }
    }

    //remove the certification from the trustzone
    async fn remove_certificate(
        &self,
        request: Request<RemoveCertificateRequest>,
    ) -> Result<Response<RemoveCertificateResponse>, Status> {
        let request = request.into_inner();
        let oid = request.oid;

        // Call the remove_trustzone_cert function using TrustZoneCtrl.
        match self.trustzone_ctrl.remove_trustzone_cert(&oid) {
            Ok(()) => {
                // Return success as true.
                Ok(Response::new(RemoveCertificateResponse { success: true }))
            }
            Err(_) => {
                // Return success as false.
                Ok(Response::new(RemoveCertificateResponse { success: false }))
            }
        }
    }

    //generate the key in the trustzone
    async fn generate_key(
        &self,
        request: Request<GenerateKeyRequest>,
    ) -> Result<Response<GenerateKeyResponse>, Status> {
        let request = request.into_inner();
        let oid = request.oid;
        let keytype = request.key_type;
        let keysize = request.key_size;
        let output_file = request.output_file;

        //map the keytype from the proto to the trustzone
        let key_type = match keytype {
            0 => KeyType::Auth,
            1 => KeyType::Enc,
            2 => KeyType::HFWU,
            3 => KeyType::DevM,
            4 => KeyType::Sign,
            5 => KeyType::Agmt,
            _ => {
                // Return an invalid argument status if the key type is not recognized.
                return Err(Status::invalid_argument("Invalid key type"));
            }
        };

        //match the keysize from the proto to the trustzone

        let key_size = match keysize {
            0 => KeySize::ECC256,
            1 => KeySize::ECC384,
            2 => KeySize::ECC521,
            3 => KeySize::BRAINPOOL256,
            4 => KeySize::BRAINPOOL384,
            5 => KeySize::BRAINPOOL512,
            _ => {
                // Return an invalid argument status if the key size is not recognized.
                return Err(Status::invalid_argument("Invalid key size"));
            }
        };

        // Your existing logic to generate the key using the mapped enums.
        // Call the generate_trustzone_key function using TrustZoneCtrl.
        match self
            .trustzone_ctrl
            .generate_trustzone_key(&oid, key_type, key_size, &output_file)
        {
            Ok(public_key) => {
                // Return the generated public key without an error message.
                Ok(Response::new(GenerateKeyResponse { public_key }))
            }
            Err(err) => {
                // Return an empty response since error information is not needed.
                Err(Status::from_error(err.into()))
            }
        }
    }

    async fn sign_data(
        &self,
        request: Request<SignDataRequest>,
    ) -> Result<Response<SignDataResponse>, Status> {
        let request = request.into_inner();
        let key_oid = request.key_oid;
        let input_file = request.input_file;
        let output_file = request.output_file;
        let hash_before_sign = request.hash_before_sign;

        // Call the sign_trustzone_data function using TrustZoneCtrl.
        match self.trustzone_ctrl.sign_trustzone_data(
            &key_oid,
            &input_file,
            &output_file,
            hash_before_sign,
        ) {
            Ok(signed_data) => {
                // Return the signed data without an error message.
                Ok(Response::new(SignDataResponse { signed_data }))
            }
            Err(err) => {
                // Return an empty response since error information is not needed.
                Err(Status::from_error(err.into()))
            }
        }
    }

    //verify data in the trustzone
    async fn verify_data(
        &self,
        request: Request<VerifyDataRequest>,
    ) -> Result<Response<VerifyDataResponse>, Status> {
        let request = request.into_inner();
        let pubkey_file = request.pubkey_file;
        let input_file = request.input_file;
        let signature_file = request.signature_file;
        let hash_before_verify = request.hash_before_verify;

        // Your existing logic to verify the data using TrustZoneCtrl.
        // Call the verify_trustzone_data function using TrustZoneCtrl.
        match self.trustzone_ctrl.verify_trustzone_data(
            &pubkey_file,
            &input_file,
            &signature_file,
            hash_before_verify,
        ) {
            Ok(_) => {
                // Return a success message.
                Ok(Response::new(VerifyDataResponse {
                    verification_result: "Verification Success.".to_string(),
                }))
            }
            Err(err) => {
                // Return an empty response since error information is not needed.
                Err(Status::from_error(err.into()))
            }
        }
    }

    //derive the key in the trustzone
    async fn derive_key(
        &self,
        request: Request<DeriveKeyRequest>,
    ) -> Result<Response<DeriveKeyResponse>, Status> {
        let request = request.into_inner();
        let secret_oid = request.secret_oid;
        let hkdf_type = request.hkdf_type;
        let info_file = request.info_file;
        let salt_file = request.salt_file;
        let output_file = request.output_file;

        // Your existing logic to derive the key using TrustZoneCtrl.
        // Call the derive_trustzone_key function using TrustZoneCtrl.
        match self.trustzone_ctrl.derive_trustzone_key(
            &secret_oid,
            hkdf_type.try_into().unwrap(),
            &info_file,
            &salt_file,
            &output_file,
        ) {
            Ok(derived_key) => {
                // Return the derived key without an error message.
                Ok(Response::new(DeriveKeyResponse { derived_key }))
            }
            Err(err) => {
                // Return an empty response since error information is not needed.
                Err(Status::from_error(err.into()))
            }
        }
    }

    //genrate HMAC in the trustzone
    async fn generate_hmac(
        &self,
        request: Request<GenerateHmacRequest>,
    ) -> Result<Response<GenerateHmacResponse>, Status> {
        let request = request.into_inner();
        let secret_oid = request.secret_oid;
        let hmac_type = request.hmac_type;
        let input_data = request.input_data;
        let output_file = request.output_file;

        // Your existing logic to generate the HMAC using TrustZoneCtrl.
        // Call the generate_trustzone_hmac function using TrustZoneCtrl.
        match self.trustzone_ctrl.generate_trustzone_hmac(
            &secret_oid,
            hmac_type.try_into().unwrap(),
            &input_data,
            &output_file,
        ) {
            Ok(generated_hmac) => {
                // Return the generated HMAC without an error message.
                Ok(Response::new(GenerateHmacResponse { generated_hmac }))
            }
            Err(err) => {
                // Return an empty response since error information is not needed.
                Err(Status::from_error(err.into()))
            }
        }
    }
}

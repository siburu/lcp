use crate::enclave_manage::errors::EnclaveManageError as Error;
#[cfg(feature = "sgx")]
use crate::sgx_reexport_prelude::*;
use anyhow::anyhow;
use attestation_report::verify_report;
use crypto::KeyManager;
use ecall_commands::{CommandParams, IASRemoteAttestationInput, IASRemoteAttestationResult};
use enclave_remote_attestation::{
    attestation::create_attestation_report, report::validate_quote_status,
};
use lcp_types::Time;
use sgx_types::{sgx_quote_sign_type_t, sgx_spid_t};
use std::format;
use std::string::String;

pub fn remote_attestation(
    input: IASRemoteAttestationInput,
    params: CommandParams,
) -> Result<IASRemoteAttestationResult, Error> {
    let spid = decode_spid(&input.spid)?;

    let key_manager = KeyManager::new(params.home);
    let pub_key = key_manager
        .get_enclave_key()
        .ok_or(Error::EnclaveKeyNotFound)?
        .get_pubkey();
    let report = create_attestation_report(
        pub_key.as_report_data(),
        sgx_quote_sign_type_t::SGX_UNLINKABLE_SIGNATURE,
        spid,
        &input.ias_key,
    )
    .map_err(Error::SGXError)?;

    verify_report(&report, Time::now())?;
    validate_quote_status(&report.get_avr()?)?;

    Ok(IASRemoteAttestationResult { report })
}

fn decode_spid(hex: &[u8]) -> Result<sgx_spid_t, Error> {
    let mut spid = sgx_spid_t::default();
    let hex = String::from_utf8_lossy(hex);
    let hex = &hex.trim();

    if hex.len() < 16 * 2 {
        Err(anyhow!("Input spid file len ({}) is incorrect!", hex.len()).into())
    } else {
        let decoded_vec = hex::decode(hex).unwrap();
        spid.id.copy_from_slice(&decoded_vec[..16]);
        Ok(spid)
    }
}

#![no_std]
#[cfg(not(target_env = "sgx"))]
extern crate sgx_tstd as std;

pub mod attestation;
mod ocalls;
pub mod report;

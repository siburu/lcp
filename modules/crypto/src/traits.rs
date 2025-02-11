#[cfg(feature = "sgx")]
use crate::sgx_reexport_prelude::*;
use crate::CryptoError as Error;
use std::vec::Vec;
use tiny_keccak::Keccak;

pub trait Verifier {
    fn get_pubkey(&self) -> Vec<u8>;
    fn get_address(&self) -> Vec<u8>;
    fn verify(&self, msg: &[u8; 32], signature: &[u8]) -> Result<(), Error>;
}

pub trait Signer {
    fn sign(&self, msg: &[u8]) -> Result<Vec<u8>, Error>;
    fn sign_hash(&self, msg: &[u8; 32]) -> Result<Vec<u8>, Error>;
    fn use_verifier(&self, f: &mut dyn FnMut(&dyn Verifier));
}

pub trait SealedKey
where
    Self: std::marker::Sized,
{
    fn seal(&self, filepath: &str) -> Result<(), Error>;
    fn unseal(filepath: &str) -> Result<Self, Error>;
}

pub trait Keccak256<T> {
    fn keccak256(&self) -> T
    where
        T: Sized;
}

impl Keccak256<[u8; 32]> for [u8] {
    fn keccak256(&self) -> [u8; 32] {
        let mut keccak = Keccak::new_keccak256();
        let mut result = [0u8; 32];
        keccak.update(self);
        keccak.finalize(result.as_mut());
        result
    }
}

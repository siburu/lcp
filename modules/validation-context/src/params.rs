#[cfg(feature = "sgx")]
use crate::sgx_reexport_prelude::*;
use crate::tendermint::TendermintValidationParams;
use serde::{Deserialize, Serialize};
use std::vec::Vec;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ValidationParams {
    Empty,
    Tendermint(TendermintValidationParams),
}

impl Default for ValidationParams {
    fn default() -> Self {
        Self::Empty
    }
}

impl ValidationParams {
    pub fn to_vec(&self) -> Vec<u8> {
        use ValidationParams::*;

        match self {
            Empty => {
                let mut bz = Vec::new();
                bz.push(0);
                bz
            }
            Tendermint(params) => {
                let mut bz = Vec::new();
                bz.push(1);
                bz.extend(params.to_vec());
                bz
            }
        }
    }

    pub fn from_bytes(bz: &[u8]) -> Self {
        use ValidationParams::*;
        assert!(bz.len() > 0);
        match bz[0] {
            0 => Empty,
            1 => Tendermint(TendermintValidationParams::from_bytes(&bz[1..])),
            id => panic!("unknown type: {}", id),
        }
    }
}

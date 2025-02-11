#[cfg(feature = "sgx")]
use crate::sgx_reexport_prelude::*;
use anyhow::{anyhow, Result};
use ibc::core::ics02_client::{client_consensus::AnyConsensusState, client_state::AnyClientState};
use lcp_types::Any;
use prost::Message;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
#[allow(unused_imports)]
use std::format;
use std::string::String;
use std::vec;
use std::vec::Vec;

pub const STATE_ID_SIZE: usize = 32;

#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StateID([u8; STATE_ID_SIZE]);

impl StateID {
    pub fn from_bytes_array(bytes: [u8; STATE_ID_SIZE]) -> Self {
        StateID(bytes)
    }

    pub fn to_string(&self) -> String {
        hex::encode(self.0)
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn is_zero(&self) -> bool {
        self == &StateID::default()
    }
}

impl TryFrom<&[u8]> for StateID {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != STATE_ID_SIZE {
            return Err(anyhow!("value length must be {}", STATE_ID_SIZE));
        }
        let mut bz: [u8; STATE_ID_SIZE] = Default::default();
        bz.copy_from_slice(value);
        Ok(Self::from_bytes_array(bz))
    }
}

// TODO define owned error types

pub fn gen_state_id(
    any_client_state: AnyClientState,
    any_consensus_state: AnyConsensusState,
) -> Result<StateID> {
    gen_state_id_from_any(
        &Any::from(any_client_state),
        &Any::from(any_consensus_state),
    )
}

pub fn gen_state_id_from_any(any_client_state: &Any, any_consensus_state: &Any) -> Result<StateID> {
    let size = any_client_state.encoded_len() + any_consensus_state.encoded_len();
    let mut buf = vec![0; size];
    any_client_state.encode(&mut buf).unwrap();
    let offset = any_client_state.encoded_len();
    let mut slice = &mut buf[offset..];
    any_consensus_state.encode(&mut slice).unwrap();
    gen_state_id_from_bytes(&buf)
}

pub fn gen_state_id_from_bytes(bz: &[u8]) -> Result<StateID> {
    let mut result: [u8; STATE_ID_SIZE] = Default::default();
    let mut hasher = Sha256::new();
    hasher.input(&bz);
    let h = hasher.result();
    result.copy_from_slice(&h);
    Ok(StateID::from_bytes_array(result))
}

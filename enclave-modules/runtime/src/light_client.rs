use corda_ibc_enclave_light_client;
use lazy_static::lazy_static;
use light_client::{LightClient, LightClientRegistry, LightClientSource};
use std::boxed::Box;
use tendermint_lc;

lazy_static! {
    pub static ref LIGHT_CLIENT_REGISTRY: LightClientRegistry = {
        let mut registry = LightClientRegistry::new();
        tendermint_lc::register_implementations(&mut registry);
        corda_ibc_enclave_light_client::register_implementations(&mut registry);
        registry
    };
}

#[derive(Default)]
pub struct GlobalLightClientRegistry {}

impl LightClientSource<'_> for GlobalLightClientRegistry {
    fn get_light_client(client_type: &str) -> Option<&'static Box<dyn LightClient>> {
        LIGHT_CLIENT_REGISTRY.get(client_type)
    }
}

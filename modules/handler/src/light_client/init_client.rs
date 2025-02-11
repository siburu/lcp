use crate::light_client::LightClientHandlerError as Error;
use commitments::prover::prove_update_client_commitment;
use context::Context;
use enclave_commands::{InitClientInput, InitClientResult, LightClientResult};
use lcp_types::Any;
use light_client::{ClientKeeper, ClientReader, LightClientSource};
use store::KVStore;

pub fn init_client<'l, S: KVStore, L: LightClientSource<'l>>(
    ctx: &mut Context<S>,
    input: InitClientInput,
) -> Result<LightClientResult, Error> {
    ctx.set_timestamp(input.current_timestamp);

    let any_client_state: Any = input.any_client_state.into();
    let lc = L::get_light_client(&any_client_state.type_url).unwrap();
    let ek = ctx.get_enclave_key();
    let res = lc
        .create_client(ctx, any_client_state, input.any_consensus_state.into())
        .map_err(Error::LightClientError)?;

    ctx.store_client_type(res.client_id.clone(), res.client_type)
        .map_err(Error::ICS02Error)?;
    ctx.store_any_client_state(res.client_id.clone(), res.any_client_state)
        .map_err(Error::ICS02Error)?;
    ctx.store_any_consensus_state(res.client_id.clone(), res.height, res.any_consensus_state)
        .map_err(Error::ICS02Error)?;
    ctx.increase_client_counter();
    ctx.store_update_time(res.client_id.clone(), res.height, ctx.host_timestamp())
        .map_err(Error::ICS02Error)?;
    ctx.store_update_height(res.client_id, res.height, ctx.host_height())
        .map_err(Error::ICS02Error)?;

    let proof =
        prove_update_client_commitment(ek, &res.commitment).map_err(Error::CommitmentError)?;
    Ok(LightClientResult::InitClient(InitClientResult(proof)))
}

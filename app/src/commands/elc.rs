use crate::enclave::load_enclave;
use crate::opts::Opts;
use anyhow::Result;
use clap::Parser;
use enclave_api::EnclaveProtoAPI;
use serde::de::DeserializeOwned;
use std::path::PathBuf;

// `client` subcommand
#[derive(Debug, Parser)]
pub enum ELCCmd {
    #[clap(display_order = 1, about = "Create Light Client")]
    CreateClient(ELCOpts),
    #[clap(display_order = 2, about = "Update Light Client")]
    UpdateClient(ELCOpts),
}

impl ELCCmd {
    fn opts(&self) -> &ELCOpts {
        match self {
            ELCCmd::CreateClient(opts) => opts,
            ELCCmd::UpdateClient(opts) => opts,
        }
    }
}

#[derive(Clone, Debug, Parser, PartialEq)]
pub struct ELCOpts {
    /// Path to the enclave binary
    #[clap(long = "enclave", help = "Path to enclave binary")]
    pub enclave: Option<PathBuf>,
    /// Path to the proto msg
    #[clap(long = "msg", help = "Path to proto msg")]
    pub msg: PathBuf,
}

impl ELCOpts {
    fn load<T: DeserializeOwned>(&self) -> Result<T> {
        let bz = std::fs::read(&self.msg)?;
        Ok(serde_json::from_slice(&bz)?)
    }
}

impl ELCCmd {
    pub fn run(&self, opts: &Opts) -> Result<()> {
        let elc_opts = self.opts();
        let enclave = load_enclave(opts, elc_opts.enclave.as_ref())?;
        match self {
            Self::CreateClient(_) => {
                let _ = enclave.proto_create_client(elc_opts.load()?)?;
            }
            Self::UpdateClient(_) => {
                let _ = enclave.proto_update_client(elc_opts.load()?)?;
            }
        }
        Ok(())
    }
}

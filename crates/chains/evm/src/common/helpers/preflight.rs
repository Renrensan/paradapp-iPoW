use ethers::types::{Bytes, U256};
use tracing::{error, info};

use crate::dependencies::context::EvmContext;


pub struct PreflightResult {
    pub static_ok: bool,
    pub static_err: Option<String>,
}
pub async fn preflight_commit_global(
    ctx: &EvmContext,
    header80_bytes: Bytes,
    height: u64,
) -> PreflightResult {
    let call = ctx
        .c_op
        .commit_global_bitcoin_header_80(
            header80_bytes,
            U256::from(height),
            vec![],
        );

    match call.call().await {
        Ok(_) => {
            info!(height, "preflight OK");
            PreflightResult {
                static_ok: true,
                static_err: None,
            }
        }
        Err(err) => {
            let msg = err.to_string();
            error!(height, error = %msg, "preflight reverted");
            PreflightResult {
                static_ok: false,
                static_err: Some(msg),
            }
        }
    }
}

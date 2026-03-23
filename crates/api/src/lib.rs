use axum::{Router, routing::get};
use paradapp_chain_evm::dependencies::config::EvmConfig;
use std::sync::{Arc, RwLock};

pub type SharedState =
    Arc<RwLock<std::collections::HashMap<String, EvmConfig>>>;

pub fn create_router(shared_state: SharedState) -> Router {
    let api_routes =
        Router::new().route("/limits", get(routes::tx_limit::get_all_limits));

    Router::new().nest("/convert", api_routes).with_state(shared_state)
}

mod routes;

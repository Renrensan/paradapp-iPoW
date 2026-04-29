use axum::{Json, extract::State};
use serde::Serialize;
use utoipa::ToSchema;

use crate::SharedState;

#[derive(Serialize, ToSchema)]
pub struct TxLimitResponse {
    pub network: String,
    pub min_limit: u64,
    pub max_limit: u64,
}

#[utoipa::path(
    get,
    path = "/api/convert/limits",
    responses(
        (status = 200, description = "Get all limits", body = [TxLimitResponse])
    )
)]
pub async fn get_all_limits(
    State(state): State<SharedState>,
) -> Json<Vec<TxLimitResponse>> {
    let configs = state.read().unwrap();

    let limits: Vec<TxLimitResponse> = configs
        .iter()
        .map(|(name, cfg)| TxLimitResponse {
            network: name.clone(),
            min_limit: cfg.min_transaction_limit,
            max_limit: cfg.max_transaction_limit,
        })
        .collect();

    Json(limits)
}

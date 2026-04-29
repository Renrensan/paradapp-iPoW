use axum::{Router, routing::get};
use paradapp_chain_evm::dependencies::config::EvmConfig;
use std::sync::{Arc, RwLock};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub type SharedState =
    Arc<RwLock<std::collections::HashMap<String, EvmConfig>>>;

#[derive(OpenApi)]
#[openapi(paths(routes::tx_limit::get_all_limits))]
pub struct ApiDoc;

pub fn create_router(shared_state: SharedState) -> Router {
    let api_routes = Router::new().nest(
        "/convert",
        Router::new().route("/limits", get(routes::tx_limit::get_all_limits)),
    );

    Router::new()
        .nest("/api", api_routes)
        .merge(
            SwaggerUi::new("/docs")
                .url("/api-doc/openapi.json", ApiDoc::openapi()),
        )
        .with_state(shared_state)
}

mod routes;

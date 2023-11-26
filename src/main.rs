pub mod api;
pub mod env_args;
pub mod jwt;
pub mod state;

use api::init_db;
use axum::{Extension, Router};
use state::AppState;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    env_logger::init();

    // Initializations
    init_db().await.unwrap();
    let state = app_state();

    let app = Router::new()
        .nest("/api", api::routes(state.api_state.clone()))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(Extension(())),
        )
        .with_state(state);

    axum::Server::bind(&env_args::ARGS.server_ip.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn app_state() -> AppState {
    AppState {
        api_state: api::state::ApiState {
            jwt_state: jwt::JwtDecoderState {
                decoding_key: jsonwebtoken::DecodingKey::from_secret(
                    env_args::ARGS.jwt_secret.as_bytes(),
                ),
                encoding_key: jsonwebtoken::EncodingKey::from_secret(
                    env_args::ARGS.jwt_secret.as_bytes(),
                ),
                validation: jsonwebtoken::Validation::default(),
            },
        },
    }
}

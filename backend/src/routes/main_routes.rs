use axum::response::Response;
use axum::routing::*;
use axum::Router;
use http::StatusCode;
use hyper::Body;
use sqlx::PgPool;

use crate::db::Store;
use crate::handlers::root;
use crate::{handlers, layers};

/// File handles all our routes and requests
pub async fn app(pool: PgPool) -> Router {
    let db = Store::with_pool(pool);

    let (cors_layer, trace_layer) = layers::get_layers();

    Router::new()
        .route("/", get(root))
        // MATCHES EXPLICITLY FROM TOP TO BOTTOM
        .route("/users", post(handlers::register))
        .route("/login", post(handlers::login))
        .route("/guess", post(handlers::guess_location))
        .route("/protected", get(handlers::protected))
        // Catch all route, AKA: 404
        .route("/*_", get(handle_404)) // '/*_' will match anything not in our routes above
        // .merge(city_routes())
        .layer(cors_layer)
        .layer(trace_layer)
        .with_state(db)
}

async fn handle_404() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("404 not found"))
        .unwrap()
}

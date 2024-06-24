use std::sync::Arc;

use axum::{body::Body, extract::State, http::StatusCode, response::Response};

use crate::ServerState;

pub async fn root(State(state): State<Arc<ServerState>>) -> Response {
    print!("GET /");

    let resp = Response::builder();

    let pages = state.pages.lock().await;

    if let Some(body) = pages.get("root") {
        return resp
            .header("content-type", "text/html")
            .status(StatusCode::OK)
            .body(Body::new(body.clone()))
            .unwrap();
    };

    resp.status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::new("Something went wrong".to_string()))
        .unwrap()
}

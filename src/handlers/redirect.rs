use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    response::Response
};

use crate::ServerState;

pub async fn redirect(State(state): State<Arc<ServerState>>, Path(path): Path<String>) -> Response {
    let resp = Response::builder();

    let urls = state.urls.lock().await;
    let pages = state.pages.lock().await;

    match urls.iter().find(|url| url.id == path) {
        Some(url) => {
            return resp
                .header("location", &url.full_url)
                .status(StatusCode::MOVED_PERMANENTLY)
                .body(Body::empty())
                .unwrap()
        },
        None => match pages.get("404") {
            Some(page) => resp
                .status(StatusCode::NOT_FOUND)
                .body(Body::new(page.clone()))
                .unwrap(),
            None => resp
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
                .unwrap(),
        }
    }
}

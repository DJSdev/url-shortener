mod handlers;
mod utils;

use axum::{
    routing::{get, post},
    Router,
};
use anyhow::Result;
use handlers::{
    redirect::redirect,
    root::root,
    shorten_url::{shorten_url, Url},
};
use utils::load_html::load_html;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

pub struct ServerState {
    pub urls: Mutex<Vec<Url>>,
    pub pages: Mutex<HashMap<String, String>>
}

#[tokio::main]
async fn main() -> Result<()>{
    // Load all the HTML content first before starting server
    let html_folder = "html";
    let pages = load_html(&html_folder).await.unwrap();

    let state = Arc::new(ServerState {
        urls: Mutex::new(vec![]),
        pages: Mutex::new(pages)
    });

    let app = Router::new()
        .route("/", get(root))
        .route("/url", post(shorten_url))
        .route("/:path", get(redirect))
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on 0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

use std::sync::Arc;

use axum::{body::Body, extract::State, http::StatusCode, response::Response, Form};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use tinytemplate::{error::Error, TinyTemplate};

use crate::ServerState;

#[derive(Deserialize)]
pub struct CreateShortUrl {
    url: String,
}

#[derive(Serialize, Clone)]
pub struct Url {
    pub id: String,
    pub full_url: String,
}

pub async fn shorten_url(
    // state of the web server
    State(state): State<Arc<ServerState>>,
    // payload from the POST request
    Form(payload): Form<CreateShortUrl>,
) -> Response {
    let mut urls = state.urls.lock().await;
    let pages = state.pages.lock().await;

    // idk just generate a random 10 char len string for now
    let url_len = 10;
    let url = Url {
        id: thread_rng()
            .sample_iter(&Alphanumeric)
            .take(url_len)
            .map(char::from)
            .collect(),
        full_url: payload.url.clone(),
    };

    println!("Shortening URL {} to {}", payload.url, url.id);

    // save the url in memory for now
    urls.push(url.clone());

    let resp = Response::builder();

    if let Some(html) = pages.get("shorten_url") {
        if let Ok(rendered_html) = render_shorten_url_template(html, url) {
            return resp
                .header("content-type", "text/html")
                .status(StatusCode::OK)
                .body(Body::new(rendered_html.clone()))
                .unwrap();
        }
    };

    // shouldn't hit here but if we do, good luck
    resp.status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::empty())
        .unwrap()
}

fn render_shorten_url_template(html: &String, url: Url) -> Result<String, Error> {
    let mut tt = TinyTemplate::new();

    tt.add_template("html", &html)?;

    Ok(tt.render("html", &url)?)
}

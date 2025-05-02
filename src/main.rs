use axum::{
    extract::Json,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;

#[derive(Deserialize)]
struct Prompt {
    text: String,
}

#[derive(Serialize)]
struct Cleaned {
    text: String,
    tokens_redacted: u32,
}

async fn scrub(Json(payload): Json<Prompt>) -> Json<Cleaned> {
    // very dumb redaction: replace "jane" or "doe"
    let (clean, hits) = payload
        .text
        .split_whitespace()
        .fold((String::new(), 0u32), |(mut acc, hits), tok| {
            if tok.eq_ignore_ascii_case("jane") || tok.eq_ignore_ascii_case("doe") {
                acc.push_str("[REDACTED] ");
                (acc, hits + 1)
            } else {
                acc.push_str(tok);
                acc.push(' ');
                (acc, hits)
            }
        });

    Json(Cleaned {
        text: clean.trim().to_owned(),
        tokens_redacted: hits,
    })
}

#[tokio::main]
async fn main() {
    // build our application
    let app = Router::new()
        .route("/v1/prompt", post(scrub))
        .layer(CorsLayer::permissive());

    // bind to an address
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9000")
        .await
        .expect("failed to bind address");

    println!(
        "🚀  Guardrails listening on http://{}",
        listener.local_addr().unwrap()
    );

    // serve!
    axum::serve(listener, app).await.unwrap();
}

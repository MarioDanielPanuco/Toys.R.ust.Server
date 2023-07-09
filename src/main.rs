use std::{
    fs,
    io::{prelude::*, BufReader, Error},
    net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream},
    path::PathBuf,
    thread,
};

use lib_email_services::{handle_contact_form, FormData};

mod model;

use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service, post, MethodRouter},
    Json, Router,
};

use serde::{Deserialize, Serialize};
use tokio::net::unix::uid_t;
// use serde_json::json;

use tower_http::{
    services::{ServeDir, ServeFile},
    trace::{DefaultMakeSpan, TraceLayer},
};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

enum Protocol {
    Http,
    Https,
}

struct HTTPVersion {
    protocol: Protocol,
}

#[tokio::main]
async fn main() {
    let ip = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));

    let h: HTTPVersion = HTTPVersion {
        protocol: Protocol::Https,
    };
    let port = match h.protocol {
        Protocol::Http => 80,
        Protocol::Https => 443,
    };

    let addr = SocketAddr::from((ip, port));

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");

    let app = Router::new()
        .route(
            "/contact_form",
            post(|form: axum::Json<FormData>| async move { handle_contact_form(form).await }),
        )
        // .route("/CSE-20/notebooks", get())
        .fallback_service(
            get_service(ServeDir::new(assets_dir).append_index_html_on_directories(true))
                .handle_error(|error: Error| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        tracing::error!("Unhandled internal error: {}", error),
                    )
                }),
        )
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        );

    // Tracing Output
    tracing::info!("Listening on: {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn character(Path(id_num): Path<i64>) -> impl IntoResponse {
    let _character_ = Json(model::character::Character {
        id: id_num,
        health: 32,
        deadeye: 33,
    });

    (StatusCode::CREATED, _character_)
}

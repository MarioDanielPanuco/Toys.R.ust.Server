use std::{
    fs,
    io::{BufReader, Error, prelude::*},
    net::{
        IpAddr,
        Ipv4Addr,
        SocketAddr,
        TcpListener,
        TcpStream,
    },
    path::PathBuf,
    thread,
};

use axum::{
    extract::Path,
    http::StatusCode,
    Json,
    response::IntoResponse,
    Router,
    routing::{
        get,
        get_service,
        post,
        MethodRouter,
    },
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::{DefaultMakeSpan, TraceLayer},
};
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

mod model;

#[warn(dead_code)]
struct Website {
    app: Router,
}

type Route = (String, MethodRouter);

impl Website {
    pub fn new(
        port: u16,
        ipv4: Ipv4Addr,
        Routes: Vec<Route>,
    ) -> Website {
        let addr = SocketAddr::from((IpAddr::V4(ipv4), port));
        let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");

        let app = Router::new()
            .route("/characters:id", get(character))
            .fallback_service(
                get_service(ServeDir::new(assets_dir)
                    .append_index_html_on_directories(true))
                    .handle_error(|error: Error| async move {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {}", error),
                        )
                    }),
            ).layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        );
        Self {
            app,
        }
    }
}

#[tokio::main]
async fn main() {
    let ip = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let port = 7878;
    let addr = SocketAddr::from((ip, port));

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets");

    let app = Router::new()
        .route("/characters:id", get(character))
        .fallback_service(
            get_service(ServeDir::new(assets_dir)
                .append_index_html_on_directories(true))
                .handle_error(|error: Error| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {}", error),
                    )
                }),
        ).layer(
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
        id:         id_num,
        health:     32,
        deadeye:    33,
    });

    (StatusCode::CREATED, _character_)
}


/*
pub fn get_current_date() -> Date {
    let current_date = Date {
        current_utc.day(),
        current_utc.month(),
        current_utc.year(),
    };

    current_date
}
*/


#[cfg(test)]
mod test {
    mod character {
        #![warn(dead_code)]

        fn test() {
            todo!()
        }
    }
}


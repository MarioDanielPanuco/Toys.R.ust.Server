

use axum::{
    http::{HeaderMap, HeaderValue, StatusCode},
    response::IntoResponse,
};
use reqwest::Url;
use std::{io::Cursor, process::Command};
use tower_http::{
    add_extension::AddExtensionLayer as HttpAddExtensionLayer, set_header::SetResponseHeaderLayer,
};

async fn notebooks(
    (user, repo, path): (String, String, String),
) -> Result<impl IntoResponse, StatusCode> {
    // Build the URL for the notebook file on GitHub
    let url = Url::parse(&format!(
        "https://raw.githubusercontent.com/{}/{}/main/{}",
        user, repo, path
    ))
    .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Download the notebook file contents using the GitHub API
    let notebook_contents = reqwest::get(url)
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?
        .text()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    // Convert the notebook contents to HTML using nbconvert
    let html_contents = Command::new("jupyter")
        .arg("nbconvert")
        .arg("--to")
        .arg("html")
        .stdin(Cursor::new(notebook_contents))
        .output()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Create a response containing the HTML contents
    let mut headers = HeaderMap::new();
    headers.insert(
        "content-type",
        HeaderValue::from_static("text/html; charset=utf-8"),
    );
    let response = String::from_utf8_lossy(&html_contents.stdout).into_response();
    let response = SetResponseHeaderLayer::new(headers).layer(response);
    Ok(response)
}


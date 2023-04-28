use axum::{http::{StatusCode}, http};
use reqwest::Url;
use std::{process::Command};
use std::io::Write;
use axum::body::Body;
use tempfile::NamedTempFile;

async fn notebooks(
    (user, repo, path): (String, String, String),
) -> Result<http::Response<Body> , StatusCode> {
    // Build the URL for the notebook file on GitHub
    let url = Url::parse(&format!(
        "https://raw.githubusercontent.com/{}/{}/master/{}",
        user, repo, path
    ))
    .map_err(|_| StatusCode::BAD_REQUEST)?;
    println!("url: {}\n", url.as_str());

    // Download the notebook file contents using the GitHub API
    let notebook_contents = reqwest::get(url)
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?
        .text()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    println!("Notebooks:\n{}", notebook_contents);
    // Write the notebook contents to a temporary file
    let mut temp_file = NamedTempFile::new().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    temp_file.write_all(notebook_contents.as_bytes()).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Convert the notebook contents to HTML using nbconvert
    let html_contents = Command::new("jupyter")
        .arg("nbconvert")
        .arg("--to")
        .arg("html")
        .arg("--output-dir")
        .arg(temp_file.path().parent().unwrap()) // Use the temp file's parent directory as output directory
        .arg(temp_file.path()) // Provide the temp file path as input
        .output()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

   // Create a response containing the HTML contents
   //  let response = Html(String::from_utf8_lossy(&html_contents.stdout).into_owned());

    // Add custom headers to the response
    // let mut headers = HeaderMap::new()
    //     .insert(
    //     "content-type",
    //     HeaderValue::from_static("text/html; charset=utf-8"),
    // ).unwrap();

    let response = http::Response::builder()
        .status(StatusCode::OK)
    .header("content-type", "text/html; charset=utf-8");

    let body = Body::from(html_contents.stdout);

    // Set the body for the response
    let response = response.body(body).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Return the response along with the headers
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::notebooks;
    use axum::http::StatusCode;
    use axum::body::{HttpBody};


    // Test a valid notebook URL
    #[tokio::test]
    async fn test_valid_notebook() {
        let user = "MarioDanielPanuco".to_string();
        let repo = "CSE-20-SI".to_string();
        let path = "notebooks/lec8.ipynb".to_string();

        let result = (notebooks((user, repo, path))).await;

        match result {
            Ok(mut response) => {
                assert_eq!(response.status(), StatusCode::OK);
                let mut body_bytes = Vec::new();
                while let Some(chunk) = response.body_mut().data().await {
                    let chunk = chunk.expect("Failed to read data");
                    body_bytes.extend(chunk);
                }
                let body_str = String::from_utf8_lossy(&body_bytes);
                println!("{:?}", body_str);
                assert!(body_str.contains("<!DOCTYPE html>"));
                assert!(body_str.contains("<html>"));
            }
            Err(status) => panic!("Unexpected error: {}", status),
        }
    }

    // Test an invalid notebook URL
    #[tokio::test]
    async fn test_invalid_notebook() {
        let user = "invalid_user".to_string();
        let repo = "invalid_repo".to_string();
        let path = "path/to/invalid/notebook.ipynb".to_string();

        let result = notebooks((user, repo, path)).await;

        assert!(matches!(result, Err(StatusCode::BAD_GATEWAY)));
    }

    // Test an invalid GitHub user or repository name
    #[tokio::test]
    async fn test_invalid_github_user_or_repo() {
        let user = "invalid_user".to_string();
        let repo = "invalid_repo".to_string();
        let path = "invalid_path".to_string();

        let result = notebooks((user, repo, path)).await;

        assert!(matches!(result, Err(StatusCode::BAD_REQUEST)));
    }
}

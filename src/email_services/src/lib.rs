use anyhow::Result;
use axum::{http::StatusCode, response::IntoResponse, Json};
use lettre::{
    message::{Mailbox, Message},
    transport::smtp::{
        authentication::{Credentials, Mechanism},
        SmtpTransport,
    },
    Transport,
};
use serde::Deserialize;

// Add this struct to represent the form data
#[derive(Deserialize, Debug)]
pub struct FormData {
    sender_email: String,
    subject_title: String,
    message_body: String,
}

// Add this function to handle the form submission
pub async fn handle_contact_form(form: Json<FormData>) -> impl IntoResponse {
    println!("Form data received: {:?}", form.0);
    // let smtp_username = std::env::var("").unwrap_or_default();
    let smtp_username = "".to_string();
    let smtp_password = "".to_string();
    let smtp_host = "localhost".to_string();
    let smtp_port: u16 = 1025;

    let result = tokio::task::spawn_blocking(move || {
        send_email(
            &smtp_username,
            &smtp_password,
            &smtp_host,
            smtp_port,
            form.0,
        )
    })
    .await;

    match result {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

fn send_email(
    smtp_username: &str,
    smtp_password: &str,
    smtp_host: &str,
    smtp_port: u16,
    form: FormData,
) -> Result<(), anyhow::Error> {
    let from_address = Mailbox::new(None, form.sender_email.parse().map_err(anyhow::Error::new)?);
    let to_address = Mailbox::new(
        None,
        "mpanuco@ucsc.edu".parse().map_err(anyhow::Error::new)?,
    );

    let email = Message::builder()
        .from(from_address)
        .to(to_address)
        .subject(form.subject_title)
        .body(form.message_body)?;

    let credentials = Credentials::new(smtp_username.to_string(), smtp_password.to_string());

    let transport = SmtpTransport::relay(smtp_host)
        .map_err(anyhow::Error::new)?
        .port(smtp_port)
        .authentication(vec![Mechanism::Plain])
        .credentials(credentials)
        .build();

    println!("Sending email...");

    match transport.send(&email) {
        Ok(_) => {
            println!("Email sent successfully");
            Ok(())
        }
        Err(e) => {
            eprintln!("Error sending email: {:?}", e);
            Err(anyhow::Error::from(e))
        }
    }
}

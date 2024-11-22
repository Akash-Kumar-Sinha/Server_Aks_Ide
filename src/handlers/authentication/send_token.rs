use actix_web::{web, HttpResponse, Responder};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub async fn send_token(req: web::Json<Value>) -> impl Responder {
    let smtp_username = env::var("EMAIL").expect("Email is not set.");
    let smtp_password = env::var("EMAIL_PASSWORD").expect("Email Password is not set.");

    let server_url = env::var("SERVER_URL").expect("SERVER_URL is not set.");

    let env_jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET is not set");

    if let Some(email) = req.get("email").and_then(Value::as_str) {
        let expiration = (Utc::now() + Duration::hours(1)).timestamp() as usize;
        let claims = Claims {
            sub: email.to_string(),
            exp: expiration,
        };

        let token = encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(env_jwt_secret.as_ref()),
        );

        match token {
            Ok(token) => {
                let email_body = format!(
                    "Hello,\n\nPlease verify your email by clicking the link below:\n\n{server_url}/auth/verify/{token}\n\nThank you!",
                );

                let mail = Message::builder()
                    .from(smtp_username.parse().expect(
                        "Failed to parse smtp_username address. Please check the EMAIL environment variable",
                    ))
                    .to(email.parse().expect("Failed to parse email address"))
                    .subject("Aks Ide - Verify your email")
                    .header(ContentType::TEXT_PLAIN)
                    .body(String::from(email_body))
                    .expect("Failed to build email");

                let creds = Credentials::new(smtp_username.to_owned(), smtp_password.to_owned());

                let mailer = SmtpTransport::relay("smtp.gmail.com")
                    .expect(
                        "Failed to connect to the SMTP server. Please check the EMAIL and EMAIL_PASSWORD environment variables",
                    )
                    .credentials(creds)
                    .build();

                match mailer.send(&mail) {
                    Ok(_) => HttpResponse::Ok().json(serde_json::json!({
                        "message": "Email sent successfully",
                        "Route": "SENT"
                    })),
                    Err(_e) => HttpResponse::InternalServerError().json(serde_json::json!({
                        "message": "Email failed to send",
                        "Route": "FAILED"
                    })),
                }
            }
            Err(_e) => HttpResponse::InternalServerError().json(serde_json::json!({
                "message": "Failed to generate token",
                "Route": "FAILED",
            })),
        }
    } else {
        HttpResponse::BadRequest().json(serde_json::json!({
            "message": "Missing or invalid 'email' field",
            "Route": "FAILED"
        }))
    }
}

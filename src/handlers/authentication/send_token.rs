use crate::database::{
    messages::{
        CreateEmailVerification, FetchEmailVerificationByEmail, UpdateEmailVerificationToken,
    },
    AppState, DbActor,
};

use actix::Addr;
use actix_web::{
    web::{self, Data},
    HttpResponse, Responder,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub async fn send_token(req: web::Json<Value>, state: Data<AppState>) -> impl Responder {
    let db: Addr<DbActor> = state.db.clone();
    let env_jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET is not set");

    if let Some(email) = req.get("email").and_then(Value::as_str) {
        let expiration = (Utc::now() + Duration::minutes(10)).timestamp() as usize;
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
                match db
                    .send(FetchEmailVerificationByEmail {
                        email: email.to_string(),
                    })
                    .await
                {
                    Ok(Ok(email_verifications)) => {
                        if email_verifications.is_empty() {
                            match send_email(&token, email).await {
                                Ok(_) => {
                                    match db
                                        .send(CreateEmailVerification {
                                            email: email.to_string(),
                                            verification_token: token.clone(),
                                        })
                                        .await
                                    {
                                        Ok(Ok(_)) => {
                                            return HttpResponse::Ok().json(serde_json::json!({
                                                "token": token,
                                                "message": "Email sent successfully",
                                                "Route": "SENT"
                                            }));
                                        }
                                        _ => {
                                            return HttpResponse::InternalServerError().json(
                                                serde_json::json!({
                                                    "message": "Failed to save token",
                                                    "Route": "FAILED"
                                                }),
                                            );
                                        }
                                    }
                                }
                                Err(e) => {
                                    return HttpResponse::InternalServerError().json(
                                        serde_json::json!({
                                            "message": format!("Email sending failed: {}", e),
                                            "Route": "FAILED"
                                        }),
                                    );
                                }
                            }
                        } else {
                            let is_email_verified =
                                email_verifications.iter().any(|email_verification| {
                                    email_verification.verified.unwrap_or(false)
                                });

                            if is_email_verified {
                                return HttpResponse::BadRequest().json(serde_json::json!({
                                    "message": "Email already verified",
                                    "Route": "PASSWORD"
                                }));
                            } else {
                                match send_email(&token, email).await {
                                    Ok(_) => {
                                        match db
                                            .send(UpdateEmailVerificationToken {
                                                email: email.to_string(),
                                                verification_token: token.clone(),
                                            })
                                            .await
                                        {
                                            Ok(Ok(_)) => {
                                                return HttpResponse::Ok().json(
                                                    serde_json::json!({
                                                        "token": token,
                                                        "message": "Email sent successfully",
                                                        "Route": "SENT"
                                                    }),
                                                );
                                            }
                                            _ => {
                                                return HttpResponse::InternalServerError().json(
                                                    serde_json::json!({
                                                        "message": "Failed to save token",
                                                        "Route": "FAILED"
                                                    }),
                                                );
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        return HttpResponse::InternalServerError().json(
                                            serde_json::json!({
                                                "message": format!("Email sending failed: {}", e),
                                                "Route": "FAILED"
                                            }),
                                        );
                                    }
                                }
                            }
                        }
                    }
                    Ok(Err(_)) => HttpResponse::InternalServerError().json(serde_json::json!({
                        "message": "Database query failed",
                        "Route": "FAILED"
                    })),
                    Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
                        "message": "Internal Server Error",
                        "Route": "FAILED"
                    })),
                }
            }
            Err(_) => {
                return HttpResponse::InternalServerError().json(serde_json::json!({
                    "message": "Failed to generate token",
                    "Route": "FAILED",
                }));
            }
        }
    } else {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "message": "Missing or invalid 'email' field",
            "Route": "FAILED"
        }));
    }
}

pub async fn send_email(token: &str, recipient_email: &str) -> Result<bool, String> {
    let smtp_username = std::env::var("EMAIL").map_err(|_| "EMAIL is not set.".to_string())?;
    let smtp_password =
        std::env::var("EMAIL_PASSWORD").map_err(|_| "EMAIL_PASSWORD is not set.".to_string())?;
    let server_url =
        std::env::var("SERVER_URL").map_err(|_| "SERVER_URL is not set.".to_string())?;

    let email_body = format!(
        "Hello,\n\nPlease verify your email by clicking the link below:\n\n{}/auth/verify/{}\n\nThank you!",
        server_url, token
    );

    let creds = Credentials::new(smtp_username.clone(), smtp_password);

    let mail = Message::builder()
        .from(smtp_username.parse().map_err(|_| {
            "Failed to parse smtp_username address. Please check the EMAIL environment variable"
                .to_string()
        })?)
        .to(recipient_email
            .parse()
            .map_err(|_| "Failed to parse recipient email address".to_string())?)
        .subject("Aks Ide - Verify your email")
        .header(ContentType::TEXT_PLAIN)
        .body(email_body)
        .map_err(|_| "Failed to build email message".to_string())?;

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .map_err(|_| "Failed to connect to the SMTP server. Please check the EMAIL and EMAIL_PASSWORD environment variables".to_string())?
        .credentials(creds)
        .build();

    match mailer.send(&mail) {
        Ok(_) => Ok(true),
        Err(e) => Err(format!("Failed to send email: {}", e)),
    }
}

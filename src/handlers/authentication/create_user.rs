use actix::Addr;
use actix_web::{
    web::{self, Data},
    HttpResponse, Responder,
};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::database::{
    messages::{CreateUsers, FetchEmailVerificationByEmail},
    AppState, DbActor,
};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    pub email: String,
    pub exp: usize,
    pub provider: String,
    pub provider_id: String,
}

pub async fn create_user(req: web::Json<Value>, state: Data<AppState>) -> impl Responder {
    println!("Create user");
    let db: Addr<DbActor> = state.db.clone();
    let email = req
        .get("email")
        .and_then(Value::as_str)
        .unwrap()
        .to_string();
    let password = req
        .get("password")
        .and_then(Value::as_str)
        .unwrap()
        .to_string();

    match db
        .send(FetchEmailVerificationByEmail {
            email: email.clone(),
        })
        .await
    {
        Ok(Ok(email_verifications)) => {
            if email_verifications.is_empty() {
                return HttpResponse::NotFound().json(serde_json::json!({
                    "message": "Email not found",
                    "Route": "FAILURE"
                }));
            } else {
                let is_email_verified = email_verifications
                    .iter()
                    .any(|email_verification| email_verification.verified.unwrap_or(false));

                if is_email_verified {
                    let hashed_password = bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap();

                    match db
                        .send(CreateUsers {
                            email: email.clone(),
                            name: None,
                            email_verified: true,
                            hashed_password,
                            avatar: None,
                            provider: "EMAIL".to_string(),
                            provider_id: "email".to_string(),
                            docker_container_id: None,
                        })
                        .await
                    {
                        Ok(_) => {
                            let env_jwt_secret =
                                std::env::var("JWT_SECRET").expect("JWT_SECRET is not set");

                            let claims = Claims {
                                email: email.clone(),
                                exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp()
                                    as usize,
                                provider: "EMAIL".to_string(),
                                provider_id: "email".to_string(),
                            };

                            let auth_token = encode(
                                &Header::new(Algorithm::HS256),
                                &claims,
                                &EncodingKey::from_secret(env_jwt_secret.as_ref()),
                            ).expect("Error encoding token");
                            
                            return HttpResponse::Ok().json(serde_json::json!({
                                "message": "User created",
                                "Route": "CREATED",
                                "token": auth_token
                            }));
                        }
                        Err(e) => {
                            println!("Error creating user: {:?}", e);
                            return HttpResponse::InternalServerError().json(serde_json::json!({
                                "message": "Error creating user",
                                "Route": "FAILED"
                            }));
                        }
                    }
                } else {
                    return HttpResponse::BadRequest().json(serde_json::json!({
                        "message": "Email found",
                        "Route": "FAILURE"
                    }));
                }
            }
        }
        Ok(Err(e)) => {
            println!("Error: {:?}", e);
            return HttpResponse::NotFound().json(serde_json::json!({
                "message": "Internal Server Error",
                "Route": "FAILURE"
            }));
        }
        Err(e) => {
            println!("Error: {:?}", e);
            return HttpResponse::NotFound().json(serde_json::json!({
                "message": "Internal Server Error",
                "Route": "FAILURE"
            }));
        }
    }
}

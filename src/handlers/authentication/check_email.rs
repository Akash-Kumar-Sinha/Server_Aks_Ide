use actix::Addr;
use actix_web::{
    web::{self, Data},
    HttpResponse, Responder,
};
use serde_json::Value;

use crate::database::{messages::FetchEmailVerificationByEmail, AppState, DbActor};

pub async fn check_email(req: web::Json<Value>, state: Data<AppState>) -> impl Responder {
    println!("Check email");
    let db: Addr<DbActor> = state.db.clone();
    match db
        .send(FetchEmailVerificationByEmail {
            email: req
                .get("email")
                .and_then(Value::as_str)
                .expect("Email is required")
                .to_string(),
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
                    return HttpResponse::Ok().json(serde_json::json!({
                        "message": "Email verified",
                        "Route": "SUCCESS"
                    }));
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

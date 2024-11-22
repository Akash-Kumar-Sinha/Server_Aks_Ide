use actix_web::{get, post, web, Responder};
use serde_json::Value;

use crate::handlers;

#[post("/send_token")]
async fn send_token(req: web::Json<Value>) -> impl Responder {
    handlers::authentication::send_token::send_token(req).await
}
#[get("/verify/{token}")]
async fn verify_email(path: web::Path<String>) -> impl Responder {
    handlers::authentication::verify_email::verify_email(path).await
}

pub fn auth_route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(verify_email)
            .service(send_token),
    );
}

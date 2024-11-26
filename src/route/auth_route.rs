use actix_web::{get, post, web::{self, Data}, HttpResponse, Responder};
use serde_json::Value;
use crate::database::AppState;

use crate::handlers;

#[post("/send_token")]
async fn send_token(req: web::Json<Value>,state: Data<AppState>) -> impl Responder {
    handlers::authentication::send_token::send_token(req, state).await
}
#[get("/verify/{token}")]
async fn verify_email(path: web::Path<String>, state: Data<AppState>) -> impl Responder {
    handlers::authentication::verify_email::verify_email(path, state).await
}

#[post("/check_email")]
async fn check_email(req: web::Json<Value>, state: Data<AppState>) -> impl Responder {
    handlers::authentication::check_email::check_email(req, state).await
}

#[post("/create_user")]
async fn create_user(req: web::Json<Value>, state: Data<AppState>) -> impl Responder {
    handlers::authentication::create_user::create_user(req, state).await
}


#[post("/google")]
async fn google_login() -> impl Responder {
    HttpResponse::Ok().body("Google login")
    // handlers::authentication::google::google_login(state).await
}
#[post("/google/callback")]
async fn google_callback() -> impl Responder {
    HttpResponse::Ok().body("Google callback")
    // handlers::authentication::google::google_callback(state).await
}


pub fn auth_route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(verify_email)
            .service(send_token)
            .service(google_login)
            .service(google_callback)
            .service(check_email)
            .service(create_user)
    );
}

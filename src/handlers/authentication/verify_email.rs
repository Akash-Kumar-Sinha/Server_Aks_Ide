use actix_web::{web, HttpResponse, Responder};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use std::env;

use super::send_token::Claims;

pub async fn verify_email(path: web::Path<String>) -> impl Responder {
    let token: String = path.into_inner().parse().expect(
        "Failed to parse token from path. Make sure the token is a valid string and try again.",
    );
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let decode_token = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(&jwt_secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    );

    match decode_token {
        Ok(token_data) => {
            HttpResponse::Ok().body(format!("Email verified: {:#?}", token_data.claims))
        }
        Err(e) => HttpResponse::Unauthorized().body(format!("Token error: {}", e)),
    }
}

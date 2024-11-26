use crate::database::{AppState, DbActor, messages::UpdateEmailVerification};
use actix::Addr;
use actix_web::{
    web::{self, Data},
    HttpResponse, Responder,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

use super::send_token::Claims;

pub async fn verify_email(path: web::Path<String>, state: Data<AppState>) -> impl Responder {
    let db: Addr<DbActor> = state.db.clone();

    let token: String = path.into_inner().parse().expect(
        "Failed to parse token from path. Make sure the token is a valid string and try again.",
    );
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let decode_token = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(&jwt_secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    );

    match decode_token {
        Ok(token_data) => {
            match db
                .send(UpdateEmailVerification {
                    email: token_data.claims.sub.clone(),
                    verified: true,
                })
                .await
            {
                Ok(Ok(_)) => HttpResponse::Ok().body("Email Verified successfully"),
                Ok(Err(_)) => HttpResponse::NotFound().json("Unable to verify email"),
                _ => HttpResponse::InternalServerError().json("Internal Server Error"),
            }
        }
        Err(e) => HttpResponse::Unauthorized().body(format!("Token error: {}", e)),
    }
}

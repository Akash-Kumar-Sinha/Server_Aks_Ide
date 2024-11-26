use crate::schema::{email_verifications, users};
use diesel::prelude::Insertable;
use serde::Serialize;

#[derive(Insertable, Serialize)]
#[table_name = "email_verifications"]
pub struct NewEmailVerification {
    pub email: String,
    pub verification_token: String,
}

#[derive(Insertable, Serialize)]
#[table_name = "users"]
pub struct NewUsers {
    pub email: String,
    pub name: Option<String>,
    pub email_verified: bool,
    pub hashed_password: String,
    pub avatar: Option<String>,
    pub provider: String,
    pub provider_id: String,
    pub docker_container_id: Option<String>,
}

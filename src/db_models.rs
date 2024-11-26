// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]


use chrono::NaiveDateTime;
use diesel::prelude::Queryable;
use serde::Serialize;
#[derive(Queryable, Debug, Serialize)]
pub struct EmailVerification {
    pub id: i32,
    pub email: String,
    pub verification_token: String,
    pub verified: Option<bool>,
}

#[derive(Queryable, Debug, Serialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub name: Option<String>,
    pub email_verified: Option<bool>,
    pub hashed_password: Option<String>,
    pub avatar: Option<String>,
    pub provider: String,
    pub provider_id: Option<String>,
    pub docker_container_id: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}


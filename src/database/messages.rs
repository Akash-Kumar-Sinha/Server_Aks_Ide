use crate::db_models::{EmailVerification,User};
use actix::Message;
use diesel::QueryResult;

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<EmailVerification>>")]
pub struct FetchEmailVerificationByEmail {
    pub email: String,
}

#[derive(Message)]
#[rtype(result = "QueryResult<EmailVerification>")]
pub struct CreateEmailVerification {
    pub email: String,
    pub verification_token: String,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<EmailVerification>>")]
pub struct UpdateEmailVerificationToken {
    pub email: String,
    pub verification_token: String,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<EmailVerification>>")]
pub struct UpdateEmailVerification{
    pub email: String,
    pub verified: bool,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<User>>")]
pub struct CreateUsers {
    pub email: String,
    pub name: Option<String>,
    pub email_verified: bool,
    pub hashed_password: String,
    pub avatar: Option<String>,
    pub provider: String,
    pub provider_id: String,
    pub docker_container_id: Option<String>,
}
use crate::database::messages::{
    CreateEmailVerification, CreateUsers, FetchEmailVerificationByEmail, UpdateEmailVerification,
    UpdateEmailVerificationToken,
};
use crate::database::models::{NewEmailVerification, NewUsers};
use crate::database::DbActor;
use crate::db_models::{EmailVerification, User};
use crate::schema::email_verifications::{dsl::*, email, id as email_verification_id};
use crate::schema::users::dsl::*;
use actix::Handler;
use diesel::{self, prelude::*};

impl Handler<FetchEmailVerificationByEmail> for DbActor {
    type Result = QueryResult<Vec<EmailVerification>>;

    fn handle(
        &mut self,
        msg: FetchEmailVerificationByEmail,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        let mut conn = self
            .0
            .get()
            .expect("fetch_email_verification: Couldn't get db connection from pool");

        email_verifications
            .filter(email.eq(msg.email))
            .get_results::<EmailVerification>(&mut conn)
    }
}

impl Handler<UpdateEmailVerificationToken> for DbActor {
    type Result = QueryResult<Vec<EmailVerification>>;

    fn handle(
        &mut self,
        msg: UpdateEmailVerificationToken,
        _ctx: &mut Self::Context,
    ) -> Self::Result {
        let mut conn = self
            .0
            .get()
            .expect("update_email_verification: Couldn't get db connection from pool");

        diesel::update(email_verifications.filter(email.eq(msg.email)))
            .set(verification_token.eq(msg.verification_token))
            .get_results::<EmailVerification>(&mut conn)
    }
}

impl Handler<UpdateEmailVerification> for DbActor {
    type Result = QueryResult<Vec<EmailVerification>>;

    fn handle(&mut self, msg: UpdateEmailVerification, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self
            .0
            .get()
            .expect("update_email_verification: Couldn't get db connection from pool");

        diesel::update(email_verifications.filter(email.eq(msg.email)))
            .set(verified.eq(msg.verified))
            .get_results::<EmailVerification>(&mut conn)
    }
}

impl Handler<CreateEmailVerification> for DbActor {
    type Result = QueryResult<EmailVerification>;

    fn handle(&mut self, msg: CreateEmailVerification, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self
            .0
            .get()
            .expect("create_email_verification: Couldn't get db connection from pool");

        let new_email_verification = NewEmailVerification {
            email: msg.email,
            verification_token: msg.verification_token,
        };

        diesel::insert_into(email_verifications)
            .values(&new_email_verification)
            .returning((email_verification_id, email, verification_token, verified))
            .get_result::<EmailVerification>(&mut conn)
    }
}

impl Handler<CreateUsers> for DbActor {
    type Result = QueryResult<Vec<User>>;

    fn handle(&mut self, msg: CreateUsers, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self
            .0
            .get()
            .expect("create_email_verification: Couldn't get db connection from pool");

        let new_user = NewUsers {
            email: msg.email,
            name: msg.name,
            email_verified: msg.email_verified,
            hashed_password: msg.hashed_password,
            avatar: msg.avatar,
            provider: msg.provider,
            provider_id: msg.provider_id,
            docker_container_id: msg.docker_container_id,
        };

        diesel::insert_into(users)
            .values(&new_user)
            .get_results::<User>(&mut conn)
    }
}

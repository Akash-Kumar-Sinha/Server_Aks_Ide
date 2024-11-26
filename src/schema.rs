// @generated automatically by Diesel CLI.

diesel::table! {
    email_verifications (id) {
        id -> Int4,
        email -> Varchar,
        verification_token -> Text,
        verified -> Nullable<Bool>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        name -> Nullable<Varchar>,
        email_verified -> Nullable<Bool>,
        hashed_password -> Nullable<Varchar>,
        avatar -> Nullable<Varchar>,
        provider -> Varchar,
        provider_id -> Nullable<Varchar>,
        docker_container_id -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    email_verifications,
    users,
);

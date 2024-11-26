-- Your SQL goes here
CREATE TABLE email_verifications (
    id SERIAL PRIMARY KEY,
    email VARCHAR NOT NULL UNIQUE,
    verification_token TEXT NOT NULL,
    verified BOOLEAN DEFAULT FALSE
);

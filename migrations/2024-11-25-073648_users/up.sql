-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR NOT NULL UNIQUE,
    name VARCHAR,
    email_verified BOOLEAN DEFAULT FALSE,
    hashed_password VARCHAR,
    avatar VARCHAR,
    provider VARCHAR NOT NULL,
    provider_id VARCHAR UNIQUE,
    docker_container_id VARCHAR,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

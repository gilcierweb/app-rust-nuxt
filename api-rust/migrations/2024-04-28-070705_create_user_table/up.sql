-- Your SQL goes here
-- CREATE EXTENSION IF NOT EXISTS "uuid-ossp"; -- old versions postgresql


CREATE TABLE IF NOT EXISTS users
(
    id          UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    email       VARCHAR(255) NOT NULL UNIQUE,
    encrypted_password    VARCHAR(255) NOT NULL,
    reset_password_token     VARCHAR(255) UNIQUE,
    reset_password_sent_at   TIMESTAMP,
    remember_created_at     TIMESTAMP,
    sign_in_count     NUMERIC,
    current_sign_in_at     TIMESTAMP,
    last_sign_in_at     TIMESTAMP,
    current_sign_in_ip   INET,
    last_sign_in_ip      INET,    
    status      BOOLEAN DEFAULT TRUE,
    created_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
 
);

CREATE INDEX users_email_idx ON users (email);
CREATE INDEX users_reset_password_token_idx ON users (reset_password_token);
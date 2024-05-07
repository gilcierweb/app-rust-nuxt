-- Your SQL goes here
CREATE TABLE IF NOT EXISTS profiles
(
    id          UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    first_name VARCHAR(255) NULL,
    last_name VARCHAR(255) NULL,
    full_name VARCHAR(255) NULL,
    nickname VARCHAR(255) NULL,
    bio TEXT NULL,
    birthday DATE NULL,
    avatar VARCHAR(255) NULL,
    phone BIGINT NULL,
    user_id UUID NOT NULL UNIQUE,
   
    created_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS users_detail_id_user_id_idx ON profiles (id, user_id);
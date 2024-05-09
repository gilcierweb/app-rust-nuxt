-- Your SQL goes here
CREATE TABLE IF NOT EXISTS posts
(
    id          UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    title       VARCHAR(255) NOT NULL,
    content TEXT,
    status BOOLEAN,
    created_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)
-- Your SQL goes here
CREATE TABLE posts
(
    id          VARCHAR(255) PRIMARY KEY,
    title       VARCHAR(255) NOT NULL,
    content TEXT,
    status BOOLEAN,
    created_at  TIMESTAMP,
    updated_at  TIMESTAMP
)
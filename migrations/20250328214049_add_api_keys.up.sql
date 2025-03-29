-- Add up migration script here
CREATE TABLE IF NOT EXISTS api_keys (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS identity,
    user_id BIGINT NOT NULL REFERENCES users (ID),
    secret TEXT NOT NULL UNIQUE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    last_used_at TIMESTAMP
);

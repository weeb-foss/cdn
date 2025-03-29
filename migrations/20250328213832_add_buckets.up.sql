-- Add up migration script here
CREATE TABLE IF NOT EXISTS buckets (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS identity,
    name TEXT NOT NULL UNIQUE,
    owner_id BIGINT NOT NULL REFERENCES users (ID),
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

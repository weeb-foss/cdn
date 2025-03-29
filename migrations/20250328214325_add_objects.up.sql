CREATE TABLE IF NOT EXISTS objects (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS identity,
    bucket_id BIGINT NOT NULL REFERENCES buckets (ID),
    path TEXT NOT NULL,
    size BIGINT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    last_modified_at TIMESTAMP NOT NULL DEFAULT NOW(),

    UNIQUE (bucket_id, path)
);

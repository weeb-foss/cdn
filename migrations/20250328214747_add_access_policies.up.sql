CREATE TABLE IF NOT EXISTS access_policies (
    id BIGINT PRIMARY KEY GENERATED ALWAYS AS identity,
    bucket_id BIGINT NOT NULL REFERENCES buckets (ID),
    user_id BIGINT NOT NULL REFERENCES users (ID),
    permission TEXT NOT NULL CHECK (permission IN ('READ', 'WRITE', 'ADMIN')),
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

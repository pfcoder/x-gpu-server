-- Add migration script here
CREATE TABLE IF NOT EXISTS hosts (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    status VARCHAR NOT NULL,
    cpu_info JSON,
    gpu_info JSON,
    ram INTEGER,
    hd INTEGER,
    ip INET,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

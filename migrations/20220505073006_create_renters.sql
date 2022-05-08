-- Add migration script here
CREATE TABLE IF NOT EXISTS renters (
    id UUID DEFAULT gen_random_uuid() PRIMARY KEY,
    user_id UUID NOT NULL,
    status VARCHAR NOT NULL,
    levle INT,
    points INT,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

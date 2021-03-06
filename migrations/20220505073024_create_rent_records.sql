-- Add migration script here
CREATE TABLE IF NOT EXISTS rent_records (
    id BIGINT  GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    renter UUID NOT NULL,
    host UUID NOT NULL,
    price INT NOT NULL,
    rent_start TIMESTAMPTZ NOT NULL,
    rent_end TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

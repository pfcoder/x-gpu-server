-- Add migration script here
ALTER TABLE hosts ALTER COLUMN status TYPE INTEGER USING (trim(status)::integer);
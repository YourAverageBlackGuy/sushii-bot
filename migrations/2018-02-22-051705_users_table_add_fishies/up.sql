-- Your SQL goes here
ALTER TABLE users ADD COLUMN fishies BIGINT NOT NULL DEFAULT 0, ADD COLUMN last_fishies TIMESTAMP;
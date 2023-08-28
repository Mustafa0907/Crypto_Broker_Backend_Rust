-- Your SQL goes here
CREATE TABLE realmoney (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    currency VARCHAR(255) NOT NULL,
    balance DOUBLE PRECISION NOT NULL,
    created_on TIMESTAMP WITH TIME ZONE,
    modified_on TIMESTAMP WITH TIME ZONE
);
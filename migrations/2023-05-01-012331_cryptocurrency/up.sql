-- Your SQL goes here
CREATE TABLE Crypto (
    id SERIAL PRIMARY KEY,
    cname VARCHAR(255) NOT NULL UNIQUE,
    symbol VARCHAR(255) NOT NULL UNIQUE,
    created_on TIMESTAMP WITH TIME ZONE,
    modified_on TIMESTAMP WITH TIME ZONE
);
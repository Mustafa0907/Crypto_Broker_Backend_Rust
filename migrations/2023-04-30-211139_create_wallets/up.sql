-- Your SQL goes here
CREATE TABLE Wallet (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    cryptocurrency_id INTEGER NOT NULL,
    balance DOUBLE PRECISION NOT NULL,
    created_on TIMESTAMP WITH TIME ZONE,
    modified_on TIMESTAMP WITH TIME ZONE
);
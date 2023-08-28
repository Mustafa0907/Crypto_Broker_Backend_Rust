-- Your SQL goes here
CREATE TABLE transactions (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    wallet_id INTEGER NOT NULL,
    cryptocurrency_id INTEGER NOT NULL,
    ttype VARCHAR(50) NOT NULL,
    amount DOUBLE PRECISION NOT NULL,
    created_on TIMESTAMP WITH TIME ZONE,
    modified_on TIMESTAMP WITH TIME ZONE,
    payment_method VARCHAR(50) NOT NULL,
    payment_amount DOUBLE PRECISION NOT NULL,
    payment_status VARCHAR(50) NOT NULL
);







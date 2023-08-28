-- Your SQL goes here
CREATE TABLE orders (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    cryptocurrency_id INTEGER NOT NULL,
    amount DOUBLE PRECISION NOT NULL,
    price DOUBLE PRECISION NOT NULL,
    otype VARCHAR(50) NOT NULL,
    created_on TIMESTAMP WITH TIME ZONE,
    modified_on TIMESTAMP WITH TIME ZONE,
    ostatus VARCHAR(50) NOT NULL,
    market_true BOOLEAN NOT NULL
);
-- Your SQL goes here
CREATE TABLE trade (
    id SERIAL PRIMARY KEY,
    buyer_id INTEGER NOT NULL,
    seller_id INTEGER NOT NULL,
    cryptocurrency_id INTEGER NOT NULL,
    amount DOUBLE PRECISION NOT NULL,
    price DOUBLE PRECISION NOT NULL,
    created_on TIMESTAMP WITH TIME ZONE,
    modified_on TIMESTAMP WITH TIME ZONE
);

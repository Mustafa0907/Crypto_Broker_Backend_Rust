// @generated automatically by Diesel CLI.

diesel::table! {
    crypto (id) {
        id -> Int4,
        cname -> Varchar,
        symbol -> Varchar,
        created_on -> Nullable<Timestamptz>,
        modified_on -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    orders (id) {
        id -> Int4,
        user_id -> Int4,
        cryptocurrency_id -> Int4,
        amount -> Float8,
        price -> Float8,
        otype -> Varchar,
        created_on -> Nullable<Timestamptz>,
        modified_on -> Nullable<Timestamptz>,
        ostatus -> Varchar,
        market_true -> Bool,
    }
}

diesel::table! {
    realmoney (id) {
        id -> Int4,
        user_id -> Int4,
        currency -> Varchar,
        balance -> Float8,
        created_on -> Nullable<Timestamptz>,
        modified_on -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    trade (id) {
        id -> Int4,
        buyer_id -> Int4,
        seller_id -> Int4,
        cryptocurrency_id -> Int4,
        amount -> Float8,
        price -> Float8,
        created_on -> Nullable<Timestamptz>,
        modified_on -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    transactions (id) {
        id -> Int4,
        user_id -> Int4,
        wallet_id -> Int4,
        cryptocurrency_id -> Int4,
        ttype -> Varchar,
        amount -> Float8,
        created_on -> Nullable<Timestamptz>,
        modified_on -> Nullable<Timestamptz>,
        payment_method -> Varchar,
        payment_amount -> Float8,
        payment_status -> Varchar,
    }
}

diesel::table! {
    user_details (id) {
        id -> Int4,
        user_name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_on -> Nullable<Timestamptz>,
        modified_on -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    wallet (id) {
        id -> Int4,
        user_id -> Int4,
        cryptocurrency_id -> Int4,
        balance -> Float8,
        created_on -> Nullable<Timestamptz>,
        modified_on -> Nullable<Timestamptz>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    crypto,
    orders,
    realmoney,
    trade,
    transactions,
    user_details,
    wallet,
);

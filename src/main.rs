
#[macro_use]
extern crate rocket;

use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
// use std::collections::HashMap;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::serde::json::Json;
use rocket::{Request, Response};
use rocket::response::{status::Created, Debug};
use rocket::{get, post };
use rocket::http::Status;


use web::schema::user_details::dsl::*;
use web::schema::wallet;
use web::schema::realmoney;
use web::schema::transactions;
use web::schema::orders;
use web::schema::trade;
use web::models::*;
use web::models::User;
use diesel::prelude::*;
use web::establish_connection;
use chrono::Utc;
use stripe::{
    CardDetailsParams, Client, CreatePaymentIntent,
    CreatePaymentMethod, CreatePaymentMethodCardUnion, Currency, PaymentIntent,
    PaymentIntentConfirmParams, PaymentMethod, PaymentMethodTypeFilter,UpdatePaymentIntent
};

// extern crate diesel;
// extern crate rocket;
// extern crate rocket_contrib;
// use diesel::pg::PgConnection;
// use diesel::prelude::*;
// use dotenvy::dotenv;
// use rocket::response::{status::Created, Debug};
// use rocket::serde::{json::Json, Deserialize, Serialize};
// use rocket::{get, launch, post, routes};


type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;
// pub trait FromParam<'a>: Sized {
//     type Error: Debug;

//     // Required method
//     fn from_param(param: &'a str) -> Result<Self, Self::Error>;
// }

// #[get("/<id3>")]
// fn hello(id3: Result<usize, &str>) -> String {
//     match id3 {
//         Ok(id_num) => format!("usize: {}", id_num),
//         Err(string) => format!("Not a usize: {}", string)
//     }
// }

#[get("/users")]
async fn get_users() -> Option<Json<Vec<User>>> {
    let mut connection = establish_connection();
    let results = user_details
        .limit(5)
        .load::<User>(&mut connection)
        .expect("Error loading users");
    Some(Json(results))
}
#[get("/users/<_id>")]
async fn get_user(_id: i32) -> Result<Json<User>, Status> {
    // use crate::schema::user_details::dsl::*;
    let mut connection = establish_connection();


    let user = user_details
        .filter(id.eq(_id))
        .first::<User>(&mut connection)
        .map_err(|_| Status::NotFound)?;

    Ok(Json(user))
}

#[put("/users/<_id>", data = "<new_user>")]
async fn update_user(_id: i32, new_user: Json<NewUser>) -> Result<Json<User>, Status>{
    use web::models::*;

    let mut connection = establish_connection();

    let new_user = new_user.into_inner();

    let user = user_details
        .filter(id.eq(_id))
        .first::<User>(&mut connection)
        .map_err(|_| Status::NotFound)?;

    diesel::update(&user)
    .set((  
    id.eq(&user.id),
        user_name.eq(new_user.user_name),
        password.eq(new_user.password),
        email.eq(new_user.email),
        created_on.eq(user.created_on),
        modified_on.eq(Utc::now().naive_utc())
    ))
    .get_result::<User>(&mut connection)
    .map(Json)
    .map_err(|_| Status::InternalServerError)

}

#[delete("/users/<_id>")]
async fn delete_user(_id: i32) -> Result<Json<User>, Status>{
    use web::models::*;

    let mut connection = establish_connection();

    diesel::delete(user_details.filter(id.eq(_id)))
        .get_result::<User>(&mut connection)
        .map(Json)
        .map_err(|_| Status::InternalServerError)

}

#[post("/users", format = "json", data = "<new_user>")]
async fn create_user(new_user: Json<NewUser>) -> Result<Created<Json<NewUser>>> {
    let mut connection = establish_connection();

    let new_user1 = NewUser {
        user_name: new_user.user_name.to_string(),
        password: new_user.password.to_string(),
        email: new_user.email.to_string(),
        created_on: Some(Utc::now().naive_utc()),
        modified_on: Some(Utc::now().naive_utc()),
    };

    diesel::insert_into(web::schema::user_details::dsl::user_details)
        .values(&new_user1)
        .execute(&mut connection)
        .expect("Error saving new post");
    Ok(Created::new("/").body(new_user))



//     // let result = diesel::insert_into(user_info::table())
//     //     .values(&new_user.into_inner())
//     //     .get_result::<User>(&mut connection);
//     // match result {
//     //     Ok(user) => Ok(Json(user)),
//     //     Err(_) => Err(Status::InternalServerError)
//     // }
}

#[derive(Debug, Deserialize, Serialize)]
struct Price {
    symbol: String,
    price: String,
}

#[get("/price/<symbol>")]
async fn index(symbol: String) -> Option<String> {
    let url = format!("https://api.binance.com/api/v3/ticker/price?symbol={}", symbol);
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", HeaderValue::from_static("rocket"));

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    let response = client.get(&url).send().await.unwrap();
    if response.status().is_success() {
        let price: Price = response.json().await.unwrap();
        return Some(price.price);
    } else {
        return None;
    }
}
pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[post("/login", format = "application/json", data = "<login_data>")]
async fn login(login_data: Json<LoginData>) -> Result<Status, Status> {
    let mut connection = establish_connection();

    let user_result = user_details
        .filter(user_name.eq(&login_data.user_name))
        .first::<User>(&mut connection);

    match user_result {
        Ok(user) => {
            if user.password == login_data.password {
                Ok(Status::Ok)
            } else {
                Err(Status::Unauthorized)
            }
        }
        Err(_) => Err(Status::NotFound),
    }
}


#[post("/wallet", format = "application/json", data = "<new_wallet>")]
async fn create_wallet(new_wallet: Json<NewWallet>) -> Result<Created<Json<NewWallet>>> {
    let mut connection = establish_connection();

    let new_wallet1 = NewWallet {
        user_id: new_wallet.user_id,
        cryptocurrency_id: new_wallet.cryptocurrency_id,
        balance: new_wallet.balance,
        created_on: Some(Utc::now().naive_utc()),
        modified_on: Some(Utc::now().naive_utc()),
    };

    diesel::insert_into(web::schema::wallet::dsl::wallet)
        .values(&new_wallet1)
        .execute(&mut connection)
        .expect("Error saving new wallet");
    Ok(Created::new("/").body(Json(new_wallet1)))
}

#[get("/wallet")]
async fn get_wallets() -> Option<Json<Vec<Wallet>>> {
    let mut connection = establish_connection();
    let results = wallet::table
        .limit(5)
        .load::<Wallet>(&mut connection)
        .expect("Error loading wallets");
    Some(Json(results))
}
#[get("/wallet/<_id>")]
async fn get_wallet(_id: i32) -> Result<Json<Wallet>, Status> {

    let mut connection = establish_connection();

    let w = wallet::table
        .filter(web::schema::wallet::id.eq(_id))
        .first::<Wallet>(&mut connection)
        .map_err(|_| Status::NotFound)?;

    Ok(Json(w))
}

#[put("/wallet/<wallet_id>", data = "<new_wallet>")]
async fn update_wallet(wallet_id: i32, new_wallet: Json<NewWallet>) -> Result<Json<Wallet>, Status>{
    use web::schema::wallet::dsl::*;

    let connection = establish_connection();
    let mut connection = connection;

    let target = wallet.filter(id.eq(wallet_id));
    let new_wallet = new_wallet.into_inner();

    let w = web::schema::wallet::table
        .filter(id.eq(wallet_id))
        .first::<Wallet>(&mut connection)
        .map_err(|_| Status::NotFound)?;

    diesel::update(target)
    .set((  
        id.eq(&w.id),
            user_id.eq(new_wallet.user_id),
            cryptocurrency_id.eq(new_wallet.cryptocurrency_id),
            balance.eq(new_wallet.balance),
            created_on.eq(w.created_on),
            modified_on.eq(Utc::now().naive_utc())
    ))
    .get_result::<Wallet>(&mut connection)
    .map(Json)
    .map_err(|_| Status::InternalServerError)
}

#[delete("/wallet/<_id>")]
async fn delete_wallet(_id: i32) -> Result<Json<Wallet>, Status>{
    use web::schema::wallet::dsl::*;

    let mut connection = establish_connection();

    diesel::delete(wallet.filter(id.eq(_id)))
        .get_result::<Wallet>(&mut connection)
        .map(Json)
        .map_err(|_| Status::InternalServerError)

}

#[get("/crypto")]
async fn get_cryptos() -> Option<Json<Vec<Crypto>>> {
    let mut connection = establish_connection();
    use web::schema::crypto::dsl::*;
    let results = crypto
        .limit(5)
        .load::<Crypto>(&mut connection)
        .expect("Error loading crypto currencies");

    Some(Json(results))
}

#[get("/crypto/<_id>")]
async fn get_crypto(_id: i32) -> Result<Json<Crypto>, Status> {
    let mut connection = establish_connection();
    use web::schema::crypto::dsl::*;
    let currency = crypto
        .filter(web::schema::crypto::id.eq(_id))
        .first::<Crypto>(&mut connection)
        .map_err(|_| Status::NotFound)?;

    Ok(Json(currency))
}

#[post("/crypto", format = "json", data = "<new_crypto>")]
async fn create_crypto(new_crypto: Json<NewCrypto>) -> Result<Created<Json<Crypto>>, Status> {
    let mut connection = establish_connection();
    use web::schema::crypto::dsl::*;
    let new_crypto = NewCrypto {
        cname: new_crypto.cname.to_string(),
        symbol: new_crypto.symbol.to_string(),
        created_on: Some(Utc::now().naive_utc()),
        modified_on: Some(Utc::now().naive_utc()),
    };

    let result = diesel::insert_into(crypto)
        .values(&new_crypto)
        .get_result::<Crypto>(&mut connection)
        .map_err(|_| Status::InternalServerError)?;

    Ok(Created::new("/crypto").body(Json(result)))
}

#[put("/crypto/<_id>", format = "json", data = "<new_crypto>")]
async fn update_crypto(_id: i32,new_crypto: Json<NewCrypto>,) -> Result<Json<Crypto>, Status> {
    let mut connection = establish_connection();
    use web::schema::crypto::dsl::*;
    let target = crypto.filter(id.eq(_id));
    let new_crypto = new_crypto.into_inner();

    let c = web::schema::crypto::table
        .filter(id.eq(_id))
        .first::<Crypto>(&mut connection)
        .map_err(|_| Status::NotFound)?;

    diesel::update(target)
    .set((  
        id.eq(&c.id),
            cname.eq(new_crypto.cname),
            symbol.eq(new_crypto.symbol),
            created_on.eq(c.created_on),
            modified_on.eq(Utc::now().naive_utc())
    ))
    .get_result::<Crypto>(&mut connection)
    .map(Json)
    .map_err(|_| Status::InternalServerError)
}

#[delete("/crypto/<_id>")]
async fn delete_crypto(_id: i32) -> Result<Json<Crypto>, Status> {
    use web::schema::crypto::dsl::*;
    let mut connection = establish_connection();

    diesel::delete(crypto.filter(id.eq(_id)))
        .get_result::<Crypto>(&mut connection)
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[post("/realmoney", format = "application/json", data = "<new_rwallet>")]
async fn create_rwallet(new_rwallet: Json<NewRealMoneyWallet>) -> Result<Created<Json<NewRealMoneyWallet>>> {
    let mut connection = establish_connection();

    let new_rwallet1 = NewRealMoneyWallet {
        user_id: new_rwallet.user_id,
        currency: new_rwallet.currency.to_string(),
        balance: new_rwallet.balance,
        created_on: Some(Utc::now().naive_utc()),
        modified_on: Some(Utc::now().naive_utc()),
    };

    diesel::insert_into(web::schema::realmoney::dsl::realmoney)
        .values(&new_rwallet1)
        .execute(&mut connection)
        .expect("Error saving new wallet");
    Ok(Created::new("/").body(Json(new_rwallet1)))
}

#[get("/realmoney")]
async fn get_rwallets() -> Option<Json<Vec<RealMoneyWallet>>> {
    let mut connection = establish_connection();
    let results = realmoney::table
        .limit(5)
        .load::<RealMoneyWallet>(&mut connection)
        .expect("Error loading wallets");
    Some(Json(results))
}
#[get("/realmoney/<_id>")]
async fn get_rwallet(_id: i32) -> Result<Json<RealMoneyWallet>, Status> {

    let mut connection = establish_connection();

    let r = realmoney::table
        .filter(web::schema::realmoney::id.eq(_id))
        .first::<RealMoneyWallet>(&mut connection)
        .map_err(|_| Status::NotFound)?;

    Ok(Json(r))
}

#[put("/realmoney/<rwallet_id>", data = "<new_rwallet>")]
async fn update_rwallet(rwallet_id: i32, new_rwallet: Json<NewRealMoneyWallet>) -> Result<Json<RealMoneyWallet>, Status>{
    use web::schema::realmoney::dsl::*;

    let mut connection = establish_connection();

    let target = realmoney.filter(id.eq(rwallet_id));
    let new_rwallet = new_rwallet.into_inner();

    let r = web::schema::realmoney::table
        .filter(id.eq(rwallet_id))
        .first::<RealMoneyWallet>(&mut connection)
        .map_err(|_| Status::NotFound)?;

    diesel::update(target)
    .set((  
        id.eq(&r.id),
            user_id.eq(new_rwallet.user_id),
            currency.eq(new_rwallet.currency),
            balance.eq(new_rwallet.balance),
            created_on.eq(r.created_on),
            modified_on.eq(Utc::now().naive_utc())
    ))
    .get_result::<RealMoneyWallet>(&mut connection)
    .map(Json)
    .map_err(|_| Status::InternalServerError)
}

#[delete("/realmoney/<_id>")]
async fn delete_rwallet(_id: i32) -> Result<Json<RealMoneyWallet>, Status>{
    use web::schema::realmoney::dsl::*;

    let mut connection = establish_connection();

    diesel::delete(realmoney.filter(id.eq(_id)))
        .get_result::<RealMoneyWallet>(&mut connection)
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

#[post("/transactions", format = "application/json", data = "<new_t>")]
async fn create_trans(new_t: Json<NewTransaction>) -> Result<Created<Json<NewTransaction>>> {
    let mut connection = establish_connection();

    let new_t1 = NewTransaction {
        user_id: new_t.user_id,
        wallet_id: new_t.wallet_id,
        cryptocurrency_id: new_t.cryptocurrency_id,
        ttype: new_t.ttype.to_string(),
        amount: new_t.amount,
        created_on: Some(Utc::now().naive_utc()),
        modified_on: Some(Utc::now().naive_utc()),
        payment_method: new_t.payment_method.to_string(),
        payment_amount: new_t.payment_amount,
        payment_status: new_t.payment_status.to_string()
    };

    diesel::insert_into(web::schema::transactions::dsl::transactions)
        .values(&new_t1)
        .execute(&mut connection)
        .expect("Error saving new wallet");
    Ok(Created::new("/").body(Json(new_t1)))
}

#[get("/transactions")]
async fn get_trans() -> Option<Json<Vec<Transaction>>> {
    let mut connection = establish_connection();
    let results = transactions::table
        .limit(5)
        .load::<Transaction>(&mut connection)
        .expect("Error loading wallets");
    Some(Json(results))
}
#[get("/transactions/<_id>")]
async fn get_tran(_id: i32) -> Result<Json<Transaction>, Status> {

    let mut connection = establish_connection();

    let t = transactions::table
        .filter(web::schema::transactions::id.eq(_id))
        .first::<Transaction>(&mut connection)
        .map_err(|_| Status::NotFound)?;

    Ok(Json(t))
}

#[put("/transactions/<_id>", data = "<new_t>")]
async fn update_trans(_id: i32, new_t: Json<NewTransaction>) -> Result<Json<Transaction>, Status>{
    use web::schema::transactions::dsl::*;

    let mut connection = establish_connection();

    let target = transactions.filter(id.eq(_id));
    let new_t = new_t.into_inner();

    let t = web::schema::transactions::table
        .filter(id.eq(_id))
        .first::<Transaction>(&mut connection)
        .map_err(|_| Status::NotFound)?;

    diesel::update(target)
    .set((  
        id.eq(&t.id),
            user_id.eq(new_t.user_id),
            wallet_id.eq(new_t.wallet_id),
            cryptocurrency_id.eq(new_t.cryptocurrency_id),
            ttype.eq(new_t.ttype),
            amount.eq(new_t.amount),
            created_on.eq(t.created_on),
            modified_on.eq(Utc::now().naive_utc()),
            payment_method.eq(new_t.payment_method),
            payment_amount.eq(new_t.payment_amount),
            payment_status.eq(new_t.payment_status)
    ))
    .get_result::<Transaction>(&mut connection)
    .map(Json)
    .map_err(|_| Status::InternalServerError)
}

#[delete("/transactions/<_id>")]
async fn delete_trans(_id: i32) -> Result<Json<Transaction>, Status>{
    use web::schema::transactions::dsl::*;

    let mut connection = establish_connection();

    diesel::delete(transactions.filter(id.eq(_id)))
        .get_result::<Transaction>(&mut connection)
        .map(Json)
        .map_err(|_| Status::InternalServerError)

}

#[post("/orders", format = "application/json", data = "<new_o>")]
async fn create_order(new_o: Json<NewOrder>) -> Result<Created<Json<NewOrder>>> {
    let mut connection = establish_connection();

    let new_o1 = NewOrder {
        user_id: new_o.user_id,
        cryptocurrency_id: new_o.cryptocurrency_id,
        amount: new_o.amount,
        price: new_o.price,
        otype: new_o.otype.to_string(),
        created_on: Some(Utc::now().naive_utc()),
        modified_on: Some(Utc::now().naive_utc()),
        ostatus: new_o.ostatus.to_string(),
        market_true: new_o.market_true
    };

    diesel::insert_into(web::schema::orders::dsl::orders)
        .values(&new_o1)
        .execute(&mut connection)
        .expect("Error saving new wallet");
    Ok(Created::new("/").body(Json(new_o1)))
}

#[get("/orders/history/<_user_id>")]
fn get_order_history(_user_id: i32) -> Json<Vec<Order>> {
    use web::schema::orders::dsl::*;

    let mut connection = establish_connection();
    let results = orders
        .filter(user_id.eq(_user_id).and(ostatus.eq("closed")))
        .load::<Order>(&mut connection)
        .expect("Error loading orders");

    Json(results)
}

#[get("/orders")]
async fn get_orders() -> Option<Json<Vec<Order>>> {
    let mut connection = establish_connection();
    let results = orders::table
        .limit(5)
        .load::<Order>(&mut connection)
        .expect("Error loading wallets");
    Some(Json(results))
}
#[get("/orders/<_id>")]
async fn get_order(_id: i32) -> Result<Json<Order>, Status> {

    let mut connection = establish_connection();

    let o = orders::table
        .filter(web::schema::orders::id.eq(_id))
        .first::<Order>(&mut connection)
        .map_err(|_| Status::NotFound)?;

    Ok(Json(o))
}

#[put("/orders/<_id>", data = "<new_o>")]
async fn update_order(_id: i32, new_o: Json<NewOrder>) -> Result<Json<Order>, Status>{
    use web::schema::orders::dsl::*;

    let mut connection = establish_connection();

    let target = orders.filter(id.eq(_id));
    let new_o = new_o.into_inner();

    let o = web::schema::orders::table
        .filter(id.eq(_id))
        .first::<Order>(&mut connection)
        .map_err(|_| Status::NotFound)?;

    diesel::update(target)
    .set((  
        id.eq(&o.id),
            user_id.eq(new_o.user_id),
            cryptocurrency_id.eq(new_o.cryptocurrency_id),
            amount.eq(new_o.amount),
            price.eq(new_o.price),
            otype.eq(new_o.otype),
            created_on.eq(o.created_on),
            modified_on.eq(Utc::now().naive_utc()),
            ostatus.eq(new_o.ostatus),
            market_true.eq(new_o.market_true)
    ))
    .get_result::<Order>(&mut connection)
    .map(Json)
    .map_err(|_| Status::InternalServerError)
}

#[delete("/orders/<_id>")]
async fn delete_order(_id: i32) -> Result<Json<Order>, Status>{
    use web::schema::orders::dsl::*;

    let mut connection = establish_connection();

    diesel::delete(orders.filter(id.eq(_id)))
        .get_result::<Order>(&mut connection)
        .map(Json)
        .map_err(|_| Status::InternalServerError)

}

#[post("/trade", format = "application/json", data = "<new_tr>")]
async fn create_trade(new_tr: Json<NewTrade>) -> Result<Created<Json<NewTrade>>> {
    let mut connection = establish_connection();

    let new_tr1 = NewTrade {
        buyer_id: new_tr.buyer_id,
        seller_id: new_tr.seller_id,
        cryptocurrency_id: new_tr.cryptocurrency_id,
        amount: new_tr.amount,
        price: new_tr.price,
        created_on: Some(Utc::now().naive_utc()),
        modified_on: Some(Utc::now().naive_utc()),
    };

    diesel::insert_into(web::schema::trade::dsl::trade)
        .values(&new_tr1)
        .execute(&mut connection)
        .expect("Error saving new wallet");
    Ok(Created::new("/").body(Json(new_tr1)))
}

#[get("/trade")]
async fn get_trade() -> Option<Json<Vec<Trade>>> {
    let mut connection = establish_connection();
    let results = trade::table
        .limit(5)
        .load::<Trade>(&mut connection)
        .expect("Error loading wallets");
    Some(Json(results))
}
#[get("/trade/<_id>")]
async fn get_trades(_id: i32) -> Result<Json<Trade>, Status> {

    let mut connection = establish_connection();

    let tr = trade::table
        .filter(web::schema::trade::id.eq(_id))
        .first::<Trade>(&mut connection)
        .map_err(|_| Status::NotFound)?;

    Ok(Json(tr))
}

#[put("/trade/<_id>", data = "<new_tr>")]
async fn update_trade(_id: i32, new_tr: Json<NewTrade>) -> Result<Json<Trade>, Status>{
    use web::schema::trade::dsl::*;

    let mut connection = establish_connection();

    let target = trade.filter(id.eq(_id));
    let new_tr = new_tr.into_inner();

    let tr = web::schema::trade::table
        .filter(id.eq(_id))
        .first::<Trade>(&mut connection)
        .map_err(|_| Status::NotFound)?;

    diesel::update(target)
    .set((  
        id.eq(&tr.id),
            buyer_id.eq(new_tr.buyer_id),
            seller_id.eq(new_tr.seller_id),
            cryptocurrency_id.eq(new_tr.cryptocurrency_id),
            amount.eq(new_tr.amount),
            price.eq(new_tr.price),
            created_on.eq(tr.created_on),
            modified_on.eq(Utc::now().naive_utc()),
    ))
    .get_result::<Trade>(&mut connection)
    .map(Json)
    .map_err(|_| Status::InternalServerError)
}

#[delete("/trade/<_id>")]
async fn delete_trade(_id: i32) -> Result<Json<Trade>, Status>{
    use web::schema::trade::dsl::*;

    let mut connection = establish_connection();

    diesel::delete(trade.filter(id.eq(_id)))
        .get_result::<Trade>(&mut connection)
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}

// #[post("/transaction/charge", format = "application/json", data = "<new_t>")]
#[post("/transaction/charge", format = "application/json", data = "<new_t2>")]
async fn charge_transaction(new_t2: Json<NewTransaction>) -> Result<Created<Json<Transaction>>, Status> {
    let mut connection = establish_connection();
    // use web::schema::transaction::dsl::*;
    let new_transaction = NewTransaction {
        user_id: new_t2.user_id,
        wallet_id: new_t2.wallet_id,
        cryptocurrency_id: new_t2.cryptocurrency_id,
        ttype: new_t2.ttype.to_string(),
        amount: new_t2.amount,
        created_on: Some(Utc::now().naive_utc()),
        modified_on: Some(Utc::now().naive_utc()),
        payment_method: new_t2.payment_method.to_string(),
        payment_amount: new_t2.payment_amount,
        payment_status: "completed".to_string()
    };

    // Create a payment intent and confirm it
    // let secret_key = std::env::var("sk_test_51MfTJqASW7Sg3dtTqoXv4ADxyD509b9h6SWzC52gLPlcrgPhwEm9PcrnAQkdzHzviGLlDzIrFdmhuq7VyGnz1Jmm00Lk0pmBeI").expect("Missing STRIPE_SECRET_KEY in env");
    let client = stripe::Client::new("sk_test_51MfTJqASW7Sg3dtTqoXv4ADxyD509b9h6SWzC52gLPlcrgPhwEm9PcrnAQkdzHzviGLlDzIrFdmhuq7VyGnz1Jmm00Lk0pmBeI");

    let card_number = &new_transaction.payment_method; // Assuming this is the card number
    let exp_month = 3; // Replace with actual value
    let exp_year = 2024; // Replace with actual value
    let cvc = "314"; // Replace with actual value
    let amount = (new_transaction.payment_amount * 100.0) as i64; // Amount in cents
    let mut create_intent = CreatePaymentIntent::new(amount, Currency::USD);
    create_intent.payment_method_types = Some(vec!["card".to_string()]);
    let payment_intent = PaymentIntent::create(&client, create_intent).await.unwrap();

    let pm = PaymentMethod::create(
        &client,
        CreatePaymentMethod {
            type_: Some(PaymentMethodTypeFilter::Card),
            card: Some(CreatePaymentMethodCardUnion::CardDetailsParams(CardDetailsParams {
                number: card_number.to_string(),
                exp_year: exp_year,
                exp_month: exp_month,
                cvc: Some(cvc.to_string()),
                ..Default::default()
            })),
            ..Default::default()
        },
    )
    .await
    .unwrap();

    let payment_intent = PaymentIntent::update(
        &client,
        &payment_intent.id,
        UpdatePaymentIntent {
            payment_method: Some(pm.id),
            ..Default::default()
        },
    )
    .await
    .unwrap();

    let payment_intent = PaymentIntent::confirm(
        &client,
        &payment_intent.id,
        PaymentIntentConfirmParams { ..Default::default() },
    )
    .await
    .unwrap();

    // new_transaction.payment_status = "completed".to_string();

    let result = diesel::insert_into(web::schema::transactions::dsl::transactions)
        .values(&new_transaction)
        .get_result::<Transaction>(&mut connection)
        .map_err(|_| Status::InternalServerError)?;

    Ok(Created::new("/transaction").body(Json(result)))
}


#[launch]
fn rocket() -> _ {
    rocket::build().attach(Cors).mount("/", routes![get_order_history,index,get_users,create_user,get_user,update_user,delete_user,login,
    create_wallet,get_wallet,get_wallets,update_wallet,delete_wallet,
    get_cryptos,get_crypto,create_crypto,update_crypto,delete_crypto,
    create_rwallet,get_rwallet,get_rwallets,update_rwallet,delete_rwallet,
    create_trans,get_tran,get_trans,update_trans,delete_trans,
    create_order,get_order,get_orders,update_order,delete_order,
    create_trade,get_trade,get_trades,update_trade,delete_trade,charge_transaction])
}

// #[macro_use]
// extern crate rocket;

// use reqwest::header::{HeaderMap, HeaderValue};
// use serde::{Deserialize, Serialize};
// use std::collections::HashMap;

// use rocket::fairing::{Fairing, Info, Kind};
// use rocket::http::Header;
// use rocket::log::private::debug;
// use rocket::serde::json::Json;
// use rocket::{Request, Response};

// use web::schema::user_info::dsl::*;
// use web::models::*;
// use diesel::prelude::*;
// use web::*;
// use web::establish_connection;

// #[derive(Debug, Deserialize, Serialize)]
// struct Price {
//     symbol: String,
//     price: String,
// }


// #[get("/users")]
// async fn get_users() -> Option<Json<Vec<User>>> {
//     let mut connection = establish_connection();
//     let results = user_info
//         .limit(5)
//         .load::<User>(&mut connection)
//         .expect("Error loading users");
//     Some(Json(results))
// }

// #[get("/price/<symbol>")]
// async fn index(symbol: String) -> Option<String> {
//     let url = format!("https://api.binance.com/api/v3/ticker/price?symbol={}", symbol);
//     let mut headers = HeaderMap::new();
//     headers.insert("User-Agent", HeaderValue::from_static("rocket"));

//     let client = reqwest::Client::builder()
//         .default_headers(headers)
//         .build()
//         .unwrap();

//     let response = client.get(&url).send().await.unwrap();
//     if response.status().is_success() {
//         let price: Price = response.json().await.unwrap();
//         return Some(price.price);
//     } else {
//         return None;
//     }
// }

// pub struct Cors;

// #[rocket::async_trait]
// impl Fairing for Cors {
//     fn info(&self) -> Info {
//         Info {
//             name: "Cross-Origin-Resource-Sharing Fairing",
//             kind: Kind::Response,
//         }
//     }

//     async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
//         response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
//         response.set_header(Header::new(
//             "Access-Control-Allow-Methods",
//             "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET",
//         ));
//         response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
//         response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
//     }
// }

// #[launch]

// fn rocket() -> Rocket<Build>{
//     // use web::schema::user_info::dsl::*;

//     // let connection = &mut establish_connection();
//     // let results = user_info
//     //     .limit(5)
//     //     .load::<User>(connection)
//     //     .expect("Error loading posts");

//     // println!("Displaying {} users", results.len());
//     // for user in results {
//     //     println!("{}", user.user_name);
//     //     println!("-----------\n");
//     //     println!("{}", user.email);
//     // }

//     rocket::build().attach(Cors).mount("/", routes![index]);
// }


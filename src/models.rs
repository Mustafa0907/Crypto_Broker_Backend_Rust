use diesel::{Queryable, Insertable, AsChangeset, Identifiable};
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
// use super::schema::user_details::dsl::*;
// use crate::web::schema::user_details::dsl::*;
use crate::schema::user_details;
use crate::schema::wallet;
use crate::schema::crypto;
use crate::schema::realmoney;
use crate::schema::transactions;
use crate::schema::orders;
use crate::schema::trade;



#[derive(Queryable, Debug, Deserialize, Serialize,AsChangeset, Identifiable)]
#[diesel(table_name = user_details)]
pub struct User {
    pub id: i32,
    pub user_name: String,
    pub password: String,
    pub email: String,
    pub created_on: Option<NaiveDateTime>,
    pub modified_on: Option<NaiveDateTime>
}

#[derive(Queryable, Debug, Deserialize, Serialize,AsChangeset)]
#[diesel(table_name = user_details)]
pub struct UserDTO{
    pub user_name: String,
    pub password: String,
    pub email: String
}

#[derive(Queryable, Insertable, Deserialize, Serialize)]
#[diesel(table_name = user_details)]
pub struct NewUser{
    pub user_name: String,
    pub password: String,
    pub email: String,
    pub created_on: Option<NaiveDateTime>,
    pub modified_on: Option<NaiveDateTime>
}

#[derive(Serialize,Deserialize)]
pub struct LoginData {
    pub user_name: String,
    pub password: String,
}

#[derive(Queryable, Debug, Deserialize, Serialize, AsChangeset, Identifiable)]
#[diesel(table_name = wallet)]
pub struct Wallet {
    pub id: i32,
    pub user_id: i32,
    pub cryptocurrency_id: i32,
    pub balance: f64,
    pub created_on: Option<NaiveDateTime>,
    pub modified_on: Option<NaiveDateTime>,
}

#[derive(Queryable, Insertable, Deserialize, Serialize)]
#[diesel(table_name = wallet)]
pub struct NewWallet {
    pub user_id: i32,
    pub cryptocurrency_id: i32,
    pub balance: f64,
    pub created_on: Option<NaiveDateTime>,
    pub modified_on: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug, Serialize, Deserialize, Identifiable, AsChangeset)]
#[diesel(table_name = crypto)]
pub struct Crypto {
    pub id: i32,
    pub cname: String,
    pub symbol: String,
    pub created_on: Option<NaiveDateTime>,
    pub modified_on: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = crypto)]
pub struct NewCrypto {
    pub cname: String,
    pub symbol: String,
    pub created_on: Option<NaiveDateTime>,
    pub modified_on: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug, Deserialize, Serialize, Identifiable, AsChangeset)]
#[diesel(table_name = realmoney)]
pub struct RealMoneyWallet {
    pub id: i32,
    pub user_id: i32,
    pub currency: String,
    pub balance: f64,
    pub created_on: Option<NaiveDateTime>,
    pub modified_on: Option<NaiveDateTime>,
}

#[derive(Insertable, Debug, Deserialize, Serialize)]
#[diesel(table_name = realmoney)]
pub struct NewRealMoneyWallet {
    pub user_id: i32,
    pub currency: String,
    pub balance: f64,
    pub created_on: Option<NaiveDateTime>,
    pub modified_on: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug, Deserialize, Serialize, AsChangeset, Identifiable)]
#[diesel(table_name = transactions)]
pub struct Transaction {
    pub id: i32,
    pub user_id: i32,
    pub wallet_id: i32,
    pub cryptocurrency_id: i32,
    pub ttype: String,
    pub amount: f64,
    pub created_on: Option<NaiveDateTime>,
    pub modified_on: Option<NaiveDateTime>,
    pub payment_method: String,
    pub payment_amount: f64,
    pub payment_status: String,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = transactions)]
pub struct NewTransaction {
    pub user_id: i32,
    pub wallet_id: i32,
    pub cryptocurrency_id: i32,
    pub ttype: String,
    pub amount: f64,
    pub created_on: Option<NaiveDateTime>,
    pub modified_on: Option<NaiveDateTime>,
    pub payment_method: String,
    pub payment_amount: f64,
    pub payment_status: String,
}

#[derive(Queryable, Debug, Serialize, Deserialize, AsChangeset, Identifiable)]
#[diesel(table_name = orders)]
pub struct Order {
    pub id: i32,
    pub user_id: i32,
    pub cryptocurrency_id: i32,
    pub amount: f64,
    pub price: f64,
    pub otype: String,
    pub created_on: Option<NaiveDateTime>,
    pub modified_on: Option<NaiveDateTime>,
    pub ostatus: String,
    pub market_true: bool,
}

#[derive(Insertable, Debug, Deserialize, Serialize)]
#[diesel(table_name = orders)]
pub struct NewOrder {
    pub user_id: i32,
    pub cryptocurrency_id: i32,
    pub amount: f64,
    pub price: f64,
    pub otype: String,
    pub created_on: Option<NaiveDateTime>,
    pub modified_on: Option<NaiveDateTime>,
    pub ostatus: String,
    pub market_true: bool,
}

#[derive(Queryable, Debug, Deserialize, Serialize, AsChangeset, Identifiable)]
#[diesel(table_name = trade)]
pub struct Trade {
    pub id: i32,
    pub buyer_id: i32,
    pub seller_id: i32,
    pub cryptocurrency_id: i32,
    pub amount: f64,
    pub price: f64,
    pub created_on: Option<NaiveDateTime>,
    pub modified_on: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = trade)]
pub struct NewTrade {
    pub buyer_id: i32,
    pub seller_id: i32,
    pub cryptocurrency_id: i32,
    pub amount: f64,
    pub price: f64,
    pub created_on: Option<NaiveDateTime>,
    pub modified_on: Option<NaiveDateTime>
}




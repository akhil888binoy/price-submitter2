use crate::data::dummydata::dummyData;
use crate::utils::helpersutils::{
    SUPPORTED_PERIODS,
    SUPPORTED_TOKENS,
    SYMBOL_TO_ADDRESS_MAPPING
};

use crate::configs::envconfig::{CHAINID_MAP , ENV};
use crate::utils::pricesutils::getTokenPricesFiltered;
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set};
use sea_orm::*;
use dotenv::dotenv;
use std::env;
use crate::utils::interfaceutils::AssetPricingInfo2;
use rocket::{get, serde::json::Json};
use crate::DbConnection;
use rocket::{ response::status::BadRequest, State};



const MAX_LIMIT : u32 = 1000;

#[get("/tickers")] 
pub async fn getPriceTickers(db: &State<DbConnection>)-> Json<Vec<AssetPricingInfo2>>{
    let tickersData = getTokenPricesFiltered(&db.0).await;
    Json(tickersData)
}

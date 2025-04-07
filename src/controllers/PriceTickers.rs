use crate::utils::interfaceutils::AssetPricingInfo2;
use crate::utils::pricesutils::get_token_prices_filtered;
use rocket::{get, serde::json::Json};
use crate::DbConnection;
use rocket::State;


#[get("/tickers")] 
pub async fn getPriceTickers(db: &State<DbConnection>)-> Json<Vec<AssetPricingInfo2>>{
    let tickersData = get_token_prices_filtered(&db.0).await;
    Json(tickersData)
}

use crate::utils::helpersutils::{
    // BINANCE_KEYS,
    // BINANCE_SYMBOL_MAP,
    // KUCOIN_KEYS,
    // KUCOIN_SYMBOL_MAP,
    // MEXC_KEYS,
    // MEXC_SYMBOL_MAP,
    // GATE_KEYS,
    // GATE_SYMBOL_MAP,
    PRICES_MAPPINGS,
    PYTH_ID,
    // OKX_KEYS,
    // OKX_SYMBOL_MAP,
    // KRAKEN_KEYS,
    // KRAKEN_SYMBOL_MAP,
    // BYBIT_KEYS,
    // BYBIT_SYMBOL_MAP,
    PYTH_ID_TO_TOKEN_MAPPING,
    SUPPORTED_TOKENS,
    SYMBOL_TO_ADDRESS_MAPPING,
    SYMBOL_TO_DECIMAL_MAPPING,
};

#[path = "../../entity/src/mod.rs"]
mod entities;

use crate::configs::envconfig::{CHAINID_MAP, ENV};

use crate::utils::responseinterfaceutils::{
    ParclDetails, ParclIdResponse, ParclResponse, PythResponse,
};
use chrono::Utc;
use entities::{prelude::*, *};
use sea_orm::entity::prelude::*;
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, QueryFilter, QueryOrder};
use std::collections::HashMap;
extern crate rand;
use num_bigint::BigInt;
use rand::Rng;

use super::interfaceutils::AssetPricingInfo2;

const PRICE_DECIMALS: usize = 4;
const PRECISION: i32 = 10;

pub fn get_pyth_price_url() -> String {
    let mut pyth_url = String::from("https://hermes.pyth.network/v2/updates/price/latest?");
    let pyth_ids = match PYTH_ID.get(&ENV.NETWORK) {
        Some(ids) => ids,
        None => return pyth_url,
    };

    for (i, id) in pyth_ids.iter().enumerate() {
        if i > 0 {
            pyth_url.push('&');
        }
        pyth_url.push_str(&format!("ids[]={}", id.to_string()));
    }

    pyth_url
}

pub async fn get_pyth_prices() -> Result<HashMap<String, f64>, Box<dyn std::error::Error>> {
    let mut result: HashMap<String, f64> = HashMap::new();

    let pyth_id_to_token_mapping = match PYTH_ID_TO_TOKEN_MAPPING.get(&ENV.NETWORK) {
        Some(ids) => ids,
        None => panic!("No mapping found for the given network"),
    };

    let client = reqwest::Client::new();
    let response = client.get(get_pyth_price_url()).send().await?;

    if response.status() != reqwest::StatusCode::OK {
        eprintln!(
            "Failed to retrieve data. Status code: {}",
            response.status()
        );
        return Err("Failed to retrieve data".into());
    }

    let response_data: PythResponse = response.json().await?;

    for price_data in response_data.parsed.iter() {
        if let Some(token) = pyth_id_to_token_mapping.get(&*price_data.id) {
            let adjusted_price = (price_data.price.price.parse::<f64>().unwrap())
                * (10f64).powi(price_data.price.expo);
            result.insert(token.to_string(), adjusted_price);

            if token.to_string() == "BTC" {
                result.insert(
                    "WBTC".to_string(),
                    price_data.price.price.parse::<f64>().unwrap() / 10f64.powi(8),
                );
            }
            if token.to_string() == "ETH" {
                result.insert(
                    "WETH".to_string(),
                    price_data.price.price.parse::<f64>().unwrap() / 10f64.powi(8),
                );
            }
        }
    }

    Ok(result)
}

pub async fn fetch_all_parcl_ids() -> Result<Vec<i64>, Box<dyn std::error::Error>> {
    let url = "https://parcl-api.com/v1/metadata/parcl-ids";
    let client = reqwest::Client::new();

    let response = client
        .get(url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36")
        .header("Origin", "https://app.parcl.co")
        .header("Referer", "https://app.parcl.co/")
        .send()
        .await?;

    if response.status() != reqwest::StatusCode::OK {
        eprintln!(
            "Failed to retrieve data. Status code: {}",
            response.status()
        );
        return Err("Failed to retrieve data".into());
    }

    let response_data: ParclIdResponse = response.json().await?;
    Ok(response_data.ids)
}

#[derive(serde::Serialize)]
pub struct Param {
    pub id: String,
}

pub async fn fetch_parcl_details(
    parcl_ids: Vec<String>,
) -> Result<HashMap<String, ParclDetails>, Box<dyn std::error::Error>> {
    let url = "https://parcl-api.com/v1/real-estate-data/parcl-info";
    let client = reqwest::Client::new();
    let mut parcl_map: HashMap<String, ParclDetails> = HashMap::new();

    for parcl_id in parcl_ids {
        let params = Param {
            id: parcl_id.clone(),
        };

        let response = client
            .get(url)
            .query(&params)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36")
            .header("Origin", "https://app.parcl.co")
            .header("Referer", "https://app.parcl.co/")
            .send()
            .await?;

        if response.status() != reqwest::StatusCode::OK {
            eprintln!(
                "Failed to retrieve data for Parcel ID {}. Status code: {}",
                parcl_id,
                response.status()
            );
            continue;
        }

        let response_data: ParclResponse = response.json().await?;
        let parcl_details = ParclDetails {
            parcl_id: parcl_id.clone(),
            name: response_data.info.name,
            current_price: response_data.info.current_price.to_string(),
        };

        parcl_map.insert(parcl_id, parcl_details);
    }

    Ok(parcl_map)
}

pub async fn gathertokenprices() -> Result<HashMap<String, Vec<f64>>, Box<dyn std::error::Error>> {
    let responses = match get_pyth_prices().await {
        Ok(data) => data,
        Err(e) => {
            panic!("Error getting Pyth prices: {}", e);
        }
    };

    let mut prices: HashMap<String, Vec<f64>> = match PRICES_MAPPINGS.get(ENV.NETWORK.as_str()) {
        Some(map) => map
            .iter()
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect(), // Convert keys to `String`
        None => {
            eprintln!("No price mappings found for network: {}", ENV.NETWORK);
            return Err("Network not found".into());
        }
    };

    const BITLAYER_NOT_SUPPORTED_BLUE_CHIPS: [&str; 1] = ["ETH"];

    for (key, value) in responses.iter() {
        let key_str = key.to_string(); // Convert borrowed key to owned `String`

        if ENV.NETWORK == "bitlayer_testnet" {
            if !BITLAYER_NOT_SUPPORTED_BLUE_CHIPS.contains(&key_str.as_str()) {
                if let Some(vec) = prices.get_mut(&key_str) {
                    vec.push(*value);
                }
            }
        } else {
            if let Some(vec) = prices.get_mut(&key_str) {
                vec.push(*value);
            }
        }
    }

    Ok(prices)
}

pub async fn get_token_prices() -> Result<HashMap<String, f64>, Box<dyn std::error::Error>> {
    let prices = match gathertokenprices().await {
        Ok(data) => data,
        Err(e) => {
            panic!("Error getting Pyth prices: {}", e);
        }
    };

    let mut rng = rand::rng();
    let mut result: HashMap<String, f64> = HashMap::new();

    for (key, token_prices) in prices.iter() {
        let mut token_prices = token_prices.clone();

        if token_prices.len() > 2 {
            token_prices.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            token_prices.pop();
            token_prices.remove(0);
        }

        let mut price_sum: f64 = 0.0;
        let mut weight_sum: u32 = 0;

        for price in &token_prices {
            let weight = rng.random_range(10..20);
            weight_sum += weight;
            price_sum += price * weight as f64;
        }

        if price_sum == 0.0 {
            println!("Token skipped: {}", key);
            continue;
        }

        result.insert(key.clone(), price_sum / weight_sum as f64);
    }

    Ok(result)
}

pub async fn gettokenpricesfromdb(db: &DatabaseConnection) -> Result<HashMap<&str, f32>, DbErr> {
    let mut grouped_tokens = HashMap::new();
    let mut result = HashMap::new();
    let mut supportedfinaltokens = Vec::new();
    let supportedtokens = match SUPPORTED_TOKENS.get(&ENV.NETWORK) {
        Some(data) => data.clone(),
        None => panic!("Error : Cannot get tokens"),
    };

    let supportedrealestatetokens = vec![
        "CLT", "DEN", "MIA", "TPA", "MIAB", "NYC", "LAX", "SAN", "SOLB", "SFO", "LAS", "PIT",
        "PHL", "AUS", "DFW", "IAH", "ATL", "SEA", "PHX", "CHI", "BOS", "PDX", "WDC", "BKN", "USA",
        "PARIS", "LCY", "CHIR", "DENR", "USDR",
    ];

    for token in supportedtokens {
        if !supportedrealestatetokens.contains(&token) {
            supportedfinaltokens.push(token);
        }
    }
    let mut token_addresses = Vec::new();

    for token_symbol in supportedfinaltokens.clone() {
        let tokenaddress = match SYMBOL_TO_ADDRESS_MAPPING.get(token_symbol) {
            Some(data) => data,
            None => panic!("Error : Cannot get address"),
        };
        token_addresses.push(tokenaddress.to_string());
    }

    let mut real_estate_token_address = Vec::new();

    if ENV.NETWORK == "bitlayer_testnet".to_string() {
        for token_symbol in supportedrealestatetokens.clone() {
            let realestatetoken = match SYMBOL_TO_ADDRESS_MAPPING.get(token_symbol) {
                Some(data) => data,
                None => panic!("Error : Cannot get token symbol"),
            };
            real_estate_token_address.push(realestatetoken.to_string());
        }
    }
    let chainid = match CHAINID_MAP.get(&ENV.NETWORK) {
        Some(data) => data,
        None => panic!("Error : Cannot get chainid"),
    };

    // Get regular token prices (1m period)
    let tokens_data = PriceCandle::find()
        .filter(price_candle::Column::Token.is_in(token_addresses.clone()))
        .filter(price_candle::Column::Period.eq("1m"))
        .filter(price_candle::Column::ChainId.eq(chainid.clone()))
        .order_by_desc(price_candle::Column::Timestamp)
        .all(db)
        .await;

    // Group by token and get latest close price
    let tokens_data = match tokens_data {
        Ok(data) => data,
        Err(_) => panic!("Error : Cannot get data from DB"),
    };

    for candle in tokens_data {
        grouped_tokens
            .entry(candle.token.clone()) // Use token as key
            .or_insert(candle.close); // Store just the close price
    }

    // Map to token symbols
    for (token_addr, close) in grouped_tokens {
        if let Some(index) = token_addresses.iter().position(|x| x == &token_addr) {
            if let Some(token_symbol) = supportedfinaltokens.get(index) {
                result.insert(*token_symbol, close);
            }
        }
    }

    // Get real estate token prices (1d period)
    let realestatedata = PriceCandle::find()
        .filter(price_candle::Column::Token.is_in(real_estate_token_address.clone()))
        .filter(price_candle::Column::Period.eq("1d"))
        .filter(price_candle::Column::ChainId.eq(chainid.clone()))
        .order_by_desc(price_candle::Column::Timestamp)
        .all(db)
        .await;

    let real_estate_data = match realestatedata {
        Ok(data) => data,
        Err(_) => panic!("Error: Cannot get data form DB"),
    };

    // Group by token and get latest close price
    let mut grouped_real_estate = HashMap::new();

    for candle in real_estate_data {
        grouped_real_estate
            .entry(candle.token.clone())
            .or_insert(candle.close);
    }

    // Map to token symbols
    for (token_addr, close) in grouped_real_estate {
        if let Some(index) = real_estate_token_address.iter().position(|x| x == &token_addr) {
            if let Some(token_symbol) = supportedrealestatetokens.get(index) {
                result.insert(token_symbol, close);
            }
        }
    }

    Ok(result)
}

pub async fn calculate_price_decimals(price: f32) -> Option<usize> {
    if price > 1.0 {
        return Some(PRICE_DECIMALS);
    } else {
        let price_string = price.to_string();

        if let Some(starting_index_exponential) = price_string.find("e") {
            let exponent_char = price_string.chars().nth(starting_index_exponential + 2)?;

            let exponent_digit = exponent_char.to_digit(10)? as usize;

            Some(exponent_digit - 1 + PRICE_DECIMALS)
        } else if let Some(starting_index) = price_string.find(".") {
            let mut trailing_zeroes = 0;
            for c in price_string[starting_index + 1..].chars() {
                match c {
                    '0' => trailing_zeroes += 1,
                    _ => break,
                }
            }

            Some(trailing_zeroes + PRICE_DECIMALS)
        } else {
            return Some(0);
        }
    }
}

pub async fn get_token_prices_filtered(db: &DatabaseConnection) -> Vec<AssetPricingInfo2> {
    let token_prices = match gettokenpricesfromdb(db).await {
        Ok(data) => data,
        Err(_) => panic!("Error : Cannot get token prices form DB"),
    };

    let mut token_prices_array = Vec::new();

    let timestamp = Utc::now();
    for (token, price) in token_prices {
        let asset_price: i32 = (price * 10f32.powi(PRECISION)).round() as i32;
        let asset_price_bigint = BigInt::from(asset_price);
        let asset_decimals = match SYMBOL_TO_DECIMAL_MAPPING.get(token) {
            Some(&decimals) => decimals,
            None => panic!("Error: Asset not found in mapping"),
        };

        let power = asset_decimals as i32 - PRECISION as i32;
        let scale_factor = BigInt::from(10).pow(power as u32);

        let token_price = asset_price_bigint * scale_factor;

        let token_prices_filtered = AssetPricingInfo2 {
            token_address: SYMBOL_TO_ADDRESS_MAPPING.get(token).unwrap().to_string(),
            token_symbol: token.to_string(),
            min_price: Some(token_price.to_string()),
            max_price: Some(token_price.to_string()),
            updated_at: timestamp,
            price_decimals: calculate_price_decimals(price).await.unwrap() as f32,
        };

        token_prices_array.push(token_prices_filtered);
    }
    token_prices_array
}

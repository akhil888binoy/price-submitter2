# 🧠 Oracle Price Submitter & Keeper (Rust)

A high-performance price oracle system that fetches real-time token prices from **Pyth Network** and stores them in a PostgreSQL database using **SeaORM**. This system combines the functionality of the original **Price Submitter** and **Pricing Keeper**.


## 📦 Architecture Overview

- **Language**: Rust  
- **Database**: PostgreSQL  
- **ORM**: SeaORM  
- **Price Source**: Pyth Network  
- **Data Granularity**: Supports candle generation for `5m`, `15m`, `1h`, `4h`, and `1d` intervals.  
- **Chains Supported**:
  - Bitlayer Testnet


## ⚙️ Features

- Submits price of tokens to DB every 5 seconds.
- Built-in support for **token candles** over multiple timeframes.
- Uses **Pyth price feeds** (ideal for tokens not listed on CEX).
- Easily scalable to support new tokens or chains.
- Maintains data indexed by timestamp and chain ID for quick retrieval.


## 🚀 Getting Started

### 1. Install Dependencies

```bash
cargo build
```
### 2. Run the Price Submitter
```bash
cargo run 

```
## 🛠 Updating Assets or Chains

#### To add a new asset or chain:

### Update asset & chain info in:
- `src/utils/helpers.rs`

    - `SUPPORTED_TOKENS`

    - `CHAINID_MAP`

    - `SUPPORTED_TOKENS_NO_CEX_FEED`

## 📚 DB Schema (PostgreSQL)

Check : `entity/src/price_candle.rs`




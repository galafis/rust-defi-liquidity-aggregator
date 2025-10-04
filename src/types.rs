use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Token {
    pub address: String,
    pub symbol: String,
    pub decimals: u8,
}

impl Token {
    pub fn new(address: impl Into<String>, symbol: impl Into<String>, decimals: u8) -> Self {
        Self {
            address: address.into(),
            symbol: symbol.into(),
            decimals,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapRequest {
    pub token_in: Token,
    pub token_out: Token,
    pub amount_in: Decimal,
    pub slippage_tolerance: Decimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwapRoute {
    pub dex: String,
    pub path: Vec<Token>,
    pub expected_output: Decimal,
    pub price_impact: Decimal,
    pub gas_estimate: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiquidityPool {
    pub dex: String,
    pub token_a: Token,
    pub token_b: Token,
    pub reserve_a: Decimal,
    pub reserve_b: Decimal,
    pub fee: Decimal,
}

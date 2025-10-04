use super::DEX;
use crate::types::{LiquidityPool, Token};
use anyhow::Result;
use async_trait::async_trait;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

pub struct SushiSwap;

impl SushiSwap {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl DEX for SushiSwap {
    fn name(&self) -> &str {
        "SushiSwap"
    }

    async fn get_pool(&self, token_a: &Token, token_b: &Token) -> Result<LiquidityPool> {
        Ok(LiquidityPool {
            dex: self.name().to_string(),
            token_a: token_a.clone(),
            token_b: token_b.clone(),
            reserve_a: dec!(950000),
            reserve_b: dec!(48000000),
            fee: dec!(0.003),
        })
    }

    async fn quote(&self, amount_in: Decimal, pool: &LiquidityPool) -> Result<Decimal> {
        let amount_in_with_fee = amount_in * (dec!(1) - pool.fee);
        let numerator = amount_in_with_fee * pool.reserve_b;
        let denominator = pool.reserve_a + amount_in_with_fee;
        Ok(numerator / denominator)
    }
}

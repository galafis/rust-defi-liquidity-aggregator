use super::DEX;
use crate::types::{LiquidityPool, Token};
use anyhow::Result;
use async_trait::async_trait;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

pub struct Curve;

impl Curve {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl DEX for Curve {
    fn name(&self) -> &str {
        "Curve Finance"
    }

    async fn get_pool(&self, token_a: &Token, token_b: &Token) -> Result<LiquidityPool> {
        Ok(LiquidityPool {
            dex: self.name().to_string(),
            token_a: token_a.clone(),
            token_b: token_b.clone(),
            reserve_a: dec!(1100000),
            reserve_b: dec!(52000000),
            fee: dec!(0.0004),
        })
    }

    async fn quote(&self, amount_in: Decimal, pool: &LiquidityPool) -> Result<Decimal> {
        // Simplified Curve pricing (actual uses StableSwap invariant)
        let amount_in_with_fee = amount_in * (dec!(1) - pool.fee);
        let numerator = amount_in_with_fee * pool.reserve_b;
        let denominator = pool.reserve_a + amount_in_with_fee;
        Ok(numerator / denominator)
    }
}

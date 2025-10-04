pub mod uniswap;
pub mod sushiswap;
pub mod curve;

use crate::types::{LiquidityPool, Token};
use anyhow::Result;
use async_trait::async_trait;
use rust_decimal::Decimal;

#[async_trait]
pub trait DEX: Send + Sync {
    fn name(&self) -> &str;
    async fn get_pool(&self, token_a: &Token, token_b: &Token) -> Result<LiquidityPool>;
    async fn quote(&self, amount_in: Decimal, pool: &LiquidityPool) -> Result<Decimal>;
}

pub fn create_dexs() -> Vec<Box<dyn DEX>> {
    vec![
        Box::new(uniswap::Uniswap::new()),
        Box::new(sushiswap::SushiSwap::new()),
        Box::new(curve::Curve::new()),
    ]
}

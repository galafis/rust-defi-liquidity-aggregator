use rust_decimal::Decimal;
use rust_decimal_macros::dec;

pub struct GasEstimator;

impl GasEstimator {
    pub fn new() -> Self {
        Self
    }

    pub fn estimate_swap_gas(&self, dex: &str) -> u64 {
        match dex {
            "Uniswap V3" => 150000,
            "SushiSwap" => 140000,
            "Curve Finance" => 200000,
            _ => 160000,
        }
    }

    pub fn calculate_gas_cost(&self, gas_used: u64, gas_price_gwei: Decimal) -> Decimal {
        Decimal::from(gas_used) * gas_price_gwei / dec!(1_000_000_000)
    }
}

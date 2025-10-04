use crate::types::SwapRoute;
use anyhow::Result;

pub struct Simulator;

impl Simulator {
    pub fn new() -> Self {
        Self
    }

    pub async fn simulate_swap(&self, route: &SwapRoute) -> Result<bool> {
        log::info!("Simulating swap on {}", route.dex);
        log::info!("Expected output: {}", route.expected_output);
        log::info!("Gas estimate: {}", route.gas_estimate);
        Ok(true)
    }
}

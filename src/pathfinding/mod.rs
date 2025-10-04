use crate::dex::DEX;
use crate::types::{SwapRequest, SwapRoute};
use anyhow::Result;
use rust_decimal_macros::dec;

pub struct PathFinder {
    dexs: Vec<Box<dyn DEX>>,
}

impl PathFinder {
    pub fn new(dexs: Vec<Box<dyn DEX>>) -> Self {
        Self { dexs }
    }

    pub async fn find_best_route(&self, request: &SwapRequest) -> Result<SwapRoute> {
        let mut best_route: Option<SwapRoute> = None;
        let mut best_output = dec!(0);

        for dex in &self.dexs {
            if let Ok(pool) = dex.get_pool(&request.token_in, &request.token_out).await {
                if let Ok(output) = dex.quote(request.amount_in, &pool).await {
                    if output > best_output {
                        best_output = output;
                        best_route = Some(SwapRoute {
                            dex: dex.name().to_string(),
                            path: vec![request.token_in.clone(), request.token_out.clone()],
                            expected_output: output,
                            price_impact: dec!(0.5),
                            gas_estimate: 150000,
                        });
                    }
                }
            }
        }

        best_route.ok_or_else(|| anyhow::anyhow!("No route found"))
    }
}

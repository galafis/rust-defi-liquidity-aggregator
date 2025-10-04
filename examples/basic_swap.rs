use anyhow::Result;
use defi_liquidity_aggregator::*;
use rust_decimal_macros::dec;

#[tokio::main]
async fn main() -> Result<()> {
    println!("=== DeFi Liquidity Aggregator - Basic Swap Example ===\n");

    // Create DEXs
    let dexs = dex::create_dexs();
    println!("✓ Connected to {} DEXs", dexs.len());

    // Create pathfinder
    let pathfinder = pathfinding::PathFinder::new(dexs);

    // Create swap request: 10 WETH -> USDC
    let request = SwapRequest {
        token_in: Token::new("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2", "WETH", 18),
        token_out: Token::new("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", "USDC", 6),
        amount_in: dec!(10.0),
        slippage_tolerance: dec!(0.5),
    };

    println!("\n📊 Swap Request:");
    println!("  From: {} {}", request.amount_in, request.token_in.symbol);
    println!("  To: {}", request.token_out.symbol);
    println!("  Slippage Tolerance: {}%", request.slippage_tolerance);

    // Find best route
    match pathfinder.find_best_route(&request).await {
        Ok(route) => {
            println!("\n✅ Best Route Found!");
            println!("  DEX: {}", route.dex);
            println!("  Expected Output: {} {}", route.expected_output, request.token_out.symbol);
            println!("  Price Impact: {}%", route.price_impact);
            println!("  Gas Estimate: {} gas", route.gas_estimate);

            // Calculate gas cost
            let gas_estimator = gas::GasEstimator::new();
            let gas_price = dec!(30); // 30 Gwei
            let gas_cost = gas_estimator.calculate_gas_cost(route.gas_estimate, gas_price);
            println!("  Gas Cost: {} ETH (at {} Gwei)", gas_cost, gas_price);
        }
        Err(e) => {
            println!("\n❌ Error: {}", e);
        }
    }

    println!("\n=== Example Complete ===");
    Ok(())
}

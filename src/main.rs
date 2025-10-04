use anyhow::Result;
use defi_liquidity_aggregator::*;
use rust_decimal_macros::dec;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    println!("=== DeFi Liquidity Aggregator Demo ===\n");

    // Create DEXs
    let dexs = dex::create_dexs();
    println!("Connected to {} DEXs", dexs.len());

    // Create pathfinder
    let pathfinder = pathfinding::PathFinder::new(dexs);

    // Create swap request
    let request = SwapRequest {
        token_in: Token::new("0x...WETH", "WETH", 18),
        token_out: Token::new("0x...USDC", "USDC", 6),
        amount_in: dec!(10.0),
        slippage_tolerance: dec!(0.5),
    };

    println!("\n--- Finding Best Route ---");
    println!("Swapping {} {} for {}", request.amount_in, request.token_in.symbol, request.token_out.symbol);

    match pathfinder.find_best_route(&request).await {
        Ok(route) => {
            println!("\n✓ Best route found!");
            println!("DEX: {}", route.dex);
            println!("Expected output: {} {}", route.expected_output, request.token_out.symbol);
            println!("Price impact: {}%", route.price_impact);
            println!("Gas estimate: {} gas", route.gas_estimate);

            // Simulate swap
            println!("\n--- Simulating Swap ---");
            let simulator = simulation::Simulator::new();
            match simulator.simulate_swap(&route).await {
                Ok(_) => println!("✓ Simulation successful!"),
                Err(e) => println!("✗ Simulation failed: {}", e),
            }

            // Calculate gas cost
            let gas_estimator = gas::GasEstimator::new();
            let gas_price = dec!(50); // 50 Gwei
            let gas_cost = gas_estimator.calculate_gas_cost(route.gas_estimate, gas_price);
            println!("\nEstimated gas cost: {} ETH (at {} Gwei)", gas_cost, gas_price);
        }
        Err(e) => println!("✗ Error finding route: {}", e),
    }

    println!("\n=== Demo Complete ===");
    Ok(())
}

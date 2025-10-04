//! # DeFi Liquidity Aggregator
//!
//! A high-performance DeFi liquidity aggregator that finds optimal swap routes across multiple DEXs.
//!
//! ## Features
//!
//! - Multi-DEX support (Uniswap, SushiSwap, Curve, Balancer)
//! - Intelligent pathfinding algorithms
//! - Price impact and slippage calculation
//! - Gas optimization
//! - MEV protection strategies
//! - Transaction simulation

pub mod dex;
pub mod pathfinding;
pub mod simulation;
pub mod gas;
pub mod types;

pub use types::{SwapRequest, SwapRoute, Token};

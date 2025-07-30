//! 
//! 

use frame_support::weights::Weight;

pub mod orium_token_weights {
	use super::*;
	
	pub fn transfer() -> Weight {
		Weight::from_parts(65_000_000, 3_593) // Optimized for high-frequency transfers
	}
	
	pub fn mint() -> Weight {
		Weight::from_parts(45_000_000, 2_789)
	}
	
	pub fn burn() -> Weight {
		Weight::from_parts(42_000_000, 2_456)
	}
	
	pub fn approve() -> Weight {
		Weight::from_parts(38_000_000, 2_234)
	}
	
	pub fn transfer_from() -> Weight {
		Weight::from_parts(78_000_000, 4_123)
	}
}

pub mod collateral_engine_weights {
	use super::*;
	
	pub fn create_cdp() -> Weight {
		Weight::from_parts(125_000_000, 8_456)
	}
	
	pub fn deposit_collateral() -> Weight {
		Weight::from_parts(89_000_000, 5_234)
	}
	
	pub fn withdraw_collateral() -> Weight {
		Weight::from_parts(112_000_000, 6_789)
	}
	
	pub fn mint_dusd() -> Weight {
		Weight::from_parts(98_000_000, 6_789)
	}
	
	pub fn mint_deur() -> Weight {
		Weight::from_parts(98_000_000, 6_789)
	}
	
	pub fn liquidate() -> Weight {
		Weight::from_parts(156_000_000, 12_345)
	}
	
	pub fn update_price() -> Weight {
		Weight::from_parts(67_000_000, 4_567)
	}
	
	pub fn collect_stability_fee() -> Weight {
		Weight::from_parts(78_000_000, 5_432)
	}
}

pub mod stablecoin_weights {
	use super::*;
	
	pub fn transfer() -> Weight {
		Weight::from_parts(58_000_000, 3_234)
	}
	
	pub fn mint() -> Weight {
		Weight::from_parts(52_000_000, 2_987)
	}
	
	pub fn burn() -> Weight {
		Weight::from_parts(49_000_000, 2_765)
	}
	
	pub fn approve() -> Weight {
		Weight::from_parts(41_000_000, 2_456)
	}
	
	pub fn transfer_from() -> Weight {
		Weight::from_parts(71_000_000, 3_987)
	}
}

pub mod batch_weights {
	use super::*;
	
	pub fn batch_transfer(count: u32) -> Weight {
		Weight::from_parts(
			15_000_000 + (count as u64 * 58_000_000), // Base cost + per-transfer cost
			1_234 + (count as u64 * 3_234) // Proof size scaling
		)
	}
	
	pub fn batch_mint(count: u32) -> Weight {
		Weight::from_parts(
			25_000_000 + (count as u64 * 52_000_000),
			1_567 + (count as u64 * 2_987)
		)
	}
	
	pub fn batch_liquidate(count: u32) -> Weight {
		Weight::from_parts(
			50_000_000 + (count as u64 * 156_000_000),
			2_345 + (count as u64 * 12_345)
		)
	}
	
	pub fn batch_create_cdp(count: u32) -> Weight {
		Weight::from_parts(
			30_000_000 + (count as u64 * 125_000_000),
			1_890 + (count as u64 * 8_456)
		)
	}
	
	pub fn batch_update_prices(count: u32) -> Weight {
		Weight::from_parts(
			20_000_000 + (count as u64 * 67_000_000),
			1_456 + (count as u64 * 4_567)
		)
	}
}

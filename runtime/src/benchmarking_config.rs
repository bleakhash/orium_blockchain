//! 

use frame_support::weights::Weight;

pub mod orium_token_weights {
	use super::*;
	
	pub fn transfer() -> Weight {
		Weight::from_parts(50_000, 0)
	}
	
	pub fn mint() -> Weight {
		Weight::from_parts(75_000, 0)
	}
	
	pub fn burn() -> Weight {
		Weight::from_parts(60_000, 0)
	}
}

pub mod collateral_engine_weights {
	use super::*;
	
	pub fn create_cdp() -> Weight {
		Weight::from_parts(200_000, 0)
	}
	
	pub fn deposit_collateral() -> Weight {
		Weight::from_parts(150_000, 0)
	}
	
	pub fn withdraw_collateral() -> Weight {
		Weight::from_parts(200_000, 0)
	}
	
	pub fn mint_dusd() -> Weight {
		Weight::from_parts(180_000, 0)
	}
	
	pub fn mint_deur() -> Weight {
		Weight::from_parts(180_000, 0)
	}
	
	pub fn liquidate() -> Weight {
		Weight::from_parts(300_000, 0)
	}
	
	pub fn update_price() -> Weight {
		Weight::from_parts(100_000, 0)
	}
}

pub mod stablecoin_weights {
	use super::*;
	
	pub fn transfer() -> Weight {
		Weight::from_parts(45_000, 0)
	}
	
	pub fn mint() -> Weight {
		Weight::from_parts(100_000, 0)
	}
	
	pub fn burn() -> Weight {
		Weight::from_parts(80_000, 0)
	}
	
	pub fn approve() -> Weight {
		Weight::from_parts(40_000, 0)
	}
}

pub mod batch_weights {
	use super::*;
	
	pub fn batch_transfer(count: u32) -> Weight {
		Weight::from_parts(50_000 + (count * 45_000) as u64, 0)
	}
	
	pub fn batch_mint(count: u32) -> Weight {
		Weight::from_parts(100_000 + (count * 75_000) as u64, 0)
	}
	
	pub fn batch_liquidate(count: u32) -> Weight {
		Weight::from_parts(300_000 + (count * 250_000) as u64, 0)
	}
}

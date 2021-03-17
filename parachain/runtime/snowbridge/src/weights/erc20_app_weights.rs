
//! Autogenerated weights for erc20_app
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-03-17, STEPS: [50, ], REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: None, WASM-EXECUTION: Interpreted, CHAIN: Some("spec.json"), DB CACHE: 128

// Executed Command:
// target/release/artemis
// benchmark
// --chain
// spec.json
// --pallet
// erc20_app
// --extrinsic
// *
// --repeat
// 20
// --steps
// 50
// --output
// runtime/snowbridge/src/weights/erc20_app_weights.rs


#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for erc20_app.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> erc20_app::WeightInfo for WeightInfo<T> {
	fn burn() -> Weight {
		(27_500_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn mint() -> Weight {
		(15_818_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}

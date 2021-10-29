//! Autogenerated weights for incentivized_channel::outbound
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-05-08, STEPS: `[50, ]`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN:
//! Some("/tmp/snowbridge-benchmark-bNy/spec.json"), DB CACHE: 128

// Executed Command:
// target/release/snowbridge
// benchmark
// --chain
// /tmp/snowbridge-benchmark-bNy/spec.json
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// incentivized_channel::outbound
// --extrinsic
// *
// --repeat
// 20
// --steps
// 50
// --output
// runtime/rococo/src/weights/incentivized_channel_outbound_weights.rs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for incentivized_channel::outbound.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> incentivized_channel::outbound::WeightInfo for WeightInfo<T> {
	fn on_initialize(m: u32, p: u32) -> Weight {
		(0 as Weight)
			// Standard Error: 78_000
			.saturating_add((25_162_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 4_000
			.saturating_add((948_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn on_initialize_non_interval() -> Weight {
		(5_021_000 as Weight).saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	fn on_initialize_no_messages() -> Weight {
		(8_561_000 as Weight).saturating_add(T::DbWeight::get().reads(2 as Weight))
	}
	fn set_fee() -> Weight {
		(3_000_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

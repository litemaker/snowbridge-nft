//! Autogenerated weights for pallet_collective
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
// pallet_collective
// --extrinsic
// *
// --repeat
// 20
// --steps
// 50
// --output
// runtime/rococo/src/weights/pallet_collective_weights.rs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for pallet_collective.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collective::WeightInfo for WeightInfo<T> {
	fn set_members(m: u32, n: u32, p: u32) -> Weight {
		(0 as Weight)
			// Standard Error: 196_000
			.saturating_add((19_824_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 196_000
			.saturating_add((391_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 6_000
			.saturating_add((10_318_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(p as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(p as Weight)))
	}
	fn execute(b: u32, m: u32) -> Weight {
		(31_301_000 as Weight)
			// Standard Error: 0
			.saturating_add((4_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 4_000
			.saturating_add((140_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	fn propose_execute(b: u32, m: u32) -> Weight {
		(38_501_000 as Weight)
			// Standard Error: 0
			.saturating_add((4_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 6_000
			.saturating_add((304_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
	}
	fn propose_proposed(b: u32, m: u32, p: u32) -> Weight {
		(59_505_000 as Weight)
			// Standard Error: 0
			.saturating_add((10_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 97_000
			.saturating_add((229_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 2_000
			.saturating_add((693_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn vote(m: u32) -> Weight {
		(57_440_000 as Weight)
			// Standard Error: 34_000
			.saturating_add((494_000 as Weight).saturating_mul(m as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn close_early_disapproved(m: u32, p: u32) -> Weight {
		(55_501_000 as Weight)
			// Standard Error: 58_000
			.saturating_add((686_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 1_000
			.saturating_add((739_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn close_early_approved(b: u32, m: u32, p: u32) -> Weight {
		(86_458_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 162_000
			.saturating_add((760_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 2_000
			.saturating_add((669_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn close_disapproved(m: u32, p: u32) -> Weight {
		(62_231_000 as Weight)
			// Standard Error: 55_000
			.saturating_add((749_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 0
			.saturating_add((737_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn close_approved(b: u32, m: u32, p: u32) -> Weight {
		(86_754_000 as Weight)
			// Standard Error: 0
			.saturating_add((5_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 165_000
			.saturating_add((1_205_000 as Weight).saturating_mul(m as Weight))
			// Standard Error: 2_000
			.saturating_add((682_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn disapprove_proposal(p: u32) -> Weight {
		(34_756_000 as Weight)
			// Standard Error: 0
			.saturating_add((718_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
}

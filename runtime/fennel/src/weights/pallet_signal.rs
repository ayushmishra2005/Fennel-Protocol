
//! Autogenerated weights for `pallet_signal`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-19, STEPS: `100`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("fennel-local"), DB CACHE: 1024

// Executed Command:
// ./target/release/fennel-node
// benchmark
// pallet
// --chain=fennel-local
// --execution=wasm
// --wasm-execution=compiled
// --pallet=pallet_signal
// --extrinsic=*
// --steps=100
// --repeat=10
// --template=./scripts/templates/parachain-weight-template.hbs
// --output=./runtime/fennel/src/weights

#![allow(unused_parens, unused_imports)]
#![allow(clippy::unnecessary_cast, clippy::missing_docs_in_private_items)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_signal` using the recommended hardware.
pub struct WeightInfo<T>(pub PhantomData<T>);
impl<T: frame_system::Config> pallet_signal::WeightInfo for WeightInfo<T> {
	// Storage: Signal RatingSignalList (r:0 w:1)
	// Proof Skipped: Signal RatingSignalList (max_values: None, max_size: None, mode: Measured)
	fn send_rating_signal() -> Weight {
		Weight::from_ref_time(10_000_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Signal RatingSignalList (r:0 w:1)
	// Proof Skipped: Signal RatingSignalList (max_values: None, max_size: None, mode: Measured)
	fn update_rating_signal() -> Weight {
		Weight::from_ref_time(10_000_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Signal RatingSignalList (r:0 w:1)
	// Proof Skipped: Signal RatingSignalList (max_values: None, max_size: None, mode: Measured)
	fn revoke_rating_signal() -> Weight {
		Weight::from_ref_time(9_000_000 as u64)
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	fn send_signal() -> Weight {
		Weight::from_ref_time(7_000_000 as u64)
	}
	fn send_service_signal() -> Weight {
		Weight::from_ref_time(8_000_000 as u64)
	}
}

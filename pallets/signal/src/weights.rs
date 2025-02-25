//! Autogenerated weights for `pallet_signal`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-22, STEPS: `100`, REPEAT: 100, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// --repeat=100
// --template=./scripts/templates/parachain-weight-template.hbs
// --output=./runtime/fennel/src/weights

#![allow(unused_parens, unused_imports)]
#![allow(clippy::unnecessary_cast, clippy::missing_docs_in_private_items)]

use core::marker::PhantomData;
use frame_support::{
    traits::Get,
    weights::{constants::RocksDbWeight, Weight},
};

/// Weight functions needed for pallet_signal.
pub trait WeightInfo {
    fn set_signal_parameter() -> Weight;
    fn send_rating_signal() -> Weight;
    fn update_rating_signal() -> Weight;
    fn revoke_rating_signal() -> Weight;
    fn send_whiteflag_rating_signal() -> Weight;
    fn send_signal() -> Weight;
    fn update_whiteflag_rating_signal() -> Weight;
    fn send_service_signal() -> Weight;
    fn revoke_whiteflag_rating_signal() -> Weight;
}

/// Weight functions for pallet_trust.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn set_signal_parameter() -> Weight {
        Weight::from_parts(7_000_000, 0)
    }

    fn send_rating_signal() -> Weight {
        Weight::from_parts(7_000_000, 0)
    }

    fn send_whiteflag_rating_signal() -> Weight {
        Weight::from_parts(7_000_000, 0)
    }

    fn update_rating_signal() -> Weight {
        Weight::from_parts(7_000_000, 0)
    }

    fn update_whiteflag_rating_signal() -> Weight {
        Weight::from_parts(7_000_000, 0)
    }

    fn revoke_rating_signal() -> Weight {
        Weight::from_parts(7_000_000, 0)
    }

    fn revoke_whiteflag_rating_signal() -> Weight {
        Weight::from_parts(7_000_000, 0)
    }

    // Storage: TrustModule TrustIssuance (r:1 w:1)
    // Storage: TrustModule CurrentIssued (r:1 w:1)
    fn send_signal() -> Weight {
        // Minimum execution time: 12_000 nanoseconds.
        Weight::from_parts(12_000_000, 0)
    }
    fn send_service_signal() -> Weight {
        // Minimum execution time: 12_000 nanoseconds.
        Weight::from_parts(12_000_000, 0)
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    fn set_signal_parameter() -> Weight {
        Weight::from_parts(7_000_000, 0)
    }

    fn send_rating_signal() -> Weight {
        Weight::from_parts(7_000_000, 0)
    }

    fn send_whiteflag_rating_signal() -> Weight {
        Weight::from_parts(7_000_000, 0)
    }

    fn update_rating_signal() -> Weight {
        Weight::from_parts(7_000_000, 0)
    }

    fn update_whiteflag_rating_signal() -> Weight {
        Weight::from_parts(7_000_000, 0)
    }

    fn revoke_rating_signal() -> Weight {
        Weight::from_parts(7_000_000, 0)
    }

    fn revoke_whiteflag_rating_signal() -> Weight {
        Weight::from_parts(7_000_000, 0)
    }

    // Storage: TrustModule TrustIssuance (r:1 w:1)
    // Storage: TrustModule CurrentIssued (r:1 w:1)
    fn send_signal() -> Weight {
        // Minimum execution time: 12_000 nanoseconds.
        Weight::from_parts(12_000_000, 0)
    }
    fn send_service_signal() -> Weight {
        // Minimum execution time: 12_000 nanoseconds.
        Weight::from_parts(12_000_000, 0)
    }
}

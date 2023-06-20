// Copyright (C) Moondance Labs Ltd.
// This file is part of Tanssi.

// Tanssi is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Tanssi is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Tanssi.  If not, see <http://www.gnu.org/licenses/>


//! Autogenerated weights for pallet_cc_authorities_noting
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-16, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `tomasz-XPS-15-9520`, CPU: `12th Gen Intel(R) Core(TM) i7-12700H`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ./target/release/container-chain-template-simple-node
// benchmark
// pallet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// 
// --extrinsic
// 
// --steps
// 50
// --repeat
// 20
// --template=./benchmarking/frame-weight-template.hbs
// --json-file
// raw.json
// --output
// weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_cc_authorities_noting.
pub trait WeightInfo {
	fn set_latest_authorities_data() -> Weight;
	fn set_authorities(x: u32, ) -> Weight;
}

/// Weights for pallet_cc_authorities_noting using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: AuthoritiesNoting DidSetOrchestratorAuthorityData (r:1 w:1)
	/// Proof Skipped: AuthoritiesNoting DidSetOrchestratorAuthorityData (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem ValidationData (r:1 w:0)
	/// Proof Skipped: ParachainSystem ValidationData (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainInfo ParachainId (r:1 w:0)
	/// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: AuthoritiesNoting Authorities (r:0 w:1)
	/// Proof Skipped: AuthoritiesNoting Authorities (max_values: Some(1), max_size: None, mode: Measured)
	fn set_latest_authorities_data() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `110`
		//  Estimated: `4789`
		// Minimum execution time: 21_879_000 picoseconds.
		Weight::from_parts(22_853_000, 4789)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: AuthoritiesNoting Authorities (r:0 w:1)
	/// Proof Skipped: AuthoritiesNoting Authorities (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `x` is `[0, 10]`.
	fn set_authorities(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_754_000 picoseconds.
		Weight::from_parts(6_736_052, 0)
			// Standard Error: 27_808
			.saturating_add(Weight::from_parts(135_701, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: AuthoritiesNoting DidSetOrchestratorAuthorityData (r:1 w:1)
	/// Proof Skipped: AuthoritiesNoting DidSetOrchestratorAuthorityData (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainSystem ValidationData (r:1 w:0)
	/// Proof Skipped: ParachainSystem ValidationData (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParachainInfo ParachainId (r:1 w:0)
	/// Proof: ParachainInfo ParachainId (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: AuthoritiesNoting Authorities (r:0 w:1)
	/// Proof Skipped: AuthoritiesNoting Authorities (max_values: Some(1), max_size: None, mode: Measured)
	fn set_latest_authorities_data() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `110`
		//  Estimated: `4789`
		// Minimum execution time: 21_879_000 picoseconds.
		Weight::from_parts(22_853_000, 4789)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: AuthoritiesNoting Authorities (r:0 w:1)
	/// Proof Skipped: AuthoritiesNoting Authorities (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `x` is `[0, 10]`.
	fn set_authorities(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_754_000 picoseconds.
		Weight::from_parts(6_736_052, 0)
			// Standard Error: 27_808
			.saturating_add(Weight::from_parts(135_701, 0).saturating_mul(x.into()))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
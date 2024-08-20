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


//! Autogenerated weights for pallet_xcm_executor_utils
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-08-05, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `benchmark-1`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/container-chain-simple-node
// benchmark
// pallet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_xcm_executor_utils
// --extrinsic
// *
// --chain=dev
// --steps
// 50
// --repeat
// 20
// --template=benchmarking/frame-weight-runtime-template.hbs
// --json-file
// raw.json
// --output
// tmp/simple_template_weights/pallet_xcm_executor_utils.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for pallet_xcm_executor_utils using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_xcm_executor_utils::WeightInfo for SubstrateWeight<T> {
	/// Storage: `XcmExecutorUtils::ReservePolicy` (r:0 w:1)
	/// Proof: `XcmExecutorUtils::ReservePolicy` (`max_values`: None, `max_size`: Some(602621), added: 605096, mode: `MaxEncodedLen`)
	fn set_reserve_policy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 7_771_000 picoseconds.
		Weight::from_parts(8_073_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `XcmExecutorUtils::ReservePolicy` (r:1 w:1)
	/// Proof: `XcmExecutorUtils::ReservePolicy` (`max_values`: None, `max_size`: Some(602621), added: 605096, mode: `MaxEncodedLen`)
	fn remove_reserve_policy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `87`
		//  Estimated: `606086`
		// Minimum execution time: 12_339_000 picoseconds.
		Weight::from_parts(12_704_000, 606086)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `XcmExecutorUtils::TeleportPolicy` (r:0 w:1)
	/// Proof: `XcmExecutorUtils::TeleportPolicy` (`max_values`: None, `max_size`: Some(602621), added: 605096, mode: `MaxEncodedLen`)
	fn set_teleport_policy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 7_725_000 picoseconds.
		Weight::from_parts(8_128_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `XcmExecutorUtils::TeleportPolicy` (r:1 w:1)
	/// Proof: `XcmExecutorUtils::TeleportPolicy` (`max_values`: None, `max_size`: Some(602621), added: 605096, mode: `MaxEncodedLen`)
	fn remove_teleport_policy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `87`
		//  Estimated: `606086`
		// Minimum execution time: 12_208_000 picoseconds.
		Weight::from_parts(12_625_000, 606086)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}
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


//! Autogenerated weights for pallet_assets
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
// pallet_assets
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
// tmp/simple_template_weights/pallet_assets.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for pallet_assets using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_assets::WeightInfo for SubstrateWeight<T> {
	fn create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 0_000 picoseconds.
		Weight::from_parts(0, 0)
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	fn force_create() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3`
		//  Estimated: `3673`
		// Minimum execution time: 12_193_000 picoseconds.
		Weight::from_parts(12_579_000, 3673)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	fn start_destroy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `273`
		//  Estimated: `3673`
		// Minimum execution time: 12_273_000 picoseconds.
		Weight::from_parts(12_798_000, 3673)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Account` (r:1001 w:1000)
	/// Proof: `ForeignAssets::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1000 w:1000)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[0, 1000]`.
	fn destroy_accounts(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `79 + c * (208 ±0)`
		//  Estimated: `3673 + c * (2607 ±0)`
		// Minimum execution time: 18_036_000 picoseconds.
		Weight::from_parts(18_350_000, 3673)
			// Standard Error: 13_361
			.saturating_add(Weight::from_parts(15_309_636, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 2607).saturating_mul(c.into()))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Approvals` (r:1001 w:1000)
	/// Proof: `ForeignAssets::Approvals` (`max_values`: None, `max_size`: Some(146), added: 2621, mode: `MaxEncodedLen`)
	/// The range of component `a` is `[0, 1000]`.
	fn destroy_approvals(a: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `408 + a * (86 ±0)`
		//  Estimated: `3673 + a * (2621 ±0)`
		// Minimum execution time: 18_326_000 picoseconds.
		Weight::from_parts(18_477_000, 3673)
			// Standard Error: 4_033
			.saturating_add(Weight::from_parts(6_092_968, 0).saturating_mul(a.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(a.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(a.into())))
			.saturating_add(Weight::from_parts(0, 2621).saturating_mul(a.into()))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Metadata` (r:1 w:0)
	/// Proof: `ForeignAssets::Metadata` (`max_values`: None, `max_size`: Some(138), added: 2613, mode: `MaxEncodedLen`)
	fn finish_destroy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `239`
		//  Estimated: `3673`
		// Minimum execution time: 14_579_000 picoseconds.
		Weight::from_parts(14_907_000, 3673)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Account` (r:1 w:1)
	/// Proof: `ForeignAssets::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	fn mint() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `239`
		//  Estimated: `3673`
		// Minimum execution time: 26_571_000 picoseconds.
		Weight::from_parts(27_065_000, 3673)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Account` (r:1 w:1)
	/// Proof: `ForeignAssets::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	fn burn() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `345`
		//  Estimated: `3673`
		// Minimum execution time: 34_918_000 picoseconds.
		Weight::from_parts(35_515_000, 3673)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Account` (r:2 w:2)
	/// Proof: `ForeignAssets::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `384`
		//  Estimated: `6204`
		// Minimum execution time: 49_215_000 picoseconds.
		Weight::from_parts(50_711_000, 6204)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Account` (r:2 w:2)
	/// Proof: `ForeignAssets::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn transfer_keep_alive() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `384`
		//  Estimated: `6204`
		// Minimum execution time: 44_235_000 picoseconds.
		Weight::from_parts(45_056_000, 6204)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Account` (r:2 w:2)
	/// Proof: `ForeignAssets::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn force_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `384`
		//  Estimated: `6204`
		// Minimum execution time: 49_848_000 picoseconds.
		Weight::from_parts(50_315_000, 6204)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:0)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Account` (r:1 w:1)
	/// Proof: `ForeignAssets::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	fn freeze() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `345`
		//  Estimated: `3673`
		// Minimum execution time: 16_956_000 picoseconds.
		Weight::from_parts(17_481_000, 3673)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:0)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Account` (r:1 w:1)
	/// Proof: `ForeignAssets::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	fn thaw() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `345`
		//  Estimated: `3673`
		// Minimum execution time: 17_078_000 picoseconds.
		Weight::from_parts(17_612_000, 3673)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	fn freeze_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `273`
		//  Estimated: `3673`
		// Minimum execution time: 12_388_000 picoseconds.
		Weight::from_parts(12_636_000, 3673)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	fn thaw_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `273`
		//  Estimated: `3673`
		// Minimum execution time: 12_083_000 picoseconds.
		Weight::from_parts(12_435_000, 3673)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Metadata` (r:1 w:0)
	/// Proof: `ForeignAssets::Metadata` (`max_values`: None, `max_size`: Some(138), added: 2613, mode: `MaxEncodedLen`)
	fn transfer_ownership() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `239`
		//  Estimated: `3673`
		// Minimum execution time: 14_985_000 picoseconds.
		Weight::from_parts(15_311_000, 3673)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	fn set_team() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `239`
		//  Estimated: `3673`
		// Minimum execution time: 12_975_000 picoseconds.
		Weight::from_parts(13_354_000, 3673)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:0)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Metadata` (r:1 w:1)
	/// Proof: `ForeignAssets::Metadata` (`max_values`: None, `max_size`: Some(138), added: 2613, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[0, 50]`.
	/// The range of component `s` is `[0, 50]`.
	fn set_metadata(_n: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `239`
		//  Estimated: `3673`
		// Minimum execution time: 15_018_000 picoseconds.
		Weight::from_parts(15_898_598, 3673)
			// Standard Error: 389
			.saturating_add(Weight::from_parts(475, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:0)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Metadata` (r:1 w:1)
	/// Proof: `ForeignAssets::Metadata` (`max_values`: None, `max_size`: Some(138), added: 2613, mode: `MaxEncodedLen`)
	fn clear_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `401`
		//  Estimated: `3673`
		// Minimum execution time: 15_653_000 picoseconds.
		Weight::from_parts(16_082_000, 3673)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:0)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Metadata` (r:1 w:1)
	/// Proof: `ForeignAssets::Metadata` (`max_values`: None, `max_size`: Some(138), added: 2613, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[0, 50]`.
	/// The range of component `s` is `[0, 50]`.
	fn force_set_metadata(_n: u32, _s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `78`
		//  Estimated: `3673`
		// Minimum execution time: 13_669_000 picoseconds.
		Weight::from_parts(14_471_002, 3673)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:0)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Metadata` (r:1 w:1)
	/// Proof: `ForeignAssets::Metadata` (`max_values`: None, `max_size`: Some(138), added: 2613, mode: `MaxEncodedLen`)
	fn force_clear_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `401`
		//  Estimated: `3673`
		// Minimum execution time: 15_313_000 picoseconds.
		Weight::from_parts(15_724_000, 3673)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	fn force_asset_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `239`
		//  Estimated: `3673`
		// Minimum execution time: 12_478_000 picoseconds.
		Weight::from_parts(12_740_000, 3673)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Approvals` (r:1 w:1)
	/// Proof: `ForeignAssets::Approvals` (`max_values`: None, `max_size`: Some(146), added: 2621, mode: `MaxEncodedLen`)
	fn approve_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `273`
		//  Estimated: `3673`
		// Minimum execution time: 20_147_000 picoseconds.
		Weight::from_parts(20_611_000, 3673)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Approvals` (r:1 w:1)
	/// Proof: `ForeignAssets::Approvals` (`max_values`: None, `max_size`: Some(146), added: 2621, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Account` (r:2 w:2)
	/// Proof: `ForeignAssets::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn transfer_approved() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `552`
		//  Estimated: `6204`
		// Minimum execution time: 60_811_000 picoseconds.
		Weight::from_parts(62_151_000, 6204)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Approvals` (r:1 w:1)
	/// Proof: `ForeignAssets::Approvals` (`max_values`: None, `max_size`: Some(146), added: 2621, mode: `MaxEncodedLen`)
	fn cancel_approval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `441`
		//  Estimated: `3673`
		// Minimum execution time: 22_621_000 picoseconds.
		Weight::from_parts(23_134_000, 3673)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Approvals` (r:1 w:1)
	/// Proof: `ForeignAssets::Approvals` (`max_values`: None, `max_size`: Some(146), added: 2621, mode: `MaxEncodedLen`)
	fn force_cancel_approval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `441`
		//  Estimated: `3673`
		// Minimum execution time: 23_256_000 picoseconds.
		Weight::from_parts(23_681_000, 3673)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	fn set_min_balance() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `239`
		//  Estimated: `3673`
		// Minimum execution time: 14_157_000 picoseconds.
		Weight::from_parts(14_508_000, 3673)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ForeignAssets::Account` (r:1 w:1)
	/// Proof: `ForeignAssets::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	fn touch() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `239`
		//  Estimated: `3673`
		// Minimum execution time: 19_919_000 picoseconds.
		Weight::from_parts(20_570_000, 3673)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `ForeignAssets::Account` (r:1 w:1)
	/// Proof: `ForeignAssets::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	fn touch_other() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `239`
		//  Estimated: `3673`
		// Minimum execution time: 18_944_000 picoseconds.
		Weight::from_parts(19_325_000, 3673)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `ForeignAssets::Account` (r:1 w:1)
	/// Proof: `ForeignAssets::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	fn refund() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `363`
		//  Estimated: `3673`
		// Minimum execution time: 18_133_000 picoseconds.
		Weight::from_parts(18_622_000, 3673)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `ForeignAssets::Account` (r:1 w:1)
	/// Proof: `ForeignAssets::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	fn refund_other() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `396`
		//  Estimated: `3673`
		// Minimum execution time: 17_249_000 picoseconds.
		Weight::from_parts(17_630_000, 3673)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `ForeignAssets::Asset` (r:1 w:0)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssets::Account` (r:1 w:1)
	/// Proof: `ForeignAssets::Account` (`max_values`: None, `max_size`: Some(132), added: 2607, mode: `MaxEncodedLen`)
	fn block() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `345`
		//  Estimated: `3673`
		// Minimum execution time: 16_954_000 picoseconds.
		Weight::from_parts(17_454_000, 3673)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}
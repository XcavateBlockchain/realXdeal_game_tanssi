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


//! Autogenerated weights for pallet_foreign_asset_creator
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-08-05, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `benchmark-1`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/tanssi-node
// benchmark
// pallet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_foreign_asset_creator
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
// tmp/dancebox_weights/pallet_foreign_asset_creator.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weights for pallet_foreign_asset_creator using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_foreign_asset_creator::WeightInfo for SubstrateWeight<T> {
	/// Storage: `ForeignAssetsCreator::AssetIdToForeignAsset` (r:1 w:1)
	/// Proof: `ForeignAssetsCreator::AssetIdToForeignAsset` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssetsCreator::ForeignAssetToAssetId` (r:0 w:1)
	/// Proof: `ForeignAssetsCreator::ForeignAssetToAssetId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn create_foreign_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `113`
		//  Estimated: `3673`
		// Minimum execution time: 21_697_000 picoseconds.
		Weight::from_parts(22_129_000, 3673)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `ForeignAssetsCreator::AssetIdToForeignAsset` (r:1 w:1)
	/// Proof: `ForeignAssetsCreator::AssetIdToForeignAsset` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ForeignAssetsCreator::ForeignAssetToAssetId` (r:0 w:2)
	/// Proof: `ForeignAssetsCreator::ForeignAssetToAssetId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn change_existing_asset_type() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `189`
		//  Estimated: `3654`
		// Minimum execution time: 17_485_000 picoseconds.
		Weight::from_parts(17_987_000, 3654)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `ForeignAssetsCreator::AssetIdToForeignAsset` (r:1 w:1)
	/// Proof: `ForeignAssetsCreator::AssetIdToForeignAsset` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ForeignAssetsCreator::ForeignAssetToAssetId` (r:0 w:1)
	/// Proof: `ForeignAssetsCreator::ForeignAssetToAssetId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn remove_existing_asset_type() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `189`
		//  Estimated: `3654`
		// Minimum execution time: 15_709_000 picoseconds.
		Weight::from_parts(16_039_000, 3654)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `ForeignAssetsCreator::AssetIdToForeignAsset` (r:1 w:1)
	/// Proof: `ForeignAssetsCreator::AssetIdToForeignAsset` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ForeignAssets::Asset` (r:1 w:1)
	/// Proof: `ForeignAssets::Asset` (`max_values`: None, `max_size`: Some(208), added: 2683, mode: `MaxEncodedLen`)
	/// Storage: `ForeignAssetsCreator::ForeignAssetToAssetId` (r:0 w:1)
	/// Proof: `ForeignAssetsCreator::ForeignAssetToAssetId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn destroy_foreign_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `429`
		//  Estimated: `3894`
		// Minimum execution time: 24_130_000 picoseconds.
		Weight::from_parts(24_721_000, 3894)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
}
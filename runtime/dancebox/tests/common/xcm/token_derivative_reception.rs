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

use {
    crate::common::xcm::{
        mocknets::{
            Dancebox, DanceboxEmptyReceiver, DanceboxPallet, DanceboxReceiver, DanceboxSender,
            EthereumEmptyReceiver, EthereumSender, FrontierTemplate, FrontierTemplatePallet,
            Westend, WestendPallet, WestendSender,
        },
        *,
    },
    container_chain_template_frontier_runtime::UNIT as FRONTIER_DEV,
    dancebox_runtime::UNIT as DANCE,
    frame_support::{
        assert_ok,
        weights::{Weight, WeightToFee},
    },
    sp_runtime::FixedU128,
    staging_xcm::{latest::prelude::*, VersionedMultiLocation, VersionedXcm},
    staging_xcm_executor::traits::ConvertLocation,
    xcm_emulator::Chain,
};

#[test]
fn receive_tokens_from_the_relay_to_tanssi() {
    // XcmPallet reserve transfer arguments
    let alice_origin = <Westend as Chain>::RuntimeOrigin::signed(WestendSender::get());

    let dancebox_dest: VersionedMultiLocation = MultiLocation {
        parents: 0,
        interior: X1(Parachain(2000u32)),
    }
    .into();

    let dancebox_beneficiary: VersionedMultiLocation = MultiLocation {
        parents: 0,
        interior: X1(AccountId32 {
            network: None,
            id: DanceboxReceiver::get().into(),
        }),
    }
    .into();

    let amount_to_send: crate::Balance = westend_runtime::ExistentialDeposit::get() * 1000;

    let assets: MultiAssets = (Here, amount_to_send).into();
    let fee_asset_item = 0;
    let westend_token_asset_id = 1u16;

    // Register the asset first
    Dancebox::execute_with(|| {
        let root_origin = <Dancebox as Chain>::RuntimeOrigin::root();

        assert_ok!(
            <Dancebox as DanceboxPallet>::ForeignAssetsCreator::create_foreign_asset(
                root_origin.clone(),
                MultiLocation::parent(),
                westend_token_asset_id,
                DanceboxReceiver::get(),
                true,
                1
            )
        );

        assert_ok!(<Dancebox as DanceboxPallet>::AssetRate::create(
            root_origin,
            bx!(1),
            FixedU128::from_u32(1)
        ));
    });

    // Send XCM message from Westend
    Westend::execute_with(|| {
        assert_ok!(
            <Westend as WestendPallet>::XcmPallet::limited_reserve_transfer_assets(
                alice_origin,
                bx!(dancebox_dest),
                bx!(dancebox_beneficiary),
                bx!(assets.into()),
                fee_asset_item,
                WeightLimit::Unlimited,
            )
        );
    });
    // We should have received the tokens
    Dancebox::execute_with(|| {
        type RuntimeEvent = <Dancebox as Chain>::RuntimeEvent;
        assert_expected_events!(
            Dancebox,
            vec![
                RuntimeEvent::DmpQueue(
                    cumulus_pallet_dmp_queue::Event::ExecutedDownward {
                        outcome, ..
                    }) => {
                    outcome: outcome.clone().ensure_complete().is_ok(),
                },
            ]
        );
        type ForeignAssets = <Dancebox as DanceboxPallet>::ForeignAssets;
        // Assert empty receiver received funds
        assert!(
            <ForeignAssets as frame_support::traits::fungibles::Inspect<_>>::balance(
                1,
                &DanceboxReceiver::get(),
            ) > 0
        );
    });
}

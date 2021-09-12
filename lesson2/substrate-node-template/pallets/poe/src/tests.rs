use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;

#[test]
fn claim_should_work() {
    new_test_ext().execute_with(|| {
        let claim = vec![0,1,2];
        let limit = StringLimit::get() as usize;

        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
        assert_eq!(Proofs::<Test>::get(&claim), (1, frame_system::Pallet::<Test>::block_number()));
        // proof string limit length check
        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), vec![0u8; limit + 1]),
            Error::<Test>::BadMetadata
        );
    })
}


#[test]
fn claim_failed_when_claim_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0,1,2];

        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
        assert_noop!(
            PoeModule::create_claim(Origin::signed(2), claim.clone()),
            Error::<Test>::ProofAlreadyClaimed
        );
    })
}

#[test]
fn revoke_should_work() {
    new_test_ext().execute_with(|| {
        let claim = vec![0,1,2];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
        assert_eq!(Proofs::<Test>::contains_key(&claim), false);
    })
}


#[test]
fn revoke_failed_when_claim_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0,1,2];

        assert_noop!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()), Error::<Test>::NoSuchProof);
    })
}

#[test]
fn revoke_failed_when_not_owner() {
    new_test_ext().execute_with(|| {
        let claim = vec![0,1,2];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_noop!(PoeModule::revoke_claim(Origin::signed(2), claim.clone()), Error::<Test>::NotProofOwner);
    })
}

#[test]
fn transfer_should_work() {
    new_test_ext().execute_with(|| {
        let claim = vec![0,1,2];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_ok!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2));
        assert_eq!(Proofs::<Test>::get(&claim), (2, frame_system::Pallet::<Test>::block_number()));
    })
}

#[test]
fn transfer_failed_when_claim_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0,1,2];

        assert_noop!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2), Error::<Test>::NoSuchProof);
    })
}

#[test]
fn transfer_failed_when_not_owner() {
    new_test_ext().execute_with(|| {
        let claim = vec![0,1,2];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_noop!(PoeModule::transfer_claim(Origin::signed(2), claim.clone(), 1), Error::<Test>::NotProofOwner);
    })
}
use crate::{Error, Config, Pallet};
use frame_support::{assert_ok, assert_noop};
use super::*;
use crate::mock::{Event, System, Origin, KittiesModule, Balances,new_test_ext,Test};

#[test]
fn create_should_work() {
    new_test_ext().execute_with(|| {
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        let kitty = Kitty([39, 140, 77, 194, 163, 1, 154, 220, 108, 18, 30, 32, 100, 223, 46, 1]);
        assert_eq!(Kitties::<Test>::contains_key(1, 0), true);
        assert_eq!(Balances::free_balance(1), 400);
        assert_eq!(Balances::reserved_balance(1), 100);
        assert_eq!(KittiesModule::next_kitty_id(), 1);
        assert_eq!(
            System::events()[1].event,
            mock::Event::kitties(crate::Event::<Test>::KittyCreated(1, 0, kitty))
        );
    });
}

#[test]
fn create_should_failed_when_money_not_enough() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            KittiesModule::create(Origin::signed(3)),
            Error::<Test>::MoneyNotEnough
        );
    });
}

#[test]
fn transfer_should_work() {
    new_test_ext().execute_with(|| {
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_eq!(Kitties::<Test>::contains_key(1, 0), true);
        assert_ok!(KittiesModule::transfer(Origin::signed(1), 2, 0));
        assert_eq!(Kitties::<Test>::contains_key(1, 0), false);
        assert_eq!(Kitties::<Test>::contains_key(2, 0), true);
        // event[reserve, create, transfer]
        assert_eq!(
            System::events()[2].event,
            mock::Event::kitties(crate::Event::<Test>::KittyTransferred(1, 2, 0))
        );
        assert_ok!(KittiesModule::transfer(Origin::signed(2), 2, 0));
        assert_eq!(Kitties::<Test>::contains_key(2, 0), true);
    });
}

#[test]
fn transfers_should_failed_when_not_owner() {
    new_test_ext().execute_with(|| {
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_noop!(KittiesModule::transfer(Origin::signed(2), 2, 0), Error::<Test>::InvalidKittyId);
    });
}

#[test]
fn breed_should_work() {
    new_test_ext().execute_with(|| {
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_ok!(KittiesModule::breed(Origin::signed(1), 0, 1));
        assert_eq!(Kitties::<Test>::contains_key(1, 2), true);
        let kitty = Kitty([39, 140, 77, 194, 163, 1, 154, 220, 108, 18, 30, 32, 100, 223, 46, 1]);
        assert_eq!(
            System::events()[4].event,
            mock::Event::kitties(crate::Event::<Test>::KittyCreated(1, 2, kitty))
        );
    });
}

#[test]
fn breed_should_failed_when_not_owner() {
    new_test_ext().execute_with(|| {
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_ok!(KittiesModule::create(Origin::signed(2)));
        assert_noop!(
            KittiesModule::breed(Origin::signed(2), 0, 1),
            Error::<Test>::InvalidKittyId
        );
    });
}

#[test]
fn set_price_should_work() {
    new_test_ext().execute_with(|| {
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_ok!(KittiesModule::set_price(Origin::signed(1), 0, Some(100)));
        assert_eq!(KittiesPrice::<Test>::get(0), Some(100));
        assert_eq!(
            System::events()[2].event,
            mock::Event::kitties(crate::Event::<Test>::KittyPriceUpdated(1, 0, Some(100)))
        );
    });
}

#[test]
fn set_price_should_failed_when_not_owner() {
    new_test_ext().execute_with(|| {
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_noop!(
            KittiesModule::set_price(Origin::signed(2), 0, Some(100)),
            Error::<Test>::NotOwner
        );
    });
}

#[test]
fn buy_should_work() {
    new_test_ext().execute_with(|| {
        assert_ok!(KittiesModule::create(Origin::signed(1))); //2
        assert_ok!(KittiesModule::set_price(Origin::signed(1), 0, Some(200))); // 1
        assert_ok!(KittiesModule::buy(Origin::signed(2), 1, 0, 200)); // 2
        assert_eq!(Balances::free_balance(1), 600);
        assert_eq!(Balances::free_balance(2), 300);
        assert_eq!(
            System::events()[4].event,
            mock::Event::kitties(crate::Event::<Test>::KittySold(2, 1, 0, 200))
        );
        assert_eq!(KittiesPrice::<Test>::contains_key(0), false);
        assert_eq!(Kitties::<Test>::contains_key(1, 0), false);
    });
}


#[test]
fn buy_should_failed_when_invalid_owner() {
    new_test_ext().execute_with(|| {
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_ok!(KittiesModule::set_price(Origin::signed(1), 0, Some(100)));
        assert_noop!(
            KittiesModule::buy(Origin::signed(200), 2, 0, 100),
            Error::<Test>::InvalidKittyId
        );
    });
}

#[test]
fn buy_should_failed_when_not_for_sale() {
    new_test_ext().execute_with(|| {
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_noop!(
            KittiesModule::buy(Origin::signed(200), 1, 0, 100),
            Error::<Test>::NotForSale
        );
    });
}

#[test]
fn buy_should_failed_when_max_price_too_low() {
    new_test_ext().execute_with(|| {
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_ok!(KittiesModule::set_price(Origin::signed(1), 0, Some(100)));
        assert_noop!(
            KittiesModule::buy(Origin::signed(200), 1, 0, 10),
            Error::<Test>::PriceTooLow
        );
    });
}

#[test]
fn buy_should_failed_when_buy_from_self() {
    new_test_ext().execute_with(|| {
        assert_ok!(KittiesModule::create(Origin::signed(1)));
        assert_ok!(KittiesModule::set_price(Origin::signed(1), 0, Some(100)));
        assert_noop!(
            KittiesModule::buy(Origin::signed(1), 1, 0, 100),
            Error::<Test>::BuyFromSelf
        );
    });
}
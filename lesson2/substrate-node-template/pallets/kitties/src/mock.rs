#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::{pallet_prelude::*, traits::Randomness,
                    parameter_types};
use sp_runtime::{traits::{ BlakeTwo256, IdentityLookup },
                 testing::Header};
use sp_core::H256;
use crate as kitties;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Module, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Module, Call, Storage, Event<T>, Config<T>},
		KittiesModule: kitties::{Module, Call, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl frame_system::Config for Test {
    type BaseCallFilter = ();
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;
}

parameter_types! {
	pub static MockRandom: H256 = Default::default();
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}
impl pallet_balances::Config for Test {
    type MaxLocks = ();
    type Balance = u64;
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
}

impl Randomness<H256> for MockRandom {
    fn random(_subject: &[u8]) -> H256 {
        MockRandom::get()
    }
}

parameter_types! {
    pub const KittyReserve: u64 = 100;
}

impl kitties::Config for Test {
    type Event = Event;
    type Randomness = MockRandom;
    type KittyIndex = u32;
    type KittyReserve = KittyReserve;
}

// BUild genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
    pallet_balances::GenesisConfig::<Test>{
        balances: vec![(1, 500), (2, 500), (3, 50)],
    }.assimilate_storage(&mut t).unwrap();
    let mut t: sp_io::TestExternalities = t.into();
    t.execute_with(|| System::set_block_number(1));
    t
}
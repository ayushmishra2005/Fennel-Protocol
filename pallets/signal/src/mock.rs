use crate as pallet_signal;
use frame_support::parameter_types;
use frame_system as system;
use sp_core::{ConstU32, H256};
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};

pub type Balance = u128;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        SignalModule: pallet_signal::{Pallet, Call, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
    pub const ExistentialDeposit: u128 = 1;
}

impl pallet_balances::Config for Test {
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type MaxLocks = ();
    type Balance = Balance;
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type HoldIdentifier = [u8; 8];
    type FreezeIdentifier = [u8; 8];
    type MaxFreezes = ();
    type MaxHolds = ();
}

impl pallet_signal::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type Currency = Balances;
    type MaxSize = ConstU32<100>;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}

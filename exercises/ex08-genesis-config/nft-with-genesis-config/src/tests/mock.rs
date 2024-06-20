use crate as pallet_nft;
use frame_support::{
	derive_impl,
    // traits::GenesisBuild,
	traits::{ConstU16, ConstU64},
};
use sp_core::H256;
use sp_runtime::{
    // testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};

// type Unchecked Extrinsic = frame_system::mocking::MockUncheckedExtrinsic<TestRuntime>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system,// ::{Pallet, Call, Config, Storage, Event<T>},
		NFTs: pallet_nft::{Pallet, Call, Storage, Event<T>},
	}
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 =42;
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Nonce = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
    pub const MaxLength: u32 = 20;
}

impl pallet_nft::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type MaxLength = MaxLength;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
}

// Mock users AccountId
pub const ALICE: u64 =1;
pub const BOB: u64 = 2;

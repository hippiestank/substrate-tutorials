use crate as pallet_imbalance;
use frame_support::{
	derive_impl,
    parameter_types,
	traits::{ConstU16, ConstU64},
};
use sp_core::H256;
use sp_runtime::{
    // testing::Header
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};

type UncheckedExrinsic = frame_system::mocking::MockUncheckedExtrinsic<TestRuntime>;
type Block = frame_system::mocking::MockBlock<TestRuntime>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum TestRuntime
	{
		System: frame_system,
		Balances: pallet_balances,
        Imbalances: pallet_imbalance,//::{Pallet, Call, Storage, Event<T>}
	}
);

parameter_types! {
    pub const BlockHashCount: u64 =250;
    pub const SS58Prefix: u8 = 42;
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for TestRuntime {
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
	type AccountData = pallet_balances::AccountData<u128>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

pub const EXISTENTIAL_DEPOSIT: u128 = 10;

parameter_types! {
    pub const ExistentialDeposit: u128 = EXISTENTIAL_DEPOSIT;
    pub const MaxLocks: u32 = 50;
    pub const MaxReserves: u32 = 50;
    pub const MaxFreezes: u32 = 100;
}

impl pallet_balances::Config for TestRuntime {
    type AccountStore = System;
    type Balance = u128;
    type DustRemoval = ();
    type RuntimeEvent = RuntimeEvent;
    type MaxLocks = MaxLocks;
    type MaxReserves = MaxReserves;
    type ReserveIdentifier = [u8; 8];
    type WeightInfo = ();
    type RuntimeHoldReason = ();
    type RuntimeFreezeReason = ();
    type ExistentialDeposit = ExistentialDeposit;
    type FreezeIdentifier = ();
    type MaxFreezes = MaxFreezes;
}

pub const TREASURY_FLAT_CUT: u128 = 50;

parameter_types! {
    pub const TreasuryAccount: u64 = TREASURY;
    pub const TreasuryFlatCut: u128 = TREASURY_FLAT_CUT;
}

impl pallet_imbalance::Config for TestRuntime {
    type Currency = Balances;
	type RuntimeEvent = RuntimeEvent;
    type TreasuryAccount = TreasuryAccount;
    type TreasuryFlatCut = TreasuryFlatCut;
}

// Mock users AccountId
pub const TREASURY: u64 = 42;
pub const ALICE: u64 = 1;
pub const BOB: u64 = 2;
pub const CHARLY: u64 = 3;

#[derive(Default)]
pub struct ExtBuilder {
	caps_endowed_accounts: Vec<(u64, u128)>,
}

impl ExtBuilder {
	pub fn balances(mut self, accounts: Vec<(u64, u128)>) -> Self {
		for account in accounts {
			self.caps_endowed_accounts.push(account);
		}
		self
	}

	pub fn build(self) -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::<TestRuntime>::default().build_storage().unwrap();

		pallet_balances::GenesisConfig::<TestRuntime> {
			balances: self.caps_endowed_accounts,
		}
		.assimilate_storage(&mut t)
		.unwrap();

		let mut ext = sp_io::TestExternalities::new(t);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}

use crate as xyk;
use crate::{AssetId, Balance, Config};
use frame_support::parameter_types;
use frame_support::sp_runtime::Permill;
use frame_system as system;
use orml_traits::parameter_type_with_key;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup, One},
    DispatchError,
};

use frame_support::traits::{Everything, GenesisBuild};

pub type Amount = i128;
pub type AccountId = u64;

pub const ALICE: AccountId = 1;
pub const BOB: AccountId = 2;
pub const CHARLIE: AccountId = 3;

pub const HDX: AssetId = 1000;
pub const DOT: AssetId = 2000;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub enum Test where
     Block = Block,
     NodeBlock = Block,
     UncheckedExtrinsic = UncheckedExtrinsic,
     {
         System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
         XYK: xyk::{Pallet, Call, Storage, Event<T>},
         Tokens: orml_tokens::{Pallet, Event<T>},
     }

);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 63;
    pub TradeFee: Permill = Permill::from_float(0.0);
    pub RegistryStringLimit: u32 = 100;
}

impl system::Config for Test {
    type BaseCallFilter = Everything;
    type BlockWeights = ();
    type BlockLength = ();
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
    type DbWeight = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_type_with_key! {
    pub ExistentialDeposits: |_currency_id: AssetId| -> Balance {
        One::one()
    };
}

impl orml_tokens::Config for Test {
    type Event = Event;
    type Balance = Balance;
    type Amount = Amount;
    type CurrencyId = AssetId;
    type WeightInfo = ();
    type ExistentialDeposits = ExistentialDeposits;
    type OnDust = ();
    type OnNewTokenAccount = ();
    type OnKilledTokenAccount = ();
    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = ();
    type DustRemovalWhitelist = Everything;
}

pub struct AccountIdConstructor();

impl crate::traits::AccountIdFor<(AssetId, AssetId)> for AccountIdConstructor {
    type AccountId = AccountId;
    type Error = DispatchError;

    fn create_account_id(assets: (AssetId, AssetId)) -> Result<Self::AccountId, Self::Error> {
        let mut a = assets.0 as u64;
        let mut b = assets.0 as u64;
        if a > b {
            std::mem::swap(&mut a, &mut b)
        }
        Ok(a * 1000 + b)
    }
}

impl Config for Test {
    type Event = Event;
    type Currency = Tokens;
    type AssetRegistry = Registry;
    type Account = AccountIdConstructor;
    type TradeFee = TradeFee;
    type WeightInfo = ();
}

pub struct ExtBuilder {
    endowed_accounts: Vec<(AccountId, AssetId, Balance)>,
}

// Returns default values for genesis config
impl Default for ExtBuilder {
    fn default() -> Self {
        Self {
            endowed_accounts: vec![
                (ALICE, HDX, 1_000_000_000_000_000u128),
                (ALICE, DOT, 1_000_000_000_000_000u128),
                (BOB, HDX, 10_000_000_000_000u128),
                (CHARLIE, HDX, 10_000_000_000_000u128),
                (CHARLIE, DOT, 10_000_000_000_000u128),
            ],
        }
    }
}

impl ExtBuilder {
    pub fn build(self) -> sp_io::TestExternalities {
        let mut t = frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();

        orml_tokens::GenesisConfig::<Test> {
            balances: self.endowed_accounts,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        t.into()
    }
}

pub struct Registry;

pub const POOL_SHARE_ASSET: AssetId = 2222;

impl crate::traits::Create<(AssetId, AssetId)> for Registry {
    type AssetId = AssetId;
    type Error = DispatchError;

    fn create_share_asset(_assets: (AssetId, AssetId)) -> Result<Self::AssetId, Self::Error> {
        Ok(POOL_SHARE_ASSET)
    }
}

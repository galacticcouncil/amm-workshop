#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{dispatch::DispatchResult, traits::Get};
use frame_system::ensure_signed;

mod weights;

use weights::WeightInfo;

pub use pallet::*;

type AssetId = u32;
type Balance = u128;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_support::traits::tokens::fungibles::{Inspect, Mutate, Transfer};
    use frame_system::pallet_prelude::OriginFor;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// Multi currency support
        type Currency: Inspect<Self::AccountId, AssetId = AssetId>
            + Mutate<Self::AccountId>
            + Transfer<Self::AccountId>;

        type WeightInfo: WeightInfo;
    }

    #[pallet::error]
    pub enum Error<T> {}

    #[pallet::event]
    #[pallet::generate_deposit(pub(crate) fn deposit_event)]
    pub enum Event<T: Config> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(<T as Config>::WeightInfo::create_pool())]
        pub fn create_pool(
            origin: OriginFor<T>,
            _asset_a: AssetId,
            _amount_a: Balance,
            _asset_b: AssetId,
            _amount_b: Balance,
        ) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            Ok(())
        }

        #[pallet::weight(<T as Config>::WeightInfo::add_liquidity())]
        pub fn add_liquidity(
            origin: OriginFor<T>,
            _asset_a: AssetId,
            _asset_b: AssetId,
            _amount_a: Balance,
            _amount_b_max_limit: Balance,
        ) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            Ok(())
        }

        #[pallet::weight(<T as Config>::WeightInfo::remove_liquidity())]
        pub fn remove_liquidity(
            origin: OriginFor<T>,
            _asset_a: AssetId,
            _asset_b: AssetId,
            _liquidity_amount: Balance,
        ) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            Ok(())
        }

        #[pallet::weight(<T as Config>::WeightInfo::sell())]
        pub fn sell(
            origin: OriginFor<T>,
            _asset_in: AssetId,
            _asset_out: AssetId,
            _amount: Balance,
            _max_limit: Balance,
            _discount: bool,
        ) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            Ok(())
        }

        #[pallet::weight(<T as Config>::WeightInfo::buy())]
        pub fn buy(
            origin: OriginFor<T>,
            _asset_out: AssetId,
            _asset_in: AssetId,
            _amount: Balance,
            _max_limit: Balance,
            _discount: bool,
        ) -> DispatchResult {
            let _who = ensure_signed(origin)?;

            Ok(())
        }
    }
}

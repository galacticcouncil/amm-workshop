#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::sp_runtime::{traits::Zero, DispatchError};
use frame_support::{dispatch::DispatchResult, ensure, traits::Get, transactional};
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
	use frame_system::pallet_prelude::OriginFor;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        type WeightInfo: WeightInfo;
	}

	#[pallet::error]
	pub enum Error<T> {
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(crate) fn deposit_event)]
	pub enum Event<T: Config> {
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(<T as Config>::WeightInfo::create_pool())]
		pub fn create_pool(
			origin: OriginFor<T>,
			asset_a: AssetId,
			amount_a: Balance,
			asset_b: AssetId,
			amount_b: Balance,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Ok(())
		}

		#[pallet::weight(<T as Config>::WeightInfo::add_liquidity())]
		pub fn add_liquidity(
			origin: OriginFor<T>,
			asset_a: AssetId,
			asset_b: AssetId,
			amount_a: Balance,
			amount_b_max_limit: Balance,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Ok(())
		}

		#[pallet::weight(<T as Config>::WeightInfo::remove_liquidity())]
		pub fn remove_liquidity(
			origin: OriginFor<T>,
			asset_a: AssetId,
			asset_b: AssetId,
			liquidity_amount: Balance,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Ok(())
		}

		#[pallet::weight(<T as Config>::WeightInfo::sell())]
		pub fn sell(
			origin: OriginFor<T>,
			asset_in: AssetId,
			asset_out: AssetId,
			amount: Balance,
			max_limit: Balance,
			discount: bool,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Ok(())
		}

		#[pallet::weight(<T as Config>::WeightInfo::buy())]
		pub fn buy(
			origin: OriginFor<T>,
			asset_out: AssetId,
			asset_in: AssetId,
			amount: Balance,
			max_limit: Balance,
			discount: bool,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Ok(())
		}
	}
}

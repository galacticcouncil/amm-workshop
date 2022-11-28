#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::sp_runtime::Permill;
use frame_support::{dispatch::DispatchResult, traits::Get};
use frame_system::ensure_signed;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
pub mod traits;
mod weights;

use weights::WeightInfo;

pub use pallet::*;

type AssetId = u32;
type Balance = u128;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use crate::traits::{AccountIdFor, Create};
    use frame_support::pallet_prelude::*;
    use frame_support::traits::tokens::fungibles::{Inspect, Mutate, Transfer};
    use frame_system::pallet_prelude::OriginFor;
    use sp_runtime::traits::Zero;

    use math::xyk::*;
    use sp_runtime::{FixedPointNumber, FixedU128};

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::hooks]
    impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// Multi currency support
        type Currency: Inspect<Self::AccountId, AssetId = AssetId, Balance = Balance>
            + Mutate<Self::AccountId>
            + Transfer<Self::AccountId>;

        /// Registry support
        type AssetRegistry: Create<(AssetId, AssetId), AssetId = AssetId, Error = DispatchError>;

        /// Pool account creations
        type Account: AccountIdFor<
            (AssetId, AssetId),
            AccountId = Self::AccountId,
            Error = DispatchError,
        >;

        //#[pallet::constant]
        //type CreationFee: Get<Permill>;

        #[pallet::constant]
        type TradeFee: Get<Permill>;

        //#[pallet::constant]
        //type WithdrawFee: Get<Permill>;

        /// The origin that can create a pool
        //type AuthorityOrigin: EnsureOrigin<Self::Origin>;

        type WeightInfo: WeightInfo;
    }

    #[pallet::storage]
    #[pallet::getter(fn pools)]
    pub(crate) type Pools<T: Config> =
        StorageMap<_, Blake2_128Concat, (AssetId, AssetId), AssetId, OptionQuery>;

    #[pallet::error]
    pub enum Error<T> {
        /// It is not allowed to create a pool between same assets.
        CannotCreatePoolWithSameAssets,

        /// Pool with given pair already exists
        PoolAlreadyExists,

        /// Pool with given pair does not exist.
        PoolNotFound,

        /// Limit
        Limit,

        /// Math
        Math,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(crate) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Pool was created.
        PoolCreated {
            who: T::AccountId,
            asset_a: AssetId,
            asset_b: AssetId,
            shares: Balance,
            share_asset_id: AssetId,
        },
        /// Liquidity added to the pool.
        LiquidityAdded {
            who: T::AccountId,
            asset_a: AssetId,
            asset_b: AssetId,
            amount_a: Balance,
            amount_b: Balance,
        },
        /// Liquidity was removed from the pool.
        LiquidityRemoved {
            who: T::AccountId,
            asset_a: AssetId,
            asset_b: AssetId,
            shares: Balance,
        },

        /// Pool was destroyed.
        PoolDestroyed {
            who: T::AccountId,
            asset_a: AssetId,
            asset_b: AssetId,
            share_asset_id: AssetId,
        },
        /// Swap executed.
        SwapExecuted {
            who: T::AccountId,
            asset_in: AssetId,
            asset_out: AssetId,
            amount_in: Balance,
            amount_out: Balance,
            fee: Balance,
        },
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

            ensure!(
                asset_a != asset_b,
                Error::<T>::CannotCreatePoolWithSameAssets
            );

            let (pair, amounts) = if asset_a < asset_b {
                ((asset_a, asset_b), (amount_a, amount_b))
            } else {
                ((asset_b, asset_a), (amount_b, amount_a))
            };

            ensure!(Self::pools(&pair).is_none(), Error::<T>::PoolAlreadyExists);

            let pool_account = T::Account::create_account_id(pair)?;

            let share_asset_id = T::AssetRegistry::create_share_asset(pair)?;

            let shares = calculate_shares(Balance::zero(), amounts.0, Balance::zero())
                .ok_or(Error::<T>::Math)?;

            T::Currency::transfer(asset_a, &who, &pool_account, amounts.0, true)?;
            T::Currency::transfer(asset_b, &who, &pool_account, amounts.1, true)?;

            T::Currency::mint_into(share_asset_id, &who, shares)?;

            <Pools<T>>::insert(&pair, share_asset_id);

            Self::deposit_event(Event::PoolCreated {
                who,
                asset_a,
                asset_b,
                shares,
                share_asset_id,
            });

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

            let pair = if asset_a < asset_b {
                (asset_a, asset_b)
            } else {
                (asset_b, asset_a)
            };

            let share_asset_id = Self::pools(&pair).ok_or(Error::<T>::PoolNotFound)?;

            let pool_account = T::Account::create_account_id(pair)?;

            let asset_a_reserve = T::Currency::balance(asset_a, &pool_account);
            let asset_b_reserve = T::Currency::balance(asset_b, &pool_account);

            let amount_b = calculate_liquidity_in(asset_a_reserve, asset_b_reserve, amount_a)
                .ok_or(Error::<T>::Math)?;

            ensure!(amount_b <= amount_b_max_limit, Error::<T>::Limit);

            let share_issuance = T::Currency::total_issuance(share_asset_id);

            let shares = calculate_shares(asset_a_reserve, amount_a, share_issuance)
                .ok_or(Error::<T>::Math)?;

            T::Currency::transfer(asset_a, &who, &pool_account, amount_a, true)?;
            T::Currency::transfer(asset_b, &who, &pool_account, amount_b, true)?;

            T::Currency::mint_into(share_asset_id, &who, shares)?;

            Self::deposit_event(Event::LiquidityAdded {
                who,
                asset_a,
                asset_b,
                amount_a,
                amount_b,
            });

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
            let pair = if asset_a < asset_b {
                (asset_a, asset_b)
            } else {
                (asset_b, asset_a)
            };

            let share_asset_id = Self::pools(&pair).ok_or(Error::<T>::PoolNotFound)?;

            let pool_account = T::Account::create_account_id(pair)?;

            let asset_a_reserve = T::Currency::balance(asset_a, &pool_account);
            let asset_b_reserve = T::Currency::balance(asset_b, &pool_account);
            let share_issuance = T::Currency::total_issuance(share_asset_id);

            let (amount_a, amount_b) = calculate_liquidity_out(
                asset_a_reserve,
                asset_b_reserve,
                liquidity_amount,
                share_issuance,
            )
            .ok_or(Error::<T>::Math)?;

            T::Currency::transfer(asset_a, &pool_account, &who, amount_a, true)?;
            T::Currency::transfer(asset_b, &pool_account, &who, amount_b, true)?;

            T::Currency::burn_from(share_asset_id, &who, liquidity_amount)?;

            Self::deposit_event(Event::LiquidityRemoved {
                who: who.clone(),
                asset_a,
                asset_b,
                shares: liquidity_amount,
            });
            let liquidity_left = share_issuance
                .checked_sub(liquidity_amount)
                .ok_or(Error::<T>::Math)?;

            if liquidity_left == 0 {
                <Pools<T>>::remove(&pair);

                Self::deposit_event(Event::PoolDestroyed {
                    who,
                    asset_a,
                    asset_b,
                    share_asset_id,
                });
            }

            Ok(())
        }

        #[pallet::weight(<T as Config>::WeightInfo::sell())]
        pub fn sell(
            origin: OriginFor<T>,
            asset_in: AssetId,
            asset_out: AssetId,
            amount_in: Balance,
            min_limit: Balance,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let pair = if asset_in < asset_out {
                (asset_in, asset_out)
            } else {
                (asset_out, asset_in)
            };

            let _ = Self::pools(&pair).ok_or(Error::<T>::PoolNotFound)?;

            let pool_account = T::Account::create_account_id(pair)?;

            let asset_in_reserve = T::Currency::balance(asset_in, &pool_account);
            let asset_out_reserve = T::Currency::balance(asset_out, &pool_account);

            let amount_out = calculate_out_given_in(asset_in_reserve, asset_out_reserve, amount_in)
                .ok_or(Error::<T>::Math)?;

            let fee = FixedU128::from(T::TradeFee::get())
                .checked_mul_int(amount_out)
                .ok_or(Error::<T>::Math)?;

            let amount_out_with_fee = amount_out.checked_sub(fee).ok_or(Error::<T>::Math)?;

            ensure!(amount_out_with_fee >= min_limit, Error::<T>::Limit);

            T::Currency::transfer(asset_in, &who, &pool_account, amount_in, true)?;
            T::Currency::transfer(asset_out, &pool_account, &who, amount_out_with_fee, true)?;

            Self::deposit_event(Event::<T>::SwapExecuted {
                who,
                asset_in,
                asset_out,
                amount_in,
                amount_out: amount_out_with_fee,
                fee,
            });

            Ok(())
        }

        #[pallet::weight(<T as Config>::WeightInfo::buy())]
        pub fn buy(
            origin: OriginFor<T>,
            asset_out: AssetId,
            asset_in: AssetId,
            amount_out: Balance,
            max_limit: Balance,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let pair = if asset_in < asset_out {
                (asset_in, asset_out)
            } else {
                (asset_out, asset_in)
            };

            let _ = Self::pools(&pair).ok_or(Error::<T>::PoolNotFound)?;

            let pool_account = T::Account::create_account_id(pair)?;

            let asset_in_reserve = T::Currency::balance(asset_in, &pool_account);
            let asset_out_reserve = T::Currency::balance(asset_out, &pool_account);

            let amount_in = calculate_in_given_out(asset_out_reserve, asset_in_reserve, amount_out)
                .ok_or(Error::<T>::Math)?;

            let fee = FixedU128::from(T::TradeFee::get())
                .checked_mul_int(amount_in)
                .ok_or(Error::<T>::Math)?;

            let amount_in_with_fee = amount_out.checked_sub(fee).ok_or(Error::<T>::Math)?;

            ensure!(amount_in_with_fee <= max_limit, Error::<T>::Limit);

            T::Currency::transfer(asset_in, &who, &pool_account, amount_in_with_fee, true)?;
            T::Currency::transfer(asset_out, &pool_account, &who, amount_out, true)?;

            Self::deposit_event(Event::<T>::SwapExecuted {
                who,
                asset_in,
                asset_out,
                amount_in: amount_in_with_fee,
                amount_out,
                fee,
            });

            Ok(())
        }
    }
}

#![cfg_attr(not(feature = "std"), no_std)]
// use serde::{Serialize, Deserialize};
use frame_support::{decl_module, decl_storage, decl_error, ensure};
use frame_support::traits::{
	Currency, Get,
	ReservableCurrency
};
use sp_runtime::{ModuleId, traits::{
	AccountIdConversion, Saturating
}};
use frame_system::ensure_signed;
use frame_support::{
	dispatch::DispatchResult,
	traits::{
		ExistenceRequirement::AllowDeath
	}
};

pub type BalanceOf<T, I=DefaultInstance> =
<<T as Config<I>>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
pub type PositiveImbalanceOf<T, I=DefaultInstance> =
<<T as Config<I>>::Currency as Currency<<T as frame_system::Config>::AccountId>>::PositiveImbalance;
pub type NegativeImbalanceOf<T, I=DefaultInstance> =
<<T as Config<I>>::Currency as Currency<<T as frame_system::Config>::AccountId>>::NegativeImbalance;

pub trait Config<I=DefaultInstance>: frame_system::Config {
	type ModuleId: Get<ModuleId>;

	type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
}


decl_storage! {
    trait Store for Module<T: Config<I>, I: Instance=DefaultInstance> as RevenueSharing {

		pub RevenueSharingAccount get(fn revenue_sharing_account): T::AccountId = <Module<T, I>>::account_id();

		pub LockAmount get(fn lock_amount): map hasher(blake2_128_concat) T::AccountId => BalanceOf<T, I>;

		pub TotalLocked get(fn total_locked): BalanceOf<T, I>;

	}
	add_extra_genesis {
		build(|_config| {
			let account_id = <Module<T, I>>::account_id();
			let min = T::Currency::minimum_balance();
			if T::Currency::free_balance(&account_id) < min {
				let _ = T::Currency::make_free_balance_be(
					&account_id,
					min,
				);
			}
		});
	}
}

decl_error! {
	pub enum Error for Module<T: Config<I>, I: Instance> {
		InsufficientFunds,
		ExceedAmountLocked
	}
}

decl_module! {
	pub struct Module<T: Config<I>, I: Instance=DefaultInstance>
		for enum Call
		where origin: T::Origin
	{
		#[weight = 10_000]
		pub fn share_revenue(origin, value: BalanceOf<T, I>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(T::Currency::transfer(&sender, &Self::account_id(), value, AllowDeath)? == (), <Error<T, I>>::InsufficientFunds);

			let current_locked = Self::lock_amount(&sender);
			<LockAmount<T, I>>::insert(&sender, current_locked.saturating_add(value));
			//
			let current_total_locked = Self::total_locked();
			<TotalLocked<T, I>>::put(current_total_locked.saturating_add(value));

			Ok(())
		}

		#[weight = 10_000]
		fn claim_revenue(origin, value: BalanceOf<T, I>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let current_locked = Self::lock_amount(&sender);
			ensure!(current_locked >= value, <Error<T, I>>::ExceedAmountLocked);
			<LockAmount<T, I>>::insert(&sender, current_locked.saturating_sub(value));

			T::Currency::transfer(&Self::account_id(), &sender, value, AllowDeath)?;

			let current_total_locked = Self::total_locked();
			<TotalLocked<T, I>>::put(current_total_locked.saturating_sub(value));

			Ok(())
		}
	}
}

impl<T: Config<I>, I: Instance> Module<T, I> {
	pub fn account_id() -> T::AccountId {
		T::ModuleId::get().into_account()
	}

	pub fn pot() -> BalanceOf<T, I> {
		T::Currency::free_balance(&Self::account_id())
	}

}

#![cfg_attr(not(feature = "std"), no_std)]
// use serde::{Serialize, Deserialize};
use frame_support::{decl_module, decl_storage};
use frame_support::traits::{
	Currency, Get,
	ReservableCurrency
};
use sp_runtime::{ModuleId, traits::{
	AccountIdConversion, Saturating
}};

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

decl_module! {
	pub struct Module<T: Config<I>, I: Instance=DefaultInstance>
		for enum Call
		where origin: T::Origin
	{

	}
}

impl<T: Config<I>, I: Instance> Module<T, I> {
	pub fn account_id() -> T::AccountId {
		T::ModuleId::get().into_account()
	}

	pub fn pot() -> BalanceOf<T, I> {
		T::Currency::free_balance(&Self::account_id())
			.saturating_sub(T::Currency::minimum_balance())
	}

}

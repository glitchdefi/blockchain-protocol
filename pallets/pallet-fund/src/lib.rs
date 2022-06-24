#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(test)]
mod test;

use sp_runtime::{traits::AccountIdConversion, ModuleId};
use sp_std::prelude::*;



use frame_support::{
	decl_event, decl_module, decl_storage, print, weights::Pays,
	dispatch::{DispatchError, DispatchResult},
	traits::{Currency, ExistenceRequirement, Imbalance, OnUnbalanced, WithdrawReasons},
};
use frame_system::{ensure_root, ensure_signed};
use frame_support::pallet_prelude::Get;
use pallet_balances::*;
use pallet_session as session;

pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
pub type PositiveImbalanceOf<T> = <<T as Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::PositiveImbalance;

/// Hardcoded pallet ID; used to create the special Pot Account
/// Must be exactly 8 characters long
const PALLET_ID: ModuleId = ModuleId(*b"fundreve");

pub trait Config: frame_system::Config {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
	/// The currency type that the charity deals in
	type Currency: Currency<Self::AccountId>;

}

decl_storage! {
	trait Store for Module<T: Config> as SimpleTreasury {
	}
	add_extra_genesis {
		build(|_config| {
			let account_id = <Module<T>>::account_id();
			let _ = T::Currency::make_free_balance_be(
				&<Module<T>>::account_id(),
				T::Currency::minimum_balance(),
			);
		});
	}
}

decl_event!(
	pub enum Event<T>
	where
		Balance = BalanceOf<T>
	{
		/// Spend fund
		SpendFund(Balance),
	}
);

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		fn deposit_event() = default;
	}
}

impl<T: Config> Module<T> {
	/// The account ID that holds the Reward's funds
	pub fn account_id() -> T::AccountId {
		PALLET_ID.into_account()
	}

	/// The Charity's balance
	fn pot() -> BalanceOf<T> {
		T::Currency::free_balance(&Self::account_id())
	}

}

// This implementation allows the charity to be the recipient of funds that are burned elsewhere in
// the runtime. For eample, it could be transaction fees, consensus-related slashing, or burns that
// align incentives in other pallets.
impl<T: Config> OnUnbalanced<PositiveImbalanceOf<T>> for Module<T> {
	fn on_nonzero_unbalanced(amount: PositiveImbalanceOf<T>) {
		let numeric_amount = amount.peek();
		if let Err(problem) = T::Currency::settle(
			&Self::account_id(),
			amount,
			WithdrawReasons::TRANSFER,
			ExistenceRequirement::KeepAlive
		) {
			print("Inconsistent state - couldn't settle imbalance for funds");
			// Nothing else to do here.
			drop(problem);
			return;
		}
		Self::deposit_event(RawEvent::SpendFund(numeric_amount));
	}
}


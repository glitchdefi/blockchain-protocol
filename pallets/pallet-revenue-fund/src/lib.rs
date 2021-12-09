#![cfg_attr(not(feature = "std"), no_std)]


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
use pallet_staking::RevenueWallet;

pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
pub type PositiveImbalanceOf<T> = <<T as Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::PositiveImbalance;

decl_storage! {
	trait Store for Module<T: Config> as WalletFund {
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

/// Hardcoded pallet ID; used to create the special Pot Account
/// Must be exactly 8 characters long
const PALLET_ID: ModuleId = ModuleId(*b"waltfund");


pub trait Config: frame_system::Config {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
	/// The currency type that the charity deals in
	type Currency: Currency<Self::AccountId>;

}

decl_event!(
	pub enum Event<T>
	where
		Balance = BalanceOf<T>,
		<T as frame_system::Config>::AccountId,
	{
		/// Donor has made a charitable donation to the charity
		DonationReceived(AccountId, Balance, Balance),

		RewardReceived(Balance),
	}
);


decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		/// Donate some funds to the charity
		#[weight = (10, Pays::No)]
		fn donate(
			origin,
			amount: BalanceOf<T>
		) -> DispatchResult {
			let donor = ensure_signed(origin)?;

			T::Currency::transfer(&donor, &Self::account_id(), amount, ExistenceRequirement::AllowDeath)
				.map_err(|_| DispatchError::Other("Can't make donation"))?;

			Self::deposit_event(RawEvent::DonationReceived(donor, amount, Self::pot()));
			Ok(())
		}

		// Transfer balance from wallet to reward fund
		#[weight = (10, Pays::No)]
		fn triger(origin) -> DispatchResult {
			ensure_signed(origin)?;
			let amount = Self::pot();
			T::Currency::transfer(&Self::account_id(),&Self::reward_fund_id(), amount,ExistenceRequirement::AllowDeath)
				.map_err(|_| DispatchError::Other("Can't make donation"))?;
			Self::deposit_event(RawEvent::RewardReceived(amount));
			Ok(())
		}
	}
}


impl<T: Config> Module<T> {
	/// The account ID that holds the revenue's funds
	pub fn account_id() -> T::AccountId {
		PALLET_ID.into_account()
	}
	/// The account ID that holds the Reward's funds
	fn reward_fund_id() -> T::AccountId { ModuleId(*b"fundreve").into_account() }

	/// The wallet's balance
	fn pot() -> BalanceOf<T> {
		T::Currency::free_balance(&Self::account_id())
	}

	pub fn balance_to_u64(input: BalanceOf<T>) -> Option<u64> { TryInto::<u64>::try_into(input).ok() }

	// pub fn u64_to_balance_option(input: u64) -> BalanceOf<T> { input.try_into().ok() }

	fn check_empty(balance: Option<u64>) -> bool {
		match balance {
			Some(x) => x > 0,
			None => false
		}
	}
}

impl<T:Config> RevenueWallet for Module<T> {
	fn trigger_wallet() {
		let amount = Self::pot();
		let amount_convert = Self::balance_to_u64(amount);

		if Self::check_empty(amount_convert) == true {
			T::Currency::transfer(&Self::account_id(),&Self::reward_fund_id(), amount,ExistenceRequirement::AllowDeath)
				.map_err(|_| DispatchError::Other("Can't make donation"));
			Self::deposit_event(RawEvent::RewardReceived(amount));
		}
	}
}



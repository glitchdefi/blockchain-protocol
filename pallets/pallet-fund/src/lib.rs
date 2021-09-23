#![cfg_attr(not(feature = "std"), no_std)]

use sp_runtime::{traits::AccountIdConversion, ModuleId};
use sp_std::prelude::*;



use frame_support::{
	decl_event, decl_module, decl_storage,
	dispatch::{DispatchError, DispatchResult},
	traits::{Currency, ExistenceRequirement::AllowDeath, Imbalance, OnUnbalanced},
};
use frame_system::{ensure_root, ensure_signed};
use frame_support::pallet_prelude::Get;
use pallet_balances::*;
use pallet_staking::*;
use pallet_staking::{self as staking};
use pallet_staking::StakerStatus::Validator;
use pallet_session as session;

pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
pub type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::NegativeImbalance;

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
		// No storage items of our own, but we still need decl_storage to initialize the pot
	}
	add_extra_genesis {
		build(|_config| {
			// Create the charity's pot of funds, and ensure it has the minimum required deposit
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
		Balance = BalanceOf<T>,
		<T as frame_system::Config>::AccountId,
	{
		/// Donor has made a charitable donation to the charity
		DonationReceived(AccountId, Balance, Balance),
		/// An imbalance from elsewhere in the runtime has been absorbed by the Charity
		ImbalanceAbsorbed(Balance, Balance),
	}
);

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		fn deposit_event() = default;

		/// Donate some funds to the charity
		#[weight = 0]
		fn donate(
			origin,
			amount: BalanceOf<T>
		) -> DispatchResult {
			let donor = ensure_signed(origin)?;

			T::Currency::transfer(&donor, &Self::account_id(), amount, AllowDeath)
				.map_err(|_| DispatchError::Other("Can't make donation"))?;

			Self::deposit_event(RawEvent::DonationReceived(donor, amount, Self::pot()));
			Ok(())
		}

		#[weight = 0]
		fn send_reward (origin, members: Vec<T::AccountId>, _value: u64) -> DispatchResult {
			let sender =  &Self::account_id();
			let balance = T::Currency::free_balance(&Self::account_id());
			let count_member = members.len();

			Ok(())
		}

	}
}

impl<T: Config> Module<T> {
	/// The account ID that holds the Charity's funds
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
impl<T: Config> OnUnbalanced<NegativeImbalanceOf<T>> for Module<T> {
	fn on_nonzero_unbalanced(amount: NegativeImbalanceOf<T>) {
		let numeric_amount = amount.peek();

		// Must resolve into existing but better to be safe.
		let _ = T::Currency::resolve_creating(&Self::account_id(), amount);

		Self::deposit_event(RawEvent::ImbalanceAbsorbed(numeric_amount, Self::pot()));
	}
}
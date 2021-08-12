#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_storage, decl_error, ensure};
use sp_runtime::{ModuleId, RuntimeDebug, traits::{
	AccountIdConversion, Saturating
}};
use frame_system::ensure_signed;
use frame_support::{
	dispatch::{
		DispatchResult
	},
	traits::{
		Currency, Get,
		ReservableCurrency,
		ExistenceRequirement::AllowDeath
	}
};
use codec::{Encode, Decode};
use sp_std::prelude::*;
use sp_core::H160;

pub type BalanceOf<T, I=DefaultInstance> =
<<T as Config<I>>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
pub type PositiveImbalanceOf<T, I=DefaultInstance> =
<<T as Config<I>>::Currency as Currency<<T as frame_system::Config>::AccountId>>::PositiveImbalance;
pub type NegativeImbalanceOf<T, I=DefaultInstance> =
<<T as Config<I>>::Currency as Currency<<T as frame_system::Config>::AccountId>>::NegativeImbalance;

#[derive(Default, Clone, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "with-codec", derive(codec::Encode, codec::Decode))]
#[cfg_attr(feature = "with-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Proposal<AccountId, Balance> {
	smart_contract_address: Vec<H160>,
	value: Balance,
	service_type: u8,
	proposer: AccountId,
}

pub trait Config<I=DefaultInstance>: frame_system::Config {
	type ModuleId: Get<ModuleId>;

	type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
}


decl_storage! {
    trait Store for Module<T: Config<I>, I: Instance=DefaultInstance> as RevenueSharing {

		pub RevenueSharingAccount get(fn revenue_sharing_account): T::AccountId;

		// TODO: should pre-set this field (or need to make tx to set it after starting chain --> very dangerous)
		pub PalletOwner get(fn pallet_owner): T::AccountId;

		// pub AcceptedProposal get(fn accepted_proposal):
		// 	map hasher(blake2_128_concat)
		// 	u32 => Proposal<T::AccountId, BalanceOf<T, I>>;

		// pub AcceptedProposalKey get(fn accepted_proposal_key): [u32];
		//
		// AcceptedProposalCount get(fn accepted_proposal_count): u32;

		pub PendingProposal get(fn pending_proposal):
			map hasher(blake2_128_concat)
			u32 => Proposal<T::AccountId, BalanceOf<T, I>>;

		// pub PendingProposalKey get(fn pending_proposal_key): [u32];
		//
		// PendingProposalCount get(fn pending_proposal_count): u32;

		// pub ServiceType get(fn service_type): [u32; 3] = [2, 3, 4];

		pub XX get(fn xx): Vec<H160>;

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

			// cache AccountId
			<RevenueSharingAccount<T, I>>::put(account_id);

		});
	}
}

decl_error! {
	pub enum Error for Module<T: Config<I>, I: Instance> {
		InsufficientFunds,
		ExceedAmountLocked,
		InvalidServiceType,
		NotOwner,
		InvalidPendingProposalID,
		InvalidOption,
		InvalidSmartContractAddress
	}
}

decl_module! {
	pub struct Module<T: Config<I>, I: Instance=DefaultInstance>
		for enum Call
		where origin: T::Origin
	{

		#[weight = 10_000]
		pub fn propose(origin, smart_contract_address: Vec<H160>, value: BalanceOf<T, I>, service_type: u8) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			// // ensure!(util::is_ethereum_address(&smart_contract_address), <Error<T, I>>::InvalidSmartContractAddress);
			//
			// // X2 | X3 | X4
			// ensure!((service_type == 2) || (service_type == 3) || (service_type == 4), <Error<T, I>>::InvalidServiceType);
			// ensure!(T::Currency::transfer(&sender, &Self::revenue_sharing_account(), value, AllowDeath)? == (), <Error<T, I>>::InsufficientFunds);
			//
			// // let d = vec![smart_contract_address[0], smart_contract_address[1]];
			// let mut d = <Vec<H160>>::new();
			// for data in smart_contract_address {
			// 	d.push(data.clone());
			// }
			// d.push(smart_contract_address[0].clone());
			// d.push(smart_contract_address[1].clone());
			//
			let new_proposal = Proposal {
				smart_contract_address: smart_contract_address.clone(),
				value,
				service_type,
				proposer: sender,
			};
			// //
			<PendingProposal<T, I>>::insert(1, new_proposal);
			// <XX<I>>::put(smart_contract_address);
			<XX<I>>::put(smart_contract_address);
			Ok(())
		}
	}
}

impl<T: Config<I>, I: Instance> Module<T, I> {
	// TODO: Can get the private key from this account?
	pub fn account_id() -> T::AccountId {
		T::ModuleId::get().into_account()
	}

	pub fn pot() -> BalanceOf<T, I> {
		T::Currency::free_balance(&Self::account_id())
	}

}
#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_storage, decl_error, ensure};
use sp_runtime::{ModuleId, traits::{
	AccountIdConversion, Saturating
}};
use frame_system::ensure_signed;
use frame_support::{
	dispatch::DispatchResult,
	traits::{
		Currency, Get,
		ReservableCurrency,
		ExistenceRequirement::AllowDeath
	}
};
use codec::{Encode, Decode};
use sp_std::prelude::*;

pub type BalanceOf<T, I=DefaultInstance> =
<<T as Config<I>>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
pub type PositiveImbalanceOf<T, I=DefaultInstance> =
<<T as Config<I>>::Currency as Currency<<T as frame_system::Config>::AccountId>>::PositiveImbalance;
pub type NegativeImbalanceOf<T, I=DefaultInstance> =
<<T as Config<I>>::Currency as Currency<<T as frame_system::Config>::AccountId>>::NegativeImbalance;


#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct Proposal<AccountId, Balance> {
	proposer: AccountId,
	value: Balance,
	service_type: u32
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

		pub AcceptedProposal get(fn accepted_proposal):
			map hasher(blake2_128_concat)
			u32 => Proposal<T::AccountId, BalanceOf<T, I>>;

		pub NumberAcceptedProposal get(fn number_accepted_proposal): u32;

		pub PendingProposal get(fn pending_proposal):
			map hasher(blake2_128_concat)
			u32 => Proposal<T::AccountId, BalanceOf<T, I>>;

		pub NumberPendingProposal get(fn number_pending_proposal): u32;

		pub TotalLockedGLCH get(fn total_locked_glch): BalanceOf<T, I>;

		pub TotalAcceptedGLCH get(fn total_accepted_glch): BalanceOf<T, I>;

		pub TotalPendingGLCH get(fn total_pending_glch): BalanceOf<T, I>;

		pub ServiceType get(fn service_type): [u32; 3] = [2, 3, 4];

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
		InvalidOption
	}
}

decl_module! {
	pub struct Module<T: Config<I>, I: Instance=DefaultInstance>
		for enum Call
		where origin: T::Origin
	{

		#[weight = 10_000]
		pub fn propose(origin, value: BalanceOf<T, I>, service_type: u32) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!((service_type == 2) || (service_type == 3) || (service_type == 4), <Error<T, I>>::InvalidServiceType);
			ensure!(T::Currency::transfer(&sender, &Self::revenue_sharing_account(), value, AllowDeath)? == (), <Error<T, I>>::InsufficientFunds);

			let current_total_locked_glch = Self::total_locked_glch();
			<TotalLockedGLCH<T, I>>::put(current_total_locked_glch.saturating_add(value));

			let current_total_pending_glch = Self::total_pending_glch();
			<TotalPendingGLCH<T, I>>::put(current_total_pending_glch.saturating_add(value));

			let number_pending_proposal = Self::number_pending_proposal().saturating_add(1);
			<NumberPendingProposal<I>>::put(number_pending_proposal);

			let new_proposal = Proposal {
				proposer: sender,
				value,
				service_type
			};

			<PendingProposal<T, I>>::insert(number_pending_proposal, new_proposal);

			Ok(())
		}

		// Make tx to set owner for this pallet. (Currently, set Alice)
		#[weight = 10_000]
		fn set_pallet_owner(origin, owner: T::AccountId) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			<PalletOwner<T, I>>::put(owner);
			Ok(())
		}

		#[weight = 10_000]
		fn owner_judge(origin, pending_proposal_id: u32, accept: u8) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!((accept == 0) || (accept == 1), <Error<T, I>>::InvalidOption);

			// Currently, Alice is owner of this pallet
			ensure!(sender == Self::pallet_owner(), <Error<T, I>>::NotOwner);

			let current_number_pending_proposal = Self::number_pending_proposal();
			ensure!((pending_proposal_id <= current_number_pending_proposal) && (pending_proposal_id > 0),
					<Error<T, I>>::InvalidPendingProposalID);

			let pending_proposal = <PendingProposal<T, I>>::get(pending_proposal_id);

			if accept == 1 {
				// add to accepted_proposal_set
				let number_accepted_proposal = Self::number_accepted_proposal().saturating_add(1);
				<NumberAcceptedProposal<I>>::put(number_accepted_proposal);
				<AcceptedProposal<T, I>>::insert(number_accepted_proposal, pending_proposal.clone());

				// increase accepted GLCH
				let current_total_accepted_glch = Self::total_accepted_glch();
				<TotalAcceptedGLCH<T, I>>::put(current_total_accepted_glch.saturating_add(pending_proposal.clone().value));
			} else {
				T::Currency::transfer(&Self::revenue_sharing_account(), &pending_proposal.clone().proposer,
										pending_proposal.clone().value, AllowDeath)?;
				let current_total_locked_glch = Self::total_locked_glch();
				<TotalLockedGLCH<T, I>>::put(current_total_locked_glch.saturating_sub(pending_proposal.clone().value));
			}

			// delete from pending_proposal_set
			let last_pending_proposal = <PendingProposal<T, I>>::get(current_number_pending_proposal);
			<PendingProposal<T, I>>::insert(pending_proposal_id, last_pending_proposal);
			<PendingProposal<T, I>>::remove(current_number_pending_proposal);
			<NumberPendingProposal<I>>::put(current_number_pending_proposal.saturating_sub(1));

			// decrease pending GLCH
			let current_total_pending_glch = Self::total_pending_glch();
			<TotalPendingGLCH<T, I>>::put(current_total_pending_glch.saturating_sub(pending_proposal.clone().value));
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

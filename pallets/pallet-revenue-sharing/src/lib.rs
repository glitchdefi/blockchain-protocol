#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_module, decl_storage, decl_error, ensure};
use sp_runtime::{ModuleId, RuntimeDebug, traits::{
	AccountIdConversion
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
use sp_std::prelude::*;
use sp_core::H160;

pub type BalanceOf<T, I=DefaultInstance> =
<<T as Config<I>>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[derive(codec::Encode, codec::Decode, Default, Clone, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "with-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Proposal<AccountId, Balance> {
	smart_contract_address: Vec<H160>,
	value: Balance,
	service_type: u8,
	proposer: AccountId,
}

#[derive(codec::Encode, codec::Decode, Default, Clone, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "with-serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PriorityContract {
	service_type: u8,
	accepted_proposal_id: u32
}

pub trait Config<I=DefaultInstance>: frame_system::Config {
	type ModuleId: Get<ModuleId>;

	type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
}


decl_storage! {
	// TODO: Currently, this pallet has a problem related to frontend (reading data from map)
	// TODO: may need to add custom rpc

    trait Store for Module<T: Config<I>, I: Instance=DefaultInstance> as RevenueSharing {

		pub RevenueSharingAccount get(fn revenue_sharing_account): T::AccountId;

		// TODO: should pre-set this field (or need to make tx to set it after starting chain --> not so dangerous)
		pub PalletOwner get(fn pallet_owner): T::AccountId;

		SetOwner get(fn set_owner): bool = false;

		pub PriorityPool get(fn priority_pool):
			map hasher(blake2_128_concat)
			H160 => PriorityContract;

		// List of accepted proposal that proposed by "proposer"
		pub AcceptedProposalIdOfProposer get(fn accepted_proposal_id_of_proposer):
			map hasher(blake2_128_concat)
			T::AccountId => Vec<u32>;

		pub AcceptedProposal get(fn accepted_proposal):
			map hasher(blake2_128_concat)
			u32 => Proposal<T::AccountId, BalanceOf<T, I>>;

		// Current ID of accepted proposals
		pub AcceptedProposalID get(fn accepted_proposal_id): Vec<u32>;

		// always increase
		AcceptedProposalCount get(fn accepted_proposal_count): u32;

		pub PendingProposal get(fn pending_proposal):
			map hasher(blake2_128_concat)
			u32 => Proposal<T::AccountId, BalanceOf<T, I>>;

		// Current ID of pending proposals
		pub PendingProposalID get(fn pending_proposal_id): Vec<u32>;

		// always increase
		PendingProposalCount get(fn pending_proposal_count): u32;

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
		InvalidServiceType,
		NotOwner,
		Decision,
		InvalidAcceptedProposalID,
		InvalidPendingProposalID,
		PendingInsertFailed,
		PendingDeleteFailed,
		AcceptedInsertFailed,
		AcceptedDeleteFailed,
		OfProposerInsertFailed,
		OfProposerDeleteFailed,
		AlreadySetOwner
	}
}

decl_module! {
	pub struct Module<T: Config<I>, I: Instance=DefaultInstance>
		for enum Call
		where origin: T::Origin
	{

		#[weight = 10_000]
		fn set_pallet_owner(origin, owner: T::AccountId) -> DispatchResult {
			let _sender = ensure_signed(origin)?;
			ensure!(Self::set_owner() == false, <Error<T, I>>::AlreadySetOwner);
			<SetOwner<I>>::put(true);
			<PalletOwner<T, I>>::put(owner);
			Ok(())
		}

		#[weight = 10_000]
		pub fn propose(origin, smart_contract_address: Vec<H160>, value: BalanceOf<T, I>, service_type: u8) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			ensure!((service_type == 2) || (service_type == 3) || (service_type == 4), <Error<T, I>>::InvalidServiceType);
			ensure!(T::Currency::transfer(&sender, &Self::revenue_sharing_account(), value, AllowDeath)? == (), <Error<T, I>>::InsufficientFunds);

			let new_proposal = Proposal {
				smart_contract_address: smart_contract_address.clone(),
				value,
				service_type,
				proposer: sender
			};

			// increase PendingProposalCount
			<PendingProposalCount<I>>::put(Self::pending_proposal_count().saturating_add(1));

			// add above ID to PendingProposalID
			let new_id = Self::pending_proposal_count();
			let mut vec_pending_proposal_id = Self::pending_proposal_id();
			let ok = match vec_pending_proposal_id.binary_search(&new_id) {
				Ok(_) => 0,
				Err(index) => {
					vec_pending_proposal_id.insert(index, new_id);
					<PendingProposalID<I>>::put(vec_pending_proposal_id);
					1
				}
			};
			ensure!(ok == 1, <Error<T, I>>::PendingInsertFailed);

			// add new_proposal to Pending_storage_map
			<PendingProposal<T, I>>::insert(new_id, new_proposal);

			Ok(())
		}

		#[weight = 10_000]
		pub fn owner_decide(origin, pending_proposal_id: u32, accept: u8) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!((accept == 0) || (accept == 1), <Error<T, I>>::Decision);
			// Currently, Alice is owner of this pallet
			ensure!(sender == Self::pallet_owner(), <Error<T, I>>::NotOwner);

			// check if "pending_proposal_id" is valid
			let vec_pending_proposal_id = Self::pending_proposal_id();
			let ok = match vec_pending_proposal_id.binary_search(&pending_proposal_id) {
				Ok(_) => 1,
				Err(_) => 0
			};
			ensure!(ok == 1, <Error<T, I>>::InvalidPendingProposalID);

			let current_pending_proposal = <PendingProposal<T, I>>::get(pending_proposal_id);

			if accept == 1 {
				// increase AcceptedProposalCount (new_id)
				<AcceptedProposalCount<I>>::put(Self::accepted_proposal_count().saturating_add(1));

				// add above ID to AcceptedProposalID
				let new_id = Self::accepted_proposal_count();
				let mut vec_accepted_proposal_id = Self::accepted_proposal_id();
				let ok = match vec_accepted_proposal_id.binary_search(&new_id) {
					Ok(_) => 0,
					Err(index) => {
						vec_accepted_proposal_id.insert(index, new_id);
						<AcceptedProposalID<I>>::put(vec_accepted_proposal_id);
						1
					}
				};
				ensure!(ok == 1, <Error<T, I>>::AcceptedInsertFailed);

				// add proposal to accepted_storage_map
				<AcceptedProposal<T, I>>::insert(new_id, current_pending_proposal.clone());

				// Push PriorityContract to PriorityPool
				for eth_address in current_pending_proposal.smart_contract_address.clone() {
					let new_priority_contract = PriorityContract {
						service_type: current_pending_proposal.service_type.clone(),
						accepted_proposal_id: new_id
					};
					<PriorityPool<I>>::insert(eth_address.clone(), new_priority_contract);
				}

				let proposer = current_pending_proposal.proposer.clone();

				// update AcceptedProposalIdOfProposer
				let mut vec_accepted_proposal_id_of_proposer = Self::accepted_proposal_id_of_proposer(proposer.clone());
				let ok = match vec_accepted_proposal_id_of_proposer.binary_search(&new_id) {
					Ok(_) => 0,
					Err(index) => {
						vec_accepted_proposal_id_of_proposer.insert(index, new_id);
						<AcceptedProposalIdOfProposer<T, I>>::insert(proposer, vec_accepted_proposal_id_of_proposer);
						1
					}
				};
				ensure!(ok == 1, <Error<T, I>>::OfProposerInsertFailed);

			} else {
				T::Currency::transfer(&Self::revenue_sharing_account(), &current_pending_proposal.clone().proposer,
																current_pending_proposal.clone().value, AllowDeath)?;
			}


			// delete pending_proposal_id from PendingProposalID
			let mut vec_pending_proposal_id = Self::pending_proposal_id();
			let ok = match vec_pending_proposal_id.binary_search(&pending_proposal_id) {
				Ok(index) => {
					vec_pending_proposal_id.remove(index);
					<PendingProposalID<I>>::put(vec_pending_proposal_id);
					1
				},
				Err(_) => 0
			};
			ensure!(ok == 1, <Error<T, I>>::PendingDeleteFailed);

			// delete proposal in pending_storage_map
			<PendingProposal<T, I>>::remove(pending_proposal_id);

			Ok(())
		}

		#[weight = 10_000]
		pub fn disable_proposal(origin, accepted_proposal_id: u32) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			// Check if accepted_proposal_id is valid
			let mut vec_acc_proposal_id_of_sender = Self::accepted_proposal_id_of_proposer(sender.clone());
			let ok = match vec_acc_proposal_id_of_sender.binary_search(&accepted_proposal_id) {
				Ok(_) => 1,
				Err(_) => 0
			};
			ensure!(ok == 1, <Error<T, I>>::InvalidAcceptedProposalID);

			let proposal = <AcceptedProposal<T, I>>::get(accepted_proposal_id).clone();

			// delete accepted_proposal_id from AcceptedProposalID
			let mut vec_accepted_proposal_id = Self::accepted_proposal_id();
			let ok = match vec_accepted_proposal_id.binary_search(&accepted_proposal_id) {
				Ok(index) => {
					vec_accepted_proposal_id.remove(index);
					<AcceptedProposalID<I>>::put(vec_accepted_proposal_id);
					1
				},
				Err(_) => 0
			};
			ensure!(ok == 1, <Error<T, I>>::AcceptedDeleteFailed);

			// delete proposal in accepted_storage_map
			<AcceptedProposal<T, I>>::remove(accepted_proposal_id);

			// delete PriorityContract in PriorityPool
			for eth_address in proposal.smart_contract_address.clone() {
				<PriorityPool<I>>::remove(eth_address);
			}

			// update AcceptedProposalIdOfProposer
			let ok = match vec_acc_proposal_id_of_sender.binary_search(&accepted_proposal_id) {
				Ok(index) => {
					vec_acc_proposal_id_of_sender.remove(index);
					<AcceptedProposalIdOfProposer<T, I>>::insert(sender, vec_acc_proposal_id_of_sender);
					1
				},
				Err(_) => 0
			};
			ensure!(ok == 1, <Error<T, I>>::OfProposerDeleteFailed);

			Ok(())
		}

	}
}

impl<T: Config<I>, I: Instance> Module<T, I> {

	pub fn account_id() -> T::AccountId {
		T::ModuleId::get().into_account()
	}

}
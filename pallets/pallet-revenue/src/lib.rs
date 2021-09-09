#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use frame_system::ensure_signed;
    use frame_support::traits::{Currency, ExistenceRequirement::AllowDeath, ReservableCurrency};
    use sp_runtime::ModuleId;
    use sp_runtime::traits::AccountIdConversion;
    use sp_core::H160;
    use frame_support::traits::Vec;

    pub type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {

        // Using for generate address to store balance
        type ModuleId: Get<ModuleId>;

        // Balance handler
        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(sp_std::marker::PhantomData<T>);

    #[pallet::storage]
    #[pallet::getter(fn proposal_map)]
    pub(super) type ProposalMap<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        u32,
        Option<(bool, BalanceOf<T>, T::AccountId, Vec<H160>)>,
        ValueQuery
    >;

    #[pallet::storage]
    #[pallet::getter(fn proposal_count)]
    pub(super) type ProposalCount<T: Config> = StorageValue<_, u32, ValueQuery>;

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Error names should be descriptive.
        NoneValue,
        /// Errors should have helpful documentation associated with them.
        StorageOverflow,
        InsufficientFunds,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn do_something(origin: OriginFor<T>, smart_contract_address: Vec<H160>, value: BalanceOf<T>) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            ensure!(T::Currency::transfer(&who, &Self::account_id(), value, AllowDeath)? == (), <Error<T>>::InsufficientFunds);

            let new_proposal = Some((false, value, who, smart_contract_address.clone()));
            // index proposal
            ProposalCount::<T>::put(Self::proposal_count().saturating_add(1));

            // add above ID to ProposalID
            let new_id = Self::proposal_count();

            // add new_proposal to Pending_storage_map
            ProposalMap::<T>::insert(new_id, new_proposal);

            // Return a successful DispatchResultWithPostInfo
            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn account_id() -> T::AccountId {
            T::ModuleId::get().into_account()
        }
    }
}


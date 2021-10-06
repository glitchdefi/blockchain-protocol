#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(feature = "std")]
use frame_support::traits::GenesisBuild;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_support::traits::Vec;
    use frame_support::traits::{Currency, ExistenceRequirement::AllowDeath, ReservableCurrency};
    use frame_system::ensure_signed;
    use frame_system::pallet_prelude::*;
    use sp_core::H160;
    use sp_runtime::traits::AccountIdConversion;
    use sp_runtime::ModuleId;

    pub type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type ModuleId: Get<ModuleId>;

        type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(sp_std::marker::PhantomData<T>);

    // TODO Propose feature
    // #[pallet::storage]
    // #[pallet::getter(fn proposal_map)]
    // pub(super) type ProposalMap<T: Config> = StorageMap<
    //     _,
    //     Blake2_128Concat,
    //     u32,
    //     Option<(bool, BalanceOf<T>, T::AccountId, Vec<H160>)>,
    //     ValueQuery
    // >;
    //
    // #[pallet::storage]
    // #[pallet::getter(fn proposal_count)]
    // pub(super) type ProposalCount<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn whitelist)]
    pub(super) type Whitelist<T: Config> = StorageMap<_, Blake2_128Concat, H160, bool, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn admin_address)]
    pub(super) type AdminAddress<T: Config> = StorageValue<_, T::AccountId, ValueQuery>;

    #[pallet::error]
    pub enum Error<T> {
        InsufficientFunds,
        NoPermission,
        NoExist,
    }

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub admin_genesis: T::AccountId,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                admin_genesis: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            AdminAddress::<T>::put(self.admin_genesis.clone());
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        // TODO Propose feature
        // #[pallet::weight(10_000)]
        // pub fn propose(origin: OriginFor<T>, smart_contract_address: Vec<H160>, value: BalanceOf<T>) -> DispatchResultWithPostInfo {
        //     let who = ensure_signed(origin)?;
        //
        //     ensure!(T::Currency::transfer(&who, &Self::account_id(), value, AllowDeath)? == (), <Error<T>>::InsufficientFunds);
        //
        //     let new_proposal = Some((false, value, who, smart_contract_address.clone()));
        //
        //     ProposalCount::<T>::put(Self::proposal_count().saturating_add(1));
        //
        //     let new_id = Self::proposal_count();
        //
        //     ProposalMap::<T>::insert(new_id, new_proposal);
        //
        //     Ok(().into())
        // }
        //
        // #[pallet::weight(10_000)]
        // pub fn approve_proposal(origin: OriginFor<T>, proposal_id: u32, is_accept: bool) -> DispatchResultWithPostInfo {
        //     let who = ensure_signed(origin)?;
        //
        //     ensure!(who == Self::admin_address(), <Error<T>>::NoPermission);
        //
        //     let proposal = Self::proposal_map(proposal_id);
        //
        //     let new_data_proposal = match proposal {
        //         None => None,
        //         Some(proposalTuple) => if is_accept {
        //             Some((true, proposalTuple.1, proposalTuple.2, proposalTuple.3))
        //         } else {
        //             None
        //         }
        //     };
        //
        //     ProposalMap::<T>::insert(proposal_id, new_data_proposal);
        //
        //
        //     Ok(().into())
        // }

        #[pallet::weight(10_000)]
        pub fn update_whitelist(
            origin: OriginFor<T>,
            smart_contract_address: Vec<H160>,
            is_add: bool,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            ensure!(who == Self::admin_address(), <Error<T>>::NoPermission);

            smart_contract_address
                .iter()
                .for_each(|addr| Whitelist::<T>::insert(addr, is_add));

            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn account_id() -> T::AccountId {
            T::ModuleId::get().into_account()
        }
    }
}

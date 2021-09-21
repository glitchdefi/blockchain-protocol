#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use sp_core::H160;
    use sp_std::vec::Vec;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::event]
    #[pallet::metadata(H160 = "address")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event emitted when an address has been added to white list. [address]
        AddressAdded(H160),
        /// Event emitted when an address has been removed from white list [address]
        AddressRemoved(H160),
        /// Event emitted when an existed address has been added to white list. [address]
        AddressExisted(H160),
        /// Event emitted when an address has been removed from white list but it wasn't in white list. [address]
        AddressNotExisted(H160)
    }

    #[pallet::error]
    pub enum Error<T> {
        NoPermission,
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    // #[pallet::getter(fn whitelist)]
    pub(super) type Whitelist<T: Config> = StorageMap<
        _, Blake2_128Concat, H160, bool, ValueQuery
    >;


    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {

        #[pallet::weight(1_000)]
        pub fn add_addresses(
            origin: OriginFor<T>,
            addresses: Vec<H160>
        ) -> DispatchResultWithPostInfo {
            for address in addresses.into_iter() {
                if Whitelist::<T>::contains_key(&address) {
                    Self::deposit_event(Event::AddressExisted(address));
                } else {
                    Whitelist::<T>::insert(address, true);
                    Self::deposit_event(Event::AddressAdded(address));
                }
            }
            Ok(().into())
        }

        #[pallet::weight(1_000)]
        pub fn remove_addresses(
            origin: OriginFor<T>,
            addresses: Vec<H160>
        ) -> DispatchResultWithPostInfo {
            for address in addresses.into_iter() {
                if !Whitelist::<T>::contains_key(&address) {
                    Self::deposit_event(Event::AddressNotExisted(address));
                } else {
                    Whitelist::<T>::remove(address);
                    Self::deposit_event(Event::AddressRemoved(address));
                }
            }
            Ok(().into())
        }

    }
}
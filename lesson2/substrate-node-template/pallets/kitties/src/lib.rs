#![cfg_attr(not(feature = "std"), no_std)]
use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*, traits::{Randomness, Currency, ReservableCurrency, ExistenceRequirement}, Parameter};
use frame_system::pallet_prelude::*;
use codec::{Encode, Decode};
use sp_runtime::{traits::{AtLeast32BitUnsigned, Bounded, One, CheckedAdd}};
use sp_io::hashing::blake2_128;

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, Eq)]
    pub struct Kitty(pub [u8; 16]);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config +  pallet_balances::Config{
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type Randomness: Randomness<Self::Hash>;
        type KittyIndex: Parameter + AtLeast32BitUnsigned + Bounded + Default + Copy;
        type KittyReserve: Get<Self::Balance>;
    }

    // Pallets use events to inform users when important changes are made.
    // Event documentation should end with an array that provides descriptive names for parameters.
    // https://substrate.dev/docs/en/knowledgebase/runtime/events
    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId", T::KittyIndex = "KittyIndex")]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        KittyCreated(T::AccountId, T::KittyIndex, Kitty),
        KittyTransferred(T::AccountId, T::AccountId, T::KittyIndex),
        KittyPriceUpdated(T::AccountId, T::KittyIndex, Option<T::Balance>),
        KittySold(T::AccountId, T::AccountId, T::KittyIndex, T::Balance),
    }

    #[pallet::error]
    pub enum Error<T> {
        KittiesCountOverFlow,
        InvalidKittyId,
        SameParentIndex,
        NotOwner,
        PriceTooLow,
        BuyFromSelf,
        NotForSale,
        MoneyNotEnough,
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn next_kitty_id)]
    pub type KittyId<T: Config> = StorageValue<_, T::KittyIndex, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn kitties)]
    pub type Kitties<T: Config> = StorageDoubleMap<_,
        Blake2_128Concat, T::AccountId,
        Blake2_128Concat, T::KittyIndex,
        Kitty, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn kitty_price)]
    pub type KittiesPrice<T: Config> = StorageMap<_,
        Blake2_128Concat, T::KittyIndex,
        T::Balance, OptionQuery
    >;

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // Dispatchable functions allows users to interact with the pallet and invoke state changes.
    // These functions materialize as "extrinsics", which are often compared to transactions.
    // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(1_000)]
        pub fn create(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;

            <pallet_balances::Pallet<T> as ReservableCurrency<T::AccountId>>::reserve(&sender, T::KittyReserve::get())
                .map_err(|_| Error::<T>::MoneyNotEnough )?;

            let kitty_id = Self::get_and_add_kitty_id()?;

            let dna = Self::random_value(&sender);

            let kitty = Kitty(dna);

            Kitties::<T>::insert(&sender, kitty_id, &kitty);

            Self::deposit_event(Event::KittyCreated(sender, kitty_id, kitty));
            Ok(().into())
        }

        #[pallet::weight(1_000)]
        pub fn transfer(origin: OriginFor<T>, to: T::AccountId, kitty_id: T::KittyIndex) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;

            Kitties::<T>::try_mutate_exists(sender.clone(), kitty_id, |kitty| -> DispatchResultWithPostInfo {
                if sender == to {
                    ensure!(kitty.is_some(), Error::<T>::InvalidKittyId);
                    return Ok(().into());
                }
                let kitty = kitty.take().ok_or(Error::<T>::InvalidKittyId)?;

                Kitties::<T>::insert(&to, kitty_id, kitty);

                Self::deposit_event(Event::KittyTransferred(sender, to, kitty_id));

                Ok(().into())
            })
        }

        #[pallet::weight(1_000)]
        pub fn breed(origin: OriginFor<T>, kitty_id_1: T::KittyIndex, kitty_id_2: T::KittyIndex) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;

            ensure!(kitty_id_1 != kitty_id_2, Error::<T>::SameParentIndex);

            let kitty_1 = Self::kitties(&sender, kitty_id_1).ok_or(Error::<T>::InvalidKittyId)?;
            let kitty_2 = Self::kitties(&sender, kitty_id_2).ok_or(Error::<T>::InvalidKittyId)?;

            let kitty_id = Self::get_and_add_kitty_id()?;

            let dna_1 = kitty_1.0;
            let dna_2 = kitty_2.0;

            let selector = Self::random_value(&sender);

            let mut new_dna = [0u8; 16];

            for i in 0..dna_1.len() {
                new_dna[i] = Self::combine_dna(dna_1[i], dna_2[i], selector[i]);
            }

            let kitty = Kitty(new_dna);

            Kitties::<T>::insert(&sender, kitty_id, &kitty);

            Self::deposit_event(Event::KittyCreated(sender, kitty_id, kitty));

            Ok(().into())
        }

        #[pallet::weight(1_000)]
        pub fn set_price(origin: OriginFor<T>, kitty_id: T::KittyIndex, new_price: Option<T::Balance>) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;

            ensure!(Kitties::<T>::contains_key(&sender, kitty_id), Error::<T>::NotOwner);

            KittiesPrice::<T>::mutate_exists(kitty_id, |price| *price = new_price.clone());

            Self::deposit_event(Event::KittyPriceUpdated(sender, kitty_id, new_price));
            Ok(().into())
        }

        #[pallet::weight(1_000)]
        pub fn buy(origin: OriginFor<T>, owner: T::AccountId, kitty_id: T::KittyIndex, max_price: T::Balance) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;

            ensure!(sender != owner, Error::<T>::BuyFromSelf);

            Kitties::<T>::try_mutate_exists(owner.clone(), kitty_id, |kitty| -> DispatchResultWithPostInfo {
                let kitty = kitty.take().ok_or(Error::<T>::InvalidKittyId)?;
                KittiesPrice::<T>::try_mutate_exists(kitty_id, |price| -> DispatchResultWithPostInfo {
                    let price = price.take().ok_or(Error::<T>::NotForSale)?;

                    ensure!(max_price >= price, Error::<T>::PriceTooLow);

                    <pallet_balances::Pallet<T> as Currency<T::AccountId>>::transfer(&sender, &owner, price, ExistenceRequirement::KeepAlive)?;

                    Kitties::<T>::insert(&sender, kitty_id, kitty);

                    Self::deposit_event(Event::KittySold(sender, owner, kitty_id, price));

                    Ok(().into())
                })
            })
        }
    }
}



impl<T: Config> Pallet<T> {
    fn random_value(sender: &T::AccountId) -> [u8; 16] {
        let payload = (
            T::Randomness::random_seed(),
            &sender,
            <frame_system::Pallet<T>>::extrinsic_index(),
        );
        payload.using_encoded(blake2_128)
    }

    fn combine_dna(dna1: u8, dna2: u8, selector: u8) -> u8 {
        (!selector & dna1) | (selector & dna2)
    }

    fn get_and_add_kitty_id() -> Result<T::KittyIndex, DispatchError> {
        KittyId::<T>::try_mutate(|next_id| -> Result<T::KittyIndex, DispatchError> {
            let current_id = *next_id;
            *next_id = next_id.checked_add(&One::one()).ok_or(Error::<T>::KittiesCountOverFlow)?;
            Ok(current_id)
        })
    }
}


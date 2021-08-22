#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        type StringLimit: Get<u32>;
    }

    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        // 存证创建时触发的事件. [创建人, 存证]
        ClaimCreated(T::AccountId, Vec<u8>),
        // 存证销毁时触发的事件. [创建人, 存证]
        ClaimRevoked(T::AccountId, Vec<u8>),
    }

    #[pallet::error]
    pub enum Error<T> {
        ProofAlreadyClaimed,
        NoSuchProof,
        NotProofOwner,
        BadMetadata
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    pub(super) type Proofs<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, (T::AccountId, T::BlockNumber), ValueQuery>;

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(1_000)]
        pub(super) fn create_claim(
            origin: OriginFor<T>,
            proof: Vec<u8>,
        ) -> DispatchResultWithPostInfo {
            // 存证限制长度
            ensure!(proof.len() <= T::StringLimit::get() as usize, Error::<T>::BadMetadata);

            let sender = ensure_signed(origin)?;

            ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofAlreadyClaimed);

            let current_block = <frame_system::Module<T>>::block_number();

            Proofs::<T>::insert(&proof, (&sender, current_block));

            Self::deposit_event(Event::ClaimCreated(sender, proof));

            Ok(().into())
        }

        #[pallet::weight(10_000)]
        pub fn revoke_claim(
            origin: OriginFor<T>,
            proof: Vec<u8>,
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;

            ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

            let (owner, _) = Proofs::<T>::get(&proof);

            ensure!(sender == owner, Error::<T>::NotProofOwner);

            Proofs::<T>::remove(&proof);

            Self::deposit_event(Event::ClaimRevoked(sender, proof));

            Ok(().into())
        }

        #[pallet::weight(10_000)]
        pub fn transfer_claim(
            origin: OriginFor<T>,
            proof: Vec<u8>,
            dist: T::AccountId
        ) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;

            ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);

            let (owner, _block_number) = Proofs::<T>::get(&proof);

            ensure!(owner == sender, Error::<T>::NotProofOwner);

            Proofs::<T>::insert(&proof, (dist, <frame_system::Module<T>>::block_number()));

            Ok(().into())
        }
    }
}
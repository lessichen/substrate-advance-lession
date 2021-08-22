#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

// 引入两个模块
#[cfg(test)]  // 标签表示：测试的时候才会被编译
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
        pub(super) fn create_claim(origin: OriginFor<T>, proof: Vec<u8>) -> DispatchResultWithPostInfo {
            // 存证限制长度
            ensure!(proof.len() <= T::StringLimit::get() as usize, Error::<T>::BadMetadata);
            // 验证签名
            let sender = ensure_signed(origin)?;
            // 验证存证是否已经存在，存在则返回Proof已存在错误
            ensure!(!Proofs::<T>::contains_key(&proof), Error::<T>::ProofAlreadyClaimed);
            // 获取当前block值
            let current_block = <frame_system::Module<T>>::block_number();
            // 插入存证
            Proofs::<T>::insert(&proof, (&sender, current_block));
            // 触发相应事件
            Self::deposit_event(Event::ClaimCreated(sender, proof));
            // 返回OK状态
            Ok(().into())
        }

        #[pallet::weight(10_000)]
        pub fn revoke_claim(origin: OriginFor<T>, proof: Vec<u8>) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;
            // 验证签名是否存在，不存在则返回无该存证错误
            ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);
            // 获取存证信息
            let (owner, _) = Proofs::<T>::get(&proof);
            // 验证请求发送人是否存证所有者，不是则报错
            ensure!(owner == sender, Error::<T>::NotProofOwner);
            // 移除存证
            Proofs::<T>::remove(&proof);
            // 触发事件
            Self::deposit_event(Event::ClaimRevoked(sender, proof));
            // 返回OK
            Ok(().into())
        }

        #[pallet::weight(10_000)]
        pub fn transfer_claim(origin: OriginFor<T>, proof: Vec<u8>, dist: T::AccountId) -> DispatchResultWithPostInfo {
            // 交易发送方、存证、转移地址
            // 验证交易时被签名的
            let sender = ensure_signed(origin)?;
            // 验证签名是否存在，不存在则返回无该存证错误
            ensure!(Proofs::<T>::contains_key(&proof), Error::<T>::NoSuchProof);
            // 获取存证信息
            let (owner, _block_number) = Proofs::<T>::get(&proof);
            // 验证请求发送人是否存证所有者，不是则报错
            ensure!(owner == sender, Error::<T>::NotProofOwner);
            // 将存证插入目的地址，使用方法获取当前块
            Proofs::<T>::insert(&proof, (dist, <frame_system::Module<T>>::block_number()));

            Ok(().into())
        }
    }
}
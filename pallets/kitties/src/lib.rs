#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	pallet_prelude::*,
	traits::Randomness,
};
use frame_system::pallet_prelude::*;
pub use pallet::*;
use scale_info::TypeInfo;
use sp_io::hashing::blake2_128;

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub struct Kitty(pub [u8; 16]);

#[frame_support::pallet]
pub mod pallet {
	use super::*;


	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_randomness_collective_flip::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::storage]
	#[pallet::getter(fn kitties)]
	pub type Kitties<T: Config> = StorageDoubleMap<_,
		Blake2_128Concat, T::AccountId,
		Blake2_128Concat, u32,
		Kitty, OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn next_kitty_id)]
	pub type NextKittyId<T: Config> = StorageValue<_, u32, ValueQuery>;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		KittyCreated(T::AccountId, u32, Kitty),
	}


	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(1000)]
		pub fn create(origin: OriginFor<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let payload = (
				<pallet_randomness_collective_flip::Pallet<T> as Randomness<T::Hash, T::BlockNumber>>::random_seed().0,
				&sender,
				<frame_system::Pallet<T>>::extrinsic_index(),
			);

			let dna = payload.using_encoded(blake2_128);
			let kitty_id = Self::next_kitty_id();
			let kitty = Kitty(dna);

			Kitties::<T>::insert(&sender, kitty_id, kitty.clone());
			NextKittyId::<T>::put(kitty_id + 1);

			Self::deposit_event(Event::KittyCreated(sender, kitty_id, kitty));

			Ok(())
		}
	}
}
#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_support::{
		sp_runtime::traits::Hash,
		traits::{tokens::ExistenceRequirement, Currency, Randomness},
		transactional,
	};
	use scale_info::prelude::vec::Vec;
	use scale_info::prelude::string::String;
	use frame_system::pallet_prelude::*;

	use scale_info::TypeInfo;
	use sp_io::hashing::blake2_128;

	#[cfg(feature = "std")]
	use frame_support::serde::{Deserialize, Serialize};
	type AccountOf<T> = <T as frame_system::Config>::AccountId;

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct UserInfo {
		pub user_name: Vec<u8>,
		pub id: Vec<u8>,
		pub address: Vec<u8>,

		pub product_name: Vec<u8>,
		pub datetime: Vec<u8>,
	}


	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The type of Randomness we want to specify for this pallet.
		type TraceRandomness: Randomness<Self::Hash, Self::BlockNumber>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		Created(T::AccountId, T::Hash),
	}

	// Storage items.

	#[pallet::storage]
	#[pallet::getter(fn user_cnt)]
	/// Keeps track of the number of Kitties in existence.
	pub(super) type UserCnt<T: Config> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn users)]
	/// Stores a Kitty's unique traits, owner and price.
	pub(super) type UserInfos<T: Config> = StorageMap<_, Twox64Concat, T::Hash, UserInfo>;

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		/// Ensures that an account has enough funds to purchase a Kitty.
		NotEnoughBalance,

		UserCntOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.


		// Login
		#[pallet::weight(100)]
		pub fn login(origin: OriginFor<T>) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let user = UserInfo {
				user_name: String::from("DuongHB").as_bytes().to_vec(),
				id: String::from("12345678").as_bytes().to_vec(),
				address: String::from("Hung Yen").as_bytes().to_vec(),
				product_name: "".as_bytes().to_vec(),
				datetime: "".as_bytes().to_vec(),
			};

			let hash_id = T::Hashing::hash_of(&user);

			// Performs this operation first as it may fail
			let new_cnt = Self::user_cnt().checked_add(1).ok_or(<Error<T>>::UserCntOverflow)?;


			<UserInfos<T>>::insert(hash_id, user);
			<UserCnt<T>>::put(new_cnt);

			Self::deposit_event(Event::Created(sender, hash_id));
			Ok(())
		}
	}
}

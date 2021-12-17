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

pub use pallet_timestamp;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_support::{
		sp_runtime::traits::Hash,
		traits::{tokens::ExistenceRequirement, Currency, Randomness},
		transactional,
	};

	use scale_info::prelude::vec::Vec;
	//use sp_std::vec::Vec;
	use frame_system::pallet_prelude::*;
	use scale_info::prelude::string::String;

	use scale_info::TypeInfo;
	use sp_io::hashing::blake2_128;

	#[cfg(feature = "std")]
	use frame_support::serde::{Deserialize, Serialize};
	type AccountOf<T> = <T as frame_system::Config>::AccountId;

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct UserInfo<T: Config> {
		pub user_name: Vec<u8>,
		pub rd: [u8; 16],
		pub address: Vec<u8>,
		pub owner: AccountOf<T>,

		pub product_name: Vec<u8>,
		pub datetime: u64,
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The maximum amount of Kitties a single account can own.
		#[pallet::constant]
		type MaxInfoOwned: Get<u32>;

		/// The type of Randomness we want to specify for this pallet.
		type TraceRandomness: Randomness<Self::Hash, Self::BlockNumber>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		// Create new
		Created(T::AccountId, T::Hash),

		// Update info
		Updated(T::AccountId, T::Hash),

		// Check log
		TraceInfo(T::AccountId, T::Hash),
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn user_cnt)]
	/// Keeps track of the number of food in existence.
	pub(super) type LogCnt<T: Config> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn log_infos)]
	/// Stores a All User Info unique traits to check Log
	pub(super) type LogInfos<T: Config> = StorageMap<_, Twox64Concat, T::Hash, UserInfo<T>>;

	#[pallet::storage]
	#[pallet::getter(fn users)]
	/// Stores a All User Info unique traits to check Log
	pub(super) type UserInfos<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, UserInfo<T>>;

	#[pallet::storage]
	#[pallet::getter(fn info_owned)]
	/// Keeps track of what accounts own what Kitty.
	pub(super) type LogInfosOwned<T: Config> =
		StorageMap<_, Twox64Concat, T::Hash, BoundedVec<UserInfo<T>, T::MaxInfoOwned>, OptionQuery>;

	/*
	#[pallet::storage]
	#[pallet::getter(fn info_owned)]
	/// Keeps track of what accounts own what Kitty.
	pub(super) type UserInfosOwned<T: Config> = StorageMap<
		_,
		Twox64Concat,
		T::AccountId,
		BoundedVec<T::Hash, T::MaxInfoOwned>,
		OptionQuery,
	>;
	*/

	/*
	// Our pallet's genesis configuration.
	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub users: Vec<(T::AccountId, [u8; 16], Vec<u8>, Vec<u8>)>,
	}

	// Required to implement default for GenesisConfig.
	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> GenesisConfig<T> {
			GenesisConfig { users: vec![] }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			// When building a kitty from genesis config, we require the dna and gender to be supplied.
			for (acct, rd, user_name, user_add) in &self.users {
				let _ = <Pallet<T>>::mint(acct, rd.clone(), user_name.clone(), user_add.clone());
			}
		}
	}
	*/

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		/// Ensures that an account has enough funds to purchase a Kitty.
		NotEnoughBalance,
		ExceedMaxLogOwned,
		UserCntOverflow,
		AccountNotExist,
		LogInfoNotExist,
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
		pub fn register_user(
			origin: OriginFor<T>,
			user_name: Vec<u8>,
			address: Vec<u8>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let user = UserInfo {
				user_name,
				owner: sender.clone(),
				address,
				rd: Self::gen_rd(),

				product_name: "".as_bytes().to_vec(),
				datetime: 0,
			};

			//let hash_id = T::Hashing::hash_of(&user);
			<UserInfos<T>>::insert(sender, user);

			Ok(())
		}

		// Login
		#[pallet::weight(100)]
		pub fn create_ability(origin: OriginFor<T>, product_name: Vec<u8>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let mut user = Self::users(&sender).ok_or(<Error<T>>::AccountNotExist)?;

			user.product_name = product_name;
			//user.datetime = pallet_timestamp::pallet;

			let hash_id =
				Self::mint(&sender, user.rd, user.user_name, user.address, user.product_name)?;

			// Logging to the console
			// log::info!("A HashID: {:?}.", hash_id);

			Self::deposit_event(Event::Created(sender, hash_id));
			Ok(())
		}

		// Login
		#[pallet::weight(100)]
		pub fn update_ability(origin: OriginFor<T>, hash_id: T::Hash) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			let user = Self::users(&sender).ok_or(<Error<T>>::AccountNotExist)?;

			// Performs this operation first as it may fail
			let new_cnt = Self::user_cnt().checked_add(1).ok_or(<Error<T>>::UserCntOverflow)?;

			// Check Hash ID exist


			// Performs this operation first because as it may fail
			<LogInfosOwned<T>>::try_mutate(&hash_id, |log_vec| log_vec.try_push(user))
				.map_err(|_| <Error<T>>::ExceedMaxLogOwned)?;

			<LogInfos<T>>::insert(hash_id, user);
			<LogCnt<T>>::put(new_cnt);
			// Logging to the console
			// log::info!("A HashID: {:?}.", hash_id);

			Ok(())
		}
	}

	//** Our helper functions.**//

	impl<T: Config> Pallet<T> {
		pub fn mint(
			owner: &T::AccountId,
			rd: [u8; 16],
			user_name: Vec<u8>,
			user_add: Vec<u8>,
			product_name: Vec<u8>,
		) -> Result<T::Hash, Error<T>> {
			let mut user_info = UserInfo::<T> {
				user_name,
				rd,
				owner: owner.clone(),
				address: user_add,
				product_name,
				datetime: 0,
			};

			let hash_id = T::Hashing::hash_of(&user_info);

			// Performs this operation first as it may fail
			let new_cnt = Self::user_cnt().checked_add(1).ok_or(<Error<T>>::UserCntOverflow)?;

			// Performs this operation first because as it may fail
			<LogInfosOwned<T>>::try_mutate(&hash_id, |log_vec| log_vec.try_push(user_info))
				.map_err(|_| <Error<T>>::ExceedMaxLogOwned)?;

			<LogInfos<T>>::insert(hash_id, user_info);
			<LogCnt<T>>::put(new_cnt);

			Ok(hash_id)
		}

		fn gen_rd() -> [u8; 16] {
			let payload = (
				T::TraceRandomness::random(&b"rd"[..]).0,
				<frame_system::Pallet<T>>::block_number(),
			);
			payload.using_encoded(blake2_128)
		}

		pub fn is_reg_exist(acct: &T::AccountId) -> Result<bool, Error<T>> {
			match Self::users(acct) {
				Some(user) => Ok(true),
				None => Err(<Error<T>>::AccountNotExist),
			}
		}
	}
}

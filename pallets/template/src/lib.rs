#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://substrate.dev/docs/en/knowledgebase/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use sp_runtime::traits::MaybeDisplay;
	use sp_runtime::traits::AtLeast32Bit;	
	use frame_support::dispatch::fmt::Debug;
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use sp_std::prelude::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Account index (aka nonce) type. This stores the number of previous transactions
		/// associated with a sender account.
		type GameID: Parameter
			+ Member
			+ MaybeSerializeDeserialize
			+ Debug
			+ Default
			+ MaybeDisplay
			+ AtLeast32Bit
			+ Copy
			+ Encode
			+ Decode;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn score)]
	pub(super) type Score<T> = StorageValue<_, u32>;

	// type Skey = Vec<u8>;
	// type Sval = Vec<u8>;
	// type DataEntry = (Skey,Sval);
	// type DataRecord = Vec<DataEntry>;
	// pub(super) type GameAccount<T:Config> = (T::GameID, T::AccountId);

	// #[pallet::storage]
	// pub(super) type WorldDataExternalMap<T: Config> = StorageMap<_, Twox64Concat, T::GameID, DataRecord, ValueQuery>;

	// #[pallet::storage]
	// pub(super) type WorldDataInternalMap<T: Config> = StorageMap<_, Twox64Concat, T::GameID, DataRecord, ValueQuery>;

	// #[pallet::storage]
	// pub(super) type UserDataInternalMap<T: Config> = StorageMap<_, Twox64Concat, GameAccount<T>, DataRecord, ValueQuery>;

	// #[pallet::storage]
	// pub(super) type UserDataExternalMap<T: Config> = StorageMap<_, Twox64Concat, GameAccount<T>, DataRecord, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn authorities_map)]
	pub(super) type AuthoritiesMap<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, Vec<T::GameID>, ValueQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://substrate.dev/docs/en/knowledgebase/runtime/events
	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),

		// [current level, who leveled up]
		LevelUp(u32, T::AccountId)
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,

		AlreadyRegisteredGame,

		AlreadyRegisteredAuthority,

		InvalidAuthority,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}	

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn increment_score(origin: OriginFor<T>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;

			let new_score;

			// Read a value from storage.
			match <Score<T>>::get() {
				None => {
					// Update the value with initial score point.
					new_score = 1;	
				}

				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					new_score = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
				},
			}

			// Update the value in storage with the incremented result.
			<Score<T>>::put(new_score);

			Self::deposit_event(Event::LevelUp(new_score, who));

			Ok(())		
		}
		
		#[pallet::weight(10_000)]
		pub fn register_game(origin: OriginFor<T>, game : T::GameID) -> DispatchResult
		{
			let who = ensure_signed(origin)?;
			
			for e in <AuthoritiesMap<T>>::iter()
			{
				let authorized_games = &e.1;
				frame_support::ensure!(!authorized_games.contains(&game), Error::<T>::AlreadyRegisteredGame);
			}

			<AuthoritiesMap<T>>::insert(&who, vec!(game));

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn add_authority(origin: OriginFor<T>, game : T::GameID, new_authority : T::AccountId) -> DispatchResult
		{			
			let who = ensure_signed(origin)?;

			// Ensure only authorities add other authorities.
			frame_support::ensure!(<AuthoritiesMap<T>>::contains_key(&who), Error::<T>::InvalidAuthority);
			frame_support::ensure!(<AuthoritiesMap<T>>::get(&who).contains(&game), Error::<T>::InvalidAuthority);

			// Ensure no duplicate new authorities.
			if !<AuthoritiesMap<T>>::contains_key(&new_authority)
			{
				<AuthoritiesMap<T>>::insert(&new_authority, vec!(game));
			}
			else
			{
				frame_support::ensure!(!<AuthoritiesMap<T>>::get(&new_authority).contains(&game), Error::<T>::InvalidAuthority);		
				// Modify the authorities map to include a new authority
				<AuthoritiesMap<T>>::mutate(&new_authority, |x| {x.push(game);} );
			}



			Ok(())
		}
	}
}

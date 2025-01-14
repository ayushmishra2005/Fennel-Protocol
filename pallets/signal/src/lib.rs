#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{
        dispatch::DispatchResult,
        pallet_prelude::*,
        traits::{Currency, LockIdentifier, LockableCurrency, WithdrawReasons},
    };
    use frame_system::pallet_prelude::*;

    use crate::weights::WeightInfo;

    const LOCK_ID: LockIdentifier = *b"fnlsignl";

    type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type WeightInfo: WeightInfo;
        type Currency: LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;
        type MaxSize: Get<u32>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::unbounded]
    #[pallet::getter(fn rating_signal_list)]
    /// Maps identity numbers to a signal transaction hash and a rating number.
    pub type RatingSignalList<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Blake2_128Concat,
        BoundedVec<u8, T::MaxSize>,
        u8,
        ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::unbounded]
    #[pallet::getter(fn whiteflag_rating_signal_list)]
    /// Maps identity numbers to a signal transaction hash and a rating number.
    pub type WhiteflagRatingSignalList<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Blake2_128Concat,
        BoundedVec<u8, T::MaxSize>,
        u8,
        ValueQuery,
    >;

    #[pallet::storage]
    #[pallet::unbounded]
    #[pallet::getter(fn signal_paramter_list)]
    /// Maps identity numbers to a signal transaction hash and a rating number.
    pub type SignalParameterList<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Blake2_128Concat,
        BoundedVec<u8, T::MaxSize>,
        u8,
        ValueQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        SignalParameterSet(BoundedVec<u8, T::MaxSize>, u8, T::AccountId),
        SignalSent(BoundedVec<u8, T::MaxSize>, T::AccountId),
        RatingSignalSent(BoundedVec<u8, T::MaxSize>, u8, T::AccountId),
        WhiteflagRatingSignalSent(BoundedVec<u8, T::MaxSize>, u8, T::AccountId),
        RatingSignalUpdated(BoundedVec<u8, T::MaxSize>, u8, T::AccountId),
        WhiteflagRatingSignalUpdated(BoundedVec<u8, T::MaxSize>, u8, T::AccountId),
        RatingSignalRevoked(BoundedVec<u8, T::MaxSize>, T::AccountId),
        WhiteflagRatingSignalRevoked(BoundedVec<u8, T::MaxSize>, T::AccountId),
        ServiceSignalSent(BoundedVec<u8, T::MaxSize>, BoundedVec<u8, T::MaxSize>, T::AccountId),
        SignalLock(<T as frame_system::Config>::AccountId, BalanceOf<T>),
        SignalLockExtended(<T as frame_system::Config>::AccountId, BalanceOf<T>),
        SignalUnlock(<T as frame_system::Config>::AccountId),
    }

    #[pallet::error]
    pub enum Error<T> {
        NoneValue,
        StorageOverflow,
        InsufficientBalance,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Defines coefficients that participants should use to weight rating functions.
        #[pallet::weight(<T as Config>::WeightInfo::set_signal_parameter())]
        #[pallet::call_index(0)]
        pub fn set_signal_parameter(
            origin: OriginFor<T>,
            name: BoundedVec<u8, T::MaxSize>,
            value: u8,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            <SignalParameterList<T>>::insert(who.clone(), name.clone(), value);
            Self::deposit_event(Event::SignalParameterSet(name, value, who));

            Ok(())
        }

        /// Creates an on-chain event with a transaction hash as a pointer and a u8 as a rating
        /// number.
        #[pallet::weight(<T as Config>::WeightInfo::send_rating_signal())]
        #[pallet::call_index(1)]
        pub fn send_rating_signal(
            origin: OriginFor<T>,
            target: BoundedVec<u8, T::MaxSize>,
            rating: u8,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            if T::Currency::free_balance(&who) <= 10u32.into() {
                return Err(Error::<T>::InsufficientBalance.into())
            }
            <RatingSignalList<T>>::insert(who.clone(), target.clone(), rating);
            T::Currency::set_lock(LOCK_ID, &who, 10u32.into(), WithdrawReasons::all());
            Self::deposit_event(Event::SignalLock(who.clone(), 10u32.into()));
            Self::deposit_event(Event::RatingSignalSent(target, rating, who));

            Ok(())
        }

        #[pallet::weight(<T as Config>::WeightInfo::send_whiteflag_rating_signal())]
        #[pallet::call_index(2)]
        pub fn send_whiteflag_rating_signal(
            origin: OriginFor<T>,
            target: BoundedVec<u8, T::MaxSize>,
            rating: u8,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            if T::Currency::free_balance(&who) <= 10u32.into() {
                return Err(Error::<T>::InsufficientBalance.into())
            }
            <WhiteflagRatingSignalList<T>>::insert(who.clone(), target.clone(), rating);
            T::Currency::set_lock(LOCK_ID, &who, 10u32.into(), WithdrawReasons::all());
            Self::deposit_event(Event::SignalLock(who.clone(), 10u32.into()));
            Self::deposit_event(Event::WhiteflagRatingSignalSent(target, rating, who));

            Ok(())
        }

        #[pallet::weight(<T as Config>::WeightInfo::update_whiteflag_rating_signal())]
        #[pallet::call_index(3)]
        pub fn update_whiteflag_rating_signal(
            origin: OriginFor<T>,
            target: BoundedVec<u8, T::MaxSize>,
            new_rating: u8,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            if T::Currency::free_balance(&who) <= 10u32.into() {
                return Err(Error::<T>::InsufficientBalance.into())
            }
            <WhiteflagRatingSignalList<T>>::insert(who.clone(), target.clone(), new_rating);
            T::Currency::extend_lock(LOCK_ID, &who, 10u32.into(), WithdrawReasons::all());
            Self::deposit_event(Event::SignalLockExtended(who.clone(), 10u32.into()));
            Self::deposit_event(Event::WhiteflagRatingSignalUpdated(target, new_rating, who));

            Ok(())
        }

        /// Updates an existing rating signal.
        #[pallet::weight(<T as Config>::WeightInfo::update_rating_signal())]
        #[pallet::call_index(4)]
        pub fn update_rating_signal(
            origin: OriginFor<T>,
            target: BoundedVec<u8, T::MaxSize>,
            new_rating: u8,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            if T::Currency::free_balance(&who) <= 10u32.into() {
                return Err(Error::<T>::InsufficientBalance.into())
            }
            <RatingSignalList<T>>::insert(who.clone(), target.clone(), new_rating);
            T::Currency::extend_lock(LOCK_ID, &who, 10u32.into(), WithdrawReasons::all());
            Self::deposit_event(Event::SignalLockExtended(who.clone(), 10u32.into()));
            Self::deposit_event(Event::RatingSignalUpdated(target, new_rating, who));

            Ok(())
        }

        /// Puts out a signal cancelling a previous rating.
        #[pallet::weight(<T as Config>::WeightInfo::revoke_rating_signal())]
        #[pallet::call_index(5)]
        pub fn revoke_rating_signal(
            origin: OriginFor<T>,
            target: BoundedVec<u8, T::MaxSize>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            <RatingSignalList<T>>::remove(who.clone(), target.clone());
            T::Currency::remove_lock(LOCK_ID, &who);
            Self::deposit_event(Event::SignalUnlock(who.clone()));
            Self::deposit_event(Event::RatingSignalRevoked(target, who));

            Ok(())
        }

        #[pallet::weight(<T as Config>::WeightInfo::revoke_whiteflag_rating_signal())]
        #[pallet::call_index(6)]
        pub fn revoke_whiteflag_rating_signal(
            origin: OriginFor<T>,
            target: BoundedVec<u8, T::MaxSize>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            <WhiteflagRatingSignalList<T>>::remove(who.clone(), target.clone());
            T::Currency::remove_lock(LOCK_ID, &who);
            Self::deposit_event(Event::SignalUnlock(who.clone()));
            Self::deposit_event(Event::WhiteflagRatingSignalRevoked(target, who));

            Ok(())
        }

        /// Creates an on-chain event with a signal payload defined as part of the transaction
        /// without relying on storage.
        #[pallet::weight(<T as Config>::WeightInfo::send_signal())]
        #[pallet::call_index(7)]
        pub fn send_signal(
            origin: OriginFor<T>,
            signal: BoundedVec<u8, T::MaxSize>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            Self::deposit_event(Event::SignalSent(signal, who));
            Ok(())
        }

        /// Sends a hexadecimal signal tagged for a particular application or service using Fennel
        /// Protocol.
        #[pallet::weight(<T as Config>::WeightInfo::send_service_signal())]
        #[pallet::call_index(8)]
        pub fn send_service_signal(
            origin: OriginFor<T>,
            service_identifier: BoundedVec<u8, T::MaxSize>,
            url: BoundedVec<u8, T::MaxSize>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            Self::deposit_event(Event::ServiceSignalSent(service_identifier, url, who));
            Ok(())
        }
    }
}

pub(crate) mod spaced_list;
pub(crate) mod range_spaced_list;
pub(crate) mod hollow_spaced_list;
pub(crate) mod hollow_range_spaced_list;

#[doc(inline)]
pub use {
    spaced_list::Manager,
    spaced_list::LockedPosition,
    spaced_list::handles::{PositionsHandle, InsertionsHandle, ValuesHandle},
    spaced_list::locks::{PositionsLock, InsertionsLock, ValuesLock},

    range_spaced_list::RangeManager,
    range_spaced_list::RangeLockedPosition,
    range_spaced_list::handles::{RangePositionsHandle, RangeInsertionsHandle, RangeValuesHandle},
    range_spaced_list::locks::{RangePositionsLock, RangeInsertionsLock, RangeValuesLock},

    hollow_spaced_list::HollowManager,
    hollow_spaced_list::HollowLockedPosition,
    hollow_spaced_list::handles::{HollowPositionsHandle, HollowInsertionsHandle},
    hollow_spaced_list::locks::{HollowPositionsLock, HollowInsertionsLock},

    hollow_range_spaced_list::HollowRangeManager,
    hollow_range_spaced_list::HollowRangeLockedPosition,
    hollow_range_spaced_list::handles::{HollowRangePositionsHandle, HollowRangeInsertionsHandle},
    hollow_range_spaced_list::locks::{HollowRangePositionsLock, HollowRangeInsertionsLock},
};

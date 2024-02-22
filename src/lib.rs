//! This crate implements list types that store the distance between their elements.
//!
//! These lists might be what you need if you:
//! 1. want a sorted list of numbers.
//! 2. want a list where the elements have a defined position.
//! 3. want a list where changing the position of one element should change the position of all
//!    elements after it.
//!
//! There are several different types of lists in this crate:
//! 1. `SpacedList`: A spaced list that stores values and the distance between them.
//! 2. `RangeSpacedList`: A spaced list that stores elements as non-overlapping ranges.
//! 3. `HollowSpacedList`: A spaced list that stores the relative position of empty nodes.
//! 4. `HollowRangeSpacedList`: A spaced list that stores empty nodes as non-overlapping ranges.
// TODO add NestedRange stuff when the manager module fully supports it

#![feature(trait_alias)]

// used ONLY for the prefetch_read_data intrinsic, which is used in loop.rs for a significant performance gain
#![allow(internal_features)]
#![feature(core_intrinsics)]

// #![warn(clippy::pedantic)]
// #![allow(clippy::needless_pass_by_value, clippy::missing_errors_doc, clippy::module_name_repetitions)]
#![allow(dead_code)]

use std::ops::{Add, AddAssign, Sub, SubAssign};

use num_traits::Zero;

pub trait Spacing = Add<Output=Self> + AddAssign + Sub<Output=Self> + SubAssign + Zero + Ord + Copy;

pub mod manager;

#[doc(inline)]
pub use {
    spaced_lists::spaced_list::SpacedList,
    spaced_lists::range_spaced_list::RangeSpacedList,
    spaced_lists::nested_range_spaced_list::NestedRangeSpacedList,
    spaced_lists::hollow_spaced_list::HollowSpacedList,
    spaced_lists::hollow_range_spaced_list::HollowRangeSpacedList,
    spaced_lists::hollow_nested_range_spaced_list::HollowNestedRangeSpacedList,

    skeleton::position::Position,
    skeleton::position::HollowPosition,
    skeleton::index::Index,
    skeleton::index::HollowIndex,
    skeleton::bound_type::BoundType,

    skeleton::Node,
    skeleton::Range,
    skeleton::NestedRange,
    skeleton::RangeKind,

    skeleton::element_ref::ElementRef,
    skeleton::element_ref::ElementRefMut,

    skeleton::node::PushError,
    skeleton::range::RangePushError,
    skeleton::range::RangeInsertionError,
    skeleton::nested_range::NestedRangePushError,
    skeleton::nested_range::NestedRangeInsertionError,
    spaced_lists::SpacingError,
};


pub(crate) mod skeleton;
pub(crate) mod spaced_lists;

pub(crate) use {
    skeleton::Skeleton,
    skeleton::ParentData,
    skeleton::ephemeral_position::EphemeralPosition,
    skeleton::ephemeral_index::EphemeralIndex,
    skeleton::traversal::iteration::BackwardsIter,
    skeleton::traversal::iteration::ForwardsIter,
};

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
#![feature(macro_metavar_expr)]
#![feature(option_take_if)]

// used ONLY for the prefetch_read_data intrinsic, which is used in loop.rs for a significant performance gain
#![allow(internal_features)]
#![feature(core_intrinsics)]

// #![warn(clippy::pedantic)]
// #![allow(clippy::needless_pass_by_value, clippy::missing_errors_doc, clippy::module_name_repetitions)]
#![allow(dead_code)]

use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign, Sub, SubAssign};

use num_traits::Zero;

pub trait Spacing = Add<Output=Self> + AddAssign + Sub<Output=Self> + SubAssign + Zero + Ord + Copy + Display + Debug;

pub mod manager;

#[doc(inline)]
pub use {
    spaced_lists::HollowNestedRangeSpacedList,
    spaced_lists::HollowRangeSpacedList,
    spaced_lists::HollowSpacedList,
    spaced_lists::NestedRangeSpacedList,
    spaced_lists::RangeSpacedList,
    spaced_lists::SpacedList,

    skeleton::Node,
    skeleton::Range,
    skeleton::NestedRange,
    skeleton::RangeKind,

    skeleton::bound_type::BoundType,
    skeleton::index::Index,
    skeleton::index::HollowIndex,
    skeleton::position::Position,
    skeleton::position::HollowPosition,
    skeleton::element_ref::ElementRef,
    skeleton::element_ref::ElementRefMut,
    skeleton::ElementSlot,

    skeleton::node::PushError,
    skeleton::range::RangePushError,
    skeleton::range::RangeInsertionError,
    skeleton::nested_range::NestedRangePushError,
    skeleton::nested_range::NestedRangeInsertionError,
    skeleton::change_spacing::SpacingError,
};


pub(crate) mod skeleton;
pub(crate) mod spaced_lists;

macro_rules! display_unwrap {
    ($arg:expr) => {
        match $arg {
            Err(error) => panic!("{}", error),
            Ok(value) => value
        }
    };
}

pub(crate) use {
    skeleton::Skeleton,
    skeleton::ParentData,
    skeleton::ephemeral_index::EphemeralIndex,
    skeleton::ephemeral_position::EphemeralPosition,
    skeleton::traversal::iteration::BackwardsIter,
    skeleton::traversal::iteration::ForwardsIter,
    display_unwrap,
};

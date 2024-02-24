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
    skeleton::bound_type::BoundType,
    skeleton::element_ref::ElementRef,
    skeleton::element_ref::ElementRefMut,
    skeleton::index::HollowIndex,
    skeleton::index::Index,
    skeleton::nested_range::NestedRangeInsertionError,
    skeleton::nested_range::NestedRangePushError,
    skeleton::NestedRange,
    skeleton::Node,
    skeleton::node::PushError,
    skeleton::position::HollowPosition,
    skeleton::position::Position,
    skeleton::Range,
    skeleton::range::RangeInsertionError,
    skeleton::range::RangePushError,
    skeleton::RangeKind,
    spaced_lists::HollowNestedRangeSpacedList,
    spaced_lists::HollowRangeSpacedList,
    spaced_lists::HollowSpacedList,
    spaced_lists::NestedRangeSpacedList,
    spaced_lists::RangeSpacedList,
    spaced_lists::SpacedList,
};


pub(crate) mod skeleton;
pub(crate) mod spaced_lists;

pub(crate) use {
    skeleton::ephemeral_index::EphemeralIndex,
    skeleton::ephemeral_position::EphemeralPosition,
    skeleton::ParentData,
    skeleton::Skeleton,
    skeleton::traversal::iteration::BackwardsIter,
    skeleton::traversal::iteration::ForwardsIter,
};

macro_rules! display_unwrap {
    ($arg:expr) => {
        match $arg {
            Err(error) => panic!("{}", error),
            Ok(value) => value
        }
    };
}

pub(crate) use display_unwrap;
#[doc(inline)]
pub use spaced_lists::spacing_error::SpacingError;

//! This crate implements list types that store the distance between their elements.
//!
//! These lists might be what you need if you:
//! 1. want a sorted list of numbers.
//! 2. want a list where the elements have a defined position.
//! 3. want a list where changing the position of one element should change the position of all
//!    elements after it.
//!
//! There are several different types of lists in this crate:
//! 1. SpacedList: A spaced list that stores values and the distance between them.
//! 2. RangeSpacedList: A spaced list that stores elements as non-overlapping ranges.
//! 3. HollowSpacedList: A spaced list that stores the relative position of empty nodes.
//! 4. HollowRangeSpacedList: A spaced list that stores empty nodes as non-overlapping ranges.

#![feature(trait_alias)]
#![feature(generic_associated_types)]

use std::ops::{Add, AddAssign, Sub, SubAssign};

use num_traits::Zero;

#[doc(inline)]
pub use skeleton::Node;
pub(crate) use skeleton::ParentData;
#[doc(inline)]
pub use skeleton::position::BoundType;
pub(crate) use skeleton::position::EphemeralPosition;
#[doc(inline)]
pub use skeleton::position::HollowPosition;
#[doc(inline)]
pub use skeleton::position::Position;
#[doc(inline)]
pub use skeleton::Range;
pub(crate) use skeleton::Skeleton;
pub(crate) use skeleton::traversal::iteration::BackwardsIter;
pub(crate) use skeleton::traversal::iteration::ForwardsIter;
#[doc(inline)]
pub use spaced_lists::HollowRangeSpacedList;
#[doc(inline)]
pub use spaced_lists::HollowSpacedList;
#[doc(inline)]
pub use spaced_lists::RangeSpacedList;
#[doc(inline)]
pub use spaced_lists::SpacedList;

pub trait Spacing = Add<Output=Self> + AddAssign + Sub<Output=Self> + SubAssign + Zero + Ord + Copy;

mod skeleton;

mod spaced_lists;

pub mod manager;


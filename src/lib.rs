//! This crate implements list types that store the distance between their elements.
//!
//! These lists might be what you need if you:
//! 1. want a sorted list of numbers.
//! 2. want a list where the elements have a defined position.
//! 3. want a list where changing the position of one element should change the position of all
//!    elements after it.
//!
//! There are several different types of lists in this crate:
//! 1. HollowSpacedList: A spaced list that does not store element values, only the distances
//!    between conceptual nodes.
//! 2. FilledSpacedList: A spaced list that does store element values and the distance between them.
//! 3. HollowRangeSpacedList: A hollow spaced list whose nodes are semantically interpreted as
//!    ranges with a start and an end.
//! 4. FilledRangeSpacedList: A filled spaced list whose nodes are semantically interpreted as
//!    ranges with a start and an end, storing an element value for each range.

#![feature(trait_alias)]
#![allow(unused)]

pub mod spaced_lists;

pub(crate) use spaced_lists::SpacedListSkeleton;

pub use spaced_lists::Spacing;
pub use spaced_lists::SpacedList;
pub use spaced_lists::HollowSpacedList;
pub use spaced_lists::FilledSpacedList;
pub use spaced_lists::HollowRangeSpacedList;
pub use spaced_lists::FilledRangeSpacedList;

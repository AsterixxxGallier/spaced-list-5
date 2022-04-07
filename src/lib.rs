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
// TODO maybe fix add-overflow errors when dealing with huge numbers (put random() as the position
//  and iterate many times to reproduce)

#![feature(trait_alias)]
#![feature(never_type)]
#![feature(default_free_fn)]
#![feature(option_get_or_insert_default)]
#![feature(slice_ptr_get)]
#![allow(unused)]

pub use spaced_lists::FilledRangeSpacedList;
pub use spaced_lists::FilledSpacedList;
pub use spaced_lists::HollowRangeSpacedList;
pub use spaced_lists::HollowSpacedList;
pub use spaced_lists::Position;
pub(crate) use spaced_lists::SpacedList;
pub(crate) use spaced_lists::SpacedListSkeleton;
pub use spaced_lists::Iter;
pub use spaced_lists::Spacing;

/// Todo type, replace every occurrence of this type with a proper type
type Todo = !;

pub mod spaced_lists;


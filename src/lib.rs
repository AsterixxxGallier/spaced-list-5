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
#![feature(label_break_value)]
#![allow(unused)]

#[doc(inline)]
pub use spaced_lists::filled_range_spaced_list::FilledRangeSpacedList;
#[doc(inline)]
pub use spaced_lists::filled_spaced_list::FilledSpacedList;
#[doc(inline)]
pub use spaced_lists::hollow_range_spaced_list::HollowRangeSpacedList;
#[doc(inline)]
pub use spaced_lists::hollow_spaced_list::HollowSpacedList;
#[doc(inline)]
pub use spaced_lists::iteration::node::Iter;
#[doc(inline)]
pub use spaced_lists::positions::node::Position;
#[doc(inline)]
pub use spaced_lists::Spacing;

pub(crate) use spaced_lists::spaced_list::SpacedList;
pub(crate) use spaced_lists::range_spaced_list::RangeSpacedList;
pub(crate) use spaced_lists::skeleton::Skeleton;

mod spaced_lists;


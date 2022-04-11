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

use std::fmt::Display;
use std::ops::{Add, AddAssign, Sub, SubAssign};

use num_traits::Zero;

pub trait Spacing = Add<Output=Self> + AddAssign + Sub<Output=Self> + SubAssign + Zero + Ord + Copy
+ Display /*todo remove*/;

macro_rules! default_as_new {
    () => {
        pub fn new() -> Self {
            default()
        }
    };
}

macro_rules! accessors {
    {
        $vis:vis $field:ident: $type:ty
        $(;$($rest:tt)*)?
    } => {
        $vis fn $field(&self) -> $type {
            self.$field
        }

        accessors! {
            $($($rest)*)?
        }
    };
    {
        $vis:vis ref $field:ident: $type:ty
        $(;$($rest:tt)*)?
    } => {
        $vis fn $field(&self) -> &$type {
            &self.$field
        }

        accessors! {
            $($($rest)*)?
        }
    };
    {
        $vis:vis mut $field:ident: $type:ty
        $(;$($rest:tt)*)?
    } => {
        paste! {
            $vis fn [<$field _mut>](&mut self) -> &mut $type {
                &mut self.$field
            }

            accessors! {
                $($($rest)*)?
            }
        }
    };
    {
        $vis:vis index $field:ident: $type:ty
        $(;$($rest:tt)*)?
    } => {
        paste! {
            $vis fn [<$field _at>](&self, index: usize) -> $type {
                self.[<$field s>][index]
            }

            accessors! {
                $($($rest)*)?
            }
        }
    };
    {
        $vis:vis index ref $field:ident: $type:ty
        $(;$($rest:tt)*)?
    } => {
        paste! {
            $vis fn [<$field _at>](&self, index: usize) -> &$type {
                &self.[<$field s>][index]
            }

            accessors! {
                $($($rest)*)?
            }
        }
    };
    {
        $vis:vis index mut $field:ident: $type:ty
        $(;$($rest:tt)*)?
    } => {
        paste! {
            $vis fn [<$field _at _mut>](&mut self, index: usize) -> &mut $type {
                &mut self.[<$field s>][index]
            }

            accessors! {
                $($($rest)*)?
            }
        }
    };
    () => {}
}

macro_rules! spaced_list {
    (@<S: Spacing$(, $T:ident)?> $Self:ty; Range) => {
        impl<S: Spacing$(, $T)?> RangeSpacedList<S> for $Self {}

        impl<S: Spacing$(, $T)?> $Self {
            delegates! {
                as RangeSpacedList<S>:

                range_starting_before(&self, position: S) -> Option<Position<S, Self>>;
                range_starting_at_or_before(&self, position: S) -> Option<Position<S, Self>>;
                range_starting_at(&self, position: S) -> Option<Position<S, Self>>;
                range_starting_at_or_after(&self, position: S) -> Option<Position<S, Self>>;
                range_starting_after(&self, position: S) -> Option<Position<S, Self>>;

                range_ending_before(&self, position: S) -> Option<Position<S, Self>>;
                range_ending_at_or_before(&self, position: S) -> Option<Position<S, Self>>;
                range_ending_at(&self, position: S) -> Option<Position<S, Self>>;
                range_ending_at_or_after(&self, position: S) -> Option<Position<S, Self>>;
                range_ending_after(&self, position: S) -> Option<Position<S, Self>>;
            }
        }
    };
    (@<S: Spacing$(, $T:ident)?> $Self:ty) => {
        impl<S: Spacing$(, $T)?> $Self {
            delegates! {
                as SpacedList<S>:

                node_before(&self, position: S) -> Option<Position<S, Self>>;
                node_at_or_before(&self, position: S) -> Option<Position<S, Self>>;
                node_at(&self, position: S) -> Option<Position<S, Self>>;
                node_at_or_after(&self, position: S) -> Option<Position<S, Self>>;
                node_after(&self, position: S) -> Option<Position<S, Self>>;
            }
        }
    };
    (@$($Range:ident)?;
        $Name:ident <S: Spacing$(, $T:ident)?> $Self:ty) => {
        paste! {
            #[derive(Clone, Eq, PartialEq)]
            pub struct $Name<S: Spacing$(, $T)?> {
                skeleton: Skeleton<S, Self>,
                $(elements: Vec<$T>,)?
            }

            impl<S: Spacing$(, $T)?> Default for $Self {
                fn default() -> Self {
                    Self {
                        skeleton: default(),
                        $(elements: Vec::<$T>::new(),)?
                    }
                }
            }

            impl<S: Spacing$(, $T)?> SpacedList<S> for $Self {
                accessors! {
                    ref skeleton: Skeleton<S, Self>;
                    mut skeleton: Skeleton<S, Self>;
                }
            }

            impl<S: Spacing$(, $T)?> $Self {
                default_as_new!();

                $(pub fn element(&self, position: Position<S, Self>) -> &$T {
                    todo!()
                }

                pub fn element_mut(&mut self, position: Position<S, Self>) -> &mut $T {
                    todo!()
                })?

                delegates! {
                    as SpacedList<S>:

                    inflate_after(&mut self, position: S, amount: S);
                    inflate_before(&mut self, position: S, amount: S);
                    deflate_after(&mut self, position: S, amount: S);
                    deflate_before(&mut self, position: S, amount: S);
                }
            }

            spaced_list!(@<S: Spacing$(, $T)?> $Self$(; $Range)?);
        }
    };
    (Hollow $($Range:ident)?) => {
        paste! {
            spaced_list!(@$($Range)?;
                [<Hollow $($Range)? SpacedList>] <S: Spacing>
                [<Hollow $($Range)? SpacedList>]<S>);
        }
    };
    (Filled $($Range:ident)?) => {
        paste! {
            spaced_list!(@$($Range)?;
                [<Filled $($Range)? SpacedList>] <S: Spacing, T>
                [<Filled $($Range)? SpacedList>]<S, T>);
        }
    };
}

macro_rules! delegates {
    {as $trait:ty:
        $fn:ident(&self$(, $param:ident: $param_type:ty)*)$( -> $return:ty)?
        $(; as $new_trait:ty: $($rest:tt)*)?} => {
        pub fn $fn(&self$(, $param: $param_type)*)$( -> $return)? {
            <Self as $trait>::$fn(self$(, $param)*)
        }

        delegates! {
            $(as $new_trait: $($rest)*)?
        }
    };
    {as $trait:ty:
        $fn:ident(&mut self$(, $param:ident: $param_type:ty)*)$( -> $return:ty)?
        $(; as $new_trait:ty: $($rest:tt)*)?} => {
        pub fn $fn(&mut self$(, $param: $param_type)*)$( -> $return)? {
            <Self as $trait>::$fn(self$(, $param)*)
        }

        delegates! {
            $(as $new_trait: $($rest)*)?
        }
    };
    {as $trait:ty:
        $fn:ident(&self$(, $param:ident: $param_type:ty)*)$( -> $return:ty)?
        $(;$($rest:tt)*)?} => {
        pub fn $fn(&self$(, $param: $param_type)*)$( -> $return)? {
            <Self as $trait>::$fn(self$(, $param)*)
        }

        delegates! {
            $(as $trait : $($rest)*)?
        }
    };
    {as $trait:ty:
        $fn:ident(&mut self$(, $param:ident: $param_type:ty)*)$( -> $return:ty)?
        $(;$($rest:tt)*)?} => {
        pub fn $fn(&mut self$(, $param: $param_type)*)$( -> $return)? {
            <Self as $trait>::$fn(self$(, $param)*)
        }

        delegates! {
            $(as $trait: $($rest)*)?
        }
    };
    {as $trait:ty:} => {};
}

pub(crate) mod skeleton;

pub(crate) mod spaced_list;

pub(crate) mod range_spaced_list;

pub(crate) mod iteration;

pub(crate) mod positions;

#[doc(inline)]
pub use spaced_list::filled_spaced_list::FilledSpacedList;
#[doc(inline)]
pub use spaced_list::hollow_spaced_list::HollowSpacedList;
#[doc(inline)]
pub use range_spaced_list::hollow_range_spaced_list::HollowRangeSpacedList;
#[doc(inline)]
pub use range_spaced_list::filled_range_spaced_list::FilledRangeSpacedList;
#[doc(inline)]
pub use iteration::node::Iter;
#[doc(inline)]
pub use positions::node::Position;

pub(crate) use spaced_list::SpacedList;
pub(crate) use range_spaced_list::RangeSpacedList;
pub(crate) use skeleton::Skeleton;


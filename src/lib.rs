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

use std::ops::{Add, AddAssign, Sub, SubAssign};

use num_traits::Zero;

pub trait Spacing = Add<Output=Self> + AddAssign + Sub<Output=Self> + SubAssign + Zero + Ord + Copy;

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

macro_rules! trait_accessors {
    {
        $field:ident: $type:ty
        $(;$($rest:tt)*)?
    } => {
        fn $field(&self) -> $type;
        trait_accessors! {
            $($($rest)*)?
        }
    };
    {
        ref $field:ident: $type:ty
        $(;$($rest:tt)*)?
    } => {
        fn $field(&self) -> &$type;
        trait_accessors! {
            $($($rest)*)?
        }
    };
    {
        mut $field:ident: $type:ty
        $(;$($rest:tt)*)?
    } => {
        paste! {
            fn [<$field _mut>](&mut self) -> &mut $type;
            trait_accessors! {
                $($($rest)*)?
            }
        }
    };
    {
        index $field:ident: $type:ty
        $(;$($rest:tt)*)?
    } => {
        paste! {
            fn [<$field _at>](&self, index: usize) -> $type;
            trait_accessors! {
                $($($rest)*)?
            }
        }
    };
    {
        index ref $field:ident: $type:ty
        $(;$($rest:tt)*)?
    } => {
        paste! {
            fn [<$field _at>](&self, index: usize) -> &$type;
            trait_accessors! {
                $($($rest)*)?
            }
        }
    };
    {
        index mut $field:ident: $type:ty
        $(;$($rest:tt)*)?
    } => {
        paste! {
            fn [<$field _at _mut>](&mut self, index: usize) -> &mut $type;
            trait_accessors! {
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
                link_lengths: Vec<S>,
                sublists: Vec<Option<Self>>,
                index_in_super_list: Option<usize>,
                link_capacity: usize,
                link_size: usize,
                link_size_deep: usize,
                node_size: usize,
                node_size_deep: usize,
                depth: usize,
                length: S,
                offset: S,
                $(elements: Vec<$T>,)?
            }

            impl<S: Spacing$(, $T)?> Default for $Self {
                fn default() -> Self {
                    Self {
                        link_lengths: vec![],
                        sublists: vec![],
                        link_capacity: 0,
                        depth: 0,
                        length: zero(),
                        offset: zero(),
                        link_size: 0,
                        link_size_deep: 0,
                        node_size: 0,
                        node_size_deep: 0,
                        index_in_super_list: None,
                        $(elements: Vec::<$T>::new(),)?
                    }
                }
            }

            impl<S: Spacing$(, $T)?> SpacedList<S> for $Self {
                accessors! {
                    index_in_super_list: Option<usize>;
                    mut index_in_super_list: Option<usize>;
                    ref link_lengths: Vec<S>;
                    mut link_lengths: Vec<S>;
                    ref sublists: Vec<Option<Self>>;
                    mut sublists: Vec<Option<Self>>;
                    link_size: usize;
                    mut link_size: usize;
                    link_size_deep: usize;
                    mut link_size_deep: usize;
                    link_capacity: usize;
                    mut link_capacity: usize;
                    node_size: usize;
                    mut node_size: usize;
                    node_size_deep: usize;
                    mut node_size_deep: usize;
                    depth: usize;
                    mut depth: usize;
                    length: S;
                    mut length: S;
                    offset: S;
                    mut offset: S;
                    index link_length: S;
                    index mut link_length: S;
                }
            }

            impl<S: Spacing$(, $T)?> $Self {
                pub fn new() -> Self {
                    Self::default()
                }

                $(pub fn element<'a>(&'a self, position: Position<'a, S, Self>) -> &'a $T {
                    &position.list().elements[position.index()]
                }

                pub fn element_mut<'a>(&'a mut self, position: Position<'a, S, Self>) -> &'a mut $T {
                    let mut list = self;
                    for super_sub_list in position.super_lists().iter().skip(1) {
                        let index = super_sub_list.index_in_super_list().unwrap();
                        list = list.sublist_at_mut(index).unwrap();
                    }
                    &mut list.elements[position.index()]
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

pub(crate) mod spaced_list;

pub(crate) mod range_spaced_list;

pub(crate) mod iteration;

pub(crate) mod positions;

pub(crate) mod traversal;

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


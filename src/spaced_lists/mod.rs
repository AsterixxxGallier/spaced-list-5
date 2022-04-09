use std::ops::{Add, AddAssign, Sub, SubAssign};

use num_traits::Zero;

pub trait Spacing = Add<Output=Self> + AddAssign + Sub<Output=Self> + SubAssign + Zero + Ord + Copy;

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
    (@Hollow $Name:ident $Self:ty) => {
        #[derive(Clone, Eq, PartialEq)]
        pub struct $Name<S: Spacing> {
            skeleton: SpacedListSkeleton<S, Self>,
        }

        impl<S: Spacing> Default for $Self {
            fn default() -> Self {
                Self {
                    skeleton: default(),
                }
            }
        }

        impl<S: Spacing> SpacedList<S> for $Self {
            accessors! {
                ref skeleton: SpacedListSkeleton<S, Self>;
                mut skeleton: SpacedListSkeleton<S, Self>;
            }
        }
    };
    (@Filled $Name:ident $Self:ty) => {
        #[derive(Clone, Eq, PartialEq)]
        pub struct $Name<S: Spacing, T> {
            skeleton: SpacedListSkeleton<S, Self>,
            elements: Vec<T>,
        }

        impl<S: Spacing, T> Default for $Self {
            fn default() -> Self {
                Self {
                    skeleton: default(),
                    elements: vec![],
                }
            }
        }

        impl<S: Spacing, T> SpacedList<S> for $Self {
            accessors! {
                ref skeleton: SpacedListSkeleton<S, Self>;
                mut skeleton: SpacedListSkeleton<S, Self>;
            }
        }
    };
    (Hollow $($ranged:ident)?) => {
        paste! {
            spaced_list!(@Hollow
                [<Hollow $($ranged)? SpacedList>] [<Hollow $($ranged)? SpacedList>]<S>);
        }
    };
    (Filled $($ranged:ident)?) => {
        paste! {
            spaced_list!(@Filled
                [<Filled $($ranged)? SpacedList>] [<Filled $($ranged)? SpacedList>]<S, T>);
        }
    }
}

macro_rules! delegates {
    {$fn:ident(&self$(, $param:ident: $param_type:ty)*)$( -> $return:ty)? $(;$($rest:tt)*)?} => {
        pub fn $fn(&self$(, $param: $param_type)*)$( -> $return)? {
            <Self as SpacedList<S>>::$fn(self$(, $param)*)
        }

        delegates! {
            $($($rest)*)?
        }
    };
    {$fn:ident(&mut self$(, $param:ident: $param_type:ty)*)$( -> $return:ty)? $(;$($rest:tt)*)?} => {
        pub fn $fn(&mut self$(, $param: $param_type)*)$( -> $return)? {
            <Self as SpacedList<S>>::$fn(self$(, $param)*)
        }

        delegates! {
            $($($rest)*)?
        }
    };
    {} => {}
}

pub(crate) mod skeleton;

pub(crate) mod spaced_list;

pub(crate) mod hollow_spaced_list;

pub(crate) mod filled_spaced_list;

pub(crate) mod hollow_range_spaced_list;

pub(crate) mod filled_range_spaced_list;

pub(crate) mod traversal;

pub(crate) mod iteration;

pub(crate) mod positions;
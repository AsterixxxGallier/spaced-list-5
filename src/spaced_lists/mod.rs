use std::rc::Rc;
use std::cell::{Ref, RefCell};
use crate::{BackwardsIter, display_unwrap, ForwardsIter, HollowPosition, NestedRange, NestedRangeInsertionError, NestedRangePushError, Node, Position, PushError, Range, RangeInsertionError, RangePushError, Skeleton, Spacing};
use paste::paste;
use itertools::Itertools;
use push_insert_functions::push_insert_functions;
use spacing_functions::spacing_functions;
use trivial_accessors::trivial_accessors;
use first_last_functions::first_last_functions;
use traversal_functions::{unconditional_traversal_function, conditional_traversal_function, all_traversal_functions};
use iter_functions::iter_functions;

mod push_insert_functions;
mod spacing_functions;
mod trivial_accessors;
mod first_last_functions;
mod traversal_functions;
mod iter_functions;

macro_rules! spaced_list {
    ($kind:ident; $name:ident, ($($T:ident)?), $type:ty, $skeleton:ty, $position_ident:ident, $position:ty) => {
        pub struct $name<S: Spacing$(, $T)?> {
            skeleton: Rc<RefCell<$skeleton>>,
            size: usize,
        }

        impl<S: Spacing$(, $T)?> Default for $type {
            fn default() -> Self {
                Self {
                    skeleton: Skeleton::new(None),
                    size: 0,
                }
            }
        }

        impl<S: Spacing$(, $T)?> $type {
            #[must_use]
            pub fn new() -> Self {
                Self::default()
            }

            push_insert_functions!($kind; ($($T)?), $position);
            spacing_functions!();
            trivial_accessors!();
            first_last_functions!($position_ident, $position);
            all_traversal_functions!($kind; unconditional_, $position);
            $(all_traversal_functions!($kind; conditional_, $position); ${ignore($T)})?
            iter_functions!($kind; $position);
        }
    }
}

spaced_list!(Node; SpacedList, (T), SpacedList<S, T>, Skeleton<Node, S, T>, Position, Position<Node, S, T>);
spaced_list!(Range; RangeSpacedList, (T), RangeSpacedList<S, T>, Skeleton<Range, S, T>, Position, Position<Range, S, T>);
spaced_list!(NestedRange; NestedRangeSpacedList, (T), NestedRangeSpacedList<S, T>, Skeleton<NestedRange, S, T>, Position, Position<NestedRange, S, T>);
spaced_list!(Node; HollowSpacedList, (), HollowSpacedList<S>, Skeleton<Node, S, ()>, HollowPosition, HollowPosition<Node, S>);
spaced_list!(Range; HollowRangeSpacedList, (), HollowRangeSpacedList<S>, Skeleton<Range, S, ()>, HollowPosition, HollowPosition<Range, S>);
spaced_list!(NestedRange; HollowNestedRangeSpacedList, (), HollowNestedRangeSpacedList<S>, Skeleton<NestedRange, S, ()>, HollowPosition, HollowPosition<NestedRange, S>);

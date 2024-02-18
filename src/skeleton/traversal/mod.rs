use std::cell::RefCell;
use std::rc::Rc;

use paste::paste;
use crate::NestedRange;

use crate::{EphemeralPosition, ParentData, Range, Skeleton, Spacing, BoundType};
use crate::skeleton::link_index;
use r#loop::*;
use after_loop::*;
use checks::*;
use traversal_function_body::*;

macro_rules! index_is_at_start_condition {
    ($index:expr) => {
        BoundType::of($index) == BoundType::Start
    };
}

macro_rules! index_is_at_end_condition {
    ($index:expr) => {
        BoundType::of($index) == BoundType::End
    };
}

macro_rules! empty_condition {
    ($index:expr) => { true };
}

macro_rules! for_all_traversals {
    ($macro:ident $($prefixes:tt)*) => {
        $macro!($($prefixes)*before: <);
        $macro!($($prefixes)*at_or_before: <=);
        $macro!($($prefixes)*at: ==);
        $macro!($($prefixes)*at_or_after: >=);
        $macro!($($prefixes)*after: >);
    };
}

macro_rules! traversal_functions {
    (shallow $pos:ident: $cmp:tt) => {
        paste! {
            pub fn [<shallow_ $pos>](this: Rc<RefCell<Self>>, target: S)
                -> Option<EphemeralPosition<Kind, S, T>> {
                traversal_function_body!(this; shallow; $cmp target with empty_condition)
            }
        }
    };
    (deep $pos:ident: $cmp:tt) => {
        pub fn $pos(this: Rc<RefCell<Self>>, target: S)
            -> Option<EphemeralPosition<Kind, S, T>> {
            traversal_function_body!(this; deep; $cmp target with empty_condition)
        }
    };
    () => {
        for_all_traversals!(traversal_functions shallow);
        for_all_traversals!(traversal_functions deep);
    };
    (shallow $bound:ident $pos:ident: $cmp:tt) => {
        paste! {
            pub fn [<shallow_ $bound ing_ $pos>](this: Rc<RefCell<Self>>, target: S)
                -> Option<EphemeralPosition<Range, S, T>> {
                traversal_function_body!(this; shallow; $cmp target with [<index_is_at_ $bound _condition>])
            }
        }
    };
    (deep $bound:ident $pos:ident: $cmp:tt) => {
        paste! {
            pub fn [<$bound ing_ $pos>](this: Rc<RefCell<Self>>, target: S)
                -> Option<EphemeralPosition<Range, S, T>> {
                traversal_function_body!(this; deep; $cmp target with [<index_is_at_ $bound _condition>])
            }
        }
    };
    (range) => {
        for_all_traversals!(traversal_functions shallow start);
        for_all_traversals!(traversal_functions shallow end);
        for_all_traversals!(traversal_functions deep start);
        for_all_traversals!(traversal_functions deep end);
    };
    (shallow nested $bound:ident $pos:ident: $cmp:tt) => {
        paste! {
            pub fn [<shallow_ $bound ing_ $pos>](this: Rc<RefCell<Self>>, target: S)
                -> Option<EphemeralPosition<NestedRange, S, T>> {
                traversal_function_body!(this; shallow; $cmp target with [<index_is_at_ $bound _condition>])
            }
        }
    };
    (deep nested $bound:ident $pos:ident: $cmp:tt) => {
        paste! {
            pub fn [<$bound ing_ $pos>](this: Rc<RefCell<Self>>, target: S)
                -> Option<EphemeralPosition<NestedRange, S, T>> {
                traversal_function_body!(this; deep; $cmp target with [<index_is_at_ $bound _condition>])
            }
        }
    };
    (nested range) => {
        for_all_traversals!(traversal_functions shallow nested start);
        for_all_traversals!(traversal_functions shallow nested end);
        for_all_traversals!(traversal_functions deep nested start);
        for_all_traversals!(traversal_functions deep nested end);
    };
}

#[allow(dead_code)]
impl<Kind, S: Spacing, T> Skeleton<Kind, S, T> {
    traversal_functions!();

    pub fn at_index(this: Rc<RefCell<Self>>, index: usize) -> Option<EphemeralPosition<Kind, S, T>> {
        if index > this.borrow().links.len() {
            return None;
        }
        let mut position = this.borrow().offset;
        let mut current_index = 0;
        for degree in (0..this.borrow().depth).rev() {
            let next_index = current_index + (1 << degree);
            if next_index <= index {
                position += this.borrow().links[current_index];
                current_index = next_index;
            }
        }
        Some(EphemeralPosition::new(this, index, position))
    }
}

#[allow(dead_code)]
impl<S: Spacing, T> Skeleton<Range, S, T> {
    traversal_functions!(range);
}

#[allow(dead_code)]
impl<S: Spacing, T> Skeleton<NestedRange, S, T> {
    traversal_functions!(nested range);
}

pub mod iteration;
mod r#loop;
mod after_loop;
mod checks;
mod traversal_function_body;
use std::cell::RefCell;
use std::cell::Ref;
use std::rc::Rc;
use std::intrinsics::prefetch_read_data;

use paste::paste;

use crate::{NestedRange, Node, ElementSlot, EphemeralPosition, EphemeralIndex, ParentData, Range, Skeleton, Spacing, BoundType};
use crate::skeleton::get_link_index;
use r#loop::*;
use after_loop::*;
use checks::*;
use traversal_function_body::*;

macro_rules! conjunctive_condition {
    ($name:ident, $a:ident, $b:ident) => {
        macro_rules! $name {
            ($$index:expr, $$skeleton:expr, $$condition:ident) => {
                $a!($$index, $$skeleton, $$condition) && $b!($$index, $$skeleton, $$condition)
            };
        }
    };
}

macro_rules! empty_condition {
    ($($rest:tt)*) => { true };
}

macro_rules! index_is_at_start_condition {
    ($index:expr, $($rest:tt)*) => {
        BoundType::of($index) == BoundType::Start
    };
}

macro_rules! index_is_at_end_condition {
    ($index:expr, $($rest:tt)*) => {
        BoundType::of($index) == BoundType::End
    };
}

macro_rules! function_condition {
    ($index:expr, $skeleton:expr, $condition:ident) => {
        $condition(EphemeralIndex::new($skeleton, $index).element())
    };
}

conjunctive_condition!(index_is_at_start_and_function_condition, index_is_at_start_condition, function_condition);
conjunctive_condition!(index_is_at_end_and_function_condition, index_is_at_end_condition, function_condition);

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
                traversal_function_body!(this; shallow; $cmp target with empty_condition ())
            }
        }
    };
    (deep $pos:ident: $cmp:tt) => {
        pub fn $pos(this: Rc<RefCell<Self>>, target: S)
            -> Option<EphemeralPosition<Kind, S, T>> {
            traversal_function_body!(this; deep; $cmp target with empty_condition ())
        }
    };
    ($kind:ident conditional shallow $pos:ident: $cmp:tt) => {
        paste! {
            pub fn [<conditional_ shallow_ $pos>]<C: Fn(Ref<ElementSlot<T>>) -> bool>(this: Rc<RefCell<Self>>, target: S, condition: C)
                -> Option<EphemeralPosition<$kind, S, T>> {
                traversal_function_body!(this; shallow; $cmp target with function_condition (condition))
            }
        }
    };
    ($kind:ident conditional deep $pos:ident: $cmp:tt) => {
        paste! {
            pub fn [<conditional_ $pos>]<C: Fn(Ref<ElementSlot<T>>) -> bool>(this: Rc<RefCell<Self>>, target: S, condition: C)
                -> Option<EphemeralPosition<$kind, S, T>> {
                traversal_function_body!(this; deep; $cmp target with function_condition (condition))
            }
        }
    };
    () => {
        for_all_traversals!(traversal_functions shallow);
        for_all_traversals!(traversal_functions deep);
    };
    (conditional $kind:ident) => {
        for_all_traversals!(traversal_functions $kind conditional shallow);
        for_all_traversals!(traversal_functions $kind conditional deep);
    };
    (shallow $bound:ident $pos:ident: $cmp:tt) => {
        paste! {
            pub fn [<shallow_ $bound ing_ $pos>](this: Rc<RefCell<Self>>, target: S)
                -> Option<EphemeralPosition<Range, S, T>> {
                traversal_function_body!(this; shallow; $cmp target with [<index_is_at_ $bound _condition>] ())
            }
        }
    };
    (deep $bound:ident $pos:ident: $cmp:tt) => {
        paste! {
            pub fn [<$bound ing_ $pos>](this: Rc<RefCell<Self>>, target: S)
                -> Option<EphemeralPosition<Range, S, T>> {
                traversal_function_body!(this; deep; $cmp target with [<index_is_at_ $bound _condition>] ())
            }
        }
    };
    (conditional shallow $bound:ident $pos:ident: $cmp:tt) => {
        paste! {
            pub fn [<conditional_ shallow_ $bound ing_ $pos>]<C: Fn(Ref<ElementSlot<T>>) -> bool>(this: Rc<RefCell<Self>>, target: S, condition: C)
                -> Option<EphemeralPosition<Range, S, T>> {
                traversal_function_body!(this; shallow; $cmp target with [<index_is_at_ $bound _and_function_condition>] (condition))
            }
        }
    };
    (conditional deep $bound:ident $pos:ident: $cmp:tt) => {
        paste! {
            pub fn [<conditional_ $bound ing_ $pos>]<C: Fn(Ref<ElementSlot<T>>) -> bool>(this: Rc<RefCell<Self>>, target: S, condition: C)
                -> Option<EphemeralPosition<Range, S, T>> {
                traversal_function_body!(this; deep; $cmp target with [<index_is_at_ $bound _and_function_condition>] (condition))
            }
        }
    };
    (@range) => {
        for_all_traversals!(traversal_functions shallow start);
        for_all_traversals!(traversal_functions shallow end);
        for_all_traversals!(traversal_functions deep start);
        for_all_traversals!(traversal_functions deep end);
        for_all_traversals!(traversal_functions conditional shallow start);
        for_all_traversals!(traversal_functions conditional shallow end);
        for_all_traversals!(traversal_functions conditional deep start);
        for_all_traversals!(traversal_functions conditional deep end);
    };
    (shallow nested $bound:ident $pos:ident: $cmp:tt) => {
        paste! {
            pub fn [<shallow_ $bound ing_ $pos>](this: Rc<RefCell<Self>>, target: S)
                -> Option<EphemeralPosition<NestedRange, S, T>> {
                traversal_function_body!(this; shallow; $cmp target with [<index_is_at_ $bound _condition>] ())
            }
        }
    };
    (deep nested $bound:ident $pos:ident: $cmp:tt) => {
        paste! {
            pub fn [<$bound ing_ $pos>](this: Rc<RefCell<Self>>, target: S)
                -> Option<EphemeralPosition<NestedRange, S, T>> {
                traversal_function_body!(this; deep; $cmp target with [<index_is_at_ $bound _condition>] ())
            }
        }
    };
    (conditional shallow nested $bound:ident $pos:ident: $cmp:tt) => {
        paste! {
            pub fn [<conditional_ shallow_ $bound ing_ $pos>]<C: Fn(Ref<ElementSlot<T>>) -> bool>(this: Rc<RefCell<Self>>, target: S, condition: C)
                -> Option<EphemeralPosition<NestedRange, S, T>> {
                traversal_function_body!(this; shallow; $cmp target with [<index_is_at_ $bound _and_function_condition>] (condition))
            }
        }
    };
    (conditional deep nested $bound:ident $pos:ident: $cmp:tt) => {
        paste! {
            pub fn [<conditional_ $bound ing_ $pos>]<C: Fn(Ref<ElementSlot<T>>) -> bool>(this: Rc<RefCell<Self>>, target: S, condition: C)
                -> Option<EphemeralPosition<NestedRange, S, T>> {
                traversal_function_body!(this; deep; $cmp target with [<index_is_at_ $bound _and_function_condition>] (condition))
            }
        }
    };
    (@nested range) => {
        for_all_traversals!(traversal_functions shallow nested start);
        for_all_traversals!(traversal_functions shallow nested end);
        for_all_traversals!(traversal_functions deep nested start);
        for_all_traversals!(traversal_functions deep nested end);
        for_all_traversals!(traversal_functions conditional shallow nested start);
        for_all_traversals!(traversal_functions conditional shallow nested end);
        for_all_traversals!(traversal_functions conditional deep nested start);
        for_all_traversals!(traversal_functions conditional deep nested end);
    };
}

#[allow(dead_code)]
impl<Kind, S: Spacing, T> Skeleton<Kind, S, T> {
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
impl<Kind, S: Spacing, T> Skeleton<Kind, S, T> {
    traversal_functions!();
}

#[allow(dead_code)]
impl<S: Spacing, T> Skeleton<Node, S, T> {
    traversal_functions!(conditional Node);
}

#[allow(dead_code)]
impl<S: Spacing, T> Skeleton<Range, S, T> {
    traversal_functions!(conditional Range);

    traversal_functions!(@range);
}

#[allow(dead_code)]
impl<S: Spacing, T> Skeleton<NestedRange, S, T> {
    traversal_functions!(conditional NestedRange);

    traversal_functions!(@nested range);
}

pub mod iteration;
mod r#loop;
mod after_loop;
mod checks;
mod traversal_function_body;
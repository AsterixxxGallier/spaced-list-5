use std::cell::{RefCell};
use std::rc::Rc;

use paste::paste;

use crate::skeleton::{link_index, ParentData, Range, Skeleton, Spacing, position::Position};

macro_rules! traverse {
    // region loop
    // region break if ==
    (@break if ==(<, $target:ident; $position:ident)) => {};
    (@break if ==(<=, $target:ident; $position:ident)) => {
        if $position == $target {
            break;
        }
    };
    // endregion

    // region into sublist if deep
    (@into sublist if deep(shallow, $($_rest:tt)*)) => {};
    (@into sublist if deep(deep, $cmp:tt, $target:ident;
        $skeleton:ident, $degree:ident, $index:ident, $position:ident)) => {
        if let Some(sub) = $skeleton.clone().borrow().sub($index) {
            let next_position = $position + sub.borrow().offset;
            if next_position $cmp $target {
                $degree = sub.borrow().depth.saturating_sub(1);
                $index = 0;
                $position = next_position;
                $skeleton = sub;
                continue;
            }
        }
    };
    // endregion

    // region redirect loop cmp
    (@loop($depth:ident, ==, $($rest:tt)*)) => {
        traverse!(@loop($depth, <=, $($rest)*))
    };
    (@loop($depth:ident, >=, $($rest:tt)*)) => {
        traverse!(@loop($depth, <=, $($rest)*))
    };
    (@loop($depth:ident, >, $($rest:tt)*)) => {
        traverse!(@loop($depth, <=, $($rest)*))
    };
    // endregion

    (@loop($depth:ident, $cmp:tt, $target:ident, $bound:ident;
        $skeleton:ident, $degree:ident, $index:ident, $position:ident)) => {
        loop {
            let link_index = link_index($index, $degree);
            if !$skeleton.borrow().link_index_is_in_bounds(link_index) {
                if $degree > 0 {
                    $degree -= 1;
                    continue;
                } else {
                    break;
                }
            }

            let next_position = $position + $skeleton.borrow().links[link_index];
            if next_position $cmp $target {
                $position = next_position;
                $index += 1 << $degree;
                traverse!(@break if ==($cmp, $target; $position));
            }

            if $degree > 0 {
                $degree -= 1;
            } else {
                traverse!(@into sublist if deep($depth, $cmp, $target;
                    $skeleton, $degree, $index, $position));
                break;
            }
        }
    };
    // endregion

    // region next
    (@next(shallow; $skeleton:ident, $index:ident, $position:ident)) => {
        if $index == $skeleton.borrow().links.len() {
            Err("Tried to move to next element but it's already the end of the skeleton")
        } else {
            $position += $skeleton.borrow().link($index);
            $index += 1;
            Ok(())
        }
    };
    (@next(deep; $skeleton:ident, $index:ident, $position:ident)) => {
        if $index == $skeleton.borrow().links.len() {
            if let Some(ParentData { parent, index_in_parent }) =
                &$skeleton.clone().borrow().parent_data {
                $position -= $skeleton.borrow().last_position();
                $skeleton = parent.upgrade().unwrap();
                $position += $skeleton.borrow().link(*index_in_parent);
                $index = index_in_parent + 1;
                Ok(())
            } else {
                Err("Tried to move to next element but it's already the end of the skeleton")
            }
        } else if let Some(sub) = $skeleton.clone().borrow().sub($index) {
            $skeleton = sub;
            $index = 0;
            $position += $skeleton.borrow().offset;
            Ok(())
        } else {
            $position += $skeleton.borrow().link($index);
            $index += 1;
            Ok(())
        }
    };
    // endregion

    // region previous
    (@previous(shallow; $skeleton:ident, $index:ident, $position:ident)) => {
        if $index == 0 {
            Err("Tried to move to previous element but it's already the start of the skeleton")
        } else {
            $index -= 1;
            $position -= $skeleton.borrow().link($index);
            Ok(())
        }
    };
    (@previous(deep; $skeleton:ident, $index:ident, $position:ident)) => {
        if $index == 0 {
            if let Some(ParentData { parent, index_in_parent }) =
                &$skeleton.clone().borrow().parent_data {
                $index = *index_in_parent;
                $position -= $skeleton.borrow().offset;
                $skeleton = parent.upgrade().unwrap();
                Ok(())
            } else {
                Err("Tried to move to previous element but it's already the start of the skeleton")
            }
        } else if let Some(sub) = $skeleton.clone().borrow().sub($index - 1) {
            $position -= $skeleton.borrow().link($index - 1);
            $skeleton = sub;
            $position += $skeleton.borrow().last_position();
            $index = $skeleton.borrow().links.len();
            Ok(())
        } else {
            $index -= 1;
            $position -= $skeleton.borrow().link($index);
            Ok(())
        }
    };
    // endregion

    // region after loop
    // region any
    (@after loop($depth:ident, <, $target:ident, any;
        $skeleton:ident, $degree:ident, $index:ident, $position:ident)) => {
        Some(Position::new($skeleton, $index, $position))
    };
    (@after loop($depth:ident, <=, $target:ident, any;
        $skeleton:ident, $degree:ident, $index:ident, $position:ident)) => {
        Some(Position::new($skeleton, $index, $position))
    };
    (@after loop($depth:ident, ==, $target:ident, any;
        $skeleton:ident, $degree:ident, $index:ident, $position:ident)) => {
        if $position == $target {
            Some(Position::new($skeleton, $index, $position))
        } else {
            None
        }
    };
    (@after loop($depth:ident, >=, $target:ident, any;
        $skeleton:ident, $degree:ident, $index:ident, $position:ident)) => {
        {
            if $position < $target {
                traverse!(@next($depth; $skeleton, $index, $position)).unwrap();
            }
            Some(Position::new($skeleton, $index, $position))
        }
    };
    (@after loop($depth:ident, >, $target:ident, any;
        $skeleton:ident, $degree:ident, $index:ident, $position:ident)) => {
        {
            traverse!(@next($depth; $skeleton, $index, $position)).unwrap();
            Some(Position::new($skeleton, $index, $position))
        }
    };
    // endregion

    // region start/end
    // region index is at bound
    (@index is at bound(start; $index:expr)) => {
        $index & 1 == 0
    };
    (@index is at bound(end; $index:expr)) => {
        $index & 1 == 1
    };
    // endregion

    (@after loop($depth:ident, <, $target:ident, $bound:ident;
        $skeleton:ident, $degree:ident, $index:ident, $position:ident)) => {
        {
            if !traverse!(@index is at bound($bound; $index)) {
                traverse!(@previous($depth; $skeleton, $index, $position)).ok()?;
            }
            Some(Position::new($skeleton, $index, $position))
        }
    };
    (@after loop($depth:ident, <=, $target:ident, $bound:ident;
        $skeleton:ident, $degree:ident, $index:ident, $position:ident)) => {
        {
            if !traverse!(@index is at bound($bound; $index)) {
                traverse!(@previous($depth; $skeleton, $index, $position)).ok()?;
            }
            Some(Position::new($skeleton, $index, $position))
        }
    };
    (@after loop($depth:ident, ==, $target:ident, $bound:ident;
        $skeleton:ident, $degree:ident, $index:ident, $position:ident)) => {
        if $position == $target && traverse!(@index is at bound($bound; $index)) {
            Some(Position::new($skeleton, $index, $position))
        } else {
            None
        }
    };
    (@after loop($depth:ident, >=, $target:ident, $bound:ident;
        $skeleton:ident, $degree:ident, $index:ident, $position:ident)) => {
        {
            if $position < $target {
                traverse!(@next($depth; $skeleton, $index, $position)).unwrap();
            }
            if !traverse!(@index is at bound($bound; $index)) {
                traverse!(@next($depth; $skeleton, $index, $position)).ok()?;
            }
            Some(Position::new($skeleton, $index, $position))
        }
    };
    (@after loop($depth:ident, >, $target:ident, $bound:ident;
        $skeleton:ident, $degree:ident, $index:ident, $position:ident)) => {
        {
            traverse!(@next($depth; $skeleton, $index, $position)).unwrap();
            if !traverse!(@index is at bound($bound; $index)) {
                traverse!(@next($depth; $skeleton, $index, $position)).ok()?;
            }
            Some(Position::new($skeleton, $index, $position))
        }
    };
    // endregion
    // endregion

    // region checked
    (@checked(shallow, $cmp:tt, $target:ident, $bound:ident; $skeleton:ident)) => {
        {
            let mut degree = $skeleton.borrow().depth - 1;
            let mut index = 0;
            let mut position = $skeleton.borrow().offset;
            traverse!(@loop(shallow, $cmp, $target, $bound;
                $skeleton, degree, index, position));
            traverse!(@after loop(shallow, $cmp, $target, $bound;
                $skeleton, degree, index, position))
        }
    };
    (@checked(deep, $cmp:tt, $target:ident, $bound:ident; $skeleton:ident)) => {
        {
            let mut skeleton = $skeleton;
            let mut degree = skeleton.borrow().depth - 1;
            let mut index = 0;
            let mut position = skeleton.borrow().offset;
            traverse!(@loop(deep, $cmp, $target, $bound;
                skeleton, degree, index, position));
            traverse!(@after loop(deep, $cmp, $target, $bound;
                skeleton, degree, index, position))
        }
    };
    // endregion

    // region checks
    // region out of bounds condition
    (@out of bounds condition(<, $target:ident; $skeleton:ident)) => {
        $target <= $skeleton.borrow().offset
    };
    (@out of bounds condition(<=, $target:ident; $skeleton:ident)) => {
        $target < $skeleton.borrow().offset
    };
    (@out of bounds condition(==, $target:ident; $skeleton:ident)) => {
        $target < $skeleton.borrow().offset ||
            $target > $skeleton.borrow().last_position()
    };
    (@out of bounds condition(>=, $target:ident; $skeleton:ident)) => {
        $target > $skeleton.borrow().last_position()
    };
    (@out of bounds condition(>, $target:ident; $skeleton:ident)) => {
        $target >= $skeleton.borrow().last_position()
    };
    // endregion

    // region if zero is at bound
    (@if zero is at bound(any) { $($then:tt)* } else { $($else:tt)* }) => {
        $($then)*
    };
    (@if zero is at bound(start) { $($then:tt)* } else { $($else:tt)* }) => {
        $($then)*
    };
    (@if zero is at bound(end) { $($then:tt)* } else { $($else:tt)* }) => {
        $($else)*
    };
    // endregion

    // region if one is at bound
    (@if one is at bound(any) { $($then:tt)* } else { $($else:tt)* }) => {
        $($then)*
    };
    (@if one is at bound(start) { $($then:tt)* } else { $($else:tt)* }) => {
        $($else)*
    };
    (@if one is at bound(end) { $($then:tt)* } else { $($else:tt)* }) => {
        $($then)*
    };
    // endregion

    // region trivial edge cases
    (@trivial edge cases(<, $target:ident, $bound:ident; $skeleton:ident)) => {
        traverse!(@if one is at bound($bound) {
            if $target > $skeleton.borrow().last_position() {
                return Some(Position::at_end($skeleton));
            }
        } else {});
    };
    (@trivial edge cases(<=, $target:ident, $bound:ident; $skeleton:ident)) => {
        traverse!(@if one is at bound($bound) {
            if $target >= $skeleton.borrow().last_position() {
                return Some(Position::at_end($skeleton));
            }
        } else {});
    };
    (@trivial edge cases(==, $target:ident, $bound:ident; $skeleton:ident)) => {
        {
            traverse!(@if zero is at bound($bound) {
                if $target == $skeleton.borrow().offset() {
                    return Some(Position::at_start($skeleton));
                }
            } else {});
            traverse!(@if one is at bound($bound) {
                if $target == $skeleton.borrow().last_position() {
                    return Some(Position::at_end($skeleton));
                }
            } else {});
        }
    };
    (@trivial edge cases(>=, $target:ident, $bound:ident; $skeleton:ident)) => {
        traverse!(@if zero is at bound($bound) {
            if $target <= $skeleton.borrow().offset() {
                return Some(Position::at_start($skeleton));
            }
        } else {});
    };
    (@trivial edge cases(>, $target:ident, $bound:ident; $skeleton:ident)) => {
        traverse!(@if zero is at bound($bound) {
            if $target < $skeleton.borrow().offset() {
                return Some(Position::at_start($skeleton));
            }
        } else {});
    };
    // endregion

    (@($depth:ident, $cmp:tt, $target:ident, $bound:ident; $skeleton:ident)) => {
        if $skeleton.borrow().elements.is_empty() {
            None
        } else if traverse!(@out of bounds condition($cmp, $target; $skeleton)) {
            None
        } else if $skeleton.borrow().links.is_empty() {
            traverse!(@if zero is at bound($bound) {
                if $skeleton.borrow().offset $cmp $target {
                    Some(Position::new($skeleton.clone(), 0, $skeleton.borrow().offset))
                } else {
                    None
                }
            } else {
                None
            })
        } else {
            traverse!(@trivial edge cases($cmp, $target, $bound; $skeleton));
            traverse!(@checked($depth, $cmp, $target, $bound; $skeleton))
        }
    };
    // endregion

    // region public
    ($skeleton:ident; $depth:ident; $cmp:tt $target:ident) => {
        traverse!(@($depth, $cmp, $target, any; $skeleton))
    };
    ($skeleton:ident; $depth:ident; $cmp:tt $target:ident at $bound:ident) => {
        traverse!(@($depth, $cmp, $target, $bound; $skeleton))
    };
    // endregion
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

macro_rules! traversal_methods {
    (@shallow $pos:ident: $cmp:tt) => {
        paste! {
            pub fn [<shallow_ $pos>](this: Rc<RefCell<Self>>, target: S)
                -> Option<Position<Kind, S, T>> {
                traverse!(this; shallow; $cmp target)
            }
        }
    };
    (@deep $pos:ident: $cmp:tt) => {
        pub fn $pos(this: Rc<RefCell<Self>>, target: S)
            -> Option<Position<Kind, S, T>> {
            traverse!(this; deep; $cmp target)
        }
    };
    () => {
        for_all_traversals!(traversal_methods @shallow);
        for_all_traversals!(traversal_methods @deep);
    };
    (@shallow $bound:ident $pos:ident: $cmp:tt) => {
        paste! {
            pub fn [<shallow_ $bound ing_ $pos>](this: Rc<RefCell<Self>>, target: S)
                -> Option<Position<Range, S, T>> {
                traverse!(this; shallow; $cmp target at $bound)
            }
        }
    };
    (@deep $bound:ident $pos:ident: $cmp:tt) => {
        paste! {
            pub fn [<$bound ing_ $pos>](this: Rc<RefCell<Self>>, target: S)
                -> Option<Position<Range, S, T>> {
                traverse!(this; deep; $cmp target at $bound)
            }
        }
    };
    (range) => {
        for_all_traversals!(traversal_methods @shallow start);
        for_all_traversals!(traversal_methods @shallow end);
        for_all_traversals!(traversal_methods @deep start);
        for_all_traversals!(traversal_methods @deep end);
    };
}

#[allow(unused)]
impl<Kind, S: Spacing, T> Skeleton<Kind, S, T> {
    traversal_methods!();
}

#[allow(unused)]
impl<S: Spacing, T> Skeleton<Range, S, T> {
    traversal_methods!(range);
}

pub mod iteration;
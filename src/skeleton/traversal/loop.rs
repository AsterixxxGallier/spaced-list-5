/// If an element <= a target is searched for, the search can be aborted as soon as an element
/// is found that equals the target.
macro_rules! break_if_equal {
    (<, $target:ident; $position:ident) => {};
    (<=, $target:ident; $position:ident) => {
        if $position == $target {
            break;
        }
    };
}

/// If the traversal is deep, a result might be found in a sublist.
macro_rules! into_sublist_if_deep {
    (shallow, $($_rest:tt)*) => {};
    (deep, $cmp:tt, $target:ident;
        $skeleton:ident, $degree:ident, $index:ident, $position:ident) => {
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
}

/// This is the core traversal loop. Three variables are most prominently mutated through its
/// iterations: the index, the degree and the position. The index is a node index, and the degree
/// is a measure of how many steps we are away from the "leaves" of the "tree" (as the Skeleton
/// struct closely resembles a binary tree). Each index-degree combination encountered in this
/// loop has an associated link index, given by the link_index function. A degree of 0 means the
/// link equals the spacing between two adjacent nodes (ignoring sublists). A degree of n means
/// the associated link equals the spacing between nodes whose difference in node index is
/// 2^n. At each step in this loop, the degree is reduced by one, corresponding to a halving
/// of the search space. This gives it a O(log n) time complexity (although sublists can worsen
/// this, their impact is manageable in most cases).
macro_rules! r#loop {
    // All five comparison operators can, for the purpose of this traversal, be reduced to two.
    // For example, to find the first element > the target, it suffices to find the last element
    // <= the target, as its successor is surely the intended result.

    // region redirect loop cmp
    ($depth:ident, ==, $($rest:tt)*) => {
        r#loop!($depth, <=, $($rest)*)
    };
    ($depth:ident, >=, $($rest:tt)*) => {
        r#loop!($depth, <=, $($rest)*)
    };
    ($depth:ident, >, $($rest:tt)*) => {
        r#loop!($depth, <=, $($rest)*)
    };
    // endregion

    ($depth:ident, $cmp:tt, $target:ident;
        $skeleton:ident, $degree:ident, $index:ident, $position:ident) => {
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
                break_if_equal!($cmp, $target; $position);
            }

            if $degree > 0 {
                $degree -= 1;
            } else {
                into_sublist_if_deep!($depth, $cmp, $target; $skeleton, $degree, $index, $position);
                break;
            }
        }
    };
}

pub(super) use {break_if_equal, into_sublist_if_deep, r#loop};
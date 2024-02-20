/// These conditions are for checking before traversal if an element matching the condition can
/// be found. For example, it is impossible for a list that ends at 5 to contain an element > 5.
macro_rules! out_of_bounds_condition {
    (<, $target:ident; $skeleton:ident) => {
        $target <= $skeleton.borrow().offset
    };
    (<=, $target:ident; $skeleton:ident) => {
        $target < $skeleton.borrow().offset
    };
    (==, $target:ident; $skeleton:ident) => {
        $target < $skeleton.borrow().offset ||
            $target > $skeleton.borrow().last_position()
    };
    (>=, $target:ident; $skeleton:ident) => {
        $target > $skeleton.borrow().last_position()
    };
    (>, $target:ident; $skeleton:ident) => {
        $target >= $skeleton.borrow().last_position()
    };
}

/// In some cases, it is trivial to find the correct element. For example, the first element > 0
/// in a list that starts at 4 is inadvertently the first element. However, if we're looking for
/// the first element with an end > 0, the result is the second node (assuming a non-nested range
/// list). Finding it is non-trivial, so this macro doesn't handle that case.
// it would be possible to handle it (in a non-nested list), but not worth the effort
macro_rules! trivial_results {
    (<, $target:ident, $condition:ident ($($args:tt),*); $skeleton:ident) => {
        if $condition!($skeleton.borrow().links.len(), $skeleton.clone(), $($args),*) && $target > $skeleton.borrow().last_position() {
            return Some(EphemeralPosition::at_end($skeleton));
        }
    };
    (<=, $target:ident, $condition:ident ($($args:tt),*); $skeleton:ident) => {
        if $condition!($skeleton.borrow().links.len(), $skeleton.clone(), $($args),*) && $target >= $skeleton.borrow().last_position() {
            return Some(EphemeralPosition::at_end($skeleton));
        }
    };
    (==, $target:ident, $condition:ident ($($args:tt),*); $skeleton:ident) => {
        if $condition!(0, $skeleton.clone(), $($args),*) && $target == $skeleton.borrow().offset() {
            return Some(EphemeralPosition::at_start($skeleton));
        } else if $condition!($skeleton.borrow().links.len(), $skeleton.clone(), $($args),*) && $target == $skeleton.borrow().last_position() {
            return Some(EphemeralPosition::at_end($skeleton));
        }
    };
    (>=, $target:ident, $condition:ident ($($args:tt),*); $skeleton:ident) => {
        if $condition!(0, $skeleton.clone(), $($args),*) && $target <= $skeleton.borrow().offset() {
            return Some(EphemeralPosition::at_start($skeleton));
        }
    };
    (>, $target:ident, $condition:ident ($($args:tt),*); $skeleton:ident) => {
        if $condition!(0, $skeleton.clone(), $($args),*) && $target < $skeleton.borrow().offset() {
            return Some(EphemeralPosition::at_start($skeleton));
        }
    };
}

pub(super) use {out_of_bounds_condition, trivial_results};
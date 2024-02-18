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

/// These are helper macros for selecting one of two code blocks depending on the bound type
/// searched for. For example, the index zero is always at the start of a range, and never at its
/// end. The "any" bound type always results in the "then"-block being chosen.
macro_rules! bound_type_conditional {
    // region if 0 is at bound
    (if 0 is at bound (any) { $($then:tt)* } else { $($else:tt)* }) => {
        $($then)*
    };
    (if 0 is at bound (start) { $($then:tt)* } else { $($else:tt)* }) => {
        $($then)*
    };
    (if 0 is at bound (end) { $($then:tt)* } else { $($else:tt)* }) => {
        $($else)*
    };
    // endregion

    // region if 1 is at bound
    (if 1 is at bound (any) { $($then:tt)* } else { $($else:tt)* }) => {
        $($then)*
    };
    (if 1 is at bound (start) { $($then:tt)* } else { $($else:tt)* }) => {
        $($else)*
    };
    (if 1 is at bound (end) { $($then:tt)* } else { $($else:tt)* }) => {
        $($then)*
    };
    // endregion
}

/// In some cases, it is trivial to find the correct element. For example, the first element > 0
/// in a list that starts at 4 is inadvertently the first element. However, if we're looking for
/// the first element with an end > 0, the result is the second node. Finding it is non-trivial,
/// so this macro doesn't handle that case.
// it would be possible to handle it, but not worth the effort
macro_rules! trivial_results {
    (<, $target:ident, $bound:ident; $skeleton:ident) => {
        bound_type_conditional!(if 1 is at bound ($bound) {
            if $target > $skeleton.borrow().last_position() {
                return Some(EphemeralPosition::at_end($skeleton));
            }
        } else {});
    };
    (<=, $target:ident, $bound:ident; $skeleton:ident) => {
        bound_type_conditional!(if 1 is at bound ($bound) {
            if $target >= $skeleton.borrow().last_position() {
                return Some(EphemeralPosition::at_end($skeleton));
            }
        } else {});
    };
    (==, $target:ident, $bound:ident; $skeleton:ident) => {
        {
            bound_type_conditional!(if 0 is at bound ($bound) {
                if $target == $skeleton.borrow().offset() {
                    return Some(EphemeralPosition::at_start($skeleton));
                }
            } else {});
            bound_type_conditional!(if 1 is at bound ($bound) {
                if $target == $skeleton.borrow().last_position() {
                    return Some(EphemeralPosition::at_end($skeleton));
                }
            } else {});
        }
    };
    (>=, $target:ident, $bound:ident; $skeleton:ident) => {
        bound_type_conditional!(if 0 is at bound ($bound) {
            if $target <= $skeleton.borrow().offset() {
                return Some(EphemeralPosition::at_start($skeleton));
            }
        } else {});
    };
    (>, $target:ident, $bound:ident; $skeleton:ident) => {
        bound_type_conditional!(if 0 is at bound ($bound) {
            if $target < $skeleton.borrow().offset() {
                return Some(EphemeralPosition::at_start($skeleton));
            }
        } else {});
    };
}

pub(super) use {out_of_bounds_condition, bound_type_conditional, trivial_results};
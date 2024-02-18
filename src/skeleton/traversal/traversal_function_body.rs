/// These fragments frame and combine [`loop`] and [`after_loop`]. They also initialize the loop's main
/// variables. However, they assume that all edge cases have been dealt with.
macro_rules! checked {
    (shallow, $cmp:tt, $target:ident, $condition:ident; $skeleton:ident) => {
        {
            let mut degree = $skeleton.borrow().depth - 1;
            let mut index = 0;
            let mut position = $skeleton.borrow().offset;
            r#loop!(shallow, $cmp, $target; $skeleton, degree, index, position);
            after_loop!(shallow, $cmp, $target, $condition; $skeleton, degree, index, position)
        }
    };
    (deep, $cmp:tt, $target:ident, $condition:ident; $skeleton:ident) => {
        {
            let mut skeleton = $skeleton;
            let mut degree = skeleton.borrow().depth - 1;
            let mut index = 0;
            let mut position = skeleton.borrow().offset;
            r#loop!(deep, $cmp, $target; skeleton, degree, index, position);
            after_loop!(deep, $cmp, $target, $condition; skeleton, degree, index, position)
        }
    };
}

/// Generates the body of a traversal function.
macro_rules! traversal_function_body {
    ($skeleton:ident; $depth:ident; $cmp:tt $target:ident with $condition:ident) => {
        if $skeleton.borrow().elements.is_empty() {
            None
        } else if out_of_bounds_condition!($cmp, $target; $skeleton) {
            None
        } else if $skeleton.borrow().links.is_empty() {
            if $condition!(0) && $skeleton.borrow().offset $cmp $target {
                Some(EphemeralPosition::new($skeleton.clone(), 0, $skeleton.borrow().offset))
            } else {
                None
            }
        } else {
            trivial_results!($cmp, $target, $condition; $skeleton);
            checked!($depth, $cmp, $target, $condition; $skeleton)
        }
    };
}

pub(super) use {checked, traversal_function_body};
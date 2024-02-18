/// These fragments frame and combine [`loop`] and [`after_loop`]. They also initialize the loop's main
/// variables. However, they assume that all edge cases have been dealt with.
macro_rules! checked {
    (shallow, $cmp:tt, $target:ident, $bound:ident; $skeleton:ident) => {
        {
            let mut degree = $skeleton.borrow().depth - 1;
            let mut index = 0;
            let mut position = $skeleton.borrow().offset;
            r#loop!(shallow, $cmp, $target, $bound; $skeleton, degree, index, position);
            after_loop!(shallow, $cmp, $target, $bound; $skeleton, degree, index, position)
        }
    };
    (deep, $cmp:tt, $target:ident, $bound:ident; $skeleton:ident) => {
        {
            let mut skeleton = $skeleton;
            let mut degree = skeleton.borrow().depth - 1;
            let mut index = 0;
            let mut position = skeleton.borrow().offset;
            r#loop!(deep, $cmp, $target, $bound; skeleton, degree, index, position);
            after_loop!(deep, $cmp, $target, $bound; skeleton, degree, index, position)
        }
    };
}

/// Generates the body of a traversal function
macro_rules! traversal_function_body {
    ($skeleton:ident; $depth:ident; $cmp:tt $target:ident) => {
        traversal_function_body!($skeleton; $depth; $cmp $target at any)
    };
    ($skeleton:ident; $depth:ident; $cmp:tt $target:ident at $bound:ident) => {
        if $skeleton.borrow().elements.is_empty() {
            None
        } else if out_of_bounds_condition!($cmp, $target; $skeleton) {
            None
        } else if $skeleton.borrow().links.is_empty() {
            bound_type_conditional!(if 0 is at bound ($bound) {
                if $skeleton.borrow().offset $cmp $target {
                    Some(EphemeralPosition::new($skeleton.clone(), 0, $skeleton.borrow().offset))
                } else {
                    None
                }
            } else {
                None
            })
        } else {
            trivial_results!($cmp, $target, $bound; $skeleton);
            checked!($depth, $cmp, $target, $bound; $skeleton)
        }
    };
}

pub(super) use {checked, traversal_function_body};
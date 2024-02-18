/// These fragments allow us to find the successor of an element. This is non-trivial in deep
/// traversals, as we might have to move out of a sublist back into the containing list, or we
/// might need to move into a sublist.
macro_rules! next {
    (shallow; $skeleton:ident, $index:ident, $position:ident) => {
        if $index == $skeleton.borrow().links.len() {
            Err("Tried to move to next element but it's already the end of the skeleton")
        } else {
            $position += $skeleton.borrow().link($index);
            $index += 1;
            Ok(())
        }
    };
    (deep; $skeleton:ident, $index:ident, $position:ident) => {
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
}

/// These fragments are for finding the predecessor. The logic is similar to that of [`next`].
macro_rules! previous {
    (shallow; $skeleton:ident, $index:ident, $position:ident) => {
        if $index == 0 {
            Err("Tried to move to previous element but it's already the start of the skeleton")
        } else {
            $index -= 1;
            $position -= $skeleton.borrow().link($index);
            Ok(())
        }
    };
    (deep; $skeleton:ident, $index:ident, $position:ident) => {
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
}

macro_rules! index_is_at_bound {
    (start; $index:expr) => {
        $index & 1 == 0
    };
    (end; $index:expr) => {
        $index & 1 == 1
    };
}

/// These fragments implement logic that needs to be executed after the loop has found a target.
/// For example, when an element is searched for that == a target, the loop only searches for
/// the last element that <= a target. In the case that it <=, but not == the target, logic in
/// here returns None instead of the wrong result.
macro_rules! after_loop {
    // region any
    ($depth:ident, <, $target:ident, any;
        $skeleton:ident, $degree:ident, $index:ident, $position:ident) => {
        Some(EphemeralPosition::new($skeleton, $index, $position))
    };
    ($depth:ident, <=, $target:ident, any;
        $skeleton:ident, $degree:ident, $index:ident, $position:ident) => {
        Some(EphemeralPosition::new($skeleton, $index, $position))
    };
    ($depth:ident, ==, $target:ident, any;
        $skeleton:ident, $degree:ident, $index:ident, $position:ident) => {
        if $position == $target {
            Some(EphemeralPosition::new($skeleton, $index, $position))
        } else {
            None
        }
    };
    ($depth:ident, >=, $target:ident, any;
        $skeleton:ident, $degree:ident, $index:ident, $position:ident) => {
        {
            if $position < $target {
                next!($depth; $skeleton, $index, $position).unwrap();
            }
            Some(EphemeralPosition::new($skeleton, $index, $position))
        }
    };
    ($depth:ident, >, $target:ident, any;
        $skeleton:ident, $degree:ident, $index:ident, $position:ident) => {
        {
            next!($depth; $skeleton, $index, $position).unwrap();
            Some(EphemeralPosition::new($skeleton, $index, $position))
        }
    };
    // endregion

    // Similar logic needs to be employed when the search is even more specific and only starts or
    // ends (= nodes with even/odd indices) are looked for.

    // region start/end
    ($depth:ident, <, $target:ident, $bound:ident;
        $skeleton:ident, $degree:ident, $index:ident, $position:ident) => {
        {
            if !index_is_at_bound!($bound; $index) {
                previous!($depth; $skeleton, $index, $position).ok()?;
            }
            Some(EphemeralPosition::new($skeleton, $index, $position))
        }
    };
    ($depth:ident, <=, $target:ident, $bound:ident;
        $skeleton:ident, $degree:ident, $index:ident, $position:ident) => {
        {
            if !index_is_at_bound!($bound; $index) {
                previous!($depth; $skeleton, $index, $position).ok()?;
            }
            Some(EphemeralPosition::new($skeleton, $index, $position))
        }
    };
    ($depth:ident, ==, $target:ident, $bound:ident;
        $skeleton:ident, $degree:ident, $index:ident, $position:ident) => {
        if $position == $target && index_is_at_bound!($bound; $index) {
            Some(EphemeralPosition::new($skeleton, $index, $position))
        } else {
            None
        }
    };
    ($depth:ident, >=, $target:ident, $bound:ident;
        $skeleton:ident, $degree:ident, $index:ident, $position:ident) => {
        {
            if $position < $target {
                next!($depth; $skeleton, $index, $position).unwrap();
            }
            if !index_is_at_bound!($bound; $index) {
                next!($depth; $skeleton, $index, $position).ok()?;
            }
            Some(EphemeralPosition::new($skeleton, $index, $position))
        }
    };
    ($depth:ident, >, $target:ident, $bound:ident;
        $skeleton:ident, $degree:ident, $index:ident, $position:ident) => {
        {
            next!($depth; $skeleton, $index, $position).unwrap();
            if !index_is_at_bound!($bound; $index) {
                next!($depth; $skeleton, $index, $position).ok()?;
            }
            Some(EphemeralPosition::new($skeleton, $index, $position))
        }
    };
    // endregion
}

pub(super) use {next, previous, index_is_at_bound, after_loop};
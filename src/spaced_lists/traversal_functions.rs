macro_rules! unconditional_traversal_function {
    ($kind:ident; $function:ident, $skeleton_function:ident, $position:ty) => {
        paste! {
            #[must_use]
            pub fn $function(&self, position: S) -> Option<$position> {
                Skeleton::<$kind, _, _>::[< conditional_ $skeleton_function >](self.skeleton.clone(), position, |slot| slot.is_some()).map(Into::into)
            }
        }
    };
}

macro_rules! conditional_traversal_function {
    ($kind:ident; $function:ident, $skeleton_function:ident, $position:ty) => {
        paste! {
            #[must_use]
            pub fn [< conditional_ $function >]<C: Fn(&T) -> bool>(&self, position: S, condition: C) -> Option<$position> {
                Skeleton::<$kind, _, _>::[< conditional_ $skeleton_function >](self.skeleton.clone(), position, |slot| {
                    match slot.deref() {
                        ElementSlot::Some(element) => condition(element),
                        ElementSlot::None => false
                    }
                }).map(Into::into)
            }
        }
    };
}

macro_rules! all_traversal_functions {
    ($kind:ident; $macro_prefix:ident, $($function_prefix:ident)?, $($skeleton_function_prefix:ident)?, $position:ty) => {
        paste! {
            [< $macro_prefix traversal_function >]!($kind; [< $($function_prefix)? before >], [< $($skeleton_function_prefix)? before >], $position);
            [< $macro_prefix traversal_function >]!($kind; [< $($function_prefix)? at_or_before >], [< $($skeleton_function_prefix)? at_or_before >], $position);
            [< $macro_prefix traversal_function >]!($kind; [< $($function_prefix)? at >], [< $($skeleton_function_prefix)? at >], $position);
            [< $macro_prefix traversal_function >]!($kind; [< $($function_prefix)? at_or_after >], [< $($skeleton_function_prefix)? at_or_after >], $position);
            [< $macro_prefix traversal_function >]!($kind; [< $($function_prefix)? after >], [< $($skeleton_function_prefix)? after >], $position);
        }
    };
    (Node; $macro_prefix:ident, $position:ty) => {
        all_traversal_functions!(Node; $macro_prefix, , , $position);
    };
    ($range_kind:ident; $macro_prefix:ident, $position:ty) => {
        all_traversal_functions!($range_kind; $macro_prefix, starting_or_ending_, , $position);
        all_traversal_functions!($range_kind; $macro_prefix, starting_, starting_, $position);
        all_traversal_functions!($range_kind; $macro_prefix, ending_, ending_, $position);
    };
}

pub(super) use {unconditional_traversal_function, conditional_traversal_function, all_traversal_functions};
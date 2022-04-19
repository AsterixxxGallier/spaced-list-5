macro_rules! maybe_stop {
    (<= $target:ident; $position:ident) => {
        if $position == $target {
            break;
        }
    };
    (< $_target:ident; $_position:ident) => {}
}

macro_rules! maybe_move_forwards {
    ($cmp:tt $target:ident; $skeleton:ident;
        $degree:ident, $index:ident, $link_index:ident, $position:ident) => {
        let next_position = $position + $skeleton.link_length_at($link_index);
        if next_position $cmp $target {
            $position = next_position;
            $index += 1 << $degree;
            maybe_stop!($cmp $target; $position);
        }
    }
}

macro_rules! descend {
    (deep; $skeleton:ident; $cmp:tt $target:ident;
        $degree:ident, $index:ident, $position:ident; $super_skeletons:ident) => {
        if $degree == 0 {
            if let Some(subskeleton) = $skeleton.subskeleton_at($index) {
                let next_position = $position + subskeleton.offset();
                if next_position $cmp $target {
                    $degree = subskeleton.depth().saturating_sub(1);
                    $index = 0;
                    $position = next_position;
                    $super_skeletons.push($skeleton);
                    $skeleton = subskeleton;
                    continue;
                }
            }
            break;
        } else {
            $degree -= 1;
        }
    };
    (shallow; $_skeleton:ident; $_cmp:tt $_target:ident;
        $degree:ident, $_index:ident, $_position:ident) => {
        if $degree == 0 {
            break;
        } else {
            $degree -= 1;
        }
    };
}

macro_rules! loop_while {
    ($depth:tt; $skeleton:ident; $cmp:tt $target:ident;
        $degree:ident, $index:ident, $position:ident$(; $super_skeletons:ident)?) => {
        loop {
            let link_index = link_index($index, $degree);
            if !$skeleton.link_index_is_in_bounds(link_index) {
                if $degree == 0 {
                    break;
                }
                $degree -= 1;
                continue;
            }
            maybe_move_forwards!($cmp $target; $skeleton;
                $degree, $index, link_index, $position);
            descend!($depth; $skeleton; $cmp $target; $degree, $index, $position$(; $super_skeletons)?);
        }
    }
}

macro_rules! next {
    ($skeleton:ident; $index:ident, $position:ident$(; $super_skeletons:ident)?) => {
        if $index == $skeleton.link_size() {
            $(if !$super_skeletons.is_empty() {
                let super_skeleton = $super_skeletons.pop().unwrap();
                let index_in_super_skeleton = $skeleton.index_in_super_skeleton().unwrap();
                $position -= $skeleton.last_position();
                $skeleton = super_skeleton;
                $position += $skeleton.link_length_at_node(index_in_super_skeleton);
                $index = index_in_super_skeleton + 1;
                Ok(())
            } else )?{
                Err("Tried to move to next node but it's already the end of the skeleton")
            }
        } else {
            $(if let Some(subskeleton) = $skeleton.subskeleton_at($index) {
                $super_skeletons.push($skeleton);
                $skeleton = subskeleton;
                $index = 0;
                $position += subskeleton.offset();
                Ok(())
            } else )?{
                $position += $skeleton.link_length_at_node($index);
                $index += 1;
                Ok(())
            }
        }
    };
}

macro_rules! previous {
    ($skeleton:ident; $index:ident, $position:ident$(; $super_skeletons:ident)?) => {
        if $index == 0 {
            $(if let Some(new_index) = $skeleton.index_in_super_skeleton() {
                $index = new_index;
                $position -= $skeleton.offset();
                $skeleton = $super_skeletons.pop().unwrap();
                Ok(())
            } else )? {
                Err("Tried to move to previous node but it's already the start of the skeleton")
            }
        }
        $(
        else {
            let index_before = $index - 1;
            if let Some(subskeleton) = $skeleton.subskeleton_at(index_before) {
                $index = subskeleton.node_size() - 1;
                $position -=
                    $skeleton.link_length_at_node(index_before)
                        - subskeleton.last_position();
                $super_skeletons.push($skeleton);
                $skeleton = subskeleton;
                Ok(())
            } else {
                $index -= 1;
                $position -= $skeleton.link_length_at_node($index);
                Ok(())
            }
        }
        )?
    };
}

macro_rules! pos {
    ($skeleton:expr; $index:expr, $position:expr; $super_skeletons:expr) => {
        Position::new($super_skeletons, $skeleton, $index, $position)
    };
    ($_skeleton:expr; $index:expr, $position:expr) => {
        ShallowPosition::new($index, $position)
    }
}

macro_rules! index_is_at_bound {
    (start; $index:expr) => {
        $index & 1 == 0
    };
    (end; $index:expr) => {
        $index & 1 == 1
    };
}

macro_rules! is_at_bound_if_range {
    (node; $_index:expr) => { true };
    ((range $bound:tt); $index:expr) => { index_is_at_bound!($bound; $index) };
}

macro_rules! traverse_unchecked_with_variables {
    (node; $depth:tt; $skeleton:ident; < $target:ident;
        $degree:ident, $index:ident, $position:ident$(; $super_skeletons:ident)?) => {
        {
            loop_while!($depth; $skeleton; < $target;
                $degree, $index, $position$(; $super_skeletons)?);
            Some(pos!($skeleton; $index, $position$(; $super_skeletons)?))
        }
    };
    (node; $depth:tt; $skeleton:ident; <= $target:ident;
        $degree:ident, $index:ident, $position:ident$(; $super_skeletons:ident)?) => {
        {
            loop_while!($depth; $skeleton; <= $target;
                $degree, $index, $position$(; $super_skeletons)?);
            Some(pos!($skeleton; $index, $position$(; $super_skeletons)?))
        }
    };
    (node; $depth:tt; $skeleton:ident; == $target:ident;
        $degree:ident, $index:ident, $position:ident$(; $super_skeletons:ident)?) => {
        {
            loop_while!($depth; $skeleton; <= $target;
                $degree, $index, $position$(; $super_skeletons)?);
            if $position == $target {
                Some(pos!($skeleton; $index, $position$(; $super_skeletons)?))
            } else {
                None
            }
        }
    };
    (node; $depth:tt; $skeleton:ident; >= $target:ident;
        $degree:ident, $index:ident, $position:ident$(; $super_skeletons:ident)?) => {
        {
            loop_while!($depth; $skeleton; <= $target;
                $degree, $index, $position$(; $super_skeletons)?);
            if $position == $target {
                Some(pos!($skeleton; $index, $position$(; $super_skeletons)?))
            } else {
                next!($skeleton; $index, $position$(; $super_skeletons)?).unwrap();
                Some(pos!($skeleton; $index, $position$(; $super_skeletons)?))
            }
        }
    };
    (node; $depth:tt; $skeleton:ident; > $target:ident;
        $degree:ident, $index:ident, $position:ident$(; $super_skeletons:ident)?) => {
        {
            loop_while!($depth; $skeleton; <= $target;
                $degree, $index, $position$(; $super_skeletons)?);
            next!($skeleton; $index, $position$(; $super_skeletons)?).unwrap();
            Some(pos!($skeleton; $index, $position$(; $super_skeletons)?))
        }
    };
    ((range $bound:tt); $depth:tt; $skeleton:ident; < $target:ident;
        $degree:ident, $index:ident, $position:ident$(; $super_skeletons:ident)?) => {
        {
            loop_while!($depth; $skeleton; < $target;
                $degree, $index, $position$(; $super_skeletons)?);
            if !index_is_at_bound!($bound; $index) {
                previous!($skeleton; $index, $position$(; $super_skeletons)?).ok()?;
            }
            Some(pos!($skeleton; $index, $position$(; $super_skeletons)?))
        }
    };
    ((range $bound:tt); $depth:tt; $skeleton:ident; <= $target:ident;
        $degree:ident, $index:ident, $position:ident$(; $super_skeletons:ident)?) => {
        {
            loop_while!($depth; $skeleton; <= $target;
                $degree, $index, $position$(; $super_skeletons)?);
            if !index_is_at_bound!($bound; $index) {
                previous!($skeleton; $index, $position$(; $super_skeletons)?).ok()?;
            }
            Some(pos!($skeleton; $index, $position$(; $super_skeletons)?))
        }
    };
    ((range $bound:tt); $depth:tt; $skeleton:ident; == $target:ident;
        $degree:ident, $index:ident, $position:ident$(; $super_skeletons:ident)?) => {
        {
            loop_while!($depth; $skeleton; <= $target;
                $degree, $index, $position$(; $super_skeletons)?);
            if $position == $target && index_is_at_bound!($bound; $index) {
                Some(pos!($skeleton; $index, $position$(; $super_skeletons)?))
            } else {
                None
            }
        }
    };
    ((range $bound:tt); $depth:tt; $skeleton:ident; >= $target:ident;
        $degree:ident, $index:ident, $position:ident$(; $super_skeletons:ident)?) => {
        {
            loop_while!($depth; $skeleton; <= $target;
                $degree, $index, $position$(; $super_skeletons)?);
            if $position == $target {
                if !index_is_at_bound!($bound; $index) {
                    next!($skeleton; $index, $position$(; $super_skeletons)?).ok()?;
                }
                Some(pos!($skeleton; $index, $position$(; $super_skeletons)?))
            } else {
                next!($skeleton; $index, $position$(; $super_skeletons)?).unwrap();
                if !index_is_at_bound!($bound; $index) {
                    next!($skeleton; $index, $position$(; $super_skeletons)?).ok()?;
                }
                Some(pos!($skeleton; $index, $position$(; $super_skeletons)?))
            }
        }
    };
    ((range $bound:tt); $depth:tt; $skeleton:ident; > $target:ident;
        $degree:ident, $index:ident, $position:ident$(; $super_skeletons:ident)?) => {
        {
            loop_while!($depth; $skeleton; <= $target;
                $degree, $index, $position$(; $super_skeletons)?);
            next!($skeleton; $index, $position$(; $super_skeletons)?).unwrap();
            if !index_is_at_bound!($bound; $index) {
                next!($skeleton; $index, $position$(; $super_skeletons)?).ok()?;
            }
            Some(pos!($skeleton; $index, $position$(; $super_skeletons)?))
        }
    };
}

macro_rules! traverse_unchecked {
    ($kind:tt; deep; $skeleton:expr; $cmp:tt $target:ident) => {
        if $skeleton.link_size() == 0 {
            if $skeleton.offset() $cmp $target {
                if is_at_bound_if_range!($kind; 0) {
                    Some(pos!($skeleton; 0, $skeleton.offset(); vec![]))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            let mut skeleton = $skeleton;
            let mut degree = $skeleton.depth() - 1;
            let mut index = 0;
            let mut position = $skeleton.offset();
            traverse_unchecked_with_variables!($kind; deep; skeleton; $cmp $target;
                degree, index, position)
        }
    };
    ($kind:tt; shallow; $skeleton:expr; $cmp:tt $target:ident) => {
        if $skeleton.node_size() == 0 {
            if $skeleton.offset() $cmp $target {
                if is_at_bound_if_range!($kind; 0) {
                    Some(pos!($skeleton; 0, $skeleton.offset()))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            let skeleton = $skeleton;
            let mut degree = $skeleton.depth() - 1;
            let mut index = 0;
            let mut position = $skeleton.offset();
            traverse_unchecked_with_variables!($kind; shallow; skeleton; $cmp $target;
                degree, index, position)
        }
    };
}

macro_rules! handle_before_offset {
    ($kind:tt; deep; $skeleton:expr) => {
        if is_at_bound_if_range!($kind; 0) {
            Some(pos!($skeleton; 0, $skeleton.offset(); vec![]))
        } else if $skeleton.link_size() >= 1 {
            let mut position = $skeleton.offset();
            if let Some(subskeleton) = $skeleton.subskeleton_at(0) {
                position += subskeleton.offset();
                Some(pos!(subskeleton; 0, position; vec![$skeleton]))
            } else {
                position += $skeleton.link_length_at(0);
                Some(pos!($skeleton; 1, position; vec![]))
            }
        } else {
            Some(Position::new(vec![], $skeleton, 0, $skeleton.offset()))
        }
    };
    ($kind:tt; shallow; $skeleton:expr) => {
        if is_at_bound_if_range!($kind; 0) {
            Some(pos!($skeleton; 0, $skeleton.offset(); vec![]))
        } else if $skeleton.link_size() >= 1 {
            Some(pos!($skeleton; 1, $skeleton.offset() + $skeleton.link_length_at(0); vec![]))
        } else {
            Some(Position::new(vec![], $skeleton, 0, $skeleton.offset()))
        }
    };
}

macro_rules! traverse {
    ($kind:tt; $depth:tt; $skeleton:expr; < $target:ident) => {
        if $target <= $skeleton.offset() {
            None
        } else {
            traverse_unchecked!($kind; $depth; $skeleton; < $target)
        }
    };
    ($kind:tt; $depth:tt; $skeleton:expr; <= $target:ident) => {
        if $target < $skeleton.offset() {
            None
        } else {
            traverse_unchecked!($kind; $depth; $skeleton; <= $target)
        }
    };
    ($kind:tt; $depth:tt; $skeleton:expr; == $target:ident) => {
        if $target < $skeleton.offset() {
            None
        } else {
            traverse_unchecked!($kind; $depth; $skeleton; == $target)
        }
    };
    ($kind:tt; $depth:tt; $skeleton:expr; >= $target:ident) => {
        if $target > $skeleton.last_position() {
            None
        } else if $target <= $skeleton.offset() {
            handle_before_offset!($kind; $depth; $skeleton)
        } else {
            traverse_unchecked!($kind; $depth; $skeleton; >= $target)
        }
    };
    ($kind:tt; $depth:tt; $skeleton:expr; > $target:ident) => {
        if $target >= $skeleton.last_position() {
            None
        } else if $target < $skeleton.offset() {
            handle_before_offset!($kind; $depth; $skeleton)
        } else {
            traverse_unchecked!($kind; $depth; $skeleton; > $target)
        }
    }
}

pub(crate) use traverse;
pub(crate) use traverse_unchecked;
pub(crate) use traverse_unchecked_with_variables;
pub(crate) use loop_while;
pub(crate) use maybe_stop;
pub(crate) use maybe_move_forwards;
pub(crate) use next;
pub(crate) use descend;
pub(crate) use pos;
pub(crate) use index_is_at_bound;
pub(crate) use previous;
pub(crate) use is_at_bound_if_range;
pub(crate) use handle_before_offset;
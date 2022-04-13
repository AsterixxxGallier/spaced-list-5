// ╭───────────────────────────────────────────────────────────────╮
// ├───────────────────────────────╮                               │
// ├───────────────╮               ├───────────────╮               │
// ├───────╮       ├───────╮       ├───────╮       ├───────╮       │
// ├───╮   ├───╮   ├───╮   ├───╮   ├───╮   ├───╮   ├───╮   ├───╮   │
// ╵ 0 ╵ 1 ╵ 0 ╵ 2 ╵ 0 ╵ 1 ╵ 0 ╵ 3 ╵ 0 ╵ 1 ╵ 0 ╵ 2 ╵ 0 ╵ 1 ╵ 0 ╵ 4 ╵
// 00000   00010   00100   00110   01000   01010   01100   01110   10000
//     00001   00011   00101   00111   01001   01011   01101   01111
//
// if it is requested to find the *range* starting or ending before//at//after a target position,
// that means that we have to find the node with a certain last bit before//at//after the target.
// therefore, if we have such a node with the wrong last bit, we need to find the node closest to
// the target that does have the correct last bit and is on the correct side of the target

pub(crate) const fn link_index(node_index: usize, degree: usize) -> usize {
    node_index + (1 << degree) - 1
}

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
        $degree:ident, $node_index:ident, $link_index:ident, $position:ident) => {
        let next_position = $position + $skeleton.link_length_at($link_index);
        if next_position $cmp $target {
            $position = next_position;
            $node_index += 1 << $degree;
            maybe_stop!($cmp $target; $position);
        }
    }
}

macro_rules! descend {
    (deep; $list:ident, $skeleton:ident; $cmp:tt $target:ident;
        $degree:ident, $node_index:ident, $position:ident; $super_lists:ident) => {
        if $degree == 0 {
            if let Some(sublist) = $skeleton.sublist_at($node_index) {
                let sub_skeleton = sublist.skeleton();
                let next_position = $position + sub_skeleton.offset();
                if next_position $cmp $target {
                    $degree = sub_skeleton.depth().saturating_sub(1);
                    $node_index = 0;
                    $position = next_position;
                    $super_lists.push($list);
                    $list = sublist;
                    $skeleton = $list.skeleton();
                    continue;
                }
            }
            break;
        } else {
            $degree -= 1;
        }
    };
    (shallow; $list:ident, $skeleton:ident; $_cmp:tt $_target:ident;
        $degree:ident, $node_index:ident, $position:ident) => {
        if $degree == 0 {
            break;
        } else {
            $degree -= 1;
        }
    };
}

macro_rules! loop_while {
    ($depth:tt; $list:ident, $skeleton:ident; $cmp:tt $target:ident;
        $degree:ident, $node_index:ident, $position:ident$(; $super_lists:ident)?) => {
        loop {
            let link_index = link_index($node_index, $degree);
            if !$skeleton.link_index_is_in_bounds(link_index) {
                if $degree == 0 {
                    break;
                }
                $degree -= 1;
                continue;
            }
            maybe_move_forwards!($cmp $target; $skeleton;
                $degree, $node_index, link_index, $position);
            descend!($depth; $list, $skeleton; $cmp $target; $degree, $node_index, $position$(; $super_lists)?);
        }
    }
}

macro_rules! next {
    ($list:ident, $skeleton:ident; $node_index:ident, $position:ident$(; $super_lists:ident)?) => {
        'next: {
            $(
            if let Some(sublist) = $skeleton.sublist_at($node_index) {
                let sub_skeleton = sublist.skeleton();
                $node_index = 0;
                $position += sub_skeleton.offset();
                $super_lists.push($list);
                $list = sublist;
                $skeleton = $list.skeleton();
                break 'next Ok(());
            }
            )?

            while $node_index == $skeleton.link_size() {
                $(if let Some(new_index) = $skeleton.index_in_super_list() {
                    $node_index = new_index;
                    $position -= $skeleton.last_position();
                    $list = $super_lists.pop().unwrap();
                    $skeleton = $list.skeleton();
                    continue;
                })?
                break 'next Err("Tried to move to next node but it's already the end of the list");
            }

            $position += $skeleton.link_length_at_node($node_index);
            $node_index += 1;

            Ok(())
        }
    };
}

macro_rules! previous {
    ($list:ident, $skeleton:ident; $node_index:ident, $position:ident$(; $super_lists:ident)?) => {
        'previous: {
            $(
            if $node_index > 0 {
                let index_before = $node_index - 1;
                if let Some(sublist) = $skeleton.sublist_at(index_before) {
                    let sub_skeleton = sublist.skeleton();
                    $node_index = sub_skeleton.node_size() - 1;
                    $position -=
                        $skeleton.link_length_at_node(index_before)
                            - sub_skeleton.last_position();
                    $super_lists.push($list);
                    $list = sublist;
                    $skeleton = $list.skeleton();
                    break 'previous Ok(());
                }
            }
            )?

            if $node_index == 0 {
                $(if let Some(new_index) = $skeleton.index_in_super_list() {
                    $node_index = new_index; // + 1 maybe?
                    $position -= $skeleton.offset();
                    $list = $super_lists.pop().unwrap();
                    $skeleton = $list.skeleton();
                    break 'previous Ok(());
                })?
                break 'previous
                    Err("Tried to move to previous node but it's already the start of the list");
            }

            $node_index -= 1;
            $position -= $skeleton.link_length_at_node($node_index);

            Ok(())
        }
    };
}

macro_rules! pos {
    ($list:expr; $node_index:expr, $position:expr; $super_lists:expr) => {
        Position::new($super_lists, $list, $node_index, $position)
    };
    ($_list:expr; $node_index:expr, $position:expr) => {
        ShallowPosition::new($node_index, $position)
    }
}

macro_rules! index_is_at_bound {
    (start; $node_index:expr) => {
        $node_index & 1 == 0
    };
    (end; $node_index:expr) => {
        $node_index & 1 == 1
    };
}

macro_rules! traverse_unchecked_with_variables {
    (node; $depth:tt; $list:ident, $skeleton:ident; < $target:ident;
        $degree:ident, $node_index:ident, $position:ident$(; $super_lists:ident)?) => {
        {
            loop_while!($depth; $list, $skeleton; < $target;
                $degree, $node_index, $position$(; $super_lists)?);
            Some(pos!($list; $node_index, $position$(; $super_lists)?))
        }
    };
    (node; $depth:tt; $list:ident, $skeleton:ident; <= $target:ident;
        $degree:ident, $node_index:ident, $position:ident$(; $super_lists:ident)?) => {
        {
            loop_while!($depth; $list, $skeleton; <= $target;
                $degree, $node_index, $position$(; $super_lists)?);
            Some(pos!($list; $node_index, $position$(; $super_lists)?))
        }
    };
    (node; $depth:tt; $list:ident, $skeleton:ident; == $target:ident;
        $degree:ident, $node_index:ident, $position:ident$(; $super_lists:ident)?) => {
        {
            loop_while!($depth; $list, $skeleton; <= $target;
                $degree, $node_index, $position$(; $super_lists)?);
            if $position == $target {
                Some(pos!($list; $node_index, $position$(; $super_lists)?))
            } else {
                None
            }
        }
    };
    (node; $depth:tt; $list:ident, $skeleton:ident; >= $target:ident;
        $degree:ident, $node_index:ident, $position:ident$(; $super_lists:ident)?) => {
        {
            loop_while!($depth; $list, $skeleton; <= $target;
                $degree, $node_index, $position$(; $super_lists)?);
            if $position == $target {
                Some(pos!($list; $node_index, $position$(; $super_lists)?))
            } else {
                next!($list, $skeleton; $node_index, $position$(; $super_lists)?).unwrap();
                Some(pos!($list; $node_index, $position$(; $super_lists)?))
            }
        }
    };
    (node; $depth:tt; $list:ident, $skeleton:ident; > $target:ident;
        $degree:ident, $node_index:ident, $position:ident$(; $super_lists:ident)?) => {
        {
            loop_while!($depth; $list, $skeleton; <= $target;
                $degree, $node_index, $position$(; $super_lists)?);
            next!($list, $skeleton; $node_index, $position$(; $super_lists)?).unwrap();
            Some(pos!($list; $node_index, $position$(; $super_lists)?))
        }
    };
    ((range $bound:tt); $depth:tt; $list:ident, $skeleton:ident; < $target:ident;
        $degree:ident, $node_index:ident, $position:ident$(; $super_lists:ident)?) => {
        {
            loop_while!($depth; $list, $skeleton; < $target;
                $degree, $node_index, $position$(; $super_lists)?);
            if !index_is_at_bound!($bound; $node_index) {
                previous!($list, $skeleton; $node_index, $position$(; $super_lists)?).ok()?;
            }
            Some(pos!($list; $node_index, $position$(; $super_lists)?))
        }
    };
    ((range $bound:tt); $depth:tt; $list:ident, $skeleton:ident; <= $target:ident;
        $degree:ident, $node_index:ident, $position:ident$(; $super_lists:ident)?) => {
        {
            loop_while!($depth; $list, $skeleton; <= $target;
                $degree, $node_index, $position$(; $super_lists)?);
            if !index_is_at_bound!($bound; $node_index) {
                previous!($list, $skeleton; $node_index, $position$(; $super_lists)?).ok()?;
            }
            Some(pos!($list; $node_index, $position$(; $super_lists)?))
        }
    };
    ((range $bound:tt); $depth:tt; $list:ident, $skeleton:ident; == $target:ident;
        $degree:ident, $node_index:ident, $position:ident$(; $super_lists:ident)?) => {
        {
            loop_while!($depth; $list, $skeleton; <= $target;
                $degree, $node_index, $position$(; $super_lists)?);
            if $position == $target && index_is_at_bound!($bound; $node_index) {
                Some(pos!($list; $node_index, $position$(; $super_lists)?))
            } else {
                None
            }
        }
    };
    ((range $bound:tt); $depth:tt; $list:ident, $skeleton:ident; >= $target:ident;
        $degree:ident, $node_index:ident, $position:ident$(; $super_lists:ident)?) => {
        {
            loop_while!($depth; $list, $skeleton; <= $target;
                $degree, $node_index, $position$(; $super_lists)?);
            if $position == $target {
                if !index_is_at_bound!($bound; $node_index) {
                    next!($list, $skeleton; $node_index, $position$(; $super_lists)?).ok()?;
                }
                Some(pos!($list; $node_index, $position$(; $super_lists)?))
            } else {
                next!($list, $skeleton; $node_index, $position$(; $super_lists)?).unwrap();
                if !index_is_at_bound!($bound; $node_index) {
                    next!($list, $skeleton; $node_index, $position$(; $super_lists)?).ok()?;
                }
                Some(pos!($list; $node_index, $position$(; $super_lists)?))
            }
        }
    };
    ((range $bound:tt); $depth:tt; $list:ident, $skeleton:ident; > $target:ident;
        $degree:ident, $node_index:ident, $position:ident$(; $super_lists:ident)?) => {
        {
            loop_while!($depth; $list, $skeleton; <= $target;
                $degree, $node_index, $position$(; $super_lists)?);
            next!($list, $skeleton; $node_index, $position$(; $super_lists)?).unwrap();
            if !index_is_at_bound!($bound; $node_index) {
                next!($list, $skeleton; $node_index, $position$(; $super_lists)?).ok()?;
            }
            Some(pos!($list; $node_index, $position$(; $super_lists)?))
        }
    };
}

macro_rules! is_at_bound_if_range {
    (node; $_node_index:expr) => { true };
    ((range $bound:tt); $node_index:expr) => { index_is_at_bound!($bound; $node_index) };
}

macro_rules! traverse_unchecked {
    ($kind:tt; deep; $list:expr, $skeleton:ident; $cmp:tt $target:ident) => {
        {
            if $skeleton.link_size() == 0 {
                if $skeleton.offset() $cmp $target {
                    if is_at_bound_if_range!($kind; 0) {
                        Some(pos!($list; 0, $skeleton.offset(); vec![]))
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                let mut list = $list;
                // TODO Optimizations may be possible if it's possible to know more precisely how
                //  many super lists should be expected to be pushed to this vector
                let mut super_lists = vec![];
                let mut degree = $skeleton.depth() - 1;
                let mut node_index = 0;
                let mut position = $skeleton.offset();
                traverse_unchecked_with_variables!($kind; deep; list, $skeleton; $cmp $target;
                    degree, node_index, position; super_lists)
            }
        }
    };
    ($kind:tt; shallow; $list:expr, $skeleton:ident; $cmp:tt $target:ident) => {
        {
            if $skeleton.node_size() == 0 {
                if $skeleton.offset() $cmp $target {
                    if is_at_bound_if_range!($kind; 0) {
                        Some(pos!($list; 0, $skeleton.offset()))
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                let list = $list;
                let mut degree = $skeleton.depth() - 1;
                let mut node_index = 0;
                let mut position = $skeleton.offset();
                traverse_unchecked_with_variables!($kind; shallow; list, $skeleton; $cmp $target;
                    degree, node_index, position)
            }
        }
    }
}

macro_rules! traverse {
    ($kind:tt; $depth:tt; $list:expr; < $target:ident) => {
        {
            let mut skeleton = $list.skeleton();
            if $target <= skeleton.offset() {
                None
            } else {
                traverse_unchecked!($kind; $depth; $list, skeleton; < $target)
            }
        }
    };
    ($kind:tt; $depth:tt; $list:expr; <= $target:ident) => {
        {
            let mut skeleton = $list.skeleton();
            if $target < skeleton.offset() {
                None
            } else {
                traverse_unchecked!($kind; $depth; $list, skeleton; <= $target)
            }
        }
    };
    ($kind:tt; $depth:tt; $list:expr; == $target:ident) => {
        {
            let mut skeleton = $list.skeleton();
            if $target < skeleton.offset() {
                None
            } else {
                traverse_unchecked!($kind; $depth; $list, skeleton; == $target)
            }
        }
    };
    ($kind:tt; $depth:tt; $list:expr; >= $target:ident) => {
        {
            let mut skeleton = $list.skeleton();
            if $target > skeleton.last_position() {
                None
            } else if $target <= skeleton.offset() {
                // TODO make a macro for this if statement and use it below too
                if is_at_bound_if_range!($kind; 0) {
                    Some(pos!($list; 0, skeleton.offset(); vec![]))
                } else if skeleton.link_size() >= 1 {
                    let mut position = skeleton.offset();
                    if let Some(sublist) = skeleton.sublist_at(0) {
                        let sub_skeleton = sublist.skeleton();
                        position += sub_skeleton.offset();
                        Some(pos!(sublist; 0, position; vec![$list]))
                    } else {
                        position += skeleton.link_length_at(0);
                        Some(pos!($list; 1, position; vec![]))
                    }
                } else {
                    Some(Position::new(vec![], $list, 0, skeleton.offset()))
                }
            } else {
                traverse_unchecked!($kind; $depth; $list, skeleton; >= $target)
            }
        }
    };
    ($kind:tt; $depth:tt; $list:expr; > $target:ident) => {
        {
            let mut skeleton = $list.skeleton();
            if $target >= skeleton.last_position() {
                None
            } else if $target < skeleton.offset() {
                if is_at_bound_if_range!($kind; 0) {
                    Some(pos!($list; 0, skeleton.offset(); vec![]))
                } else if skeleton.link_size() >= 1 {
                    let mut position = skeleton.offset();
                    if let Some(sublist) = skeleton.sublist_at(0) {
                        let sub_skeleton = sublist.skeleton();
                        position += sub_skeleton.offset();
                        Some(pos!(sublist; 0, position; vec![$list]))
                    } else {
                        position += skeleton.link_length_at(0);
                        Some(pos!($list; 1, position; vec![]))
                    }
                } else {
                    Some(Position::new(vec![], $list, 0, skeleton.offset()))
                }
            } else {
                traverse_unchecked!($kind; $depth; $list, skeleton; > $target)
            }
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
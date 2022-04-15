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
    ($cmp:tt $target:ident; $list:ident;
        $degree:ident, $node_index:ident, $link_index:ident, $position:ident) => {
        let next_position = $position + $list.link_length_at($link_index);
        if next_position $cmp $target {
            $position = next_position;
            $node_index += 1 << $degree;
            maybe_stop!($cmp $target; $position);
        }
    }
}

macro_rules! descend {
    (deep; $list:ident; $cmp:tt $target:ident;
        $degree:ident, $node_index:ident, $position:ident; $super_lists:ident) => {
        if $degree == 0 {
            if let Some(sublist) = $list.sublist_at($node_index) {
                let next_position = $position + sublist.offset();
                if next_position $cmp $target {
                    $degree = sublist.depth().saturating_sub(1);
                    $node_index = 0;
                    $position = next_position;
                    $super_lists.push($list);
                    $list = sublist;
                    continue;
                }
            }
            break;
        } else {
            $degree -= 1;
        }
    };
    (shallow; $_list:ident; $_cmp:tt $_target:ident;
        $degree:ident, $_node_index:ident, $_position:ident) => {
        if $degree == 0 {
            break;
        } else {
            $degree -= 1;
        }
    };
}

macro_rules! loop_while {
    ($depth:tt; $list:ident; $cmp:tt $target:ident;
        $degree:ident, $node_index:ident, $position:ident$(; $super_lists:ident)?) => {
        loop {
            let link_index = link_index($node_index, $degree);
            if !$list.link_index_is_in_bounds(link_index) {
                if $degree == 0 {
                    break;
                }
                $degree -= 1;
                continue;
            }
            maybe_move_forwards!($cmp $target; $list;
                $degree, $node_index, link_index, $position);
            descend!($depth; $list; $cmp $target; $degree, $node_index, $position$(; $super_lists)?);
        }
    }
}

macro_rules! next {
    ($list:ident; $node_index:ident, $position:ident$(; $super_lists:ident)?) => {
        $(
        if let Some(sublist) = $list.sublist_at($node_index) {
            $node_index = 0;
            $position += sublist.offset();
            $super_lists.push($list);
            $list = sublist;
            Ok(())
        } else
        )?
        {
            match loop {
                if $node_index < $list.link_size() {
                    break Ok(())
                }
                $(if let Some(new_index) = $list.index_in_super_list() {
                    $node_index = new_index;
                    $position -= $list.last_position();
                    $list = $super_lists.pop().unwrap();
                    continue;
                })?
                break Err("Tried to move to next node but it's already the end of the list");
            } {
                Ok(()) => {
                    $position += $list.link_length_at_node($node_index);
                    $node_index += 1;

                    Ok(())
                }
                err => {
                    err
                }
            }
        }
    };
}

macro_rules! previous {
    ($list:ident; $node_index:ident, $position:ident$(; $super_lists:ident)?) => {
        'previous: {
            $(
            if $node_index > 0 {
                let index_before = $node_index - 1;
                if let Some(sublist) = $list.sublist_at(index_before) {
                    $node_index = sublist.node_size() - 1;
                    $position -=
                        $list.link_length_at_node(index_before)
                            - sublist.last_position();
                    $super_lists.push($list);
                    $list = sublist;
                    break 'previous Ok(());
                }
            }
            )?

            if $node_index == 0 {
                $(if let Some(new_index) = $list.index_in_super_list() {
                    $node_index = new_index; // + 1 maybe?
                    $position -= $list.offset();
                    $list = $super_lists.pop().unwrap();
                    break 'previous Ok(());
                })?
                break 'previous
                    Err("Tried to move to previous node but it's already the start of the list");
            }

            $node_index -= 1;
            $position -= $list.link_length_at_node($node_index);

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

macro_rules! is_at_bound_if_range {
    (node; $_node_index:expr) => { true };
    ((range $bound:tt); $node_index:expr) => { index_is_at_bound!($bound; $node_index) };
}

macro_rules! traverse_unchecked_with_variables {
    (node; $depth:tt; $list:ident; < $target:ident;
        $degree:ident, $node_index:ident, $position:ident$(; $super_lists:ident)?) => {
        {
            loop_while!($depth; $list; < $target;
                $degree, $node_index, $position$(; $super_lists)?);
            Some(pos!($list; $node_index, $position$(; $super_lists)?))
        }
    };
    (node; $depth:tt; $list:ident; <= $target:ident;
        $degree:ident, $node_index:ident, $position:ident$(; $super_lists:ident)?) => {
        {
            loop_while!($depth; $list; <= $target;
                $degree, $node_index, $position$(; $super_lists)?);
            Some(pos!($list; $node_index, $position$(; $super_lists)?))
        }
    };
    (node; $depth:tt; $list:ident; == $target:ident;
        $degree:ident, $node_index:ident, $position:ident$(; $super_lists:ident)?) => {
        {
            loop_while!($depth; $list; <= $target;
                $degree, $node_index, $position$(; $super_lists)?);
            if $position == $target {
                Some(pos!($list; $node_index, $position$(; $super_lists)?))
            } else {
                None
            }
        }
    };
    (node; $depth:tt; $list:ident; >= $target:ident;
        $degree:ident, $node_index:ident, $position:ident$(; $super_lists:ident)?) => {
        {
            loop_while!($depth; $list; <= $target;
                $degree, $node_index, $position$(; $super_lists)?);
            if $position == $target {
                Some(pos!($list; $node_index, $position$(; $super_lists)?))
            } else {
                next!($list; $node_index, $position$(; $super_lists)?).unwrap();
                Some(pos!($list; $node_index, $position$(; $super_lists)?))
            }
        }
    };
    (node; $depth:tt; $list:ident; > $target:ident;
        $degree:ident, $node_index:ident, $position:ident$(; $super_lists:ident)?) => {
        {
            loop_while!($depth; $list; <= $target;
                $degree, $node_index, $position$(; $super_lists)?);
            next!($list; $node_index, $position$(; $super_lists)?).unwrap();
            Some(pos!($list; $node_index, $position$(; $super_lists)?))
        }
    };
    ((range $bound:tt); $depth:tt; $list:ident; < $target:ident;
        $degree:ident, $node_index:ident, $position:ident$(; $super_lists:ident)?) => {
        {
            loop_while!($depth; $list; < $target;
                $degree, $node_index, $position$(; $super_lists)?);
            if !index_is_at_bound!($bound; $node_index) {
                previous!($list; $node_index, $position$(; $super_lists)?).ok()?;
            }
            Some(pos!($list; $node_index, $position$(; $super_lists)?))
        }
    };
    ((range $bound:tt); $depth:tt; $list:ident; <= $target:ident;
        $degree:ident, $node_index:ident, $position:ident$(; $super_lists:ident)?) => {
        {
            loop_while!($depth; $list; <= $target;
                $degree, $node_index, $position$(; $super_lists)?);
            if !index_is_at_bound!($bound; $node_index) {
                previous!($list; $node_index, $position$(; $super_lists)?).ok()?;
            }
            Some(pos!($list; $node_index, $position$(; $super_lists)?))
        }
    };
    ((range $bound:tt); $depth:tt; $list:ident; == $target:ident;
        $degree:ident, $node_index:ident, $position:ident$(; $super_lists:ident)?) => {
        {
            loop_while!($depth; $list; <= $target;
                $degree, $node_index, $position$(; $super_lists)?);
            if $position == $target && index_is_at_bound!($bound; $node_index) {
                Some(pos!($list; $node_index, $position$(; $super_lists)?))
            } else {
                None
            }
        }
    };
    ((range $bound:tt); $depth:tt; $list:ident; >= $target:ident;
        $degree:ident, $node_index:ident, $position:ident$(; $super_lists:ident)?) => {
        {
            loop_while!($depth; $list; <= $target;
                $degree, $node_index, $position$(; $super_lists)?);
            if $position == $target {
                if !index_is_at_bound!($bound; $node_index) {
                    next!($list; $node_index, $position$(; $super_lists)?).ok()?;
                }
                Some(pos!($list; $node_index, $position$(; $super_lists)?))
            } else {
                next!($list; $node_index, $position$(; $super_lists)?).unwrap();
                if !index_is_at_bound!($bound; $node_index) {
                    next!($list; $node_index, $position$(; $super_lists)?).ok()?;
                }
                Some(pos!($list; $node_index, $position$(; $super_lists)?))
            }
        }
    };
    ((range $bound:tt); $depth:tt; $list:ident; > $target:ident;
        $degree:ident, $node_index:ident, $position:ident$(; $super_lists:ident)?) => {
        {
            loop_while!($depth; $list; <= $target;
                $degree, $node_index, $position$(; $super_lists)?);
            next!($list; $node_index, $position$(; $super_lists)?).unwrap();
            if !index_is_at_bound!($bound; $node_index) {
                next!($list; $node_index, $position$(; $super_lists)?).ok()?;
            }
            Some(pos!($list; $node_index, $position$(; $super_lists)?))
        }
    };
}

macro_rules! traverse_unchecked {
    ($kind:tt; deep; $list:expr; $cmp:tt $target:ident) => {
        if $list.link_size() == 0 {
            if $list.offset() $cmp $target {
                if is_at_bound_if_range!($kind; 0) {
                    Some(pos!($list; 0, $list.offset(); vec![]))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            let mut list = $list;
            // TODO Optimizations may be possible if it's possible to know how many super lists
            //  should be expected to be pushed to this vector
            let mut super_lists = vec![];
            let mut degree = $list.depth() - 1;
            let mut node_index = 0;
            let mut position = $list.offset();
            traverse_unchecked_with_variables!($kind; deep; list; $cmp $target;
                degree, node_index, position; super_lists)
        }
    };
    ($kind:tt; shallow; $list:expr; $cmp:tt $target:ident) => {
        if $list.node_size() == 0 {
            if $list.offset() $cmp $target {
                if is_at_bound_if_range!($kind; 0) {
                    Some(pos!($list; 0, $list.offset()))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            let list = $list;
            let mut degree = $list.depth() - 1;
            let mut node_index = 0;
            let mut position = $list.offset();
            traverse_unchecked_with_variables!($kind; shallow; list; $cmp $target;
                degree, node_index, position)
        }
    }
}

macro_rules! handle_before_offset {
    ($kind:tt; deep; $list:expr) => {
        if is_at_bound_if_range!($kind; 0) {
            Some(pos!($list; 0, $list.offset(); vec![]))
        } else if $list.link_size() >= 1 {
            let mut position = $list.offset();
            if let Some(sublist) = $list.sublist_at(0) {
                position += sublist.offset();
                Some(pos!(sublist; 0, position; vec![$list]))
            } else {
                position += $list.link_length_at(0);
                Some(pos!($list; 1, position; vec![]))
            }
        } else {
            Some(Position::new(vec![], $list, 0, $list.offset()))
        }
    };
    ($kind:tt; shallow; $list:expr) => {
        if is_at_bound_if_range!($kind; 0) {
            Some(pos!($list; 0, $list.offset(); vec![]))
        } else if $list.link_size() >= 1 {
            Some(pos!($list; 1, $list.offset() + $list.link_length_at(0); vec![]))
        } else {
            Some(Position::new(vec![], $list, 0, $list.offset()))
        }
    };
}

macro_rules! traverse {
    ($kind:tt; $depth:tt; $list:expr; < $target:ident) => {
        if $target <= $list.offset() {
            None
        } else {
            traverse_unchecked!($kind; $depth; $list; < $target)
        }
    };
    ($kind:tt; $depth:tt; $list:expr; <= $target:ident) => {
        if $target < $list.offset() {
            None
        } else {
            traverse_unchecked!($kind; $depth; $list; <= $target)
        }
    };
    ($kind:tt; $depth:tt; $list:expr; == $target:ident) => {
        if $target < $list.offset() {
            None
        } else {
            traverse_unchecked!($kind; $depth; $list; == $target)
        }
    };
    ($kind:tt; $depth:tt; $list:expr; >= $target:ident) => {
        if $target > $list.last_position() {
            None
        } else if $target <= $list.offset() {
            handle_before_offset!($kind; $depth; $list)
        } else {
            traverse_unchecked!($kind; $depth; $list; >= $target)
        }
    };
    ($kind:tt; $depth:tt; $list:expr; > $target:ident) => {
        if $target >= $list.last_position() {
            None
        } else if $target < $list.offset() {
            handle_before_offset!($kind; $depth; $list)
        } else {
            traverse_unchecked!($kind; $depth; $list; > $target)
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
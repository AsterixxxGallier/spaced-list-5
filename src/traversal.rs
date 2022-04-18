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

#[inline(always)]
pub(crate) const fn link_index(index: usize, degree: usize) -> usize {
    index + (1 << degree) - 1
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
        $degree:ident, $index:ident, $link_index:ident, $position:ident) => {
        let next_position = $position + $list.link_length_at($link_index);
        if next_position $cmp $target {
            $position = next_position;
            $index += 1 << $degree;
            maybe_stop!($cmp $target; $position);
        }
    }
}

macro_rules! descend {
    (deep; $list:ident; $cmp:tt $target:ident;
        $degree:ident, $index:ident, $position:ident; $super_lists:ident) => {
        if $degree == 0 {
            if let Some(sublist) = $list.sublist_at($index) {
                let next_position = $position + sublist.offset();
                if next_position $cmp $target {
                    $degree = sublist.depth().saturating_sub(1);
                    $index = 0;
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
        $degree:ident, $_index:ident, $_position:ident) => {
        if $degree == 0 {
            break;
        } else {
            $degree -= 1;
        }
    };
}

macro_rules! loop_while {
    ($depth:tt; $list:ident; $cmp:tt $target:ident;
        $degree:ident, $index:ident, $position:ident$(; $super_lists:ident)?) => {
        loop {
            let link_index = link_index($index, $degree);
            if !$list.link_index_is_in_bounds(link_index) {
                if $degree == 0 {
                    break;
                }
                $degree -= 1;
                continue;
            }
            maybe_move_forwards!($cmp $target; $list;
                $degree, $index, link_index, $position);
            descend!($depth; $list; $cmp $target; $degree, $index, $position$(; $super_lists)?);
        }
    }
}

macro_rules! next {
    ($list:ident; $index:ident, $position:ident$(; $super_lists:ident)?) => {
        if $index == $list.link_size() {
            $(if !$super_lists.is_empty() {
                let super_list = $super_lists.pop().unwrap();
                let index_in_super_list = $list.index_in_super_list().unwrap();
                $position -= $list.last_position();
                $list = super_list;
                $position += $list.link_length_at_node(index_in_super_list);
                $index = index_in_super_list + 1;
                Ok(())
            } else )?{
                Err("Tried to move to next node but it's already the end of the list")
            }
        } else {
            $(if let Some(sublist) = $list.sublist_at($index) {
                $super_lists.push($list);
                $list = sublist;
                $index = 0;
                $position += sublist.offset();
                Ok(())
            } else )?{
                $position += $list.link_length_at_node($index);
                $index += 1;
                Ok(())
            }
        }
    };
}

macro_rules! previous {
    ($list:ident; $index:ident, $position:ident$(; $super_lists:ident)?) => {
        if $index == 0 {
            $(if let Some(new_index) = $list.index_in_super_list() {
                $index = new_index;
                $position -= $list.offset();
                $list = $super_lists.pop().unwrap();
                Ok(())
            } else )? {
                Err("Tried to move to previous node but it's already the start of the list")
            }
        }
        $(
        else {
            let index_before = $index - 1;
            if let Some(sublist) = $list.sublist_at(index_before) {
                $index = sublist.node_size() - 1;
                $position -=
                    $list.link_length_at_node(index_before)
                        - sublist.last_position();
                $super_lists.push($list);
                $list = sublist;
                Ok(())
            } else {
                $index -= 1;
                $position -= $list.link_length_at_node($index);
                Ok(())
            }
        }
        )?
    };
}

macro_rules! pos {
    ($list:expr; $index:expr, $position:expr; $super_lists:expr) => {
        Position::new($super_lists, $list, $index, $position)
    };
    ($_list:expr; $index:expr, $position:expr) => {
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
    (node; $depth:tt; $list:ident; < $target:ident;
        $degree:ident, $index:ident, $position:ident$(; $super_lists:ident)?) => {
        {
            loop_while!($depth; $list; < $target;
                $degree, $index, $position$(; $super_lists)?);
            Some(pos!($list; $index, $position$(; $super_lists)?))
        }
    };
    (node; $depth:tt; $list:ident; <= $target:ident;
        $degree:ident, $index:ident, $position:ident$(; $super_lists:ident)?) => {
        {
            loop_while!($depth; $list; <= $target;
                $degree, $index, $position$(; $super_lists)?);
            Some(pos!($list; $index, $position$(; $super_lists)?))
        }
    };
    (node; $depth:tt; $list:ident; == $target:ident;
        $degree:ident, $index:ident, $position:ident$(; $super_lists:ident)?) => {
        {
            loop_while!($depth; $list; <= $target;
                $degree, $index, $position$(; $super_lists)?);
            if $position == $target {
                Some(pos!($list; $index, $position$(; $super_lists)?))
            } else {
                None
            }
        }
    };
    (node; $depth:tt; $list:ident; >= $target:ident;
        $degree:ident, $index:ident, $position:ident$(; $super_lists:ident)?) => {
        {
            loop_while!($depth; $list; <= $target;
                $degree, $index, $position$(; $super_lists)?);
            if $position == $target {
                Some(pos!($list; $index, $position$(; $super_lists)?))
            } else {
                next!($list; $index, $position$(; $super_lists)?).unwrap();
                Some(pos!($list; $index, $position$(; $super_lists)?))
            }
        }
    };
    (node; $depth:tt; $list:ident; > $target:ident;
        $degree:ident, $index:ident, $position:ident$(; $super_lists:ident)?) => {
        {
            loop_while!($depth; $list; <= $target;
                $degree, $index, $position$(; $super_lists)?);
            next!($list; $index, $position$(; $super_lists)?).unwrap();
            Some(pos!($list; $index, $position$(; $super_lists)?))
        }
    };
    ((range $bound:tt); $depth:tt; $list:ident; < $target:ident;
        $degree:ident, $index:ident, $position:ident$(; $super_lists:ident)?) => {
        {
            loop_while!($depth; $list; < $target;
                $degree, $index, $position$(; $super_lists)?);
            if !index_is_at_bound!($bound; $index) {
                previous!($list; $index, $position$(; $super_lists)?).ok()?;
            }
            Some(pos!($list; $index, $position$(; $super_lists)?))
        }
    };
    ((range $bound:tt); $depth:tt; $list:ident; <= $target:ident;
        $degree:ident, $index:ident, $position:ident$(; $super_lists:ident)?) => {
        {
            loop_while!($depth; $list; <= $target;
                $degree, $index, $position$(; $super_lists)?);
            if !index_is_at_bound!($bound; $index) {
                previous!($list; $index, $position$(; $super_lists)?).ok()?;
            }
            Some(pos!($list; $index, $position$(; $super_lists)?))
        }
    };
    ((range $bound:tt); $depth:tt; $list:ident; == $target:ident;
        $degree:ident, $index:ident, $position:ident$(; $super_lists:ident)?) => {
        {
            loop_while!($depth; $list; <= $target;
                $degree, $index, $position$(; $super_lists)?);
            if $position == $target && index_is_at_bound!($bound; $index) {
                Some(pos!($list; $index, $position$(; $super_lists)?))
            } else {
                None
            }
        }
    };
    ((range $bound:tt); $depth:tt; $list:ident; >= $target:ident;
        $degree:ident, $index:ident, $position:ident$(; $super_lists:ident)?) => {
        {
            loop_while!($depth; $list; <= $target;
                $degree, $index, $position$(; $super_lists)?);
            if $position == $target {
                if !index_is_at_bound!($bound; $index) {
                    next!($list; $index, $position$(; $super_lists)?).ok()?;
                }
                Some(pos!($list; $index, $position$(; $super_lists)?))
            } else {
                next!($list; $index, $position$(; $super_lists)?).unwrap();
                if !index_is_at_bound!($bound; $index) {
                    next!($list; $index, $position$(; $super_lists)?).ok()?;
                }
                Some(pos!($list; $index, $position$(; $super_lists)?))
            }
        }
    };
    ((range $bound:tt); $depth:tt; $list:ident; > $target:ident;
        $degree:ident, $index:ident, $position:ident$(; $super_lists:ident)?) => {
        {
            loop_while!($depth; $list; <= $target;
                $degree, $index, $position$(; $super_lists)?);
            next!($list; $index, $position$(; $super_lists)?).unwrap();
            if !index_is_at_bound!($bound; $index) {
                next!($list; $index, $position$(; $super_lists)?).ok()?;
            }
            Some(pos!($list; $index, $position$(; $super_lists)?))
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
            // Optimizations may be possible if it's possible to know how many super lists
            // should be expected to be pushed to this vector (maybe, use SmallVec to avoid heap
            // allocations for the case where only a couple levels of sublists are traversed into)
            let mut super_lists = vec![];
            let mut degree = $list.depth() - 1;
            let mut index = 0;
            let mut position = $list.offset();
            traverse_unchecked_with_variables!($kind; deep; list; $cmp $target;
                degree, index, position; super_lists)
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
            let mut index = 0;
            let mut position = $list.offset();
            traverse_unchecked_with_variables!($kind; shallow; list; $cmp $target;
                degree, index, position)
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
// ╭───────────────────────────────────────────────────────────────╮
// ├───────────────────────────────╮                               │
// ├───────────────╮               ├───────────────╮               │
// ├───────╮       ├───────╮       ├───────╮       ├───────╮       │
// ├───╮   ├───╮   ├───╮   ├───╮   ├───╮   ├───╮   ├───╮   ├───╮   │
// ╵ 0 ╵ 1 ╵ 0 ╵ 2 ╵ 0 ╵ 1 ╵ 0 ╵ 3 ╵ 0 ╵ 1 ╵ 0 ╵ 2 ╵ 0 ╵ 1 ╵ 0 ╵ 4 ╵
// 00000   00010   00100   00110   01000   01010   01100   01110   10000
//     00001   00011   00101   00111   01001   01011   01101   01111
//
// backwards structure, does not make a lot of sense unfortunately:
// ╭───────────────────────────────────────────────────────────────╮
// │                               ╭───────────────────────────────┤
// │               ╭───────────────┤               ╭───────────────┤
// │       ╭───────┤       ╭───────┤       ╭───────┤       ╭───────┤
// │   ╭───┤   ╭───┤   ╭───┤   ╭───┤   ╭───┤   ╭───┤   ╭───┤   ╭───┤
// ╵ 0 ╵ 1 ╵ 0 ╵ 2 ╵ 0 ╵ 1 ╵ 0 ╵ 3 ╵ 0 ╵ 1 ╵ 0 ╵ 2 ╵ 0 ╵ 1 ╵ 0 ╵ 4 ╵
// 00000   00010   00100   00110   01000   01010   01100   01110   10000
//     00001   00011   00101   00111   01001   01011   01101   01111

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
    ($cmp:tt $target:ident; $skeleton:ident, $link_index:ident,
        $degree:ident, $node_index:ident, $position:ident) => {
        let next_position = $position + $skeleton.link_length_at($link_index);
        if next_position $cmp $target {
            $position = next_position;
            $node_index += 1 << $degree;
            maybe_stop!($cmp $target; $position);
        }
    }
}

macro_rules! descend {
    (deep; $cmp:tt $target:ident, $skeleton:ident,
        $list:ident, $super_lists:ident, $degree:ident, $node_index:ident, $position:ident) => {
        if $degree == 0 {
            if $skeleton.sublist_index_is_in_bounds($node_index) {
                if let Some(sublist) = $skeleton.sublist_at($node_index) {
                    let sub_skeleton = sublist.skeleton();
                    let next_position = $position + sub_skeleton.offset();
                    if next_position $cmp $target {
                        $degree = sub_skeleton.depth().saturating_sub(1);
                        $node_index = 0;
                        $position = next_position;
                        $super_lists.push($list);
                        $list = sublist;
                        continue;
                    }
                }
            }
            break;
        } else {
            $degree -= 1;
        }
    };
    (shallow; $_cmp:tt $_target:ident, $skeleton:ident,
        $list:ident, $degree:ident, $node_index:ident, $position:ident) => {
        if $degree == 0 {
            break;
        } else {
            $degree -= 1;
        }
    };
}

macro_rules! loop_while {
    ($depth:tt; $cmp:tt $target:ident;
        $list:ident, $degree:ident, $node_index:ident, $position:ident$(, $super_lists:ident)?) => {
        loop {
            let skeleton = $list.skeleton();
            let link_index = link_index($node_index, $degree);
            if !skeleton.link_index_is_in_bounds(link_index) {
                if $degree == 0 {
                    break;
                }
                $degree -= 1;
                continue;
            }
            maybe_move_forwards!($cmp $target; skeleton, link_index,
                $degree, $node_index, $position);
            descend!($depth; $cmp $target, skeleton, $list, $($super_lists, )?$degree, $node_index, $position);
        }
    }
}

macro_rules! next {
    ($skeleton:ident, $list:ident, $node_index:ident, $position:ident$(, $super_lists:ident)?) => {
        'next: {
            $(
            let skeleton = $list.skeleton();
            if skeleton.sublist_index_is_in_bounds($node_index) {
                if let Some(sublist) = skeleton.sublist_at($node_index) {
                    let sub_skeleton = sublist.skeleton();
                    $node_index = 0;
                    $position += sub_skeleton.offset();
                    $super_lists.push($list);
                    $list = sublist;
                    break 'next;
                }
            }
            )?

            loop {
                let skeleton = $list.skeleton();
                if $node_index < skeleton.link_size() {
                    break;
                }
                $(if let Some(new_index) = skeleton.index_in_super_list() {
                    $node_index = new_index;
                    $position -= skeleton.length() + skeleton.offset();
                    $list = $super_lists.pop().unwrap();
                    continue
                })?
                panic!("Tried to move to next node but it's already the end of the list")
            }

            let skeleton = $list.skeleton();
            let mut degree = 0;
            loop {
                if degree < $node_index.trailing_zeros() as usize {
                    break;
                }
                $position -= skeleton.link_length_at($node_index - 1);
                $node_index -= 1 << degree;
                degree += 1;
            }

            $node_index += 1 << degree;
            $position += skeleton.link_length_at($node_index - 1);
        }
    };
}

macro_rules! pos {
    ($list:expr, $node_index:expr, $position:expr, $super_lists:expr) => {
        Position::new($super_lists, $list, $node_index, $position)
    };
    ($_list:expr, $node_index:expr, $position:expr) => {
        ShallowPosition::new($node_index, $position)
    }
}

macro_rules! traverse_unchecked_with_variables {
    ($depth:tt; < $target:ident;
        $list:ident, $degree:ident, $node_index:ident, $position:ident$(, $super_lists:ident)?) => {
        {
            loop_while!($depth; < $target;
                $list, $degree, $node_index, $position$(, $super_lists)?);
            Some(pos!($list, $node_index, $position$(, $super_lists)?))
        }
    };
    ($depth:tt; <= $target:ident;
        $list:ident, $degree:ident, $node_index:ident, $position:ident$(, $super_lists:ident)?) => {
        {
            loop_while!($depth; <= $target;
                $list, $degree, $node_index, $position$(, $super_lists)?);
            Some(pos!($list, $node_index, $position$(, $super_lists)?))
        }
    };
    ($depth:tt; == $target:ident;
        $list:ident, $degree:ident, $node_index:ident, $position:ident$(, $super_lists:ident)?) => {
        {
            loop_while!($depth; <= $target;
                $list, $degree, $node_index, $position$(, $super_lists)?);
            if $position == $target {
                Some(pos!($list, $node_index, $position$(, $super_lists)?))
            } else {
                None
            }
        }
    };
    ($depth:tt; >= $target:ident;
        $list:ident, $degree:ident, $node_index:ident, $position:ident$(, $super_lists:ident)?) => {
        {
            loop_while!($depth; <= $target;
                $list, $degree, $node_index, $position$(, $super_lists)?);
            if $position == $target {
                Some(pos!($list, $node_index, $position$(, $super_lists)?))
            } else {
                let skeleton = $list.skeleton();
                next!(skeleton, $list, $node_index, $position$(, $super_lists)?);
                Some(pos!($list, $node_index, $position$(, $super_lists)?))
            }
        }
    };
    ($depth:tt; > $target:ident;
        $list:ident, $degree:ident, $node_index:ident, $position:ident$(, $super_lists:ident)?) => {
        {
            loop_while!($depth; <= $target;
                $list, $degree, $node_index, $position$(, $super_lists)?);
            let skeleton = $list.skeleton();
            next!(skeleton, $list, $node_index, $position$(, $super_lists)?);
            Some(pos!($list, $node_index, $position$(, $super_lists)?))
        }
    }
}

macro_rules! traverse_unchecked {
    (deep; $list:expr; $cmp:tt $target:ident) => {
        {
            let skeleton = $list.skeleton();
            if skeleton.link_size() == 0 {
                if skeleton.offset() $cmp $target {
                    Some(pos!($list, 0, skeleton.offset(), vec![]))
                } else {
                    None
                }
            } else {
                let mut list = $list;
                let mut super_lists = vec![];
                let mut degree = skeleton.depth() - 1;
                let mut node_index = 0;
                let mut position = skeleton.offset();
                traverse_unchecked_with_variables!(deep; $cmp $target;
                    list, degree, node_index, position, super_lists)
            }
        }
    };
    (shallow; $list:expr; $cmp:tt $target:ident) => {
        {
            let skeleton = $list.skeleton();
            if skeleton.node_size() == 0 {
                if skeleton.offset() $cmp $target {
                    Some(pos!($list, 0, skeleton.offset()))
                } else {
                    None
                }
            } else {
                let list = $list;
                let mut degree = skeleton.depth() - 1;
                let mut node_index = 0;
                let mut position = skeleton.offset();
                traverse_unchecked_with_variables!(shallow; $cmp $target;
                    list, degree, node_index, position)
            }
        }
    }
}

macro_rules! traverse {
    ($depth:tt; $list:expr; < $target:ident) => {
        if $target <= $list.skeleton().offset() {
            None
        } else {
            traverse_unchecked!($depth; $list; < $target)
        }
    };
    ($depth:tt; $list:expr; <= $target:ident) => {
        if $target < $list.skeleton().offset() {
            None
        } else {
            traverse_unchecked!($depth; $list; <= $target)
        }
    };
    ($depth:tt; $list:expr; == $target:ident) => {
        if $target < $list.skeleton().offset() {
            None
        } else {
            traverse_unchecked!($depth; $list; == $target)
        }
    };
    ($depth:tt; $list:expr; >= $target:ident) => {
        if $target > $list.skeleton().length() + $list.skeleton().offset() {
            None
        } else if $target <= $list.skeleton().offset() {
            Some(Position::new(vec![], $list, 0, $list.skeleton().offset()))
        } else {
            traverse_unchecked!($depth; $list; >= $target)
        }
    };
    ($depth:tt; $list:expr; > $target:ident) => {
        if $target >= $list.skeleton().length() + $list.skeleton().offset() {
            None
        } else if $target < $list.skeleton().offset() {
            Some(Position::new(vec![], $list, 0, $list.skeleton().offset()))
        } else {
            traverse_unchecked!($depth; $list; > $target)
        }
    }
}

// TODO make skeleton a mutable variable

pub(crate) use traverse;
pub(crate) use traverse_unchecked;
pub(crate) use traverse_unchecked_with_variables;
pub(crate) use loop_while;
pub(crate) use maybe_stop;
pub(crate) use maybe_move_forwards;
pub(crate) use next;
pub(crate) use descend;
pub(crate) use pos;
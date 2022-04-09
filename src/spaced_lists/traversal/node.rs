macro_rules! maybe_move_forwards {
    (<= $target:expr; $skeleton:expr, $link_index:expr,
        $list:ident, $super_lists:ident, $degree:ident, $node_index:ident, $position:ident) => {
        let next_position = $position + $skeleton.get_link_length_at($link_index);
        if next_position <= $target {
            $position = next_position;
            $node_index += 1 << $degree;
            if $position == $target {
                if $skeleton.sublist_index_is_in_bounds($node_index) {
                    // TODO don't just descend into sublists like that when you have offsets
                    while let Some(sublist) = $skeleton.get_sublist_at($node_index) {
                        $node_index = 0;
                        $super_lists.push($list);
                        $list = sublist;
                    }
                }
                break;
            }
        }
    };
    (< $target:expr; $skeleton:expr, $link_index:expr,
        $list:ident, $super_lists:ident, $degree:ident, $node_index:ident, $position:ident) => {
        let next_position = $position + $skeleton.get_link_length_at($link_index);
        if next_position < $target {
            $position = next_position;
            $node_index += 1 << $degree;
        }
    }
}

macro_rules! descend {
    ($skeleton:expr, $list:ident, $super_lists:ident, $degree:ident, $node_index:ident, $position:ident) => {
        if $degree == 0 {
            if $skeleton.sublist_index_is_in_bounds($node_index) {
                if let Some(sublist) = $skeleton.get_sublist_at($node_index) {
                    // TODO check too that position + sublist.offset < target
                    let sub_skeleton = sublist.skeleton();
                    $degree = sub_skeleton.depth() - 1;
                    $node_index = 0;
                    $super_lists.push($list);
                    $list = sublist;
                } else {
                    break;
                }
            } else {
                break;
            }
        } else {
            $degree -= 1;
        }
    };
}

macro_rules! loop_while {
    ($cmp:tt $target:expr;
        $list:ident, $super_lists:ident, $degree:ident, $node_index:ident, $position:ident) => {
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
            maybe_move_forwards!($cmp $target; skeleton, link_index, $list, $super_lists, $degree,
                $node_index, $position);
            descend!(skeleton, $list, $super_lists, $degree, $node_index, $position);
        }
    }
}

macro_rules! next {
    ($skeleton:expr, $list:ident, $super_lists:ident, $node_index:ident, $position:ident) => {
        {
            while $node_index == $list.skeleton().size() {
                if let Some(node_index) = $skeleton.index_in_super_list() {
                    $node_index = node_index;
                    $position -= $skeleton.length();
                    $list = $super_lists.pop().unwrap();
                } else {
                    panic!("Tried to move to next node but it's already the end of the list")
                };
            }

            let skeleton = $skeleton;
            let mut degree = 0;
            loop {
                if degree < $node_index.trailing_zeros() as usize {
                    break;
                }
                $position -= skeleton.get_link_length_at($node_index - 1);
                $node_index -= 1 << degree;
                degree += 1;
            }

            $node_index += 1 << degree;
            $position += skeleton.get_link_length_at($node_index - 1);
        }
    };
}

macro_rules! traverse_unchecked_with_variables {
    (< $target:expr;
        $list:ident, $super_lists:ident, $degree:ident, $node_index:ident, $position:ident) => {
        {
            loop_while!(< $target; $list, $super_lists, $degree, $node_index, $position);
            Some(Position::new($super_lists, $list, $node_index, $position))
        }
    };
    (<= $target:expr;
        $list:ident, $super_lists:ident, $degree:ident, $node_index:ident, $position:ident) => {
        {
            loop_while!(<= $target; $list, $super_lists, $degree, $node_index, $position);
            Some(Position::new($super_lists, $list, $node_index, $position))
        }
    };
    (== $target:expr;
        $list:ident, $super_lists:ident, $degree:ident, $node_index:ident, $position:ident) => {
        {
            loop_while!(<= $target; $list, $super_lists, $degree, $node_index, $position);
            if $position == $target {
                Some(Position::new($super_lists, $list, $node_index, $position))
            } else {
                None
            }
        }
    };
    (>= $target:expr;
        $list:ident, $super_lists:ident, $degree:ident, $node_index:ident, $position:ident) => {
        {
            loop_while!(<= $target; $list, $super_lists, $degree, $node_index, $position);
            if $position == $target {
                Some(Position::new($super_lists, $list, $node_index, $position))
            } else {
                next!($list.skeleton(), $list, $super_lists, $node_index, $position);
                Some(Position::new($super_lists, $list, $node_index, $position))
            }
        }
    };
    (> $target:expr;
        $list:ident, $super_lists:ident, $degree:ident, $node_index:ident, $position:ident) => {
        {
            loop_while!(<= $target; $list, $super_lists, $degree, $node_index, $position);
            next!($list.skeleton(), $list, $super_lists, $node_index, $position);
            Some(Position::new($super_lists, $list, $node_index, $position))
        }
    }
}

macro_rules! traverse_unchecked {
    ($list:expr; $cmp:tt $target:expr) => {
        {
            let mut list = $list;
            let mut super_lists = vec![];
            let mut degree = list.skeleton().depth() - 1;
            let mut node_index = 0;
            // TODO start at offset
            let mut position = zero();
            traverse_unchecked_with_variables!($cmp $target;
                list, super_lists, degree, node_index, position)
        }
    }
}

macro_rules! traverse {
    ($list:expr; < $target:expr) => {
        // TODO check if it's smaller than or equal to offset instead
        if $target <= zero() {
            None
        } else {
            traverse_unchecked!($list; < $target)
        }
    };
    ($list:expr; <= $target:expr) => {
        // TODO check if it's smaller than offset instead
        if $target < zero() {
            None
        } else {
            traverse_unchecked!($list; <= $target)
        }
    };
    ($list:expr; == $target:expr) => {
        // TODO check if it's smaller than offset instead
        if $target < zero() {
            None
        } else {
            traverse_unchecked!($list; == $target)
        }
    };
    ($list:expr; >= $target:expr) => {
        // TODO check if it's larger than offset + length instead
        if $target > $list.skeleton().length() {
            None
            // TODO replace zero() with offset
        } else if $target <= zero() {
            Some(Position::new(vec![], $list, 0, zero()))
        } else {
            traverse_unchecked!($list; >= $target)
        }
    };
    ($list:expr; > $target:expr) => {
        // TODO check if it's larger than or equal to offset + length instead
        if $target >= $list.skeleton().length() {
            None
            // TODO replace zero() with offset
        } else if $target < zero() {
            Some(Position::new(vec![], $list, 0, zero()))
        } else {
            traverse_unchecked!($list; > $target)
        }
    }
}

pub(crate) use traverse;
pub(crate) use traverse_unchecked;
pub(crate) use traverse_unchecked_with_variables;
pub(crate) use loop_while;
pub(crate) use maybe_move_forwards;
pub(crate) use next;
pub(crate) use descend;
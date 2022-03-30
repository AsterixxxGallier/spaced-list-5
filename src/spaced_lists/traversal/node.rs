use num_traits::zero;

use crate::{SpacedList, Spacing};
use crate::spaced_lists::traversal::Position;

pub struct Traversal<'a, S, List, Continue, Stop>
    where S: 'a + Spacing,
          List: SpacedList<S>,
          Continue: Fn(S) -> bool,
          Stop: Fn(S) -> bool {
    lists: Vec<&'a List>,
    continue_condition: Continue,
    stop_condition: Option<Stop>,
    local_offset: usize,
    local_mask: usize,
    degree: usize,
    node_index: usize,
    link_index: usize,
    position: S,
}

const fn mask(size: usize) -> usize {
    !(!0 << size)
}

impl<'a, S, List, Continue, Stop> Traversal<'a, S, List, Continue, Stop>
    where S: Spacing,
          List: SpacedList<S>,
          Continue: Fn(S) -> bool,
          Stop: Fn(S) -> bool {
    pub fn new(list: &'a List, continue_condition: Continue, stop_condition: Option<Stop>) -> Self {
        Self {
            lists: vec![list],
            continue_condition,
            stop_condition,
            local_offset: 0,
            local_mask: mask(list.skeleton().depth()),
            degree: list.skeleton().depth() - 1,
            node_index: 0,
            link_index: list.skeleton().size() - 1,
            position: zero(),
        }
    }

    /// Index system (pretending usize is 16 bits):
    /// empty l4 l3 l2l1  l0
    /// 0000-01-110-01-0-0111
    /// 0000011100100111
    /// level 0 depth: 4 capacity: 8
    /// level 1 depth: 1 capacity: 1
    /// level 2 depth: 2 capacity: 2
    /// level 3 depth: 3 capacity: 4
    /// level 4 depth: 2 capacity: 2
    fn localize(&self, index: usize) -> usize {
        index >> self.local_offset & self.local_mask
    }

    pub fn run(&mut self) {
        // ┌──┬──┬──┬── 0000
        //A│  │  │  │0000
        // │  │  │  ╰── 0001
        // │  │  │   0001
        // │  │  ╰──┬── 0010
        // │  │     │0010
        // │  │     ╰── 0011
        // │  │      0011
        // │ B╰──┬──┬── 0100
        // │     │  │0100
        // │     │  ╰── 0101
        // │     │   0101
        // │    C╰──┬──┬──┬── 0110-00
        // │       D│ E│  │0110-00
        // │        │  │ F╰── 0110-01
        // │        │  │   0110-01
        // │        │  ╰───── 0110-10
        // │        │0110
        // │        ╰── 0111
        // │         0111
        // ╰─────────── 1000
        loop {
            if let Some(condition) = &self.stop_condition {
                if condition(self.position) {
                    while self.descend() {
                        // condition has side effects
                    }
                    break;
                }
            }
            let local_list = *self.lists.last().unwrap();
            let local_link_index = self.localize(self.link_index);
            let local_skeleton = local_list.skeleton();
            let next_position = self.position + local_skeleton.get_link_length_at(local_link_index);
            if (self.continue_condition)(next_position) {
                self.position = next_position;
                self.node_index += 1 << self.degree << self.local_offset;
                self.link_index += 1 << self.degree << self.local_offset;
            } else if self.degree > 0 {
                self.link_index -= 1 << (self.degree - 1) << self.local_offset;
            } else if self.link_index >> self.local_offset > 0 {
                // TODO check that this branch actually makes sense
                self.link_index -= 1 << self.local_offset;
            }
            if self.descend() {
                continue;
            } else {
                break;
            }
        }
    }

    fn descend(&mut self) -> bool {
        let local_skeleton = self.lists.last().unwrap().skeleton();
        if self.degree > 0 {
            self.degree -= 1;
            true
        } else if local_skeleton.sublist_index_is_in_bounds(self.node_index) {
            let sublist = local_skeleton.get_sublist_at(self.node_index);
            if let Some(sublist) = sublist {
                let sub_skeleton = sublist.skeleton();
                self.local_offset += local_skeleton.depth();
                self.local_mask = mask(sub_skeleton.depth());
                self.degree = sub_skeleton.depth() - 1;
                self.link_index += (sub_skeleton.size() - 1) << self.local_offset;
                self.lists.push(sublist);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn next(&mut self) -> Result<(), &str> {
        let local_list = *self.lists.last().unwrap();
        let local_skeleton = local_list.skeleton();
        if self.localize(self.node_index) == local_list.size() {
            return if self.lists.len() > 1 {
                self.link_index &= !(self.local_mask << self.local_offset);
                self.node_index &= !(self.local_mask << self.local_offset);
                self.position -= local_skeleton.length();
                self.degree = 0;
                self.lists.pop();
                let local_skeleton = self.lists.last().unwrap().skeleton();
                self.local_offset -= local_skeleton.depth();
                self.local_mask = mask(local_skeleton.depth());
                return self.next();
            } else {
                Err("Called next on a Traversal that is already at the end of the list")
            }
        }

        ///
        ///
        ///
        ///
        ///
        ///
        ///
        ///
        ///
        ///
        /// ╭───────────────────╮
        /// ├───────────╮       │
        /// ├───╮       ├───╮   │
        /// ╵ 0 │     1 ╵ 2 ╵ 3 ╵
        /// 0   ├───╮  30  50  80
        ///     ╵ 0 ╵
        ///    20  25
        ///
        /// 3   x   1   x  (2) = trailing zeros
        /// ╭───────────────╮
        /// ├───────╮       │
        /// ├───╮   ├───╮   │
        /// ╵ 0 ╵ 1 ╵ 2 ╵ 3 ╵
        ///000 001 010 011 100
        /// 0  20  30  50  80
        ///
        /// move up-left until there is a link to follow, then follow that link
        ///
        /// move up left:
        /// (move link index left)
        /// move position left (- link length at node_index-1)
        /// move node index left
        /// move degree up
        ///
        /// there is a link to follow:
        /// self.degree < node_index.trailing_zeros()
        ///
        /// follow that link:
        /// TODOOO

        let degree_before = self.degree;
        let local_skeleton = self.lists.last().unwrap().skeleton();
        loop {
            let local_node_index = self.localize(self.node_index);
            if self.degree < local_node_index.trailing_zeros() as usize {
                break
            }
            self.position -= local_skeleton.get_link_length_at(local_node_index - 1);
            self.node_index -= 1 << self.degree << self.local_offset;
            self.degree += 1;
        }

        self.node_index += 1 << self.degree << self.local_offset;
        self.link_index = self.node_index + (1 << degree_before << self.local_offset) - 1;
        self.position += local_skeleton.get_link_length_at(self.node_index - 1);
        self.degree = degree_before;

        Ok(())
    }

    pub fn position(&self) -> Position<'a, S, List> {
        Position {
            lists: self.lists.clone(),
            index: self.node_index,
            position: self.position,
            link_index: self.link_index,
            offset: self.local_offset,
            mask: self.local_mask
        }
    }
}

#[cfg(test)]
mod tests {
    use std::default::default;

    use crate::{HollowSpacedList, SpacedList};
    use crate::spaced_lists::traversal::node::Traversal;

    #[test]
    fn run() {
        let mut list: HollowSpacedList<u32> = HollowSpacedList::new();
        list.append_node(20);
        list.append_node(10);
        list.append_node(20);
        list.append_node(30);

        let mut le_20 = Traversal::new(&list, |pos| pos <= 20, None::<fn(_) -> _>);
        le_20.run();
        assert_eq!(le_20.position, 20);
        assert_eq!(le_20.node_index, 1);
        assert_eq!(le_20.link_index, 1);
        assert_eq!(le_20.local_offset, 0);
        assert_eq!(le_20.local_mask, 0b111);
        assert_eq!(le_20.degree, 0);
        assert!(le_20.lists[0] == &list);

        let mut lt_30 = Traversal::new(&list, |pos| pos < 30, None::<fn(_) -> _>);
        lt_30.run();
        assert_eq!(le_20.position, lt_30.position);
        assert_eq!(le_20.node_index, lt_30.node_index);
        assert_eq!(le_20.link_index, lt_30.link_index);
        assert_eq!(le_20.local_offset, lt_30.local_offset);
        assert_eq!(le_20.local_mask, lt_30.local_mask);
        assert_eq!(le_20.degree, lt_30.degree);
        assert!(le_20.lists == lt_30.lists);

        let mut le_30 = Traversal::new(&list, |pos| pos <= 30, None::<fn(_) -> _>);
        le_30.run();
        assert_eq!(le_30.position, 30);
        assert_eq!(le_30.node_index, 2);
        assert_eq!(le_30.link_index, 2);
        assert_eq!(le_30.local_offset, 0);
        assert_eq!(le_30.local_mask, 0b111);
        assert_eq!(le_30.degree, 0);
        assert!(le_30.lists[0] == &list);

        let skeleton = list.skeleton_mut();
        let sublist = skeleton.get_sublist_at_mut(1).insert(default());
        sublist.append_node(5);
        let sublist = list.skeleton().get_sublist_at(1).as_ref().unwrap();

        let mut le_20 = Traversal::new(&list, |pos| pos <= 20, None::<fn(_) -> _>);
        le_20.run();
        assert_eq!(le_20.position, 20);
        assert_eq!(le_20.node_index, 1);
        assert_eq!(le_20.link_index, 1);
        assert_eq!(le_20.local_offset, 3);
        assert_eq!(le_20.local_mask, 0b1);
        assert_eq!(le_20.degree, 0);
        assert!(le_20.lists[0] == &list);
        assert!(le_20.lists[1] == sublist);

        let mut lt_30 = Traversal::new(&list, |pos| pos < 30, None::<fn(_) -> _>);
        lt_30.run();
        assert_eq!(lt_30.position, 25);
        assert_eq!(lt_30.node_index, 1 + (1 << 3));
        assert_eq!(lt_30.link_index, 1 + (1 << 3));
        assert_eq!(lt_30.local_offset, 3);
        assert_eq!(lt_30.local_mask, 0b1);
        assert_eq!(lt_30.degree, 0);
        assert!(lt_30.lists[0] == &list);
        assert!(lt_30.lists[1] == sublist);

        lt_30.next();
        assert_eq!(lt_30.position, 30);
        assert_eq!(lt_30.node_index, 2);
        assert_eq!(lt_30.link_index, 2);
        assert_eq!(lt_30.local_offset, 0);
        assert_eq!(lt_30.local_mask, 0b111);
        assert_eq!(lt_30.degree, 0);
        assert!(lt_30.lists[0] == &list);

        let mut le_30 = Traversal::new(&list, |pos| pos <= 30, None::<fn(_) -> _>);
        le_30.run();
        assert_eq!(le_30.position, 30);
        assert_eq!(le_30.node_index, 2);
        assert_eq!(le_30.link_index, 2);
        assert_eq!(le_30.local_offset, 0);
        assert_eq!(le_30.local_mask, 0b111);
        assert_eq!(le_30.degree, 0);
        assert!(le_30.lists[0] == &list);
    }
}
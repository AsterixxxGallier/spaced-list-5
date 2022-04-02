use std::fmt::{Debug, Formatter};
use num_traits::zero;

use crate::{SpacedList, Spacing};

pub struct Traversal<'list, S, List, Continue, Stop>
    where S: 'list + Spacing,
          List: SpacedList<S>,
          Continue: Fn(S) -> bool,
          Stop: Fn(S) -> bool {
    list: &'list List,
    continue_condition: Continue,
    stop_condition: Option<Stop>,
    degree: usize,
    node_index: usize,
    link_index: usize,
    position: S,
}

const fn mask(size: usize) -> usize {
    !(!0 << size)
}

impl<'list, S, List, Continue, Stop> Traversal<'list, S, List, Continue, Stop>
    where S: Spacing,
          List: SpacedList<S>,
          Continue: Fn(S) -> bool,
          Stop: Fn(S) -> bool {
    pub fn new(list: &'list List, continue_condition: Continue, stop_condition: Option<Stop>) -> Self {
        Self {
            list,
            continue_condition,
            stop_condition,
            degree: list.skeleton().depth() - 1,
            node_index: 0,
            link_index: list.skeleton().size() - 1,
            position: zero(),
        }
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
        let mut last_iteration = false;
        loop {
            if let Some(condition) = &self.stop_condition {
                if condition(self.position) {
                    todo!("descend until hitting rock bottom, then return")
                }
            }
            let skeleton = self.list.skeleton();
            if self.link_index >= self.list.size() {
                if self.descend(true) {
                    continue;
                } else {
                    break;
                }
            }
            let next_position = self.position + skeleton.get_link_length_at(self.link_index);
            if (self.continue_condition)(next_position) {
                self.position = next_position;
                self.node_index += 1 << self.degree;
                self.link_index += 1 << self.degree;
            }
            if last_iteration {
                if self.descend(true) {
                    last_iteration = false;
                    continue;
                } else {
                    break
                }
            }
            if self.degree > 0 {
                self.descend(true);
                continue
            } else {
                last_iteration = true;
            }
        }
    }

    fn descend(&mut self, change_link_index: bool) -> bool {
        let skeleton = self.list.skeleton();
        if self.degree > 0 {
            self.degree -= 1;
            if change_link_index {
                self.link_index -= 1 << self.degree;
            }
            true
        } else if skeleton.sublist_index_is_in_bounds(self.node_index) {
            let sublist = skeleton.get_sublist_at(self.node_index);
            if let Some(sublist) = sublist {
                let sub_skeleton = sublist.skeleton();
                self.degree = sub_skeleton.depth() - 1;
                self.node_index = 0;
                self.link_index = sub_skeleton.size() - 1;
                self.list = sublist;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn next(&mut self) -> Result<(), &str> {
        let skeleton = self.list.skeleton();
        if self.node_index == self.list.size() {
            return if self.list.sublist_data().is_some() {
                self.position -= skeleton.length();
                self.degree = 0;
                return self.next();
            } else {
                Err("Called next on a Traversal that is already at the end of the list")
            }
        }

        let degree_before = self.degree;
        let skeleton = self.list.skeleton();
        loop {
            if self.degree < self.node_index.trailing_zeros() as usize {
                break
            }
            self.position -= skeleton.get_link_length_at(self.node_index - 1);
            self.node_index -= 1 << self.degree;
            self.degree += 1;
        }

        self.node_index += 1 << self.degree;
        self.link_index = self.node_index + (1 << degree_before) - 1;
        self.position += skeleton.get_link_length_at(self.node_index - 1);
        self.degree = degree_before;

        Ok(())
    }

    pub fn position(&self) -> Position<'list, S, List> {
        Position {
            list: self.list,
            index: self.node_index,
            position: self.position,
            link_index: self.link_index,
        }
    }
}

pub struct Position<'list, S:  Spacing, List: SpacedList<S>> {
    list: &'list List,
    pub index: usize,
    pub position: S,
    link_index: usize,
}

impl<'list, S: Spacing, List: SpacedList<S>> Clone for Position<'list, S, List> {
    fn clone(&self) -> Self {
        Self {
            list: self.list,
            index: self.index,
            position: self.position,
            link_index: self.link_index
        }
    }
}

impl<'list, S: Spacing, List: SpacedList<S>> Copy for Position<'list, S, List> {}

impl<S: Spacing + Debug, List: SpacedList<S>> Debug for Position<'_, S, List> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Position")
            .field("index", &self.index)
            .field("position", &self.position)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use std::default::default;

    use crate::{HollowSpacedList, SpacedList};
    use crate::spaced_lists::traversal::node::Traversal;

/*    #[test]
    fn run() {
        let mut list: HollowSpacedList<u32> = HollowSpacedList::new();
        list.append_node(20);
        list.append_node(10);
        list.append_node(20);
        list.append_node(30);

        let mut le_20 = Traversal::new(&list, |pos| pos <= 20, None::<fn(_) -> _>);
        le_20.run();
        assert_eq!(le_20.position, 20);
        // assert_eq!(le_20.node_index, 1);
        // assert_eq!(le_20.link_index, 1);
        assert_eq!(le_20.local_offset, 0);
        assert_eq!(le_20.local_mask, 0b111);
        assert_eq!(le_20.degree, 0);
        assert!(le_20.lists[0] == &list);

        let mut lt_30 = Traversal::new(&list, |pos| pos < 30, None::<fn(_) -> _>);
        lt_30.run();
        assert_eq!(le_20.position, lt_30.position);
        assert_eq!(le_20.node_index, lt_30.node_index);
        assert_eq!(le_20.link_index, lt_30.link_index);
        // assert_eq!(le_20.local_offset, lt_30.local_offset);
        // assert_eq!(le_20.local_mask, lt_30.local_mask);
        assert_eq!(le_20.degree, lt_30.degree);
        assert!(le_20.lists == lt_30.lists);

        let mut le_30 = Traversal::new(&list, |pos| pos <= 30, None::<fn(_) -> _>);
        le_30.run();
        assert_eq!(le_30.position, 30);
        assert_eq!(le_30.node_index, 2);
        assert_eq!(le_30.link_index, 2);
        // assert_eq!(le_30.local_offset, 0);
        // assert_eq!(le_30.local_mask, 0b111);
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
        // assert_eq!(le_20.local_offset, 3);
        // assert_eq!(le_20.local_mask, 0b1);
        assert_eq!(le_20.degree, 0);
        assert!(le_20.lists[0] == &list);
        assert!(le_20.lists[1] == sublist);

        let mut lt_30 = Traversal::new(&list, |pos| pos < 30, None::<fn(_) -> _>);
        lt_30.run();
        assert_eq!(lt_30.position, 25);
        // assert_eq!(lt_30.node_index, 1 + (1 << 3));
        // assert_eq!(lt_30.link_index, 1 + (1 << 3));
        // assert_eq!(lt_30.local_offset, 3);
        // assert_eq!(lt_30.local_mask, 0b1);
        assert_eq!(lt_30.degree, 0);
        assert!(lt_30.lists[0] == &list);
        assert!(lt_30.lists[1] == sublist);

        lt_30.next();
        assert_eq!(lt_30.position, 30);
        assert_eq!(lt_30.node_index, 2);
        assert_eq!(lt_30.link_index, 2);
        // assert_eq!(lt_30.local_offset, 0);
        // assert_eq!(lt_30.local_mask, 0b111);
        assert_eq!(lt_30.degree, 0);
        assert!(lt_30.lists[0] == &list);

        let mut le_30 = Traversal::new(&list, |pos| pos <= 30, None::<fn(_) -> _>);
        le_30.run();
        assert_eq!(le_30.position, 30);
        assert_eq!(le_30.node_index, 2);
        assert_eq!(le_30.link_index, 2);
        // assert_eq!(le_30.local_offset, 0);
        // assert_eq!(le_30.local_mask, 0b111);
        assert_eq!(le_30.degree, 0);
        assert!(le_30.lists[0] == &list);
    }
*/}
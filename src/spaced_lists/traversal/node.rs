use std::fmt::{Debug, Formatter};

use num_traits::zero;

use crate::{SpacedList, Spacing};
use crate::spaced_lists::spaced_list::SublistData;

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
            return if let Some(&SublistData {
                                   containing_list,
                                   node_index,
                                   position
                               }) = self.list.sublist_data() {
                // TODO add SAFETY comment
                self.list = unsafe { &*containing_list };
                println!("moving up, node_index: {}", node_index);
                assert!(self.list.skeleton().sublist_index_is_in_bounds(node_index),
                        "{} not in bounds", node_index);
                self.degree = 0;
                self.node_index = node_index;
                self.link_index = node_index;
                self.position = position;
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
            // FIXME an integer underflow happened here after moving up
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
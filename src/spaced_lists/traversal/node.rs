use std::fmt::{Debug, Formatter};

use num_traits::zero;

use crate::{SpacedList, Spacing};

pub struct Traversal<'list, S, List, Continue, Stop>
    where S: 'list + Spacing,
          List: SpacedList<S>,
          Continue: Fn(S) -> bool,
          Stop: Fn(S) -> bool {
    super_lists: Vec<&'list List>,
    list: &'list List,
    continue_condition: Continue,
    stop_condition: Option<Stop>,
    degree: usize,
    node_index: usize,
    link_index: usize,
    position: S,
}

impl<'list, S, List, Continue, Stop> Traversal<'list, S, List, Continue, Stop>
    where S: Spacing,
          List: SpacedList<S>,
          Continue: Fn(S) -> bool,
          Stop: Fn(S) -> bool {
    pub fn new(list: &'list List, continue_condition: Continue, stop_condition: Option<Stop>) -> Self {
        Self {
            super_lists: vec![],
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
        let mut last_iteration = false;
        'outer: loop {
            if let Some(condition) = &self.stop_condition {
                if condition(self.position) {
                    while self.descend().is_ok() {
                        continue;
                    }
                    break
                }
            }
            while self.link_index >= self.list.size() {
                if self.descend().is_ok() {
                    continue;
                } else {
                    break 'outer;
                }
            }
            let skeleton = self.list.skeleton();
            let next_position = self.position + skeleton.get_link_length_at(self.link_index);
            if (self.continue_condition)(next_position) {
                self.position = next_position;
                self.node_index += 1 << self.degree;
                self.link_index += 1 << self.degree;
            }
            if last_iteration {
                if self.descend().is_ok() {
                    last_iteration = false;
                    continue;
                } else {
                    break;
                }
            }
            if self.degree > 0 {
                self.descend().unwrap();
                continue;
            } else {
                last_iteration = true;
            }
        }
    }

    fn descend(&mut self) -> Result<(), ()> {
        if self.degree > 0 {
            self.degree -= 1;
            self.link_index -= 1 << self.degree;
            Ok(())
        } else {
            let skeleton = self.list.skeleton();
            if skeleton.sublist_index_is_in_bounds(self.node_index) {
                if let Some(sublist) = skeleton.get_sublist_at(self.node_index) {
                    let sub_skeleton = sublist.skeleton();
                    self.degree = sub_skeleton.depth() - 1;
                    self.node_index = 0;
                    self.link_index = sub_skeleton.size() - 1;
                    self.super_lists.push(self.list);
                    self.list = sublist;
                    Ok(())
                } else {
                    Err(())
                }
            } else {
                Err(())
            }
        }
    }

    pub fn next(&mut self) -> Result<(), &str> {
        if self.node_index == self.list.size() {
            return if let Some(node_index) = self.list.index_in_super_list() {
                self.degree = 0;
                self.node_index = node_index;
                self.link_index = node_index;
                self.position -= self.list.length();
                self.list = self.super_lists.pop().unwrap();
                return self.next();
            } else {
                Err("Called next on a Traversal that is already at the end of the list")
            };
        }

        let degree_before = self.degree;
        let skeleton = self.list.skeleton();
        loop {
            if self.degree < self.node_index.trailing_zeros() as usize {
                break;
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

    pub fn into_position(self) -> Position<'list, S, List> {
        Position {
            list: self.list,
            index: self.node_index,
            position: self.position,
            link_index: self.link_index,
        }
    }
}

pub struct Position<'list, S: Spacing, List: SpacedList<S>> {
    list: &'list List,
    index: usize,
    position: S,
    link_index: usize,
}

impl<'list, S: Spacing, List: SpacedList<S>> Position<'list, S, List> {
    pub(crate) fn new(list: &'list List, index: usize, position: S, link_index: usize) -> Self {
        Self {
            list,
            index,
            position,
            link_index,
        }
    }

    pub fn position(&self) -> S {
        self.position
    }
}

impl<'list, S: Spacing, List: SpacedList<S>> Clone for Position<'list, S, List> {
    fn clone(&self) -> Self {
        Self {
            list: self.list,
            index: self.index,
            position: self.position,
            link_index: self.link_index,
        }
    }
}

impl<'list, S: Spacing, List: SpacedList<S>> Copy for Position<'list, S, List> {}

impl<S: Spacing + Debug, List: SpacedList<S>> Debug for Position<'_, S, List> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Position")
            .field("index", &self.index)
            .field("position", &self.position)
            .finish_non_exhaustive()
    }
}
use num_traits::zero;

use crate::{SpacedList, Spacing};

pub struct ShallowTraversal<'list, S, List, Continue, Stop>
    where S: Spacing,
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

impl<'list, S, List, Continue, Stop> ShallowTraversal<'list, S, List, Continue, Stop>
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
            link_index: list.skeleton().capacity() - 1,
            position: zero(),
        }
    }

    pub fn run(&mut self) {
        let mut last_iteration = false;
        loop {
            if let Some(condition) = &self.stop_condition {
                if condition(self.position) {
                    // TODO maybe change link index?
                    self.degree = 0;
                    break;
                }
            }
            // TODO maybe check if link index is in bounds?
            let next_position = self.position + self.list.skeleton().get_link_length_at(self.link_index);
            if (self.continue_condition)(next_position) {
                self.position = next_position;
                self.node_index += 1 << self.degree;
                self.link_index += 1 << self.degree;
            }
            if last_iteration {
                break;
            }
            if self.degree > 0 {
                self.degree -= 1;
                self.link_index -= 1 << self.degree;
            } else {
                // TODO it might not always make sense to have this extra iteration
                last_iteration = true;
            }
        }
    }

    pub fn position(&self) -> ShallowPosition<S> {
        ShallowPosition {
            index: self.node_index,
            position: self.position,
        }
    }
}

pub struct ShallowPosition<S: Spacing> {
    pub index: usize,
    pub position: S,
}
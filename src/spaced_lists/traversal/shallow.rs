use num_traits::zero;

use crate::{SpacedList, Spacing};

pub struct ShallowTraversal<'a, S, List, Continue, Stop>
    where S: 'a + Spacing,
          List: SpacedList<S>,
          Continue: Fn(S) -> bool,
          Stop: Fn(S) -> bool {
    list: &'a List,
    continue_condition: Continue,
    stop_condition: Option<Stop>,
    degree: usize,
    node_index: usize,
    link_index: usize,
    position: S,
}

impl<'a, S, List, Continue, Stop> ShallowTraversal<'a, S, List, Continue, Stop>
    where S: Spacing,
          List: SpacedList<S>,
          Continue: Fn(S) -> bool,
          Stop: Fn(S) -> bool {
    pub fn new(list: &'a List, continue_condition: Continue, stop_condition: Option<Stop>) -> Self {
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
        // │    C╰──┬── 0110
        // │        │0110
        // │        ╰── 0111
        // │         0111
        // ╰─────────── 1000
        loop {
            if let Some(condition) = &self.stop_condition {
                if condition(self.position) {
                    // TODO maybe change link index?
                    self.degree = 0;
                    break;
                }
            }
            let next_position = self.position + self.list.skeleton().get_link_length_at(self.link_index);
            if (self.continue_condition)(next_position) {
                self.position = next_position;
                self.node_index += 1 << self.degree;
                self.link_index += 1 << self.degree;
            } else if self.degree > 0 {
                self.link_index -= 1 << (self.degree - 1);
            } else if self.link_index > 0 {
                // TODO check that this branch actually makes sense
                self.link_index -= 1;
            }
            if self.degree > 0 {
                // TODO maybe change link index?
                self.degree -= 1;
            } else {
                break;
            }
        }
    }

    pub fn position(&self) -> ShallowPosition<S, List> {
        ShallowPosition {
            list: self.list,
            index: self.node_index,
            position: self.position,
            link_index: self.link_index
        }
    }
}

pub struct ShallowPosition<'a, S: Spacing, List: SpacedList<S>> {
    list: &'a List,
    pub index: usize,
    pub position: S,
    link_index: usize
}
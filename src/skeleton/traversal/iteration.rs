use std::cell::RefCell;
use std::rc::Rc;

use crate::{ParentData, Position, Skeleton, Spacing};

// TODO implement DoubleEndedIterator too for all structs implementing Iterator in this module

pub(crate) struct Iter<Kind, S: Spacing, T> {
    position: Position<Kind, S, T>,
    finished: bool,
}

impl<Kind, S: Spacing, T> Iter<Kind, S, T> {
    pub(crate) fn new(skeleton: Rc<RefCell<Skeleton<Kind, S, T>>>) -> Self {
        Self {
            position: Position::at_start(skeleton),
            finished: false,
        }
    }
}

impl<Kind, S: Spacing, T> Iterator for Iter<Kind, S, T> {
    type Item = Position<Kind, S, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None
        }

        let last_position = self.position.clone();

        // if self.index == self.list.link_size() {
        //     match self.super_lists.pop() {
        //         Some(super_list) => {
        //             let index = self.list.index_in_super_list().unwrap();
        //             self.position -= self.list.last_position();
        //             self.list = super_list;
        //             self.position += self.list.link_length_at_node(index);
        //             self.index = index + 1;
        //         }
        //         None => {
        //             self.finished = true;
        //         }
        //     }
        // } else {
        //     match self.list.sublist_at(self.index) {
        //         Some(sublist) => {
        //             self.super_lists.push(self.list);
        //             self.list = sublist;
        //             self.index = 0;
        //             self.position += sublist.offset();
        //         }
        //         None => {
        //             self.position += self.list.link_length_at_node(self.index);
        //             self.index += 1;
        //         }
        //     }
        // }

        if self.position.index == self.position.skeleton.borrow().links.len() {
            if let Some(ParentData { parent, index_in_parent }) =
                self.position.skeleton.borrow().parent_data {
                self.position.position -= self.position.skeleton.borrow().last_position();
                self.position.skeleton = parent.upgrade().unwrap();
                self.position.position += self.position.skeleton.borrow().link(index_in_parent);
                self.position.index = index_in_parent + 1;
            } else {
                self.finished = true;
            }
        } else if let Some(sub) =
            self.position.skeleton.borrow().sub(self.position.index) {
            self.position.skeleton = sub;
            self.position.index = 0;
            self.position.position += sub.borrow().offset;
        } else {
            self.position.position += self.position.skeleton.borrow().link(self.position.index);
            self.position.index += 1;
        }

        Some(last_position)
    }
}
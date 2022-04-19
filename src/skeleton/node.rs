use std::cell::{Ref, RefCell};
use std::mem;
use std::rc::Rc;

use paste::paste;

use crate::skeleton::{link_index, Node, Skeleton, Spacing};

use super::traversal::*;

macro_rules! traversal_methods {
    (@$pos:ident: $cmp:tt) => {
        paste! {
            fn [<$pos>]<'a>(&'a self, target: S) -> Option<Position<'a, S, Self>>
                where S: 'a,
                      Self: SpacedList<S> {
                traverse!(node; deep; self; $cmp target)
            }
        }
    };
    () => {
        for_all_traversals!(traversal_methods @);
    }
}

impl<S: Spacing, T> Skeleton<Node, S, T> {
    pub(crate) fn push(&mut self, distance: S, element: T) {
        if self.elements.is_empty() {
            self.offset = distance;
            self.elements.push(element);
            return;
        }
        let index = self.push_link();
        self.inflate(index, distance);
        self.elements.push(element);
    }

    pub(crate) fn insert(&mut self, position: S, element: T) {
        if self.elements.is_empty() {
            return self.push(position, element);
        }
        if position < self.offset {
            let previous_first_position = self.offset;
            let previous_first_element = mem::replace(&mut self.elements[0], element);
            let inflation_amount = previous_first_position - position;
            if !self.links.is_empty() {
                self.inflate(0, inflation_amount);
                if let Some(sub) = self.sub(0) {
                    sub.borrow_mut().offset += inflation_amount;
                }
            }
            self.offset = position;
            return self.insert(previous_first_position, previous_first_element);
        }
        if position >= self.last_position() {
            return self.push(position - self.last_position(), element);
        }
        todo!("Traverse this skeleton and insert into sublist")
    }

    // fn node_before<'a>(&'a self, target: S) -> Option<Position<'a, S, Self>>
    // where
    //     S: 'a,
    //     Self: SpacedList<S>,
    // {
    //     if target <= self.offset() {
    //         None
    //     } else {
    //         if self.link_size() == 0 {
    //             if self.offset() < target {
    //                 if true {
    //                     Some(Position::new(
    //                         ::alloc::vec::Vec::new(),
    //                         self,
    //                         0,
    //                         self.offset(),
    //                     ))
    //                 } else {
    //                     None
    //                 }
    //             } else {
    //                 None
    //             }
    //         } else {
    //             let mut list = self;
    //             let mut super_lists = ::alloc::vec::Vec::new();
    //             let mut degree = self.depth() - 1;
    //             let mut index = 0;
    //             let mut position = self.offset();
    //             {
    //                 loop {
    //                     let link_index = link_index(index, degree);
    //                     if !list.link_index_is_in_bounds(link_index) {
    //                         if degree == 0 {
    //                             break;
    //                         }
    //                         degree -= 1;
    //                         continue;
    //                     }
    //                     let next_position = position + list.link_length_at(link_index);
    //                     if next_position < target {
    //                         position = next_position;
    //                         index += 1 << degree;
    //                     };
    //                     if degree == 0 {
    //                         if let Some(sublist) = list.sublist_at(index) {
    //                             let next_position = position + sublist.offset();
    //                             if next_position < target {
    //                                 degree = sublist.depth().saturating_sub(1);
    //                                 index = 0;
    //                                 position = next_position;
    //                                 super_lists.push(list);
    //                                 list = sublist;
    //                                 continue;
    //                             }
    //                         }
    //                         break;
    //                     } else {
    //                         degree -= 1;
    //                     };
    //                 }
    //                 Some(Position::new(super_lists, list, index, position))
    //             }
    //         }
    //     }
    // }

    // CLion falsely warns that the 'a lifetime could be elided, but it can't
    // noinspection RsNeedlessLifetimes
    fn before<'a>(self: Ref<'a, Self>, target: S) -> Option<Position<'a, S, T>> {
        if self.elements.is_empty() || target <= self.offset {
            None
        } else if self.links.is_empty() {
            if self.offset < target {
                Some(Position::new(Ref::clone(&self), 0, self.offset))
            } else {
                None
            }
        } else {
            // let mut list = self;
            // let mut super_lists = ::alloc::vec::Vec::new();
            // let mut degree = self.depth() - 1;
            // let mut index = 0;
            // let mut position = self.offset();
            // {
            //     loop {
            //         let link_index = link_index(index, degree);
            //         if !list.link_index_is_in_bounds(link_index) {
            //             if degree == 0 {
            //                 break;
            //             }
            //             degree -= 1;
            //             continue;
            //         }
            //         let next_position = position + list.link_length_at(link_index);
            //         if next_position < target {
            //             position = next_position;
            //             index += 1 << degree;
            //         };
            //         if degree == 0 {
            //             if let Some(sublist) = list.sublist_at(index) {
            //                 let next_position = position + sublist.offset();
            //                 if next_position < target {
            //                     degree = sublist.depth().saturating_sub(1);
            //                     index = 0;
            //                     position = next_position;
            //                     super_lists.push(list);
            //                     list = sublist;
            //                     continue;
            //                 }
            //             }
            //             break;
            //         } else {
            //             degree -= 1;
            //         };
            //     }
            //     Some(Position::new(super_lists, list, index, position))
            // }
            let mut skeleton: Ref<'a, Skeleton<Node, S, T>> = self;
            let mut degree = skeleton.depth - 1;
            let mut index = 0;
            let mut position = skeleton.offset;
            loop {
                let link_index = link_index(index, degree);
                if !skeleton.link_index_is_in_bounds(link_index) {
                    if degree == 0 {
                        break;
                    }
                    degree -= 1;
                    continue;
                }

                let next_position = position + skeleton.links[link_index];
                if next_position < target {
                    position = next_position;
                    index += 1 << degree;
                }

                if degree > 0 {
                    degree -= 1;
                } else {
                    if let Some(sub) = skeleton.sub_ref(index) {
                        let next_position = position + sub.offset;
                        if next_position < target {
                            degree = sub.depth.saturating_sub(1);
                            index = 0;
                            position = next_position;
                            skeleton = sub;
                        }
                    }
                    break;
                }
            }
            todo!()
        }
    }

    /*// CLion falsely warns that the 'a lifetime could be elided, but it can't
    // noinspection RsNeedlessLifetimes
    fn before<'a>(this: Rc<RefCell<Self>>, target: S) -> Option<Position<'a, S, T>> {
        let borrow = this.borrow();
        if borrow.elements.is_empty() || target <= borrow.offset {
            None
        } else if borrow.links.is_empty() {
            if borrow.offset < target {
                Some(Position::new(Ref::clone(&borrow), 0, borrow.offset))
            } else {
                None
            }
        } else {
            // let mut list = self;
            // let mut super_lists = ::alloc::vec::Vec::new();
            // let mut degree = self.depth() - 1;
            // let mut index = 0;
            // let mut position = self.offset();
            // {
            //     loop {
            //         let link_index = link_index(index, degree);
            //         if !list.link_index_is_in_bounds(link_index) {
            //             if degree == 0 {
            //                 break;
            //             }
            //             degree -= 1;
            //             continue;
            //         }
            //         let next_position = position + list.link_length_at(link_index);
            //         if next_position < target {
            //             position = next_position;
            //             index += 1 << degree;
            //         };
            //         if degree == 0 {
            //             if let Some(sublist) = list.sublist_at(index) {
            //                 let next_position = position + sublist.offset();
            //                 if next_position < target {
            //                     degree = sublist.depth().saturating_sub(1);
            //                     index = 0;
            //                     position = next_position;
            //                     super_lists.push(list);
            //                     list = sublist;
            //                     continue;
            //                 }
            //             }
            //             break;
            //         } else {
            //             degree -= 1;
            //         };
            //     }
            //     Some(Position::new(super_lists, list, index, position))
            // }
            let mut skeleton: Rc<RefCell<Skeleton<Node, S, T>>> = this;
            let mut degree = skeleton.borrow().depth - 1;
            let mut index = 0;
            let mut position = skeleton.borrow().offset;
            loop {
                let link_index = link_index(index, degree);
                if !skeleton.borrow().link_index_is_in_bounds(link_index) {
                    if degree == 0 {
                        break;
                    }
                    degree -= 1;
                    continue;
                }

                let next_position = position + skeleton.borrow().links[link_index];
                if next_position < target {
                    position = next_position;
                    index += 1 << degree;
                }

                if degree > 0 {
                    degree -= 1;
                } else {
                    if let Some(sub) = skeleton.borrow().sub(index) {
                        let next_position = position + sub.borrow().offset;
                        if next_position < target {
                            degree = sub.borrow().depth.saturating_sub(1);
                            index = 0;
                            position = next_position;
                            skeleton = sub;
                        }
                    }
                    break;
                }
            }
            todo!()
        }
    }*/
}

pub struct Position<'a, S: Spacing, T> {
    skeleton: Ref<'a, Skeleton<Node, S, T>>,
    index: usize,
    position: S,
}

impl<'a, S: Spacing, T> Clone for Position<'a, S, T> {
    fn clone(&self) -> Self {
        Self {
            skeleton: Ref::clone(&self.skeleton),
            index: self.index,
            position: self.position,
        }
    }
}

impl<'a, S: Spacing, T> Position<'a, S, T> {
    pub(crate) fn new(skeleton: Ref<'a, Skeleton<Node, S, T>>, index: usize, position: S) -> Self {
        Self {
            skeleton,
            index,
            position,
        }
    }

    pub fn position(&self) -> S {
        self.position
    }

    pub fn element(&self) -> &T {
        &self.skeleton.elements[self.index]
    }
}
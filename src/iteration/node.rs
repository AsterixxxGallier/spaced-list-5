use num_traits::zero;

use crate::{Position, SpacedList, Spacing};
use crate::skeleton::traversal::link_index;

struct IterPos<'list, S: 'list + Spacing, List: SpacedList<S>> {
    list: &'list List,
    position: S,
    node_index: usize,
    degree: usize,
}

pub struct Iter<'list, S: 'list + Spacing, List: SpacedList<S>> {
    positions: Vec<IterPos<'list, S, List>>,
    super_lists: Vec<&'list List>,
}

impl<'list, S: 'list + Spacing, List: SpacedList<S>> Iter<'list, S, List> {
    pub fn new(list: &'list List) -> Iter<'list, S, List> {
        let mut this = Iter {
            positions: Vec::with_capacity(list.skeleton().depth()),
            super_lists: vec![],
        };

        this.positions.push(IterPos {
            list,
            position: list.skeleton().offset(),
            node_index: 0,
            degree: list.skeleton().depth() - 1,
        });

        this.descend();

        this
    }

    fn position(&self) -> Option<Position<'list, S, List>> {
        let last = self.positions.last()?;
        Some(Position::new(self.super_lists.clone(), last.list, last.node_index, last.position))
    }

    fn next(&mut self) -> Result<(), ()> {
        let last = self.positions.last().unwrap();
        let skeleton = last.list.skeleton();
        if skeleton.sublist_index_is_in_bounds(last.node_index) {
            if let Some(sublist) = skeleton.sublist_at(last.node_index) {
                let sub_skeleton = sublist.skeleton();
                self.super_lists.push(last.list);
                let next_position = last.position + sub_skeleton.offset();
                self.positions.push(IterPos {
                    list: sublist,
                    position: next_position,
                    node_index: 0,
                    degree: 0,
                });
                return Ok(());
            }
        }

        loop {
            let last = self.positions.last().unwrap();
            if last.node_index < last.list.skeleton().link_size() {
                break
            }
            let mut len = self.positions.len() - last.list.skeleton().depth();
            if last.list.skeleton().link_size() == 0 {
                len -= 1;
            }
            self.positions.truncate(len);
            self.super_lists.pop();
            if len == 0 {
                return Err(())
            }
        }

        let last = self.positions.last().unwrap();

        let len = self.positions.len() - last.node_index.trailing_ones() as usize;
        self.positions.truncate(len);

        self.next_unchecked();

        self.descend();

        Ok(())
    }

    fn next_unchecked(&mut self) {
        let last = self.positions.last_mut().unwrap();
        last.position += last.list.skeleton().link_length_at(link_index(last.node_index, last.degree));
        last.node_index += 1 << last.degree;
    }

    fn descend(&mut self) {
        loop {
            let IterPos {
                list,
                position,
                node_index,
                degree,
            } = *self.positions.last().unwrap();
            for degree in (0..degree).rev() {
                self.positions.push(IterPos {
                    list,
                    position,
                    node_index,
                    degree,
                })
            }
            let skeleton = list.skeleton();
            if skeleton.sublist_index_is_in_bounds(node_index) {
                if let Some(sublist) = skeleton.sublist_at(node_index) {
                    let sub_skeleton = sublist.skeleton();
                    if sub_skeleton.offset() == zero() {
                        self.super_lists.push(list);
                        self.positions.push(IterPos {
                            list: sublist,
                            position,
                            node_index: 0,
                            degree: sub_skeleton.depth().saturating_sub(1),
                        });
                        continue;
                    }
                }
            }
            break;
        }
    }
}

impl<'list, S: 'list + Spacing, List: SpacedList<S>> Iterator for Iter<'list, S, List> {
    type Item = Position<'list, S, List>;

    fn next<'a>(&'a mut self) -> Option<Position<'list, S, List>> {
        let position = Iter::<'_, _, _>::position(self)?;
        // if we're at the end of the list, the line above will return None in the next iteration
        let _err_if_at_end = self.next();
        Some(position)
    }
}
use num_traits::zero;

use crate::{Position, SpacedList, Spacing};
use crate::traversal::link_index;

struct IterPos<'list, S: 'list + Spacing, List: SpacedList<S>> {
    list: &'list List,
    position: S,
    node_index: usize,
    degree: usize,
}

impl<'list, S: 'list + Spacing, List: SpacedList<S>> Clone for IterPos<'list, S, List> {
    fn clone(&self) -> Self {
        Self {
            list: self.list,
            position: self.position,
            node_index: self.node_index,
            degree: self.degree,
        }
    }
}

impl<'list, S: 'list + Spacing, List: SpacedList<S>> Copy for IterPos<'list, S, List> {}

pub struct Iter<'list, S: 'list + Spacing, List: SpacedList<S>> {
    positions: Vec<IterPos<'list, S, List>>,
    super_lists: Vec<&'list List>,
}

impl<'list, S: 'list + Spacing, List: SpacedList<S>> Iter<'list, S, List> {
    pub fn new(list: &'list List) -> Iter<'list, S, List> {
        let mut this = Iter {
            positions: Vec::with_capacity(list.depth()),
            super_lists: vec![],
        };

        this.positions.push(IterPos {
            list,
            position: list.offset(),
            node_index: 0,
            degree: list.depth() - 1,
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
        if let Some(sublist) = last.list.sublist_at(last.node_index) {
            self.super_lists.push(last.list);
            let next_position = last.position + sublist.offset();
            self.positions.push(IterPos {
                list: sublist,
                position: next_position,
                node_index: 0,
                degree: 0,
            });
            return Ok(());
        }

        loop {
            let last = self.positions.last().unwrap();
            if last.node_index < last.list.link_size() {
                break;
            }
            let mut len = self.positions.len() - last.list.depth();
            if last.list.link_size() == 0 {
                len -= 1;
            }
            self.positions.truncate(len);
            self.super_lists.pop();
            if len == 0 {
                return Err(());
            }
        }

        let last = self.positions.last().unwrap();

        let len = self.positions.len() - last.node_index.trailing_ones() as usize;
        self.positions.truncate(len);

        let last = self.positions.last_mut().unwrap();
        last.position += last.list.link_length_at(link_index(last.node_index, last.degree));
        last.node_index += 1 << last.degree;

        self.descend();

        Ok(())
    }

    fn descend(&mut self) {
        loop {
            let last = *self.positions.last().unwrap();
            for degree in (0..last.degree).rev() {
                self.positions.push(IterPos {
                    degree,
                    ..last
                })
            }
            if let Some(sublist) = last.list.sublist_at(last.node_index) {
                if sublist.offset() == zero() {
                    self.super_lists.push(last.list);
                    self.positions.push(IterPos {
                        list: sublist,
                        position: last.position,
                        node_index: 0,
                        degree: sublist.depth().saturating_sub(1),
                    });
                    continue;
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
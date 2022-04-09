use num_traits::zero;

use crate::{Position, SpacedList, Spacing};

struct IterPos<'list, S: 'list + Spacing, List: SpacedList<S>> {
    list: &'list List,
    position: S,
    node_index: usize,
    link_index: usize,
    degree: usize,
}

pub struct Iter<'list, S: 'list + Spacing, List: SpacedList<S>> {
    positions: Vec<IterPos<'list, S, List>>,
    super_lists: Vec<&'list List>
}

impl<'list, S: 'list + Spacing, List: SpacedList<S>> Iter<'list, S, List> {
    pub fn new(list: &'list List) -> Iter<'list, S, List> {
        let mut this = Iter {
            positions: Vec::with_capacity(list.skeleton().depth()),
            super_lists: vec![]
        };

        this.positions.push(IterPos {
            list,
            position: zero(),
            node_index: 0,
            link_index: list.skeleton().capacity() - 1,
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
        if last.node_index == last.list.skeleton().size() {
            let len = self.positions.len() - last.list.skeleton().depth();
            self.positions.truncate(len);
            self.super_lists.pop();
            if len == 0 {
                Err(())
            } else {
                self.next()
            }
        } else {
            let len = self.positions.len() - last.node_index.trailing_ones() as usize;
            self.positions.truncate(len);

            self.next_unchecked();

            self.descend();

            Ok(())
        }
    }

    fn next_unchecked(&mut self) {
        let last = self.positions.last_mut().unwrap();
        last.position += last.list.skeleton().link_length_at(last.link_index);
        last.node_index += 1 << last.degree;
        last.link_index += 1 << last.degree;
    }

    /// Descends as far down as possible
    fn descend(&mut self) {
        loop {
            let IterPos {
                list,
                position,
                node_index,
                link_index,
                degree,
            } = *self.positions.last().unwrap();
            let mut link_index = link_index;
            for degree in (0..degree).rev() {
                link_index -= 1 << degree;
                self.positions.push(IterPos {
                    list,
                    position,
                    node_index,
                    link_index,
                    degree,
                })
            }
            let skeleton = list.skeleton();
            if skeleton.sublist_index_is_in_bounds(node_index) {
                if let Some(sublist) = skeleton.sublist_at(node_index) {
                    self.super_lists.push(list);
                    let sub_skeleton = sublist.skeleton();
                    self.positions.push(IterPos {
                        list: sublist,
                        position,
                        node_index: 0,
                        link_index: sub_skeleton.capacity() - 1,
                        degree: sub_skeleton.depth() - 1,
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
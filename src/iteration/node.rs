use crate::{Position, SpacedList, Spacing};

pub struct Iter<'list, S: 'list + Spacing, List: SpacedList<S>> {
    finished: bool,
    super_lists: Vec<&'list List>,
    list: &'list List,
    index: usize,
    position: S,
}

impl<'list, S: 'list + Spacing, List: SpacedList<S>> Iter<'list, S, List> {
    pub fn new(list: &'list List) -> Self {
        Self {
            finished: false,
            super_lists: vec![],
            list,
            index: 0,
            position: list.offset(),
        }
    }
}

impl<'list, S: 'list + Spacing, List: SpacedList<S>> Iterator for Iter<'list, S, List> {
    type Item = Position<'list, S, List>;

    fn next<'a>(&'a mut self) -> Option<Position<'list, S, List>> {
        if self.finished {
            return None
        }

        // TODO avoid the clone here
        let old_position =
            Position::new(self.super_lists.clone(), self.list, self.index, self.position);

        if self.index == self.list.link_size() {
            match self.super_lists.pop() {
                Some(super_list) => {
                    let index = self.list.index_in_super_list().unwrap();
                    self.position -= self.list.last_position();
                    self.list = super_list;
                    self.position += self.list.link_length_at_node(index);
                    self.index = index + 1;
                }
                None => {
                    self.finished = true;
                }
            }
        } else {
            match self.list.sublist_at(self.index) {
                Some(sublist) => {
                    self.super_lists.push(self.list);
                    self.list = sublist;
                    self.index = 0;
                    self.position += sublist.offset();
                }
                None => {
                    self.position += self.list.link_length_at_node(self.index);
                    self.index += 1;
                }
            }
        }

        Some(old_position)
    }
}
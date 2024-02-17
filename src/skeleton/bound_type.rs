#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum BoundType {
    Start,
    End,
}

impl BoundType {
    pub(crate) fn of(index: isize) -> Self {
        match index & 1 {
            0 => Self::Start,
            1 => Self::End,
            _ => unreachable!()
        }
    }
}

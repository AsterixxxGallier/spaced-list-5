#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum BoundType {
    Start,
    End,
}

impl BoundType {
    #[inline(always)]
    pub(crate) const fn of(index: usize) -> Self {
        match index & 1 {
            0 => Self::Start,
            1 => Self::End,
            _ => unreachable!()
        }
    }

    #[inline(always)]
    pub(crate) const fn of_signed(index: isize) -> Self {
        match index & 1 {
            0 => Self::Start,
            1 => Self::End,
            _ => unreachable!()
        }
    }
}

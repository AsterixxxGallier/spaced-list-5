use num_traits::zero;
use crate::Spacing;

pub struct Skeleton<S: Spacing, T> {
    links: Vec<S>,
    elements: Vec<T>,
    subs: Vec<Self>,
    offset: S,
    length: S,
    link_size: usize,
    element_size: usize
}

impl<S: Spacing, T> Skeleton<S, T> {
    pub(crate) fn new() -> Self {
        Self {
            links: vec![],
            elements: vec![],
            subs: vec![],
            offset: zero(),
            length: zero(),
            link_size: 0,
            element_size: 0
        }
    }
}
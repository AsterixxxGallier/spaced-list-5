use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::{Rc, Weak};

use nohash_hasher::{IntMap, IntSet};
use num_traits::zero;

use crate::{Spacing, EphemeralIndex, Index};

pub struct Node;

mod seal {
    pub trait Seal {}
}
pub trait RangeKind : seal::Seal {}
impl seal::Seal for Range {}
impl seal::Seal for NestedRange {}
impl RangeKind for Range {}
impl RangeKind for NestedRange {}

pub struct Range;

// works like Range, but when a range is inserted inside of another range, then it's put in a
// sub, such that even indices are still start indices and odd indices are end indices (genius!)
pub struct NestedRange;

pub(crate) struct ParentData<Parent> {
    pub(crate) parent: Weak<RefCell<Parent>>,
    pub(crate) index_in_parent: usize,
}

// TODO major optimization opportunity for big lists:
//  source of problem: all links, elements and subs are stored in a singular Vec
//  problem 1: When a power of two in size is crossed, allocating the larger array can take a long time, causing up to
//             seconds-long freezes.
//  problem 2: Traversal always starts by accessing a link in the second half of the list, and then moves through the
//             skeleton in jumps of exponentially decaying distances. At the beginning, these distances are extremely
//             large. This is bad news for the cache.
//  solution: When a certain threshold in link Vec size is crossed (maybe 2^16 bytes) when pushing an element onto the
//            skeleton, *don't* append it to this skeleton. Instead, create a new skeleton. Replace the old, "full"
//            one with this new one in whatever position it was ("root" skeleton of a spaced list, or sub in a parent
//            skeleton). Add the old skeleton as a sub at index zero to the new one. Create another sub at index one,
//            and push the element onto that one. Make sure the spacing's all right.
//            Let's call the new skeleton "hyperskeleton", or "hyper" for short. Hypers have no elements, and a sub at
//            every index. Traversal through a hyper may never yield an index in the hyper itself, only one in a sub.
//            This means that all subs of a hyper should have an offset of zero. Iteration through a hyper must
//            behave like the conjoined iteration through each of its consecutive subs. In short, the existence of the
//            hyper should be invisible to everyone using this crate, similarly to the existence of subs.
//            It seems logical to introduce "Hyper" as a new skeleton Kind.
//            When a skeleton "overflows" (as described above) that is already a sub in a hyper, add a sublist at its
//            next free index; no need to create a new hyper. Using estimates of 2^12 to 2^16 bytes in a full skeleton,
//            one hyper can be filled with 2^24 (16 million) to 2^32 (4 billion) bytes of links, and a hyper of
//            hypers (also possible!) can be filled with 2^36 (69 billion) to 2^48 (281 trillion) link bytes. Safe to
//            say, hypers don't need to be nested deeply at these sizes.
//            This mechanism guarantees an upper bound not only on allocated array size, but also on jump distances,
//            except when entering a sub. In this way, most of the really bad cache misses can be avoided.
// TODO optimization opportunity: store a small list of recently/often-accessed indices with their respective positions
//  for quick access (in other words: a shortcuts cache)
// TODO optimization opportunity: instead of introducing a sub, actually splice the element into the vec and recalculate
//  spacings accordingly (only when it's faster, so for small skeletons)
// TODO double-check that subs never have a negative offset
// TODO optimization opportunity: in empty element slots, store information that makes it O(log n) to skip many empty
//  slots; this can be achieved by using a similar tree structure as for the spacings (boolean tree with AND and element
//  access)
// TODO integrate subs into element slots?
// TODO insert functions that (also) take an index as a parameter (?)
// the last element slot must always be full!
pub(crate) struct Skeleton<Kind, S: Spacing, T> {
    links: Vec<S>,
    elements: Vec<ElementSlot<T>>,
    subs: Vec<Option<Rc<RefCell<Self>>>>,
    parent_data: Option<ParentData<Self>>,
    offset: S,
    length: S,
    depth: usize,
    first_persistent_index: isize,
    /// When an element is removed, its persistent index is inserted into this set.
    dangling_persistent_indices: IntSet<isize>,
    from_persistent: IntMap<isize, EphemeralIndex<Kind, S, T>>,
    into_persistent: IntMap<usize, Index<Kind, S, T>>,
    _kind: PhantomData<Kind>,
}

pub type ElementSlot<T> = Option<T>;

#[inline(always)]
pub(crate) const fn get_link_index(index: usize, degree: usize) -> usize {
    index | ((1 << degree) - 1)
}

#[inline(always)]
pub(crate) const fn relative_depth(index: usize, size: usize) -> usize {
    (usize::BITS - (size ^ index).leading_zeros()) as usize
}

impl<Kind, S: Spacing, T> Skeleton<Kind, S, T> {
    pub(crate) fn new(parent_data: Option<ParentData<Self>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            links: vec![],
            elements: vec![],
            subs: vec![],
            parent_data,
            offset: zero(),
            length: zero(),
            depth: 0,
            first_persistent_index: 0,
            dangling_persistent_indices: IntSet::default(),
            from_persistent: IntMap::default(),
            into_persistent: IntMap::default(),
            _kind: PhantomData::<Kind>,
        }))
    }

    fn link_index_is_in_bounds(&self, index: usize) -> bool {
        index < self.links.len()
    }

    fn link(&self, index: usize) -> S {
        let mut length = self.links[index];
        for degree in 0..index.trailing_ones() {
            length -= self.links[index - (1 << degree)];
        }
        length
    }

    fn push_link(&mut self) -> usize {
        let mut length = zero();
        let index = self.links.len();
        for degree in 0..index.trailing_ones() {
            length += self.links[index - (1 << degree)];
        }
        self.links.push(length);
        if self.links.len().is_power_of_two() {
            self.depth += 1;
        }
        self.subs.push(None);
        index
    }

    pub fn offset(&self) -> S {
        self.offset
    }

    pub fn length(&self) -> S {
        self.length
    }

    pub fn last_position(&self) -> S {
        self.offset + self.length
    }

    fn sub(&self, index: usize) -> Option<Rc<RefCell<Self>>> {
        self.subs.get(index).cloned().flatten()
    }

    fn ensure_sub(this: Rc<RefCell<Self>>, index: usize) -> Rc<RefCell<Self>> {
        match &mut this.borrow_mut().subs[index] {
            Some(sub) => sub.clone(),
            none =>
                none.insert(Skeleton::new(Some(
                    ParentData {
                        parent: Rc::downgrade(&this),
                        index_in_parent: index,
                    }))).clone()
        }
    }
}

pub mod change_spacing;
pub mod node;
pub mod range;
pub mod nested_range;
pub mod traversal;
pub mod index;
pub mod ephemeral_index;
pub mod position;
pub mod ephemeral_position;
pub mod bound_type;
pub mod element_ref;

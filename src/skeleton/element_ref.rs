use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;
use std::ops::{Deref, DerefMut};

use ouroboros::self_referencing;

use crate::{Node, Spacing, RangeKind, Skeleton};
use crate::skeleton::ElementSlot;

/// The spaced list that the referenced element is contained in cannot be mutated for the lifetime
/// of the [`ElementRef`]. If you run into problems with this, consider storing a [`Position`] or
/// [`Index`] instead, and create [`ElementRef`]s from them whenever they are needed.
#[self_referencing]
pub struct ElementRef<Kind: 'static, S: Spacing + 'static, T: 'static> {
    skeleton: Rc<RefCell<Skeleton<Kind, S, T>>>,
    #[borrows(skeleton)]
    #[covariant]
    skeleton_ref: Ref<'this, Skeleton<Kind, S, T>>,
    index: usize,
}

impl<Kind, S: Spacing, T> ElementRef<Kind, S, T> {
    //noinspection RsUnresolvedReference
    pub(crate) fn new_(skeleton: Rc<RefCell<Skeleton<Kind, S, T>>>, index: usize) -> Self {
        ElementRefBuilder {
            skeleton,
            index,
            skeleton_ref_builder: |skeleton: &Rc<RefCell<Skeleton<Kind, S, T>>>| skeleton.borrow()
        }.build()
    }
}

impl<S: Spacing, T> Deref for ElementRef<Node, S, T> {
    type Target = ElementSlot<T>;

    fn deref(&self) -> &ElementSlot<T> {
        &self.borrow_skeleton_ref().elements[*self.borrow_index()]
    }
}

impl<Kind: RangeKind, S: Spacing, T> Deref for ElementRef<Kind, S, T> {
    type Target = ElementSlot<T>;

    fn deref(&self) -> &ElementSlot<T> {
        &self.borrow_skeleton_ref().elements[self.borrow_index() / 2]
    }
}

/// During the lifetime of the [`ElementRefMut`], there cannot be any other references into the
/// spaced list that contains the referenced element. For example, you won't be able to create an
/// [`ElementRef`] for a different element. If you run into problems with this, consider storing a
/// [`Position`] or [`Index`] instead, and create [`ElementRefMut`]s from them whenever they are
/// needed.
#[self_referencing]
pub struct ElementRefMut<Kind: 'static, S: Spacing + 'static, T: 'static> {
    skeleton: Rc<RefCell<Skeleton<Kind, S, T>>>,
    #[borrows(skeleton)]
    #[covariant]
    skeleton_ref: RefMut<'this, Skeleton<Kind, S, T>>,
    index: usize,
}

impl<Kind, S: Spacing, T> ElementRefMut<Kind, S, T> {
    //noinspection RsUnresolvedReference
    pub(crate) fn new_(skeleton: Rc<RefCell<Skeleton<Kind, S, T>>>, index: usize) -> Self {
        ElementRefMutBuilder {
            skeleton,
            index,
            skeleton_ref_builder: |skeleton: &Rc<RefCell<Skeleton<Kind, S, T>>>| skeleton.borrow_mut()
        }.build()
    }
}

impl<S: Spacing, T> Deref for ElementRefMut<Node, S, T> {
    type Target = ElementSlot<T>;

    fn deref(&self) -> &ElementSlot<T> {
        &self.borrow_skeleton_ref().elements[*self.borrow_index()]
    }
}

impl<Kind: RangeKind, S: Spacing, T> Deref for ElementRefMut<Kind, S, T> {
    type Target = ElementSlot<T>;

    fn deref(&self) -> &ElementSlot<T> {
        &self.borrow_skeleton_ref().elements[self.borrow_index() / 2]
    }
}

impl<S: Spacing, T> DerefMut for ElementRefMut<Node, S, T> {
    fn deref_mut(&mut self) -> &mut ElementSlot<T> {
        let index = *self.borrow_index();
        self.with_skeleton_ref_mut(|skeleton_ref| &mut skeleton_ref.elements[index])
    }
}

impl<Kind: RangeKind, S: Spacing, T> DerefMut for ElementRefMut<Kind, S, T> {
    fn deref_mut(&mut self) -> &mut ElementSlot<T> {
        let index = self.borrow_index() / 2;
        self.with_skeleton_ref_mut(|skeleton_ref| &mut skeleton_ref.elements[index])
    }
}
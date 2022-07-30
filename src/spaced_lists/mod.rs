use std::cell::RefCell;
use std::rc::Rc;

use itertools::Itertools;

use crate::{BackwardsIter, BoundType, ForwardsIter, HollowPosition, Spacing};
use crate::skeleton::{Node, ClosedRange, Skeleton, OpenNestedRange};
use crate::skeleton::position::Position;

// TODO implement try_ versions of all public methods that can fail
// TODO add good error handling to all public methods that can fail
// TODO add conditional traversal methods (ones that will only return positions where the elements
//  match a custom condition)

macro_rules! spacing_methods {
    () => {
        pub fn increase_spacing_after(&mut self, position: S, spacing: S) {
            Skeleton::inflate_after(self.skeleton.clone(), position, spacing)
        }

        pub fn increase_spacing_before(&mut self, position: S, spacing: S) {
            Skeleton::inflate_before(self.skeleton.clone(), position, spacing)
        }

        pub fn decrease_spacing_after(&mut self, position: S, spacing: S) {
            Skeleton::deflate_after(self.skeleton.clone(), position, spacing)
        }

        pub fn decrease_spacing_before(&mut self, position: S, spacing: S) {
            Skeleton::deflate_before(self.skeleton.clone(), position, spacing)
        }
    }
}

macro_rules! trivial_accessors {
    () => {
        pub fn size(&self) -> usize {
            self.size
        }

        pub fn is_empty(&self) -> bool {
            self.size == 0
        }

        pub fn length(&self) -> S {
            self.skeleton.borrow().length()
        }

        pub fn start(&self) -> S {
            self.skeleton.borrow().offset()
        }

        pub fn end(&self) -> S {
            self.skeleton.borrow().last_position()
        }
    }
}

pub struct SpacedList<S: Spacing, T> {
    skeleton: Rc<RefCell<Skeleton<Node, S, T>>>,
    size: usize,
}

impl<S: Spacing, T> Default for SpacedList<S, T> {
    fn default() -> Self {
        Self {
            skeleton: Skeleton::new(None),
            size: 0,
        }
    }
}

impl<S: Spacing, T> SpacedList<S, T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, spacing: S, value: T) -> Position<Node, S, T> {
        self.size += 1;
        Skeleton::<Node, _, _>::push(self.skeleton.clone(), spacing, value).into()
    }

    pub fn insert(&mut self, position: S, value: T) -> Position<Node, S, T> {
        self.size += 1;
        Skeleton::<Node, _, _>::insert(self.skeleton.clone(), position, value).into()
    }


    spacing_methods!();


    pub fn first(&self) -> Option<Position<Node, S, T>> {
        if self.is_empty() {
            None
        } else {
            Some(Position::at_start(self.skeleton.clone()))
        }
    }

    pub fn last(&self) -> Option<Position<Node, S, T>> {
        if self.is_empty() {
            None
        } else {
            Some(Position::at_end(self.skeleton.clone()))
        }
    }


    pub fn before(&self, position: S) -> Option<Position<Node, S, T>> {
        Skeleton::<Node, _, _>::before(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn at_or_before(&self, position: S) -> Option<Position<Node, S, T>> {
        Skeleton::<Node, _, _>::at_or_before(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn at(&self, position: S) -> Option<Position<Node, S, T>> {
        Skeleton::<Node, _, _>::at(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn at_or_after(&self, position: S) -> Option<Position<Node, S, T>> {
        Skeleton::<Node, _, _>::at_or_after(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn after(&self, position: S) -> Option<Position<Node, S, T>> {
        Skeleton::<Node, _, _>::after(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }


    pub fn iter(&self) -> impl Iterator<Item=Position<Node, S, T>> {
        ForwardsIter::from_start(self.skeleton.clone())
    }

    pub fn into_iter(self) -> impl Iterator<Item=Position<Node, S, T>> {
        ForwardsIter::from_start(self.skeleton)
    }

    pub fn iter_backwards(&self) -> impl Iterator<Item=Position<Node, S, T>> {
        BackwardsIter::from_end(self.skeleton.clone())
    }

    pub fn into_iter_backwards(self) -> impl Iterator<Item=Position<Node, S, T>> {
        BackwardsIter::from_end(self.skeleton)
    }

    trivial_accessors!();
}

pub struct RangeSpacedList<S: Spacing, T> {
    skeleton: Rc<RefCell<Skeleton<ClosedRange, S, T>>>,
    size: usize,
}

impl<S: Spacing, T> Default for RangeSpacedList<S, T> {
    fn default() -> Self {
        Self {
            skeleton: Skeleton::new(None),
            size: 0,
        }
    }
}

impl<S: Spacing, T> RangeSpacedList<S, T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, spacing: S, span: S, value: T) -> Position<ClosedRange, S, T> {
        self.size += 2;
        Skeleton::<ClosedRange, _, _>::push(self.skeleton.clone(), spacing, span, value).into()
    }

    pub fn insert(&mut self, start: S, end: S, value: T) -> Position<ClosedRange, S, T> {
        self.insert_with_span(start, end - start, value)
    }

    pub fn insert_with_span(&mut self, start: S, span: S, value: T) -> Position<ClosedRange, S, T> {
        self.size += 2;
        Skeleton::<ClosedRange, _, _>::insert(self.skeleton.clone(), start, span, value).into()
    }


    spacing_methods!();


    pub fn first(&self) -> Option<Position<ClosedRange, S, T>> {
        if self.is_empty() {
            None
        } else {
            Some(Position::at_start(self.skeleton.clone()))
        }
    }

    pub fn last(&self) -> Option<Position<ClosedRange, S, T>> {
        if self.is_empty() {
            None
        } else {
            Some(Position::at_end(self.skeleton.clone()))
        }
    }


    pub fn starting_or_ending_before(&self, position: S) -> Option<Position<ClosedRange, S, T>> {
        Skeleton::<ClosedRange, _, _>::before(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn starting_or_ending_at_or_before(&self, position: S) -> Option<Position<ClosedRange, S, T>> {
        Skeleton::<ClosedRange, _, _>::at_or_before(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn starting_or_ending_at(&self, position: S) -> Option<Position<ClosedRange, S, T>> {
        Skeleton::<ClosedRange, _, _>::at(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn starting_or_ending_at_or_after(&self, position: S) -> Option<Position<ClosedRange, S, T>> {
        Skeleton::<ClosedRange, _, _>::at_or_after(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn starting_or_ending_after(&self, position: S) -> Option<Position<ClosedRange, S, T>> {
        Skeleton::<ClosedRange, _, _>::after(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }


    pub fn starting_before(&self, position: S) -> Option<Position<ClosedRange, S, T>> {
        Skeleton::<ClosedRange, _, _>::starting_before(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn starting_at_or_before(&self, position: S) -> Option<Position<ClosedRange, S, T>> {
        Skeleton::<ClosedRange, _, _>::starting_at_or_before(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn starting_at(&self, position: S) -> Option<Position<ClosedRange, S, T>> {
        Skeleton::<ClosedRange, _, _>::starting_at(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn starting_at_or_after(&self, position: S) -> Option<Position<ClosedRange, S, T>> {
        Skeleton::<ClosedRange, _, _>::starting_at_or_after(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn starting_after(&self, position: S) -> Option<Position<ClosedRange, S, T>> {
        Skeleton::<ClosedRange, _, _>::starting_after(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }


    pub fn ending_before(&self, position: S) -> Option<Position<ClosedRange, S, T>> {
        Skeleton::<ClosedRange, _, _>::ending_before(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn ending_at_or_before(&self, position: S) -> Option<Position<ClosedRange, S, T>> {
        Skeleton::<ClosedRange, _, _>::ending_at_or_before(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn ending_at(&self, position: S) -> Option<Position<ClosedRange, S, T>> {
        Skeleton::<ClosedRange, _, _>::ending_at(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn ending_at_or_after(&self, position: S) -> Option<Position<ClosedRange, S, T>> {
        Skeleton::<ClosedRange, _, _>::ending_at_or_after(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn ending_after(&self, position: S) -> Option<Position<ClosedRange, S, T>> {
        Skeleton::<ClosedRange, _, _>::ending_after(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }


    pub fn iter(&self) -> impl Iterator<Item=Position<ClosedRange, S, T>> {
        ForwardsIter::from_start(self.skeleton.clone())
    }

    pub fn into_iter(self) -> impl Iterator<Item=Position<ClosedRange, S, T>> {
        ForwardsIter::from_start(self.skeleton)
    }

    pub fn iter_ranges(&self) -> impl Iterator<Item=(Position<ClosedRange, S, T>, Position<ClosedRange, S, T>)> {
        self.iter().tuples()
    }

    pub fn into_iter_ranges(self) -> impl Iterator<Item=(Position<ClosedRange, S, T>, Position<ClosedRange, S, T>)> {
        self.into_iter().tuples()
    }

    pub fn iter_backwards(&self) -> impl Iterator<Item=Position<ClosedRange, S, T>> {
        BackwardsIter::from_end(self.skeleton.clone())
    }

    pub fn into_iter_backwards(self) -> impl Iterator<Item=Position<ClosedRange, S, T>> {
        BackwardsIter::from_end(self.skeleton)
    }

    pub fn iter_ranges_backwards(&self) -> impl Iterator<Item=(Position<ClosedRange, S, T>, Position<ClosedRange, S, T>)> {
        self.iter_backwards().tuples()
    }

    pub fn into_iter_ranges_backwards(self) -> impl Iterator<Item=(Position<ClosedRange, S, T>, Position<ClosedRange, S, T>)> {
        self.into_iter_backwards().tuples()
    }


    trivial_accessors!();
}

pub struct NestedRangeSpacedList<S: Spacing, T> {
    skeleton: Rc<RefCell<Skeleton<OpenNestedRange, S, T>>>,
    size: usize,
}

impl<S: Spacing, T> Default for NestedRangeSpacedList<S, T> {
    fn default() -> Self {
        Self {
            skeleton: Skeleton::new(None),
            size: 0,
        }
    }
}

impl<S: Spacing, T> NestedRangeSpacedList<S, T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, spacing: S, bound_type: BoundType, value: T) -> Position<OpenNestedRange, S, T> {
        self.size += 1;
        Skeleton::<OpenNestedRange, _, _>::push(self.skeleton.clone(), spacing, bound_type, value).into()
    }

    pub fn push_range(&mut self, spacing: S, span: S, value: T) -> Position<OpenNestedRange, S, T> {
        self.size += 2;
        Skeleton::<OpenNestedRange, _, _>::push_range(self.skeleton.clone(), spacing, span, value).into()
    }

    pub fn insert_range(&mut self, start: S, end: S, value: T) -> Position<OpenNestedRange, S, T> {
        self.insert_range_with_span(start, end - start, value)
    }

    pub fn insert_range_with_span(&mut self, start: S, span: S, value: T) -> Position<OpenNestedRange, S, T> {
        self.size += 2;
        Skeleton::<OpenNestedRange, _, _>::insert_range(self.skeleton.clone(), start, span, value).into()
    }


    spacing_methods!();


    pub fn first(&self) -> Option<Position<OpenNestedRange, S, T>> {
        if self.is_empty() {
            None
        } else {
            Some(Position::at_start(self.skeleton.clone()))
        }
    }

    pub fn last(&self) -> Option<Position<OpenNestedRange, S, T>> {
        if self.is_empty() {
            None
        } else {
            Some(Position::at_end(self.skeleton.clone()))
        }
    }


    pub fn starting_or_ending_before(&self, position: S) -> Option<Position<OpenNestedRange, S, T>> {
        Skeleton::<OpenNestedRange, _, _>::before(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn starting_or_ending_at_or_before(&self, position: S) -> Option<Position<OpenNestedRange, S, T>> {
        Skeleton::<OpenNestedRange, _, _>::at_or_before(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn starting_or_ending_at(&self, position: S) -> Option<Position<OpenNestedRange, S, T>> {
        Skeleton::<OpenNestedRange, _, _>::at(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn starting_or_ending_at_or_after(&self, position: S) -> Option<Position<OpenNestedRange, S, T>> {
        Skeleton::<OpenNestedRange, _, _>::at_or_after(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn starting_or_ending_after(&self, position: S) -> Option<Position<OpenNestedRange, S, T>> {
        Skeleton::<OpenNestedRange, _, _>::after(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }


    pub fn starting_before(&self, position: S) -> Option<Position<OpenNestedRange, S, T>> {
        Skeleton::<OpenNestedRange, _, _>::starting_before(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn starting_at_or_before(&self, position: S) -> Option<Position<OpenNestedRange, S, T>> {
        Skeleton::<OpenNestedRange, _, _>::starting_at_or_before(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn starting_at(&self, position: S) -> Option<Position<OpenNestedRange, S, T>> {
        Skeleton::<OpenNestedRange, _, _>::starting_at(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn starting_at_or_after(&self, position: S) -> Option<Position<OpenNestedRange, S, T>> {
        Skeleton::<OpenNestedRange, _, _>::starting_at_or_after(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn starting_after(&self, position: S) -> Option<Position<OpenNestedRange, S, T>> {
        Skeleton::<OpenNestedRange, _, _>::starting_after(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }


    pub fn ending_before(&self, position: S) -> Option<Position<OpenNestedRange, S, T>> {
        Skeleton::<OpenNestedRange, _, _>::ending_before(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn ending_at_or_before(&self, position: S) -> Option<Position<OpenNestedRange, S, T>> {
        Skeleton::<OpenNestedRange, _, _>::ending_at_or_before(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn ending_at(&self, position: S) -> Option<Position<OpenNestedRange, S, T>> {
        Skeleton::<OpenNestedRange, _, _>::ending_at(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn ending_at_or_after(&self, position: S) -> Option<Position<OpenNestedRange, S, T>> {
        Skeleton::<OpenNestedRange, _, _>::ending_at_or_after(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn ending_after(&self, position: S) -> Option<Position<OpenNestedRange, S, T>> {
        Skeleton::<OpenNestedRange, _, _>::ending_after(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }


    pub fn iter(&self) -> impl Iterator<Item=Position<OpenNestedRange, S, T>> {
        ForwardsIter::from_start(self.skeleton.clone())
    }

    pub fn into_iter(self) -> impl Iterator<Item=Position<OpenNestedRange, S, T>> {
        ForwardsIter::from_start(self.skeleton)
    }

    // TODO nested range iterator
    pub fn iter_ranges(&self) -> impl Iterator<Item=(Position<OpenNestedRange, S, T>, Position<OpenNestedRange, S, T>)> {
        self.iter().tuples()
    }

    pub fn into_iter_ranges(self) -> impl Iterator<Item=(Position<OpenNestedRange, S, T>, Position<OpenNestedRange, S, T>)> {
        self.into_iter().tuples()
    }

    pub fn iter_backwards(&self) -> impl Iterator<Item=Position<OpenNestedRange, S, T>> {
        BackwardsIter::from_end(self.skeleton.clone())
    }

    pub fn into_iter_backwards(self) -> impl Iterator<Item=Position<OpenNestedRange, S, T>> {
        BackwardsIter::from_end(self.skeleton)
    }

    pub fn iter_ranges_backwards(&self) -> impl Iterator<Item=(Position<OpenNestedRange, S, T>, Position<OpenNestedRange, S, T>)> {
        self.iter_backwards().tuples()
    }

    pub fn into_iter_ranges_backwards(self) -> impl Iterator<Item=(Position<OpenNestedRange, S, T>, Position<OpenNestedRange, S, T>)> {
        self.into_iter_backwards().tuples()
    }


    trivial_accessors!();
}

pub struct HollowSpacedList<S: Spacing> {
    skeleton: Rc<RefCell<Skeleton<Node, S, ()>>>,
    size: usize,
}

impl<S: Spacing> Default for HollowSpacedList<S> {
    fn default() -> Self {
        Self {
            skeleton: Skeleton::new(None),
            size: 0,
        }
    }
}

impl<S: Spacing> HollowSpacedList<S> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, spacing: S) -> HollowPosition<Node, S> {
        self.size += 1;
        let position: Position<Node, S, ()> =
            Skeleton::<Node, _, _>::push(self.skeleton.clone(), spacing, ()).into();
        position.into()
    }

    pub fn insert(&mut self, position: S) -> HollowPosition<Node, S> {
        self.size += 1;
        let position: Position<Node, S, ()> =
            Skeleton::<Node, _, _>::insert(self.skeleton.clone(), position, ()).into();
        position.into()
    }


    spacing_methods!();


    pub fn first(&self) -> Option<HollowPosition<Node, S>> {
        if self.is_empty() {
            None
        } else {
            Some(HollowPosition::at_start(self.skeleton.clone()))
        }
    }

    pub fn last(&self) -> Option<HollowPosition<Node, S>> {
        if self.is_empty() {
            None
        } else {
            Some(HollowPosition::at_end(self.skeleton.clone()))
        }
    }


    pub fn before(&self, position: S) -> Option<HollowPosition<Node, S>> {
        Skeleton::<Node, _, _>::before(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<Node, S, ()> = position.into();
                position.into()
            })
    }

    pub fn at_or_before(&self, position: S) -> Option<HollowPosition<Node, S>> {
        Skeleton::<Node, _, _>::at_or_before(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<Node, S, ()> = position.into();
                position.into()
            })
    }

    pub fn at(&self, position: S) -> Option<HollowPosition<Node, S>> {
        Skeleton::<Node, _, _>::at(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<Node, S, ()> = position.into();
                position.into()
            })
    }

    pub fn at_or_after(&self, position: S) -> Option<HollowPosition<Node, S>> {
        Skeleton::<Node, _, _>::at_or_after(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<Node, S, ()> = position.into();
                position.into()
            })
    }

    pub fn after(&self, position: S) -> Option<HollowPosition<Node, S>> {
        Skeleton::<Node, _, _>::after(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<Node, S, ()> = position.into();
                position.into()
            })
    }


    pub fn iter(&self) -> impl Iterator<Item=HollowPosition<Node, S>> {
        ForwardsIter::from_start(self.skeleton.clone()).map_into()
    }

    pub fn into_iter(self) -> impl Iterator<Item=HollowPosition<Node, S>> {
        ForwardsIter::from_start(self.skeleton).map_into()
    }

    pub fn iter_backwards(&self) -> impl Iterator<Item=HollowPosition<Node, S>> {
        BackwardsIter::from_end(self.skeleton.clone()).map_into()
    }

    pub fn into_iter_backwards(self) -> impl Iterator<Item=HollowPosition<Node, S>> {
        BackwardsIter::from_end(self.skeleton).map_into()
    }

    trivial_accessors!();
}

pub struct HollowRangeSpacedList<S: Spacing> {
    skeleton: Rc<RefCell<Skeleton<ClosedRange, S, ()>>>,
    size: usize,
}

impl<S: Spacing> Default for HollowRangeSpacedList<S> {
    fn default() -> Self {
        Self {
            skeleton: Skeleton::new(None),
            size: 0,
        }
    }
}

impl<S: Spacing> HollowRangeSpacedList<S> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, spacing: S, span: S) -> HollowPosition<ClosedRange, S> {
        self.size += 2;
        let position: Position<ClosedRange, S, ()> =
            Skeleton::<ClosedRange, _, _>::push(self.skeleton.clone(), spacing, span, ()).into();
        position.into()
    }

    pub fn insert(&mut self, start: S, end: S) -> HollowPosition<ClosedRange, S> {
        let position: Position<ClosedRange, S, ()> =
            self.insert_with_span(start, end - start).into();
        position.into()
    }

    pub fn insert_with_span(&mut self, start: S, span: S) -> HollowPosition<ClosedRange, S> {
        self.size += 2;
        let position: Position<ClosedRange, S, ()> =
            Skeleton::<ClosedRange, _, _>::insert(self.skeleton.clone(), start, span, ()).into();
        position.into()
    }


    spacing_methods!();


    pub fn first(&self) -> Option<HollowPosition<ClosedRange, S>> {
        if self.is_empty() {
            None
        } else {
            Some(HollowPosition::at_start(self.skeleton.clone()))
        }
    }

    pub fn last(&self) -> Option<HollowPosition<ClosedRange, S>> {
        if self.is_empty() {
            None
        } else {
            Some(HollowPosition::at_end(self.skeleton.clone()))
        }
    }


    pub fn starting_or_ending_before(&self, position: S) -> Option<HollowPosition<ClosedRange, S>> {
        Skeleton::<ClosedRange, _, _>::before(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<ClosedRange, S, ()> = position.into();
                position.into()
            })
    }

    pub fn starting_or_ending_at_or_before(&self, position: S) -> Option<HollowPosition<ClosedRange, S>> {
        Skeleton::<ClosedRange, _, _>::at_or_before(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<ClosedRange, S, ()> = position.into();
                position.into()
            })
    }

    pub fn starting_or_ending_at(&self, position: S) -> Option<HollowPosition<ClosedRange, S>> {
        Skeleton::<ClosedRange, _, _>::at(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<ClosedRange, S, ()> = position.into();
                position.into()
            })
    }

    pub fn starting_or_ending_at_or_after(&self, position: S) -> Option<HollowPosition<ClosedRange, S>> {
        Skeleton::<ClosedRange, _, _>::at_or_after(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<ClosedRange, S, ()> = position.into();
                position.into()
            })
    }

    pub fn starting_or_ending_after(&self, position: S) -> Option<HollowPosition<ClosedRange, S>> {
        Skeleton::<ClosedRange, _, _>::after(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<ClosedRange, S, ()> = position.into();
                position.into()
            })
    }


    pub fn starting_before(&self, position: S) -> Option<HollowPosition<ClosedRange, S>> {
        Skeleton::<ClosedRange, _, _>::starting_before(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<ClosedRange, S, ()> = position.into();
                position.into()
            })
    }

    pub fn starting_at_or_before(&self, position: S) -> Option<HollowPosition<ClosedRange, S>> {
        Skeleton::<ClosedRange, _, _>::starting_at_or_before(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<ClosedRange, S, ()> = position.into();
                position.into()
            })
    }

    pub fn starting_at(&self, position: S) -> Option<HollowPosition<ClosedRange, S>> {
        Skeleton::<ClosedRange, _, _>::starting_at(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<ClosedRange, S, ()> = position.into();
                position.into()
            })
    }

    pub fn starting_at_or_after(&self, position: S) -> Option<HollowPosition<ClosedRange, S>> {
        Skeleton::<ClosedRange, _, _>::starting_at_or_after(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<ClosedRange, S, ()> = position.into();
                position.into()
            })
    }

    pub fn starting_after(&self, position: S) -> Option<HollowPosition<ClosedRange, S>> {
        Skeleton::<ClosedRange, _, _>::starting_after(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<ClosedRange, S, ()> = position.into();
                position.into()
            })
    }


    pub fn ending_before(&self, position: S) -> Option<HollowPosition<ClosedRange, S>> {
        Skeleton::<ClosedRange, _, _>::ending_before(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<ClosedRange, S, ()> = position.into();
                position.into()
            })
    }

    pub fn ending_at_or_before(&self, position: S) -> Option<HollowPosition<ClosedRange, S>> {
        Skeleton::<ClosedRange, _, _>::ending_at_or_before(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<ClosedRange, S, ()> = position.into();
                position.into()
            })
    }

    pub fn ending_at(&self, position: S) -> Option<HollowPosition<ClosedRange, S>> {
        Skeleton::<ClosedRange, _, _>::ending_at(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<ClosedRange, S, ()> = position.into();
                position.into()
            })
    }

    pub fn ending_at_or_after(&self, position: S) -> Option<HollowPosition<ClosedRange, S>> {
        Skeleton::<ClosedRange, _, _>::ending_at_or_after(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<ClosedRange, S, ()> = position.into();
                position.into()
            })
    }

    pub fn ending_after(&self, position: S) -> Option<HollowPosition<ClosedRange, S>> {
        Skeleton::<ClosedRange, _, _>::ending_after(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<ClosedRange, S, ()> = position.into();
                position.into()
            })
    }


    pub fn iter(&self) -> impl Iterator<Item=HollowPosition<ClosedRange, S>> {
        ForwardsIter::from_start(self.skeleton.clone()).map_into()
    }

    pub fn into_iter(self) -> impl Iterator<Item=HollowPosition<ClosedRange, S>> {
        ForwardsIter::from_start(self.skeleton).map_into()
    }

    pub fn iter_ranges(&self) -> impl Iterator<Item=(HollowPosition<ClosedRange, S>, HollowPosition<ClosedRange, S>)> {
        self.iter().tuples()
    }

    pub fn into_iter_ranges(self) -> impl Iterator<Item=(HollowPosition<ClosedRange, S>, HollowPosition<ClosedRange, S>)> {
        self.into_iter().tuples()
    }

    pub fn iter_backwards(&self) -> impl Iterator<Item=HollowPosition<ClosedRange, S>> {
        BackwardsIter::from_end(self.skeleton.clone()).map_into()
    }

    pub fn into_iter_backwards(self) -> impl Iterator<Item=HollowPosition<ClosedRange, S>> {
        BackwardsIter::from_end(self.skeleton).map_into()
    }

    pub fn iter_ranges_backwards(&self) -> impl Iterator<Item=(HollowPosition<ClosedRange, S>, HollowPosition<ClosedRange, S>)> {
        self.iter_backwards().tuples()
    }

    pub fn into_iter_ranges_backwards(self) -> impl Iterator<Item=(HollowPosition<ClosedRange, S>, HollowPosition<ClosedRange, S>)> {
        self.into_iter_backwards().tuples()
    }


    trivial_accessors!();
}

// TODO HollowNestedRangeSpacedList
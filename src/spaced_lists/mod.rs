use std::cell::RefCell;
use std::rc::Rc;

use itertools::Itertools;

use crate::{BackwardsIter, ForwardsIter, HollowPosition, NestedRange, Spacing};
use crate::skeleton::{Node, Range, Skeleton};
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
    skeleton: Rc<RefCell<Skeleton<Range, S, T>>>,
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

    pub fn push(&mut self, spacing: S, span: S, value: T) -> Position<Range, S, T> {
        self.size += 1;
        Skeleton::<Range, _, _>::push(self.skeleton.clone(), spacing, span, value).into()
    }

    pub fn insert(&mut self, start: S, end: S, value: T) -> Position<Range, S, T> {
        self.insert_with_span(start, end - start, value)
    }

    pub fn insert_with_span(&mut self, start: S, span: S, value: T) -> Position<Range, S, T> {
        self.size += 1;
        Skeleton::<Range, _, _>::insert(self.skeleton.clone(), start, span, value).into()
    }


    spacing_methods!();


    pub fn first(&self) -> Option<Position<Range, S, T>> {
        if self.is_empty() {
            None
        } else {
            Some(Position::at_start(self.skeleton.clone()))
        }
    }

    pub fn last(&self) -> Option<Position<Range, S, T>> {
        if self.is_empty() {
            None
        } else {
            Some(Position::at_end(self.skeleton.clone()))
        }
    }


    pub fn starting_or_ending_before(&self, position: S) -> Option<Position<Range, S, T>> {
        Skeleton::<Range, _, _>::before(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn starting_or_ending_at_or_before(&self, position: S) -> Option<Position<Range, S, T>> {
        Skeleton::<Range, _, _>::at_or_before(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn starting_or_ending_at(&self, position: S) -> Option<Position<Range, S, T>> {
        Skeleton::<Range, _, _>::at(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn starting_or_ending_at_or_after(&self, position: S) -> Option<Position<Range, S, T>> {
        Skeleton::<Range, _, _>::at_or_after(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn starting_or_ending_after(&self, position: S) -> Option<Position<Range, S, T>> {
        Skeleton::<Range, _, _>::after(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }


    pub fn starting_before(&self, position: S) -> Option<Position<Range, S, T>> {
        Skeleton::<Range, _, _>::starting_before(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn starting_at_or_before(&self, position: S) -> Option<Position<Range, S, T>> {
        Skeleton::<Range, _, _>::starting_at_or_before(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn starting_at(&self, position: S) -> Option<Position<Range, S, T>> {
        Skeleton::<Range, _, _>::starting_at(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn starting_at_or_after(&self, position: S) -> Option<Position<Range, S, T>> {
        Skeleton::<Range, _, _>::starting_at_or_after(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn starting_after(&self, position: S) -> Option<Position<Range, S, T>> {
        Skeleton::<Range, _, _>::starting_after(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }


    pub fn ending_before(&self, position: S) -> Option<Position<Range, S, T>> {
        Skeleton::<Range, _, _>::ending_before(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn ending_at_or_before(&self, position: S) -> Option<Position<Range, S, T>> {
        Skeleton::<Range, _, _>::ending_at_or_before(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn ending_at(&self, position: S) -> Option<Position<Range, S, T>> {
        Skeleton::<Range, _, _>::ending_at(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn ending_at_or_after(&self, position: S) -> Option<Position<Range, S, T>> {
        Skeleton::<Range, _, _>::ending_at_or_after(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn ending_after(&self, position: S) -> Option<Position<Range, S, T>> {
        Skeleton::<Range, _, _>::ending_after(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }


    pub fn iter(&self) -> impl Iterator<Item=Position<Range, S, T>> {
        ForwardsIter::from_start(self.skeleton.clone())
    }

    pub fn into_iter(self) -> impl Iterator<Item=Position<Range, S, T>> {
        ForwardsIter::from_start(self.skeleton)
    }

    pub fn iter_ranges(&self) -> impl Iterator<Item=(Position<Range, S, T>, Position<Range, S, T>)> {
        self.iter().tuples()
    }

    pub fn into_iter_ranges(self) -> impl Iterator<Item=(Position<Range, S, T>, Position<Range, S, T>)> {
        self.into_iter().tuples()
    }

    pub fn iter_backwards(&self) -> impl Iterator<Item=Position<Range, S, T>> {
        BackwardsIter::from_end(self.skeleton.clone())
    }

    pub fn into_iter_backwards(self) -> impl Iterator<Item=Position<Range, S, T>> {
        BackwardsIter::from_end(self.skeleton)
    }

    pub fn iter_ranges_backwards(&self) -> impl Iterator<Item=(Position<Range, S, T>, Position<Range, S, T>)> {
        self.iter_backwards().tuples()
    }

    pub fn into_iter_ranges_backwards(self) -> impl Iterator<Item=(Position<Range, S, T>, Position<Range, S, T>)> {
        self.into_iter_backwards().tuples()
    }


    trivial_accessors!();
}

pub struct NestedRangeSpacedList<S: Spacing, T> {
    skeleton: Rc<RefCell<Skeleton<NestedRange, S, T>>>,
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

    pub fn push(&mut self, spacing: S, span: S, value: T) -> Position<NestedRange, S, T> {
        self.size += 1;
        Skeleton::<NestedRange, _, _>::push(self.skeleton.clone(), spacing, span, value).into()
    }

    pub fn insert(&mut self, start: S, end: S, value: T) -> Position<NestedRange, S, T> {
        self.insert_with_span(start, end - start, value)
    }

    pub fn insert_with_span(&mut self, start: S, span: S, value: T) -> Position<NestedRange, S, T> {
        self.size += 1;
        Skeleton::<NestedRange, _, _>::insert(self.skeleton.clone(), start, span, value).into()
    }


    spacing_methods!();


    pub fn first(&self) -> Option<Position<NestedRange, S, T>> {
        if self.is_empty() {
            None
        } else {
            Some(Position::at_start(self.skeleton.clone()))
        }
    }

    pub fn last(&self) -> Option<Position<NestedRange, S, T>> {
        if self.is_empty() {
            None
        } else {
            Some(Position::at_end(self.skeleton.clone()))
        }
    }


    pub fn starting_or_ending_before(&self, position: S) -> Option<Position<NestedRange, S, T>> {
        Skeleton::<NestedRange, _, _>::before(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn starting_or_ending_at_or_before(&self, position: S) -> Option<Position<NestedRange, S, T>> {
        Skeleton::<NestedRange, _, _>::at_or_before(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn starting_or_ending_at(&self, position: S) -> Option<Position<NestedRange, S, T>> {
        Skeleton::<NestedRange, _, _>::at(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn starting_or_ending_at_or_after(&self, position: S) -> Option<Position<NestedRange, S, T>> {
        Skeleton::<NestedRange, _, _>::at_or_after(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn starting_or_ending_after(&self, position: S) -> Option<Position<NestedRange, S, T>> {
        Skeleton::<NestedRange, _, _>::after(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }


    /*pub fn starting_before(&self, position: S) -> Option<Position<NestedRange, S, T>> {
        Skeleton::<NestedRange, _, _>::starting_before(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn starting_at_or_before(&self, position: S) -> Option<Position<NestedRange, S, T>> {
        Skeleton::<NestedRange, _, _>::starting_at_or_before(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }*/

    pub fn starting_at(&self, position: S) -> Option<Position<NestedRange, S, T>> {
        Skeleton::<NestedRange, _, _>::starting_at(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    /*pub fn starting_at_or_after(&self, position: S) -> Option<Position<NestedRange, S, T>> {
        Skeleton::<NestedRange, _, _>::starting_at_or_after(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn starting_after(&self, position: S) -> Option<Position<NestedRange, S, T>> {
        Skeleton::<NestedRange, _, _>::starting_after(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }


    pub fn ending_before(&self, position: S) -> Option<Position<NestedRange, S, T>> {
        Skeleton::<NestedRange, _, _>::ending_before(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn ending_at_or_before(&self, position: S) -> Option<Position<NestedRange, S, T>> {
        Skeleton::<NestedRange, _, _>::ending_at_or_before(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }*/

    pub fn ending_at(&self, position: S) -> Option<Position<NestedRange, S, T>> {
        Skeleton::<NestedRange, _, _>::ending_at(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    /*pub fn ending_at_or_after(&self, position: S) -> Option<Position<NestedRange, S, T>> {
        Skeleton::<NestedRange, _, _>::ending_at_or_after(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }

    pub fn ending_after(&self, position: S) -> Option<Position<NestedRange, S, T>> {
        Skeleton::<NestedRange, _, _>::ending_after(self.skeleton.clone(), position)
            .map(|ephemeral| ephemeral.into())
    }*/


    pub fn iter(&self) -> impl Iterator<Item=Position<NestedRange, S, T>> {
        ForwardsIter::from_start(self.skeleton.clone())
    }

    pub fn into_iter(self) -> impl Iterator<Item=Position<NestedRange, S, T>> {
        ForwardsIter::from_start(self.skeleton)
    }

    /*pub fn iter_ranges(&self) -> impl Iterator<Item=(Position<NestedRange, S, T>, Position<NestedRange, S, T>)> {
        self.iter().tuples()
    }

    pub fn into_iter_ranges(self) -> impl Iterator<Item=(Position<NestedRange, S, T>, Position<NestedRange, S, T>)> {
        self.into_iter().tuples()
    }*/

    pub fn iter_backwards(&self) -> impl Iterator<Item=Position<NestedRange, S, T>> {
        BackwardsIter::from_end(self.skeleton.clone())
    }

    pub fn into_iter_backwards(self) -> impl Iterator<Item=Position<NestedRange, S, T>> {
        BackwardsIter::from_end(self.skeleton)
    }

    /*pub fn iter_ranges_backwards(&self) -> impl Iterator<Item=(Position<NestedRange, S, T>, Position<NestedRange, S, T>)> {
        self.iter_backwards().tuples()
    }

    pub fn into_iter_ranges_backwards(self) -> impl Iterator<Item=(Position<NestedRange, S, T>, Position<NestedRange, S, T>)> {
        self.into_iter_backwards().tuples()
    }*/


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
    skeleton: Rc<RefCell<Skeleton<Range, S, ()>>>,
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

    pub fn push(&mut self, spacing: S, span: S) -> HollowPosition<Range, S> {
        self.size += 1;
        let position: Position<Range, S, ()> =
            Skeleton::<Range, _, _>::push(self.skeleton.clone(), spacing, span, ()).into();
        position.into()
    }

    pub fn insert(&mut self, start: S, end: S) -> HollowPosition<Range, S> {
        let position: Position<Range, S, ()> =
            self.insert_with_span(start, end - start).into();
        position.into()
    }

    pub fn insert_with_span(&mut self, start: S, span: S) -> HollowPosition<Range, S> {
        self.size += 1;
        let position: Position<Range, S, ()> =
            Skeleton::<Range, _, _>::insert(self.skeleton.clone(), start, span, ()).into();
        position.into()
    }


    spacing_methods!();


    pub fn first(&self) -> Option<HollowPosition<Range, S>> {
        if self.is_empty() {
            None
        } else {
            Some(HollowPosition::at_start(self.skeleton.clone()))
        }
    }

    pub fn last(&self) -> Option<HollowPosition<Range, S>> {
        if self.is_empty() {
            None
        } else {
            Some(HollowPosition::at_end(self.skeleton.clone()))
        }
    }


    pub fn starting_or_ending_before(&self, position: S) -> Option<HollowPosition<Range, S>> {
        Skeleton::<Range, _, _>::before(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<Range, S, ()> = position.into();
                position.into()
            })
    }

    pub fn starting_or_ending_at_or_before(&self, position: S) -> Option<HollowPosition<Range, S>> {
        Skeleton::<Range, _, _>::at_or_before(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<Range, S, ()> = position.into();
                position.into()
            })
    }

    pub fn starting_or_ending_at(&self, position: S) -> Option<HollowPosition<Range, S>> {
        Skeleton::<Range, _, _>::at(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<Range, S, ()> = position.into();
                position.into()
            })
    }

    pub fn starting_or_ending_at_or_after(&self, position: S) -> Option<HollowPosition<Range, S>> {
        Skeleton::<Range, _, _>::at_or_after(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<Range, S, ()> = position.into();
                position.into()
            })
    }

    pub fn starting_or_ending_after(&self, position: S) -> Option<HollowPosition<Range, S>> {
        Skeleton::<Range, _, _>::after(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<Range, S, ()> = position.into();
                position.into()
            })
    }


    pub fn starting_before(&self, position: S) -> Option<HollowPosition<Range, S>> {
        Skeleton::<Range, _, _>::starting_before(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<Range, S, ()> = position.into();
                position.into()
            })
    }

    pub fn starting_at_or_before(&self, position: S) -> Option<HollowPosition<Range, S>> {
        Skeleton::<Range, _, _>::starting_at_or_before(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<Range, S, ()> = position.into();
                position.into()
            })
    }

    pub fn starting_at(&self, position: S) -> Option<HollowPosition<Range, S>> {
        Skeleton::<Range, _, _>::starting_at(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<Range, S, ()> = position.into();
                position.into()
            })
    }

    pub fn starting_at_or_after(&self, position: S) -> Option<HollowPosition<Range, S>> {
        Skeleton::<Range, _, _>::starting_at_or_after(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<Range, S, ()> = position.into();
                position.into()
            })
    }

    pub fn starting_after(&self, position: S) -> Option<HollowPosition<Range, S>> {
        Skeleton::<Range, _, _>::starting_after(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<Range, S, ()> = position.into();
                position.into()
            })
    }


    pub fn ending_before(&self, position: S) -> Option<HollowPosition<Range, S>> {
        Skeleton::<Range, _, _>::ending_before(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<Range, S, ()> = position.into();
                position.into()
            })
    }

    pub fn ending_at_or_before(&self, position: S) -> Option<HollowPosition<Range, S>> {
        Skeleton::<Range, _, _>::ending_at_or_before(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<Range, S, ()> = position.into();
                position.into()
            })
    }

    pub fn ending_at(&self, position: S) -> Option<HollowPosition<Range, S>> {
        Skeleton::<Range, _, _>::ending_at(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<Range, S, ()> = position.into();
                position.into()
            })
    }

    pub fn ending_at_or_after(&self, position: S) -> Option<HollowPosition<Range, S>> {
        Skeleton::<Range, _, _>::ending_at_or_after(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<Range, S, ()> = position.into();
                position.into()
            })
    }

    pub fn ending_after(&self, position: S) -> Option<HollowPosition<Range, S>> {
        Skeleton::<Range, _, _>::ending_after(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<Range, S, ()> = position.into();
                position.into()
            })
    }


    pub fn iter(&self) -> impl Iterator<Item=HollowPosition<Range, S>> {
        ForwardsIter::from_start(self.skeleton.clone()).map_into()
    }

    pub fn into_iter(self) -> impl Iterator<Item=HollowPosition<Range, S>> {
        ForwardsIter::from_start(self.skeleton).map_into()
    }

    pub fn iter_ranges(&self) -> impl Iterator<Item=(HollowPosition<Range, S>, HollowPosition<Range, S>)> {
        self.iter().tuples()
    }

    pub fn into_iter_ranges(self) -> impl Iterator<Item=(HollowPosition<Range, S>, HollowPosition<Range, S>)> {
        self.into_iter().tuples()
    }

    pub fn iter_backwards(&self) -> impl Iterator<Item=HollowPosition<Range, S>> {
        BackwardsIter::from_end(self.skeleton.clone()).map_into()
    }

    pub fn into_iter_backwards(self) -> impl Iterator<Item=HollowPosition<Range, S>> {
        BackwardsIter::from_end(self.skeleton).map_into()
    }

    pub fn iter_ranges_backwards(&self) -> impl Iterator<Item=(HollowPosition<Range, S>, HollowPosition<Range, S>)> {
        self.iter_backwards().tuples()
    }

    pub fn into_iter_ranges_backwards(self) -> impl Iterator<Item=(HollowPosition<Range, S>, HollowPosition<Range, S>)> {
        self.into_iter_backwards().tuples()
    }


    trivial_accessors!();
}

pub struct HollowNestedRangeSpacedList<S: Spacing> {
    skeleton: Rc<RefCell<Skeleton<NestedRange, S, ()>>>,
    size: usize,
}

impl<S: Spacing> Default for HollowNestedRangeSpacedList<S> {
    fn default() -> Self {
        Self {
            skeleton: Skeleton::new(None),
            size: 0,
        }
    }
}

impl<S: Spacing> HollowNestedRangeSpacedList<S> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, spacing: S, span: S) -> HollowPosition<NestedRange, S> {
        self.size += 1;
        let position: Position<NestedRange, S, ()> =
            Skeleton::<NestedRange, _, _>::push(self.skeleton.clone(), spacing, span, ()).into();
        position.into()
    }

    pub fn insert(&mut self, start: S, end: S) -> HollowPosition<NestedRange, S> {
        let position: Position<NestedRange, S, ()> =
            self.insert_with_span(start, end - start).into();
        position.into()
    }

    pub fn insert_with_span(&mut self, start: S, span: S) -> HollowPosition<NestedRange, S> {
        self.size += 1;
        let position: Position<NestedRange, S, ()> =
            Skeleton::<NestedRange, _, _>::insert(self.skeleton.clone(), start, span, ()).into();
        position.into()
    }


    spacing_methods!();


    pub fn first(&self) -> Option<HollowPosition<NestedRange, S>> {
        if self.is_empty() {
            None
        } else {
            Some(HollowPosition::at_start(self.skeleton.clone()))
        }
    }

    pub fn last(&self) -> Option<HollowPosition<NestedRange, S>> {
        if self.is_empty() {
            None
        } else {
            Some(HollowPosition::at_end(self.skeleton.clone()))
        }
    }


    pub fn starting_or_ending_before(&self, position: S) -> Option<HollowPosition<NestedRange, S>> {
        Skeleton::<NestedRange, _, _>::before(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<NestedRange, S, ()> = position.into();
                position.into()
            })
    }

    pub fn starting_or_ending_at_or_before(&self, position: S) -> Option<HollowPosition<NestedRange, S>> {
        Skeleton::<NestedRange, _, _>::at_or_before(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<NestedRange, S, ()> = position.into();
                position.into()
            })
    }

    pub fn starting_or_ending_at(&self, position: S) -> Option<HollowPosition<NestedRange, S>> {
        Skeleton::<NestedRange, _, _>::at(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<NestedRange, S, ()> = position.into();
                position.into()
            })
    }

    pub fn starting_or_ending_at_or_after(&self, position: S) -> Option<HollowPosition<NestedRange, S>> {
        Skeleton::<NestedRange, _, _>::at_or_after(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<NestedRange, S, ()> = position.into();
                position.into()
            })
    }

    pub fn starting_or_ending_after(&self, position: S) -> Option<HollowPosition<NestedRange, S>> {
        Skeleton::<NestedRange, _, _>::after(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<NestedRange, S, ()> = position.into();
                position.into()
            })
    }


    /*pub fn starting_before(&self, position: S) -> Option<HollowPosition<NestedRange, S>> {
        Skeleton::<NestedRange, _, _>::starting_before(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<NestedRange, S, ()> = position.into();
                position.into()
            })
    }

    pub fn starting_at_or_before(&self, position: S) -> Option<HollowPosition<NestedRange, S>> {
        Skeleton::<NestedRange, _, _>::starting_at_or_before(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<NestedRange, S, ()> = position.into();
                position.into()
            })
    }*/

    pub fn starting_at(&self, position: S) -> Option<HollowPosition<NestedRange, S>> {
        Skeleton::<NestedRange, _, _>::starting_at(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<NestedRange, S, ()> = position.into();
                position.into()
            })
    }

    /*pub fn starting_at_or_after(&self, position: S) -> Option<HollowPosition<NestedRange, S>> {
        Skeleton::<NestedRange, _, _>::starting_at_or_after(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<NestedRange, S, ()> = position.into();
                position.into()
            })
    }

    pub fn starting_after(&self, position: S) -> Option<HollowPosition<NestedRange, S>> {
        Skeleton::<NestedRange, _, _>::starting_after(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<NestedRange, S, ()> = position.into();
                position.into()
            })
    }


    pub fn ending_before(&self, position: S) -> Option<HollowPosition<NestedRange, S>> {
        Skeleton::<NestedRange, _, _>::ending_before(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<NestedRange, S, ()> = position.into();
                position.into()
            })
    }

    pub fn ending_at_or_before(&self, position: S) -> Option<HollowPosition<NestedRange, S>> {
        Skeleton::<NestedRange, _, _>::ending_at_or_before(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<NestedRange, S, ()> = position.into();
                position.into()
            })
    }*/

    pub fn ending_at(&self, position: S) -> Option<HollowPosition<NestedRange, S>> {
        Skeleton::<NestedRange, _, _>::ending_at(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<NestedRange, S, ()> = position.into();
                position.into()
            })
    }

    /*pub fn ending_at_or_after(&self, position: S) -> Option<HollowPosition<NestedRange, S>> {
        Skeleton::<NestedRange, _, _>::ending_at_or_after(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<NestedRange, S, ()> = position.into();
                position.into()
            })
    }

    pub fn ending_after(&self, position: S) -> Option<HollowPosition<NestedRange, S>> {
        Skeleton::<NestedRange, _, _>::ending_after(self.skeleton.clone(), position)
            .map(|position| {
                let position: Position<NestedRange, S, ()> = position.into();
                position.into()
            })
    }*/


    pub fn iter(&self) -> impl Iterator<Item=HollowPosition<NestedRange, S>> {
        ForwardsIter::from_start(self.skeleton.clone()).map_into()
    }

    pub fn into_iter(self) -> impl Iterator<Item=HollowPosition<NestedRange, S>> {
        ForwardsIter::from_start(self.skeleton).map_into()
    }

    /*pub fn iter_ranges(&self) -> impl Iterator<Item=(HollowPosition<NestedRange, S>, HollowPosition<NestedRange, S>)> {
        self.iter().tuples()
    }

    pub fn into_iter_ranges(self) -> impl Iterator<Item=(HollowPosition<NestedRange, S>, HollowPosition<NestedRange, S>)> {
        self.into_iter().tuples()
    }*/

    pub fn iter_backwards(&self) -> impl Iterator<Item=HollowPosition<NestedRange, S>> {
        BackwardsIter::from_end(self.skeleton.clone()).map_into()
    }

    pub fn into_iter_backwards(self) -> impl Iterator<Item=HollowPosition<NestedRange, S>> {
        BackwardsIter::from_end(self.skeleton).map_into()
    }

    /*pub fn iter_ranges_backwards(&self) -> impl Iterator<Item=(HollowPosition<NestedRange, S>, HollowPosition<NestedRange, S>)> {
        self.iter_backwards().tuples()
    }

    pub fn into_iter_ranges_backwards(self) -> impl Iterator<Item=(HollowPosition<NestedRange, S>, HollowPosition<NestedRange, S>)> {
        self.into_iter_backwards().tuples()
    }*/


    trivial_accessors!();
}
use std::cell::RefCell;
use std::rc::Rc;

use crate::skeleton::{Node, Range, Skeleton};
use crate::skeleton::position::{HollowPosition, Position};
use crate::Spacing;

// TODO implement try_ versions of all public methods that can fail

pub struct SpacedList<S: Spacing, T> {
    skeleton: Rc<RefCell<Skeleton<Node, S, T>>>,
    size: usize,
}

impl<S: Spacing, T> SpacedList<S, T> {
    pub fn push(&mut self, spacing: S, value: T) -> Position<Node, S, T> { todo!() }

    pub fn insert(&mut self, position: S, value: T) -> Position<Node, S, T> { todo!() }


    pub fn increase_spacing_after(&mut self, position: S, spacing: S) { todo!() }

    pub fn increase_spacing_before(&mut self, position: S, spacing: S) { todo!() }

    pub fn decrease_spacing_after(&mut self, position: S, spacing: S) { todo!() }

    pub fn decrease_spacing_before(&mut self, position: S, spacing: S) { todo!() }


    pub fn before(&self, position: S) -> Option<Position<Node, S, T>> { todo!() }

    pub fn at_or_before(&self, position: S) -> Option<Position<Node, S, T>> { todo!() }

    pub fn at(&self, position: S) -> Option<Position<Node, S, T>> { todo!() }

    pub fn at_or_after(&self, position: S) -> Option<Position<Node, S, T>> { todo!() }

    pub fn after(&self, position: S) -> Option<Position<Node, S, T>> { todo!() }


    pub fn iter(&self) -> impl Iterator<Item=Position<Node, S, T>> { todo!() }

    pub fn into_iter(self) -> impl Iterator<Item=Position<Node, S, T>> { todo!() }


    pub fn size(&self) -> usize { todo!() }

    pub fn is_empty(&self) -> bool { todo!() }
}

pub struct RangeSpacedList<S: Spacing, T> {
    skeleton: Rc<RefCell<Skeleton<Range, S, T>>>,
    size: usize,
}

impl<S: Spacing, T> RangeSpacedList<S, T> {
    pub fn push(&mut self, spacing: S, span: S, value: T) -> Position<Node, S, T> { todo!() }

    pub fn insert(&mut self, start: S, end: S, value: T) -> Position<Node, S, T> { todo!() }

    pub fn insert_with_span(&mut self, start: S, span: S, value: T) -> Position<Node, S, T> { todo!() }


    pub fn increase_spacing_after(&mut self, position: S, spacing: S) { todo!() }

    pub fn increase_spacing_before(&mut self, position: S, spacing: S) { todo!() }

    pub fn decrease_spacing_after(&mut self, position: S, spacing: S) { todo!() }

    pub fn decrease_spacing_before(&mut self, position: S, spacing: S) { todo!() }


    pub fn starting_or_ending_before(&self, position: S) -> Option<Position<Node, S, T>> { todo!() }

    pub fn starting_or_ending_at_or_before(&self, position: S) -> Option<Position<Node, S, T>> { todo!() }

    pub fn starting_or_ending_at(&self, position: S) -> Option<Position<Node, S, T>> { todo!() }

    pub fn starting_or_ending_at_or_after(&self, position: S) -> Option<Position<Node, S, T>> { todo!() }

    pub fn starting_or_ending_after(&self, position: S) -> Option<Position<Node, S, T>> { todo!() }


    pub fn starting_before(&self, position: S) -> Option<Position<Node, S, T>> { todo!() }

    pub fn starting_at_or_before(&self, position: S) -> Option<Position<Node, S, T>> { todo!() }

    pub fn starting_at(&self, position: S) -> Option<Position<Node, S, T>> { todo!() }

    pub fn starting_at_or_after(&self, position: S) -> Option<Position<Node, S, T>> { todo!() }

    pub fn starting_after(&self, position: S) -> Option<Position<Node, S, T>> { todo!() }


    pub fn ending_before(&self, position: S) -> Option<Position<Node, S, T>> { todo!() }

    pub fn ending_at_or_before(&self, position: S) -> Option<Position<Node, S, T>> { todo!() }

    pub fn ending_at(&self, position: S) -> Option<Position<Node, S, T>> { todo!() }

    pub fn ending_at_or_after(&self, position: S) -> Option<Position<Node, S, T>> { todo!() }

    pub fn ending_after(&self, position: S) -> Option<Position<Node, S, T>> { todo!() }


    pub fn iter(&self) -> impl Iterator<Item=Position<Node, S, T>> { todo!() }

    pub fn into_iter(self) -> impl Iterator<Item=Position<Node, S, T>> { todo!() }

    pub fn iter_ranges(&self) -> impl Iterator<Item=(Position<Node, S, T>, Position<Node, S, T>)> { todo!() }

    pub fn into_iter_ranges(self) -> impl Iterator<Item=(Position<Node, S, T>, Position<Node, S, T>)> { todo!() }


    pub fn size(&self) -> usize { todo!() }

    pub fn is_empty(&self) -> bool { todo!() }
}

pub struct HollowSpacedList<S: Spacing> {
    skeleton: Rc<RefCell<Skeleton<Node, S, ()>>>,
    size: usize,
}

impl<S: Spacing> HollowSpacedList<S> {
    pub fn push(&mut self, spacing: S) -> HollowPosition<Node, S> { todo!() }

    pub fn insert(&mut self, position: S) -> HollowPosition<Node, S> { todo!() }


    pub fn increase_spacing_after(&mut self, position: S, spacing: S) { todo!() }

    pub fn increase_spacing_before(&mut self, position: S, spacing: S) { todo!() }

    pub fn decrease_spacing_after(&mut self, position: S, spacing: S) { todo!() }

    pub fn decrease_spacing_before(&mut self, position: S, spacing: S) { todo!() }


    pub fn before(&self, position: S) -> Option<HollowPosition<Node, S>> { todo!() }

    pub fn at_or_before(&self, position: S) -> Option<HollowPosition<Node, S>> { todo!() }

    pub fn at(&self, position: S) -> Option<HollowPosition<Node, S>> { todo!() }

    pub fn at_or_after(&self, position: S) -> Option<HollowPosition<Node, S>> { todo!() }

    pub fn after(&self, position: S) -> Option<HollowPosition<Node, S>> { todo!() }


    pub fn iter(&self) -> impl Iterator<Item=HollowPosition<Node, S>> { todo!() }

    pub fn into_iter(self) -> impl Iterator<Item=HollowPosition<Node, S>> { todo!() }


    pub fn size(&self) -> usize { todo!() }

    pub fn is_empty(&self) -> bool { todo!() }
}

pub struct HollowRangeSpacedList<S: Spacing> {
    skeleton: Rc<RefCell<Skeleton<Range, S, ()>>>,
    size: usize,
}

impl<S: Spacing> HollowRangeSpacedList<S> {
    pub fn push(&mut self, spacing: S, span: S) -> HollowPosition<Node, S> { todo!() }

    pub fn insert(&mut self, start: S, end: S) -> HollowPosition<Node, S> { todo!() }

    pub fn insert_with_span(&mut self, start: S, span: S) -> HollowPosition<Node, S> { todo!() }


    pub fn increase_spacing_after(&mut self, position: S, spacing: S) { todo!() }

    pub fn increase_spacing_before(&mut self, position: S, spacing: S) { todo!() }

    pub fn decrease_spacing_after(&mut self, position: S, spacing: S) { todo!() }

    pub fn decrease_spacing_before(&mut self, position: S, spacing: S) { todo!() }


    pub fn starting_or_ending_before(&self, position: S) -> Option<HollowPosition<Node, S>> { todo!() }

    pub fn starting_or_ending_at_or_before(&self, position: S) -> Option<HollowPosition<Node, S>> { todo!() }

    pub fn starting_or_ending_at(&self, position: S) -> Option<HollowPosition<Node, S>> { todo!() }

    pub fn starting_or_ending_at_or_after(&self, position: S) -> Option<HollowPosition<Node, S>> { todo!() }

    pub fn starting_or_ending_after(&self, position: S) -> Option<HollowPosition<Node, S>> { todo!() }


    pub fn starting_before(&self, position: S) -> Option<HollowPosition<Node, S>> { todo!() }

    pub fn starting_at_or_before(&self, position: S) -> Option<HollowPosition<Node, S>> { todo!() }

    pub fn starting_at(&self, position: S) -> Option<HollowPosition<Node, S>> { todo!() }

    pub fn starting_at_or_after(&self, position: S) -> Option<HollowPosition<Node, S>> { todo!() }

    pub fn starting_after(&self, position: S) -> Option<HollowPosition<Node, S>> { todo!() }


    pub fn ending_before(&self, position: S) -> Option<HollowPosition<Node, S>> { todo!() }

    pub fn ending_at_or_before(&self, position: S) -> Option<HollowPosition<Node, S>> { todo!() }

    pub fn ending_at(&self, position: S) -> Option<HollowPosition<Node, S>> { todo!() }

    pub fn ending_at_or_after(&self, position: S) -> Option<HollowPosition<Node, S>> { todo!() }

    pub fn ending_after(&self, position: S) -> Option<HollowPosition<Node, S>> { todo!() }


    pub fn iter(&self) -> impl Iterator<Item=HollowPosition<Node, S>> { todo!() }

    pub fn into_iter(self) -> impl Iterator<Item=HollowPosition<Node, S>> { todo!() }

    pub fn iter_ranges(&self) -> impl Iterator<Item=(HollowPosition<Node, S>, HollowPosition<Node, S>)> { todo!() }

    pub fn into_iter_ranges(self) -> impl Iterator<Item=(HollowPosition<Node, S>, HollowPosition<Node, S>)> { todo!() }


    pub fn size(&self) -> usize { todo!() }

    pub fn is_empty(&self) -> bool { todo!() }
}
use std::cell::{Ref, RefCell, RefMut};
use std::mem;
use std::rc::Rc;

use crate::skeleton::{link_index, Node, Skeleton, Spacing, ParentData};

macro_rules! traverse {
    // region loop
    // region break if ==
    (@break if ==(<, $target:ident; $position:ident)) => {};
    (@break if ==(<=, $target:ident; $position:ident)) => {
        if $position == $target {
            break;
        }
    };
    // endregion

    // region into sublist if deep
    (@into sublist if deep(shallow, $($_rest:tt)*)) => {};
    (@into sublist if deep(deep, $cmp:tt, $target:ident;
        $skeleton:ident, $degree:ident, $index:ident, $position:ident)) => {
        if let Some(sub) = $skeleton.clone().borrow().sub($index) {
            let next_position = $position + sub.borrow().offset;
            if next_position $cmp $target {
                $degree = sub.borrow().depth.saturating_sub(1);
                $index = 0;
                $position = next_position;
                $skeleton = sub;
                continue;
            }
        }
    };
    // endregion

    // region redirect loop cmp
    (@loop($depth:ident, ==, $($rest:tt)*)) => {
        traverse!(@loop($depth, <=, $($rest)*))
    };
    (@loop($depth:ident, >=, $($rest:tt)*)) => {
        traverse!(@loop($depth, <=, $($rest)*))
    };
    (@loop($depth:ident, >, $($rest:tt)*)) => {
        traverse!(@loop($depth, <=, $($rest)*))
    };
    // endregion

    (@loop($depth:ident, $cmp:tt, $target:ident, $bound:ident;
        $skeleton:ident, $degree:ident, $index:ident, $position:ident)) => {
        loop {
            let link_index = link_index($index, $degree);
            if !$skeleton.borrow().link_index_is_in_bounds(link_index) {
                if $degree > 0 {
                    $degree -= 1;
                    continue;
                } else {
                    break;
                }
            }

            let next_position = $position + $skeleton.borrow().links[link_index];
            if next_position $cmp $target {
                $position = next_position;
                $index += 1 << $degree;
                traverse!(@break if ==($cmp, $target; $position));
            }

            if $degree > 0 {
                $degree -= 1;
            } else {
                traverse!(@into sublist if deep($depth, $cmp, $target;
                    $skeleton, $degree, $index, $position));
                break;
            }
        }
    };
    // endregion

    // region next
    (@next(shallow; $skeleton:ident, $index:ident, $position:ident)) => {
        if $index == $skeleton.borrow().links.len() {
            Err("Tried to move to next element but it's already the end of the skeleton")
        } else {
            $position += $skeleton.borrow().link($index);
            $index += 1;
            Ok(())
        }
    };
    (@next(deep; $skeleton:ident, $index:ident, $position:ident)) => {
        if $index == $skeleton.borrow().links.len() {
            if let Some(ParentData { parent, index_in_parent }) =
                &$skeleton.clone().borrow().parent_data {
                $position -= $skeleton.borrow().last_position();
                $skeleton = parent.upgrade().unwrap();
                $position += $skeleton.borrow().link(*index_in_parent);
                $index = index_in_parent + 1;
                Ok(())
            } else {
                Err("Tried to move to next element but it's already the end of the skeleton")
            }
        } else if let Some(sub) = $skeleton.clone().borrow().sub($index) {
            $skeleton = sub;
            $index = 0;
            $position += $skeleton.borrow().offset;
            Ok(())
        } else {
            $position += $skeleton.borrow().link($index);
            $index += 1;
            Ok(())
        }
    };
    // endregion

    // region after loop
    (@after loop($depth:ident, <, $target:ident, any;
        $skeleton:ident, $degree:ident, $index:ident, $position:ident)) => {
        Some(Position::new($skeleton, $index, $position))
    };
    (@after loop($depth:ident, <=, $target:ident, any;
        $skeleton:ident, $degree:ident, $index:ident, $position:ident)) => {
        Some(Position::new($skeleton, $index, $position))
    };
    (@after loop($depth:ident, ==, $target:ident, any;
        $skeleton:ident, $degree:ident, $index:ident, $position:ident)) => {
        if $position == $target {
            Some(Position::new($skeleton, $index, $position))
        } else {
            None
        }
    };
    (@after loop($depth:ident, >=, $target:ident, any;
        $skeleton:ident, $degree:ident, $index:ident, $position:ident)) => {
        if $position == $target {
            Some(Position::new($skeleton, $index, $position))
        } else {
            traverse!(@next($depth; $skeleton, $index, $position)).unwrap();
            Some(Position::new($skeleton, $index, $position))
        }
    };
    (@after loop($depth:ident, >, $target:ident, any;
        $skeleton:ident, $degree:ident, $index:ident, $position:ident)) => {
        {
            traverse!(@next($depth; $skeleton, $index, $position)).unwrap();
            Some(Position::new($skeleton, $index, $position))
        }
    };
    // TODO after loop for start and end
    // endregion

    // region checked
    (@checked(shallow, $cmp:tt, $target:ident, $bound:ident; $skeleton:ident)) => {
        {
            let mut degree = $skeleton.borrow().depth - 1;
            let mut index = 0;
            let mut position = $skeleton.borrow().offset;
            traverse!(@loop(shallow, $cmp, $target, $bound;
                $skeleton, degree, index, position));
            traverse!(@after loop(shallow, $cmp, $target, $bound;
                $skeleton, degree, index, position))
        }
    };
    (@checked(deep, $cmp:tt, $target:ident, $bound:ident; $skeleton:ident)) => {
        {
            let mut skeleton = $skeleton;
            let mut degree = skeleton.borrow().depth - 1;
            let mut index = 0;
            let mut position = skeleton.borrow().offset;
            traverse!(@loop(deep, $cmp, $target, $bound;
                skeleton, degree, index, position));
            traverse!(@after loop(deep, $cmp, $target, $bound;
                skeleton, degree, index, position))
        }
    };
    // endregion

    // region checks
    // region out of bounds condition
    (@out of bounds condition(<, $target:ident; $skeleton:ident)) => {
        $target <= $skeleton.borrow().offset
    };
    (@out of bounds condition(<=, $target:ident; $skeleton:ident)) => {
        $target < $skeleton.borrow().offset
    };
    (@out of bounds condition(==, $target:ident; $skeleton:ident)) => {
        $target < $skeleton.borrow().offset ||
            $target > $skeleton.borrow().last_position()
    };
    (@out of bounds condition(>=, $target:ident; $skeleton:ident)) => {
        $target > $skeleton.borrow().last_position()
    };
    (@out of bounds condition(>, $target:ident; $skeleton:ident)) => {
        $target >= $skeleton.borrow().last_position()
    };
    // endregion

    // region if zero is at bound
    (@if zero is at bound(any) { $($then:tt)* } else { $($else:tt)* }) => {
        $($then)*
    };
    (@if zero is at bound(start) { $($then:tt)* } else { $($else:tt)* }) => {
        $($then)*
    };
    (@if zero is at bound(end) { $($then:tt)* } else { $($else:tt)* }) => {
        $($else)*
    };
    // endregion

    (@($depth:ident, $cmp:tt, $target:ident, $bound:ident; $skeleton:ident)) => {
        if $skeleton.borrow().elements.is_empty() {
            None
        } else if traverse!(@out of bounds condition($cmp, $target; $skeleton)) {
            None
        } else if $skeleton.borrow().links.is_empty() {
            traverse!(@if zero is at bound($bound) {
                if $skeleton.borrow().offset $cmp $target {
                    Some(Position::new($skeleton.clone(), 0, $skeleton.borrow().offset))
                } else {
                    None
                }
            } else {
                None
            })
        } else {
            traverse!(@checked($depth, $cmp, $target, $bound; $skeleton))
        }
    };
    // endregion

    // region public
    ($skeleton:ident; $depth:ident; $cmp:tt $target:ident) => {
        traverse!(@($depth, $cmp, $target, any; $skeleton))
    };
    ($skeleton:ident; $depth:ident; $cmp:tt $target:ident at $bound:ident) => {
        traverse!(@($depth, $cmp, $target, $bound; $skeleton))
    };
    // endregion
}

impl<S: Spacing, T> Skeleton<Node, S, T> {
    pub(crate) fn push(&mut self, distance: S, element: T) {
        if self.elements.is_empty() {
            self.offset = distance;
            self.elements.push(element);
            return;
        }
        let index = self.push_link();
        self.inflate(index, distance);
        self.elements.push(element);
    }

    pub(crate) fn insert(this: Rc<RefCell<Self>>, position: S, element: T) {
        if this.borrow().elements.is_empty() {
            return this.borrow_mut().push(position, element);
        }
        if position < this.borrow().offset {
            let previous_first_position = this.borrow().offset;
            let previous_first_element = mem::replace(&mut this.borrow_mut().elements[0], element);
            let inflation_amount = previous_first_position - position;
            if !this.borrow().links.is_empty() {
                this.borrow_mut().inflate(0, inflation_amount);
                if let Some(sub) = this.borrow().sub(0) {
                    sub.borrow_mut().offset += inflation_amount;
                }
            }
            this.borrow_mut().offset = position;
            return Self::insert(this, previous_first_position, previous_first_element);
        }
        if position >= this.borrow().last_position() {
            return this.borrow_mut().push(position - this.borrow().last_position(), element);
        }
        let result = Self::shallow_at_or_before(this.clone(), position).unwrap();
        let sub = Self::ensure_sub(this, result.index);
        return Self::insert(sub, position - result.position, element);
    }

    fn shallow_before(this: Rc<RefCell<Self>>, target: S) -> Option<Position<S, T>> {
        traverse!(this; shallow; < target)
    }

    fn shallow_at_or_before(this: Rc<RefCell<Self>>, target: S) -> Option<Position<S, T>> {
        traverse!(this; shallow; <= target)
    }

    fn shallow_at(this: Rc<RefCell<Self>>, target: S) -> Option<Position<S, T>> {
        traverse!(this; shallow; == target)
    }

    fn shallow_at_or_after(this: Rc<RefCell<Self>>, target: S) -> Option<Position<S, T>> {
        traverse!(this; shallow; >= target)
    }

    fn shallow_after(this: Rc<RefCell<Self>>, target: S) -> Option<Position<S, T>> {
        traverse!(this; shallow; > target)
    }

    fn before(this: Rc<RefCell<Self>>, target: S) -> Option<Position<S, T>> {
        traverse!(this; deep; < target)
    }

    fn at_or_before(this: Rc<RefCell<Self>>, target: S) -> Option<Position<S, T>> {
        traverse!(this; deep; <= target)
    }

    fn at(this: Rc<RefCell<Self>>, target: S) -> Option<Position<S, T>> {
        traverse!(this; deep; == target)
    }

    fn at_or_after(this: Rc<RefCell<Self>>, target: S) -> Option<Position<S, T>> {
        traverse!(this; deep; >= target)
    }

    fn after(this: Rc<RefCell<Self>>, target: S) -> Option<Position<S, T>> {
        traverse!(this; deep; > target)
    }
}

pub struct Position<S: Spacing, T> {
    skeleton: Rc<RefCell<Skeleton<Node, S, T>>>,
    index: usize,
    position: S,
}

impl<S: Spacing, T> Clone for Position<S, T> {
    fn clone(&self) -> Self {
        Self {
            skeleton: self.skeleton.clone(),
            index: self.index,
            position: self.position,
        }
    }
}

impl<S: Spacing, T> Position<S, T> {
    pub(crate) fn new(skeleton: Rc<RefCell<Skeleton<Node, S, T>>>, index: usize, position: S) -> Self {
        Self {
            skeleton,
            index,
            position,
        }
    }

    pub fn position(&self) -> S {
        self.position
    }

    pub fn element(&self) -> Ref<T> {
        Ref::map(RefCell::borrow(&self.skeleton),
                 |skeleton| &skeleton.elements[self.index])
    }

    pub fn element_mut(&self) -> RefMut<T> {
        RefMut::map(RefCell::borrow_mut(&self.skeleton),
                    |skeleton| &mut skeleton.elements[self.index])
    }
}
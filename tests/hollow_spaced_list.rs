#![allow(dead_code)]

use std::collections::BTreeSet;
use std::time::Instant;
use itertools::Itertools;
use rand::prelude::StdRng;
use rand::{random, Rng, SeedableRng};

use spaced_list_5::HollowSpacedList;

#[allow(unused_variables)]
#[test]
fn randomized() {
    let mut list: HollowSpacedList<i32> = HollowSpacedList::new();
    let mut set: BTreeSet<i32> = BTreeSet::new();
    let range = -100_000_000..100_000_000;

    let seed = random();
    // let seed = 2;

    let timestamp = Instant::now();
    let mut rng = StdRng::seed_from_u64(seed);
    for _n in 0..40_000 {
        let pos = rng.gen_range(range.clone());
        // let pos = _n;

        set.insert(pos);
    }
    println!("insert into set: {:?}", timestamp.elapsed());

    let timestamp = Instant::now();
    let mut rng = StdRng::seed_from_u64(seed);
    for _n in 0..40_000 {
        let pos = rng.gen_range(range.clone());
        // let pos = _n;

        // println!("{}", pos);

        if list.at(pos).is_none() {
            list.insert(pos);
        }
        // set.insert(pos);
    }
    println!("insert into list: {:?}", timestamp.elapsed());

    let timestamp = Instant::now();
    let mut list_iter = list.iter();
    let set_iter = set.iter();
    for (index, &position) in set_iter.enumerate() {
        let list_next_position = list_iter.next().unwrap().position();
        // println!("{}: expected: {}, actual: {}", index, position, list_next_position);
        assert_eq!(position, list_next_position);
    }
    println!("iterate over both: {:?}", timestamp.elapsed());

    let timestamp = Instant::now();
    let mut list_iter = list.iter_backwards();
    let set_iter = set.iter().rev();
    for (index, &position) in set_iter.enumerate() {
        let list_next_position = list_iter.next().unwrap().position();
        // println!("{}: expected: {}, actual: {}", index, position, list_next_position);
        assert_eq!(position, list_next_position);
    }
    println!("iterate over both in reverse: {:?}", timestamp.elapsed());

    let timestamp = Instant::now();
    for _ in 0..1_000 {
        let pos = rng.gen_range(range.clone());
        assert_eq!(list.before(pos).map(|it| it.position()),
                   set.iter().take_while(|it| **it < pos).last().copied());
        assert_eq!(list.at_or_before(pos).map(|it| it.position()),
                   set.iter().take_while(|it| **it <= pos).last().copied());
        assert_eq!(list.at(pos).map(|it| it.position()),
                   set.get(&pos).copied());
        assert_eq!(list.at_or_after(pos).map(|it| it.position()),
                   set.iter().rev().take_while(|it| **it >= pos).last().copied());
        assert_eq!(list.after(pos).map(|it| it.position()),
                   set.iter().rev().take_while(|it| **it > pos).last().copied());
    }
    println!("traversal tests: {:?}", timestamp.elapsed());
}

#[test]
fn iterate() {
    let mut list: HollowSpacedList<u64> = HollowSpacedList::new();
    list.insert(13);
    list.insert(7);
    list.insert(8);
    list.insert(15);
    list.insert(20);
    list.insert(16);
    let mut iter = list.iter();
    assert_eq!(iter.next().unwrap().position(), 7);
    assert_eq!(iter.next().unwrap().position(), 8);
    assert_eq!(iter.next().unwrap().position(), 13);
    assert_eq!(iter.next().unwrap().position(), 15);
    assert_eq!(iter.next().unwrap().position(), 16);
    assert_eq!(iter.next().unwrap().position(), 20);
    let mut iter = list.iter_backwards();
    assert_eq!(iter.next().unwrap().position(), 20);
    assert_eq!(iter.next().unwrap().position(), 16);
    assert_eq!(iter.next().unwrap().position(), 15);
    assert_eq!(iter.next().unwrap().position(), 13);
    assert_eq!(iter.next().unwrap().position(), 8);
    assert_eq!(iter.next().unwrap().position(), 7);
}

#[test]
fn change_spacing() {
    let mut list: HollowSpacedList<u64> = HollowSpacedList::new();
    list.insert(47);
    list.insert(5);
    list.insert(14);
    list.insert(13);
    list.increase_spacing_after(5, 3);
    assert_eq!(list.at(5).unwrap().position(), 5);
    assert!(list.at(13).is_none());
    assert_eq!(list.at(16).unwrap().position(), 16);
    assert_eq!(list.at(17).unwrap().position(), 17);
    assert_eq!(list.after(20).unwrap().position(), 50);
    list.increase_spacing_after(0, 100);
    assert_eq!(list.at(105).unwrap().position(), 105);
    assert_eq!(list.at(117).unwrap().position(), 117);
    assert_eq!(list.at(150).unwrap().position(), 150);
    list.decrease_spacing_after(105, 10);
    assert_eq!(list.at(105).unwrap().position(), 105);
    assert_eq!(list.at(107).unwrap().position(), 107);
    assert_eq!(list.at(140).unwrap().position(), 140);
    list.increase_spacing_before(105, 20);
    assert_eq!(list.at(125).unwrap().position(), 125);
    assert_eq!(list.at(160).unwrap().position(), 160);
    list.decrease_spacing_before(130, 10);
    assert_eq!(list.at(125).unwrap().position(), 125);
    assert_eq!(list.at(150).unwrap().position(), 150);
}

#[test]
fn queries() {
    let positions = [0, 1, 3, 5];
    for positions in positions.iter().permutations(positions.len()) {
        let mut list: HollowSpacedList<i64> = HollowSpacedList::new();
        for &position in positions {
            list.insert(position);
        }

        // region -1
        let query_pos = -1;

        let pos = list.before(query_pos);
        assert!(pos.is_none());

        let pos = list.at_or_before(query_pos);
        assert!(pos.is_none());

        let pos = list.at(query_pos);
        assert!(pos.is_none());

        let pos = list.at_or_after(query_pos).unwrap();
        assert_eq!(pos.position(), 0);

        let pos = list.after(query_pos).unwrap();
        assert_eq!(pos.position(), 0);
        // endregion

        // region 0
        let query_pos = 0;

        let pos = list.before(query_pos);
        assert!(pos.is_none());

        let pos = list.at_or_before(query_pos).unwrap();
        assert_eq!(pos.position(), 0);

        let pos = list.at(query_pos).unwrap();
        assert_eq!(pos.position(), 0);

        let pos = list.at_or_after(query_pos).unwrap();
        assert_eq!(pos.position(), 0);

        let pos = list.after(query_pos).unwrap();
        assert_eq!(pos.position(), 1);
        // endregion

        // region 1
        let query_pos = 1;

        let pos = list.before(query_pos).unwrap();
        assert_eq!(pos.position(), 0);

        let pos = list.at_or_before(query_pos).unwrap();
        assert_eq!(pos.position(), 1);

        let pos = list.at(query_pos).unwrap();
        assert_eq!(pos.position(), 1);

        let pos = list.at_or_after(query_pos).unwrap();
        assert_eq!(pos.position(), 1);

        let pos = list.after(query_pos).unwrap();
        assert_eq!(pos.position(), 3);
        // endregion

        // region 2
        let query_pos = 2;

        let pos = list.before(query_pos).unwrap();
        assert_eq!(pos.position(), 1);

        let pos = list.at_or_before(query_pos).unwrap();
        assert_eq!(pos.position(), 1);

        let pos = list.at(query_pos);
        assert!(pos.is_none());

        let pos = list.at_or_after(query_pos).unwrap();
        assert_eq!(pos.position(), 3);

        let pos = list.after(query_pos).unwrap();
        assert_eq!(pos.position(), 3);
        // endregion

        // region 3
        let query_pos = 3;

        let pos = list.before(query_pos).unwrap();
        assert_eq!(pos.position(), 1);

        let pos = list.at_or_before(query_pos).unwrap();
        assert_eq!(pos.position(), 3);

        let pos = list.at(query_pos).unwrap();
        assert_eq!(pos.position(), 3);

        let pos = list.at_or_after(query_pos).unwrap();
        assert_eq!(pos.position(), 3);

        let pos = list.after(query_pos).unwrap();
        assert_eq!(pos.position(), 5);
        // endregion

        // region 4
        let query_pos = 4;

        let pos = list.before(query_pos).unwrap();
        assert_eq!(pos.position(), 3);

        let pos = list.at_or_before(query_pos).unwrap();
        assert_eq!(pos.position(), 3);

        let pos = list.at(query_pos);
        assert!(pos.is_none());

        let pos = list.at_or_after(query_pos).unwrap();
        assert_eq!(pos.position(), 5);

        let pos = list.after(query_pos).unwrap();
        assert_eq!(pos.position(), 5);
        // endregion

        // region 5
        let query_pos = 5;

        let pos = list.before(query_pos).unwrap();
        assert_eq!(pos.position(), 3);

        let pos = list.at_or_before(query_pos).unwrap();
        assert_eq!(pos.position(), 5);

        let pos = list.at(query_pos).unwrap();
        assert_eq!(pos.position(), 5);

        let pos = list.at_or_after(query_pos).unwrap();
        assert_eq!(pos.position(), 5);

        let pos = list.after(query_pos);
        assert!(pos.is_none());
        // endregion

        // region 6
        let query_pos = 6;

        let pos = list.before(query_pos).unwrap();
        assert_eq!(pos.position(), 5);

        let pos = list.at_or_before(query_pos).unwrap();
        assert_eq!(pos.position(), 5);

        let pos = list.at(query_pos);
        assert!(pos.is_none());

        let pos = list.at_or_after(query_pos);
        assert!(pos.is_none());

        let pos = list.after(query_pos);
        assert!(pos.is_none());
        // endregion
    }
}

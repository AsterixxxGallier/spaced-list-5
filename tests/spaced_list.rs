use spaced_list_5::SpacedList;

#[test]
fn conditional_traversal_test() {
    let mut list = SpacedList::new();
    list.insert(5, "foo");
    list.insert(6, "bar");
    list.insert(7, "baz");
    assert_eq!(list.conditional_after(0, |str| str.starts_with('f')).unwrap().element().clone(), "foo");
    assert_eq!(list.conditional_after(0, |str| str.starts_with('b')).unwrap().element().clone(), "bar");
    assert_eq!(list.conditional_before(10, |str| str.starts_with('b')).unwrap().element().clone(), "baz");
    assert_eq!(list.conditional_before(10, |str| str.starts_with('f')).unwrap().element().clone(), "foo");
}
use spaced_list_5::HollowRangeSpacedList;

#[test]
fn ranges() {
    let mut list: HollowRangeSpacedList<u64> = HollowRangeSpacedList::new();
    list.append_range(5, 3);
    list.append_range(2, 1);
    list.insert_range(1, 2);
}
use algorithms_exercises::sortsofsorts::bubble_sort;

#[test]
pub fn bubble_sort_test() {
    let mut array = vec![4, 5, 6, 1];
    bubble_sort(&mut array);
    assert_eq!(vec![1, 4, 5, 6], array);
}

#[test]
pub fn bubble_sort_test2() {
    let mut array = vec![30, 24, 19, 3, 2, 7, 13];
    bubble_sort(&mut array);
    assert_eq!(vec![2, 3, 7, 13, 19, 24, 30], array);
}

#[test]
pub fn bubbl_sort_duplication_test() {
    let mut array = vec![30, 24, 19, 30, 3, 2, 7, 7, 13];
    bubble_sort(&mut array);
    assert_eq!(vec![2, 3, 7, 7, 13, 19, 24, 30, 30], array);
}

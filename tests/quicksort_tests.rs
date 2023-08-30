use algorithms_exercises::sortsofsorts::quicksort;

#[test]
pub fn quicksort_test() {
    let mut array = vec![4, 5, 6, 1];
    array = quicksort(array);
    assert_eq!(vec![1, 4, 5, 6], array);
}

#[test]
pub fn quicksort_test2() {
    let mut array = vec![30, 24, 19, 3, 2, 7, 13];
    array = quicksort(array);
    assert_eq!(vec![2, 3, 7, 13, 19, 24, 30], array);
}

#[test]
pub fn quicksort_duplication_test() {
    let mut array = vec![30, 24, 19, 30, 3, 2, 7, 7, 13];
    array = quicksort(array);
    assert_eq!(vec![2, 3, 7, 7, 13, 19, 24, 30, 30], array);
}

#[test]
pub fn quicksort_string_test() {
    let mut array = vec!["z", "t", "e", "a", "b", "c", "d"];
    array = quicksort(array);
    assert_eq!(vec!["a", "b", "c", "d", "e", "t", "z"], array);
}

#[test]
pub fn quicksort_string_test2() {
    let mut array = vec![
        "zdfea", "teda", "mele", "yeua", "ebwb", "eawb", "eawb", "xdfepc", "red",
    ];
    array = quicksort(array);
    assert_eq!(
        vec!["eawb", "eawb", "ebwb", "mele", "red", "teda", "xdfepc", "yeua", "zdfea",],
        array
    );
}

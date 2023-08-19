pub fn insertion_sort(arr: &mut Vec<i32>) {
    for j in 1..arr.len() {
        let mut key = j;
        while key > 0 && arr[key - 1] > arr[key] {
            arr.swap(key - 1, key);
            key -= 1;
        }
    }
}

pub fn bubble_sort(arr: &mut Vec<i32>) {
    assert!(arr.len() > 0);
    for _ in 0..arr.len() {
        let mut flag = false;
        for j in 0..(arr.len() - 1) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                flag = true;
            }
        }
        if flag == false {
            //there has been no reordering
            break;
        }
    }
}

#[test]
pub fn insertion_sort_test() {
    let mut array = vec![4, 5, 6, 1];
    insertion_sort(&mut array);
    assert_eq!(vec![1, 4, 5, 6], array);
}

#[test]
pub fn insertion_sort_test2() {
    let mut array = vec![30, 24, 19, 3, 2, 7, 13];
    insertion_sort(&mut array);
    assert_eq!(vec![2, 3, 7, 13, 19, 24, 30], array);
}

#[test]
pub fn insertion_sort_duplication_test() {
    let mut array = vec![30, 24, 19, 30, 3, 2, 7, 7, 13];
    insertion_sort(&mut array);
    assert_eq!(vec![2, 3, 7, 7, 13, 19, 24, 30, 30], array);
}

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

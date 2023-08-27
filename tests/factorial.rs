use algorithms_exercises::sortsofsorts::factorial_sum;

#[test]
pub fn factorial_sum_test() {
    let mut array: Vec<i64> = vec![4, 5, 6, 1];
    assert_eq!(16, factorial_sum(&mut array));
}

#[test]
pub fn factorial_sum_test2() {
    let mut array: Vec<i64> = (1..1000).collect();
    assert_eq!(499500, factorial_sum(&mut array));
}

#[test]
pub fn factorial_sum_test3() {
    let mut array: Vec<i64> = (1..3000).collect();
    assert_eq!(4498500, factorial_sum(&mut array));
}

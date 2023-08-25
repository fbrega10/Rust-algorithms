use algorithms_exercises::sortsofsorts::factorial_recursion;

#[test]
pub fn recursion_test() {
    let fact = factorial_recursion(6);
    let result: i64 = 720;
    assert_eq!(result, fact)
}

#[test]
pub fn recursion_test2() {
    let result = std::panic::catch_unwind(|| factorial_recursion(0));
    assert!(result.is_err())
}

#[test]
pub fn recursion_test3() {
    assert_eq!(40320, factorial_recursion(8))
}

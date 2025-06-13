use testing::splish;

#[test]
pub fn test_more_splish() {
    let test_a = splish(30, 10);
    assert!(test_a == 10, "a is {}", test_a);
}

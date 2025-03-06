use ready_set_boole::gray_code;

#[test]
fn test_zero() {
    assert_eq!(gray_code(0), 0);
}

#[test]
fn test_one() {
    assert_eq!(gray_code(1), 1);
}

#[test]
fn test_small_numbers() {
    assert_eq!(gray_code(2), 3);
    assert_eq!(gray_code(3), 2);
    assert_eq!(gray_code(4), 6);
    assert_eq!(gray_code(5), 7);
}

#[test]
fn test_large_numbers() {
    assert_eq!(gray_code(1024), 1536);
}

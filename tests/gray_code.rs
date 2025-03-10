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
fn test_adjacent_values() {
    for a in 0..u8::MAX as u32 {
        let ga = gray_code(a);
        let gb = gray_code(a + 1);
        let diff = ga ^ gb;
        assert_eq!(diff.count_ones(),  1, "{ga:b} {gb:b}");
    }
}

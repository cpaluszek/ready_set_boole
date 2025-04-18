use ready_set_boole::curve::{map, reverse_map};

#[test]
fn reversible_reversible() {
    for x in (0..=u16::MAX).step_by(42) {
        for y in (0..=u16::MAX).step_by(127) {
            let n = map(x, y);
            assert_eq!((x, y), reverse_map(n));
        }
    }
}


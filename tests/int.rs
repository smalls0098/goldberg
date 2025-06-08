use goldberg::{goldberg_int, goldberg_stringify};

#[test]
fn test_int() {
    let value = goldberg_int!(4u32);
    assert_eq!(value, 4);
}

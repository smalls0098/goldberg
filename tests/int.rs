use goldberg::{goldberg_int};

#[test]
fn test_int() {
    let value = goldberg_int!(4u32);
    assert_eq!(value, 4);
}
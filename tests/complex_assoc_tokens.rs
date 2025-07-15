use delegate_match::delegate_match;

#[derive(Debug)]
enum Ty {
    I32,
    I64,
}

impl Ty {
    fn size(&self) -> usize {
        delegate_match! {
            match self {
                // Normally we would simply put the type as associated tokens,
                // but this test verifies that we can use more complex streams instead.
                Ty::{ I32: core::mem::size_of::<i32>(), I64: core::mem::size_of::<i64>() } => {
                    $assoc_ts
                }
            }
        }
    }
}

#[test]
fn test_complex_assoc_tokens() {
    assert_eq!(Ty::I32.size(), std::mem::size_of::<i32>());
    assert_eq!(Ty::I64.size(), std::mem::size_of::<i64>());
}

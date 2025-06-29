use delegate_match::delegate_match;

#[derive(Debug)]
enum NumKind {
    I32,
    I64,
}

fn byte_size(k: NumKind) -> usize {
    delegate_match! {
        match k {
            NumKind::{ I32: core::mem::size_of::<i32>(), I64: core::mem::size_of::<i64>() } => {
                $assoc_ts
            }
        }
    }
}

#[test]
fn test_complex_assoc_tokens() {
    use NumKind::*;
    assert_eq!(byte_size(I32), std::mem::size_of::<i32>());
    assert_eq!(byte_size(I64), std::mem::size_of::<i64>());
}

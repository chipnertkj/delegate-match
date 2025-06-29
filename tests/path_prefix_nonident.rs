use delegate_match::delegate_match;

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

// Returns true for any input – the purpose is to ensure the macro expands
// when the entry pattern is a non-identifier (here: wildcard) while a path
// prefix (`Color::`) is present.
fn always_true(c: Color) -> bool {
    delegate_match! {
        match c {
            Color::{ _ } => {
                true
            }
        }
    }
}

#[test]
fn test_non_ident_path_prefix() {
    use Color::*;
    assert!(always_true(Red));
    assert!(always_true(Green));
    assert!(always_true(Blue));
}

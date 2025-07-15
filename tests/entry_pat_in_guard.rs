use delegate_match::delegate_match;

#[derive(Debug)]
enum Letter {
    A,
    B,
    C,
}

fn matches_letter(letter: Letter, expected: &str) -> bool {
    delegate_match! {
        match letter {
            // `$entry_pat` is used inside the guard expression.
            Letter::{ A, B, C } if expected == stringify!($entry_pat) => true,
            _ => false
        }
    }
}

#[test]
fn test_entry_pat_in_guard() {
    use Letter::*;
    assert!(matches_letter(A, "A"));
    assert!(matches_letter(B, "B"));
    assert!(matches_letter(C, "C"));
    assert!(!matches_letter(A, "B"));
}

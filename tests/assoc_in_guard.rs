use delegate_match::delegate_match;

#[derive(Debug)]
enum Num {
    A(i32),
    B(i32),
}

fn matches_assoc(num: Num) -> bool {
    delegate_match! {
        #[allow(clippy::redundant_guards, reason = "intentional test case")]
        match num {
            // `$assoc_ts` is used inside the guard expression.
            Num::{ A: 1, B: 2 }(v) if v == $assoc_ts => true,
            _ => false
        }
    }
}

#[test]
fn test_placeholder_in_guard() {
    use Num::*;
    assert!(matches_assoc(A(1)));
    assert!(matches_assoc(B(2)));
    assert!(!matches_assoc(A(2)));
    assert!(!matches_assoc(B(1)));
}

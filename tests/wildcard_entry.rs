use delegate_match::delegate_match;

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn is_red(&self) -> bool {
        delegate_match! {
            #[allow(clippy::match_single_binding, reason = "intentional test case")]
            match self {
                // Wildcard used as entry pattern.
                Color::{ Red: true, _: false } => $assoc_ts
            }
        }
    }
}

#[test]
fn test_wildcard_entry() {
    assert!(Color::Red.is_red());
    assert!(!Color::Green.is_red());
    assert!(!Color::Blue.is_red());
}

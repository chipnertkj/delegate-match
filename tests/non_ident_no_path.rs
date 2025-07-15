use delegate_match::delegate_match;

fn always_true() -> bool {
    delegate_match! {
        #[allow(clippy::match_single_binding, reason = "intentional test case")]
        match () {
            // The entry pattern is a wildcard with no path prefix.
            { _ } => true,
        }
    }
}

#[test]
fn test_non_ident_no_path() {
    assert!(always_true());
}

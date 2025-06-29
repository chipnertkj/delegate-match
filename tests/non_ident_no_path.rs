use delegate_match::delegate_match;

// The entry pattern is a wildcard `_` with *no* path prefix. This was previously only
// tested in a compile-fail scenario when an *extra* pattern followed. Here we show that
// the basic case expands correctly.
fn always_true() -> bool {
    delegate_match! {
        #[allow(clippy::match_single_binding)]
        match () {
            { _ } => true,
        }
    }
}

#[test]
fn test_non_ident_no_path() {
    assert!(always_true());
}

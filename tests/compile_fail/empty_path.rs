use delegate_match::delegate_match;

enum E {
    A,
}

fn main() {
    delegate_match! {
        match E::A {
            // Empty path prefix is not allowed.
            // Omit `::` to use entry patterns without prefixes.
            ::{ A } => {},
        }
    }
}

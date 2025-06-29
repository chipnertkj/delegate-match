use delegate_match::delegate_match;

enum E {
    A,
}

fn main() {
    delegate_match! {
        match E::A {
            ::{ A } => {},
        }
    }
}

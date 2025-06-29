use delegate_match::delegate_match;

enum E {
    A,
    B,
}

fn main() {
    delegate_match! {
        match E::A {
            E::{ A: ??, B: ?? } => {
                // Substitution of `$assoc_ts` yields `??`, which is not a valid Rust expr.
                $assoc_ts
            }
        }
    }
}

use delegate_match::delegate_match;

fn main() {
    delegate_match! {
        match 0 {
            { 1 2 3 } => {},
        }
    }
}

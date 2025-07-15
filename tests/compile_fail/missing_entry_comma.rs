use delegate_match::delegate_match;

fn main() {
    delegate_match! {
        match 0 {
            // Missing commas between entry patterns.
            { 1 2 3 } => {},
        }
    }
}

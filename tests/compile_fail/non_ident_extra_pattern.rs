use delegate_match::delegate_match;

fn main() {
    delegate_match! {
        match 5 {
            // This arm pattern is not compatible with the entry pattern.
            { 1 }(val) => val,
        }
    }
}

use delegate_match::delegate_match;

fn main() {
    delegate_match! {
        match 5 {
            { 1 }(val) => val,
        }
    }
}

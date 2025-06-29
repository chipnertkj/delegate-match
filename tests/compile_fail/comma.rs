use delegate_match::delegate_match;

fn main() {
    delegate_match! {
        match 1 {
            {1, 2} => ()
            _ => ()
        }
    }
}

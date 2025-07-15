use delegate_match::delegate_match;

fn main() {
    delegate_match! {
        match 1 {
            // This arm is missing a comma.
            // Macro should still work, but compiler will complain as expected.
            {1, 2} => ()
            _ => ()
        }
    }
}

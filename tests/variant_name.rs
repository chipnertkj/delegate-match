use delegate_match::delegate_match;

#[derive(Debug)]
enum Message {
    Hello,
    Bye,
}

// Entry pattern is used as a string literal to display the name of the variant.
impl AsRef<str> for Message {
    fn as_ref(&self) -> &str {
        delegate_match! {
            match self {
                Message::{ Hello, Bye } => {
                    stringify!($entry_pat)
                }
            }
        }
    }
}

#[test]
fn test_variant_name() {
    assert_eq!(Message::Hello.as_ref(), "Hello");
    assert_eq!(Message::Bye.as_ref(), "Bye");
}

use delegate_match::delegate_match;

#[derive(Debug)]
enum Message {
    Hello,
    Bye,
}

fn variant_name(msg: Message) -> &'static str {
    delegate_match! {
        match msg {
            Message::{ Hello, Bye } => {
                // `$entry_pat` is replaced by the actual variant identifier, so
                // the stringify! macro produces the variant name at compile time.
                stringify!($entry_pat)
            }
        }
    }
}

#[test]
fn test_variant_name() {
    assert_eq!(variant_name(Message::Hello), "Hello");
    assert_eq!(variant_name(Message::Bye), "Bye");
}

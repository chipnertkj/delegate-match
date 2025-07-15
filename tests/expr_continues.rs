use delegate_match::delegate_match;

#[derive(Copy, Clone)]
struct Data {
    value: i32,
}

fn get_value(d: Option<Data>) -> i32 {
    delegate_match! {
        match d {
            // Test body expression parsing with field access.
            { Some }(v) => v.value,
            _ => 0,
        }
    }
}

#[test]
fn test_expr_continues_field_access() {
    let d = Data { value: 7 };
    assert_eq!(get_value(Some(d)), 7);
    assert_eq!(get_value(None), 0);
}

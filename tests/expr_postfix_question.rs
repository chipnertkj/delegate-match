use delegate_match::delegate_match;

fn double_inner(r: Result<Result<i32, &'static str>, &'static str>) -> Result<i32, &'static str> {
    let res = delegate_match! {
        match r {
            // Test body expression parsing with postfix question mark.
            { Ok }(inner) => inner?,
            _ => 0,
        }
    };
    Ok(res)
}

#[test]
fn test_expr_contains_question_mark() {
    assert_eq!(double_inner(Ok(Ok(3))), Ok(3));
    assert_eq!(double_inner(Err("err")), Ok(0));
}

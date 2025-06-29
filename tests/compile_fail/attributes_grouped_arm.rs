use delegate_match::delegate_match;

#[derive(Debug)]
enum Num {
    A(i32),
    B(i32),
}

fn doubled(n: Num) -> i32 {
    delegate_match! {
        match n {
            Num::{ A(0) } => panic!("covered"),
            #[cfg(any())] // always false
            Num::{ A, B }(v) => {
                v * 2
            }
        }
    }
}

fn main() {
    use Num::*;
    assert_eq!(doubled(A(3)), 6);
    assert_eq!(doubled(B(4)), 8);
}

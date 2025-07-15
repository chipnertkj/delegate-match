use delegate_match::delegate_match;

struct A<T>(T);

trait B {
    fn as_i64(&self) -> i64;
}

impl<T: Into<i64> + Copy> B for A<T> {
    fn as_i64(&self) -> i64 {
        self.0.into()
    }
}

enum C {
    X(i32),
    Y(i32),
}

fn test(c: C) -> Box<dyn B> {
    delegate_match! {
        match c {
            // Expression.
            C::{ X: { 27 + 3 } + 1, Y: 317 }(v) if v == 0 => {
                Box::new(A($assoc_ts)) as Box<dyn B>
            }
            // Let expression.
            #[allow(clippy::let_and_return, reason = "intentional test case")]
            C::{ X: let b = Box::new(A(0)) as Box<dyn B> }(v) if v == 1 => {
                $assoc_ts;
                b
            }
            // Trait object type.
            C::{ Y: dyn B }(v) if v == 1 => {
                Box::new(A(0)) as Box<$assoc_ts>
            }
            // Concrete generic type.
            C::{ X: A::<i32>, Y: A::<i64> }(_) => {
                Box::new($assoc_ts(0)) as Box<dyn B>
            }
        }
    }
}

fn main() {
    let a = test(C::X(1));
    assert_eq!(a.as_i64(), 31);
    let a = test(C::Y(1));
    assert_eq!(a.as_i64(), 317);
}

use delegate_match::delegate_match;

struct A(i16);
struct B(i16);

enum Data {
    A(A),
    B(B),
}

impl A {
    fn calc(&self, x: i16) -> i16 {
        self.0 * x
    }
}

impl B {
    fn calc(&self, x: i16) -> i16 {
        self.0 / x
    }
}

impl Data {
    fn calc(&self, x: i16) -> i16 {
        delegate_match! {
            match self {
                Data::{ A, B }(val) if std::convert::identity::<_>(x) > 0 => {
                    val.calc(x)
                }
                Data::{ A, B }(val) => {
                    val.calc(x) * 2
                }
            }
        }
    }
}

fn main() {
    assert_eq!(Data::calc(&Data::A(A(10)), 10), 100);
    assert_eq!(Data::calc(&Data::B(B(10)), -10), -2);
}

use delegate_match::delegate_match;

struct Inner {
    data: usize,
}

struct A {
    inner: Inner,
}

struct B {
    inner: Inner,
}

enum Test {
    A(A),
    B(B),
    C(B), // also B
}

impl Test {
    fn a(data: usize) -> Self {
        Self::A(A {
            inner: Inner { data },
        })
    }
    fn b(data: usize) -> Self {
        Self::B(B {
            inner: Inner { data },
        })
    }
    fn c(data: usize) -> Self {
        Self::C(B {
            inner: Inner { data },
        })
    }

    fn data(&self) -> usize {
        delegate_match! {
            match self {
                Test::{ A, B | C }(case) => {
                    case.inner.data
                }
            }
        }
    }
}

fn main() {
    let tests = [Test::a, Test::b, Test::c];

    tests.into_iter().enumerate().for_each(|(i, test)| {
        assert_eq!(test(i).data(), i);
    });
}

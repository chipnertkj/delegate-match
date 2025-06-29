use delegate_match::delegate_match;

struct A {
    a: i32,
}

struct B {
    a: A,
    b: i32,
}

struct C {
    a: A,
    b: B,
}

enum D {
    A(A),
    B(B),
    C(C),
}

fn calc_a(d: &D) -> i32 {
    delegate_match! {
        match d {
            D::{ A(A { a }), B(B { b: _, a: A { a } }), C(C { a: A{a}, b: _ }) } => *a,
        }
    }
}

fn calc_b(d: &D) -> i32 {
    delegate_match! {
        match d {
            D::{ B(B { b, a: _ }), C(C { a: _, b: B {b, a: _} }) } => *b,
            D::A(_) => 0,
        }
    }
}

#[test]
fn test_complex_struct_pat() {
    let a = D::A(A { a: 1 });
    assert_eq!(calc_a(&a), 1);
    assert_eq!(calc_b(&a), 0);
    let b = D::B(B {
        a: A { a: 1 },
        b: 2,
    });
    assert_eq!(calc_a(&b), 1);
    assert_eq!(calc_b(&b), 2);
    let c = D::C(C {
        a: A { a: 1 },
        b: B {
            a: A { a: 2 },
            b: 3,
        },
    });
    assert_eq!(calc_a(&c), 1);
    assert_eq!(calc_b(&c), 3);
}

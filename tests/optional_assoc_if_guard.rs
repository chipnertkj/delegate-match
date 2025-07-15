use delegate_match::delegate_match;

enum Data {
    I16(i16),
    I32(i32),
}

fn test(x: Data) -> i32 {
    delegate_match! {
        match x {
            // - If guard in a delegate arm.
            // - `$assoc_ts` only available for I16.
            Data::{ I16: as i32, I32 }(val) if val > 0 => {
                (val * 2) $assoc_ts
            }
            Data::{ I16: as i32, I32 }(val) => {
                -val $assoc_ts
            },
        }
    }
}

fn main() {
    assert_eq!(test(Data::I16(1)), 2);
    assert_eq!(test(Data::I32(-1)), 1);
}

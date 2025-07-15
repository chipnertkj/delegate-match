use delegate_match::delegate_match;

enum Data {
    I16(i16),
    I32(i32),
}

fn test(x: Data) -> i32 {
    delegate_match! {
        #[allow(clippy::let_and_return, redundant_semicolons, reason = "intentional test case")]
        match x {
            // - If guard in a delegate arm.
            // - `$assoc_ts` only available for I16.
            Data::{ I16: let val = val as i32, I32 }(val) if val > 0 => {
                let val = (val * 2);
                $assoc_ts;
                val
            }
            Data::{ I16: let val = val as i32, I32 }(val) => {
                let val = -val;
                $assoc_ts;
                val
            },
        }
    }
}

fn main() {
    assert_eq!(test(Data::I16(1)), 2);
    assert_eq!(test(Data::I32(-1)), 1);
}

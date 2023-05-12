mod other;

fn estimate_size(x: u32) -> u32 {
    if x < 256 {
        if x < 128 {
            return 1;
        } else {
            return 3;
        }
    } else if x < 1024 {
        if x > 1022 {
            panic!("Oh no, a failing corner case!");
        } else {
            return 5;
        }
    } else {
        if x < 2048 {
            return 7;
        } else {
            return 9;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn it_works() {
        assert_eq!(estimate_size(1024), 7);
    }

    // ANCHOR: proptest
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(10000))]
        #[test]
        fn doesnt_crash(x: u32) {
            estimate_size(x);
        }
    }
    // ANCHOR_END: proptest
}

#[cfg(kani)]
#[kani::proof]
fn harness_true() {
    assert!(1==1);
}

#[cfg(kani)]
#[kani::proof]
fn check_estimate_size() {
    let x: u32 = kani::any();
    estimate_size(x);
}

#[cfg(kani)]
#[kani::proof]
pub fn harness_i64() {
    let i64_1: i64 = kani::any();
    let i64_2: i64 = kani::any();
    let i64_3: i64 = kani::any();
    let i64_4: i64 = kani::any();
    let i64_5: i64 = kani::any();
    assert!(
        !(i64_1 == i64::MIN && i64_2 == -101 && i64_3 == 0 && i64_4 == 101 && i64_5 == i64::MAX)
    );
}

#[cfg(kani)]
#[kani::proof]
pub fn harness_i8() {
    let i8_1: i8 = kani::any();
    let i8_2: i8 = kani::any();
    let i8_3: i8 = kani::any();
    let i8_4: i8 = kani::any();
    let i8_5: i8 = kani::any();
    assert!(!(i8_1 == i8::MIN && i8_2 == -101 && i8_3 == 0 && i8_4 == 101 && i8_5 == i8::MAX));
}

fn main() {
}

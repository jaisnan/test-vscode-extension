#[cfg(test)]
mod test {
    #[test]
    #[cfg_attr(kani, kani::proof)]
    fn insert_test_80978342() {
        // Make sure the two packet numbers are not the same
        assert!(1 == 1);
    }
}

#[cfg(kani)]
mod harnesses {
    #[kani::proof]
    fn my_harness() {
        let result_1: Result<u8, u8> = kani::any();
        let result_2: Result<u8, u8> = kani::any();
        assert!(!(result_1 == Ok(101) && result_2 == Err(102)));
    }
    #[test]
    fn kani_concrete_playback_my_harness_5205172929174653629() {
        let concrete_vals: Vec<Vec<u8>> = vec![
            // 1
            vec![1],
            // 101
            vec![101],
            // 0
            vec![0],
            // 102
            vec![102],
        ];
        kani::concrete_playback_run(concrete_vals, my_harness);
    }
    // Test will be here.
}

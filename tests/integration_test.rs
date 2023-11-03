#![cfg(feature = "test_bin")]

mod common_tester;

#[cfg(test)]
mod tests {
    use crate::common_tester::test_bin;

    #[test]
    fn test_rerooting_abc222_g() {
        test_bin("rerooting_abc222-g", vec!["sample1"]);
    }
}

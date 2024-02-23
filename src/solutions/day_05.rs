use crate::common::Solution;
use crate::solutions::Day05;

impl Solution for Day05 {
    fn sol_1(input: String) {
        println!("input = `{input}`");
    }

    fn sol_2(input: String) {
        println!("input = `{input}`");
    }
}

#[test]
fn d0501() {
    let test_input: String = "hello"
        .to_string();

    Day05::sol_1(test_input);
}

#[test]
fn d0502() {
    let test_input: String = "hello"
        .to_string();

    Day05::sol_2(test_input);
}

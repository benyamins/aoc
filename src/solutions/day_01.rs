use crate::common::Solution;

use super::Day01;

impl Solution for Day01 {
    fn sol_1(input: String) {
        let mut result: u32 = 0;
        let mut calibration_value: Vec<char> = vec![];

        for l in input.lines() {
            for c in l.chars() {
                if c.is_digit(10) {
                    calibration_value.push(c);
                }
            }
            result += if calibration_value.len() > 1 {
                format!(
                    "{}{}",
                    calibration_value[0],
                    calibration_value.last().unwrap()
                )
                .parse::<u32>()
                .unwrap()
            } else {
                format!("{}{}", calibration_value[0], calibration_value[0])
                    .to_string()
                    .parse()
                    .unwrap()
            };
            calibration_value.clear();
        }

        println!("RESULT SOL1 {result}");
    }

    fn sol_2(input: String) {
        println!("{input}");
    }
}

#[test]
fn day01() {
    let test_input: String = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet".to_string();

    Day01::sol_1(test_input);
}

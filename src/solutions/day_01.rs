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
        fn find_calibration_values(line: String) -> String {
            let mut cal_values: Vec<(u8, usize)> = vec![];
            const DIGITS_DICT: [(u8, &str); 19] = [
                (1, "1"),
                (2, "2"),
                (3, "3"),
                (4, "4"),
                (5, "5"),
                (6, "6"),
                (7, "7"),
                (8, "8"),
                (9, "9"),
                (0, "0"),
                (1, "one"),
                (2, "two"),
                (3, "three"),
                (4, "four"),
                (5, "five"),
                (6, "six"),
                (7, "seven"),
                (8, "eight"),
                (9, "nine"),
            ];
            for (digit, d_text) in DIGITS_DICT {
                for i in line.match_indices(d_text).map(|(pos, _)| pos) {
                    cal_values.push((digit, i))
                }
            }
            cal_values.sort_by_key(|&(_, k)| k);
            if cal_values.len() > 1 {
                format!("{}{}", cal_values[0].0, cal_values.last().unwrap().0)
            } else {
                format!("{}{}", cal_values[0].0, cal_values[0].0)
            }
        }
        let mut result: u32 = 0;

        for l in input.lines() {
            result += (
                l,
                find_calibration_values(l.to_string())
                    .parse::<u32>()
                    .unwrap(),
            )
                .1;
        }

        println!("RESULT SOLUTION 2 `{result}`");
    }
}

#[test]
fn day01() {
    let test_input: String = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
        .to_string();

    Day01::sol_1(test_input);

    let test_input_02: String = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
        .to_string();

    Day01::sol_2(test_input_02);
}

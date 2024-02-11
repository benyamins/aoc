
use crate::common::Solution;
use crate::solutions::Day03;


impl Solution for Day03 {
    fn sol_1(input: String) {
        println!("RESULT SOLUTION 1 `{input}`");
    }

    fn sol_2(input: String) {
        println!("RESULT SOLUTION 2 `{input}`");
    }
}

#[test]
fn d0301() {
    let test_input: String = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
        .to_string();

    Day03::sol_1(test_input);
}

#[test]
fn d0302() {
    let test_input: String = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
        .to_string();

    Day03::sol_2(test_input);
}

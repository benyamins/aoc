use std::usize;

use crate::common::Solution;
use crate::solutions::Day03;

#[derive(Debug)]
struct PartNumber {
    number: usize,
    min_pos: usize,
    max_pos: usize,
}

impl Solution for Day03 {
    fn sol_1(input: String) {
        let line_len = input.lines().clone().next().unwrap().chars().count() + 1;
        let info_lines: Vec<&str> = input.lines().flat_map(|x| x.split("")).collect();
        let mut info: Vec<char> = vec![];
        for s in info_lines {
            s.chars().collect_into(&mut info);
        }

        let mut adjacent_positions: Vec<usize> = vec![];
        let mut building_num = String::new();
        let mut schematic_numbers: Vec<PartNumber> = vec![];
        let mut engine_number: usize = 0;
        let mut min_pos: usize = info.len();
        let mut max_pos: usize = 0;

        for (i, c) in info.iter().enumerate() {
            if c.is_numeric() {
                building_num.push(c.clone());
                max_pos = i.max(max_pos);
                min_pos = i.min(min_pos);
            } else if !c.is_numeric() && building_num.len() > 0 {
                if let Ok(num) = building_num.parse() {
                    schematic_numbers.push(PartNumber {
                        number: num,
                        min_pos,
                        max_pos,
                    })
                }
                building_num.clear();
                min_pos = info.len();
                max_pos = 0;
            }

            if *c != '.' && !c.is_numeric() {
                adjacent_positions.extend([
                    i - line_len,
                    i - (line_len - 1),
                    i - (line_len - 2),
                    i - 1,
                    i + 1,
                    i + (line_len - 2),
                    i + (line_len - 1),
                    i + line_len,
                ])
            }
        }

        // schematic_numbers.iter().for_each(|e| println!("{:?}", e));
        // println!("{:?}", adjacent_positions);

        for part_number in schematic_numbers {
            if adjacent_positions.contains(&part_number.max_pos)
                || adjacent_positions.contains(&part_number.min_pos)
            {
                // println!("{:?}", part_number);
                engine_number += part_number.number;
            }
        }

        println!("RESULT SOLUTION 1 `{engine_number}`");
    }

    fn sol_2(input: String) {
        let line_len = input.lines().clone().next().unwrap().chars().count() + 1;
        let info_lines: Vec<&str> = input.lines().flat_map(|x| x.split("")).collect();
        let mut info: Vec<char> = vec![];
        for s in info_lines {
            s.chars().collect_into(&mut info);
        }

        let mut adjacent_positions: Vec<[usize; 8]> = vec![];
        let mut building_num = String::new();
        let mut schematic_numbers: Vec<PartNumber> = vec![];
        let mut min_pos: usize = info.len();
        let mut max_pos: usize = 0;

        for (i, c) in info.iter().enumerate() {
            if c.is_numeric() {
                building_num.push(c.clone());
                max_pos = i.max(max_pos);
                min_pos = i.min(min_pos);
            } else if !c.is_numeric() && building_num.len() > 0 {
                if let Ok(num) = building_num.parse() {
                    schematic_numbers.push(PartNumber {
                        number: num,
                        min_pos,
                        max_pos,
                    })
                }
                building_num.clear();
                min_pos = info.len();
                max_pos = 0;
            }

            if *c == '*' {
                adjacent_positions.push([
                    i - line_len,
                    i - (line_len - 1),
                    i - (line_len - 2),
                    i - 1,
                    i + 1,
                    i + (line_len - 2),
                    i + (line_len - 1),
                    i + line_len,
                ])
            }
        }

        // schematic_numbers.iter().for_each(|e| println!("{:?}", e));
        // println!("{:?}", adjacent_positions);
        // schematic_numbers.iter().for_each(|e| println!("{:?}", e));

        let mut engine_numbers: Vec<usize> = vec![];
        let mut sum_gear_ratio: usize = 0;
        for gear_positions in adjacent_positions {
            for position in gear_positions {
                for schematic_number in &schematic_numbers {
                    if (position == schematic_number.min_pos
                        || position == schematic_number.max_pos)
                        && !engine_numbers.contains(&schematic_number.number)
                    {
                        engine_numbers.push(schematic_number.number);
                    }
                }
            }
            // println!("{:?}", engine_numbers);
            if engine_numbers.len() == 2 {
                // println!("pos: {:?}", gear_positions);
                sum_gear_ratio += engine_numbers[0] * engine_numbers[1];
            }
            engine_numbers.clear();
        }

        println!("RESULT SOLUTION 2 `{sum_gear_ratio}`");
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

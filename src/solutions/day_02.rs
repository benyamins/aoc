use crate::common::Solution;
use crate::solutions::Day02;

use std::collections::HashMap;

struct MaxNColors {
    blue: u32,
    green: u32,
    red: u32,
}

impl Solution for Day02 {
    fn sol_1(input: String) {
        let mut result: u32 = 0;

        let mut cube_colors: HashMap<String, u32> = HashMap::new();
        let max_cube_colors = MaxNColors {
            blue: 14,
            green: 13,
            red: 12,
        };

        cube_colors.insert("blue".to_string(), 0);
        cube_colors.insert("green".to_string(), 0);
        cube_colors.insert("red".to_string(), 0);

        let mut num: u32 = 0;
        let mut game_num: u32;
        for line in input.lines() {
            let game_details: Vec<&str> = line.split(' ').collect();
            game_num = game_details[1].trim_end_matches(":").parse().unwrap();
            for element in game_details {
                if element.starts_with("blue") {
                    if let Some(val) = cube_colors.get_mut("blue") {
                        if *val < num {
                            *val = num
                        };
                    }
                } else if element.starts_with("green") {
                    if let Some(val) = cube_colors.get_mut("green") {
                        if *val < num {
                            *val = num
                        };
                    }
                } else if element.starts_with("red") {
                    if let Some(val) = cube_colors.get_mut("red") {
                        if *val < num {
                            *val = num
                        };
                    }
                }
                num = match element.parse() {
                    Ok(val) => val,
                    Err(_) => continue,
                };
            }
            println!("{:?}", cube_colors);
            if cube_colors["blue"] <= max_cube_colors.blue
                && cube_colors["green"] <= max_cube_colors.green
                && cube_colors["red"] <= max_cube_colors.red
            {
                result += game_num;
            }

            for v in cube_colors.values_mut() {
                *v = 0;
            }
        }
        println!("RESULT SOLUTION 1 `{result}`");
    }

    fn sol_2(input: String) {
        let mut result: u32 = 0;

        let mut cube_colors: HashMap<String, u32> = HashMap::new();

        cube_colors.insert("blue".to_string(), 0);
        cube_colors.insert("green".to_string(), 0);
        cube_colors.insert("red".to_string(), 0);

        let mut num: u32 = 0;
        for line in input.lines() {
            let game_details: Vec<&str> = line.split(' ').collect();
            for element in game_details {
                if element.starts_with("blue") {
                    if let Some(val) = cube_colors.get_mut("blue") {
                        if *val < num {
                            *val = num
                        };
                    }
                } else if element.starts_with("green") {
                    if let Some(val) = cube_colors.get_mut("green") {
                        if *val < num {
                            *val = num
                        };
                    }
                } else if element.starts_with("red") {
                    if let Some(val) = cube_colors.get_mut("red") {
                        if *val < num {
                            *val = num
                        };
                    }
                }
                num = match element.parse() {
                    Ok(val) => val,
                    Err(_) => continue,
                };
            }

            result += cube_colors["blue"] * cube_colors["green"] * cube_colors["red"];

            // reset the game numbers
            for v in cube_colors.values_mut() {
                *v = 0;
            }
        }
        println!("RESULT SOLUTION 2 `{result}`");
    }
}

#[test]
fn d0201() {
    let test_input: String = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
        .to_string();

    Day02::sol_1(test_input);
}

#[test]
fn d0202() {
    let test_input: String = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
        .to_string();

    Day02::sol_2(test_input);
}

use crate::common::Solution;
use crate::solutions::Day04;

use std::collections::HashMap;

impl Solution for Day04 {
    fn sol_1(input: String) {
        let binding = input.clone();
        let card_delimitr_pos = binding
            .lines()
            .next()
            .unwrap()
            .split_whitespace()
            .position(|s| s == "|")
            .unwrap();
        let mut total_points = 0;
        for line in input.lines() {
            let card: Vec<_> = line.split_whitespace().collect();
            let (winning_nums, your_nums) = card.split_at(card_delimitr_pos); //.filter_map(|s| s.parse().ok()).collect();
            let winning_nums: Vec<u32> =
                winning_nums.iter().filter_map(|s| s.parse().ok()).collect();
            let your_nums: Vec<u32> = your_nums.iter().filter_map(|s| s.parse().ok()).collect();

            let mut points = 0;
            for winning_num in winning_nums {
                points = if your_nums.contains(&winning_num) {
                    match points {
                        0 => 1,
                        _ => points * 2,
                    }
                } else {
                    points
                };
            }
            total_points += points;
        }
        println!("Total Points = `{total_points}`");
    }

    fn sol_2(input: String) {
        let binding = input.clone();
        let card_delimitr_pos = binding
            .lines()
            .next()
            .unwrap()
            .split_whitespace()
            .position(|s| s == "|")
            .unwrap();
        let mut card_and_matches: Vec<(u32, u32)> = vec![];
        let mut card_instances: HashMap<u32, u32> = HashMap::new();
        for (i, line) in input.lines().enumerate() {
            let card: Vec<_> = line.split_whitespace().collect();
            let (winning_nums, your_nums) = card.split_at(card_delimitr_pos);
            let winning_nums: Vec<u32> =
                winning_nums.iter().filter_map(|s| s.parse().ok()).collect();
            let your_nums: Vec<u32> = your_nums.iter().filter_map(|s| s.parse().ok()).collect();

            let mut n_matches = 0;

            let card_number = i as u32 + 1;

            winning_nums.iter().for_each(|e| {
                if your_nums.contains(&e) {
                    n_matches += 1
                };
                card_instances.insert(card_number, 1);
            });

            card_and_matches.push((card_number, n_matches));
        }

        for (card, matched) in &card_and_matches {
            for i in card + 1..matched + card {
                if let Some(instance) = card_instances.get_mut(&i) {
                    *instance += 1;
                    println!("card = {card} card_instance_num = {i} instances = {instance}");
                } else {
                    println!("Index `{i}` not found");
                }
            }
        }

        println!(" = `{:?}` \n = {:?} ", card_and_matches, card_instances);
    }
}

#[test]
fn d0401() {
    let test_input: String = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
        .to_string();

    Day04::sol_1(test_input);
}

#[test]
fn d0402() {
    let test_input: String = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
        .to_string();

    Day04::sol_2(test_input);
}

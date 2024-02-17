#![feature(iter_collect_into)]
mod common;
mod solutions;
use crate::common::Solution;

use std::env;
use std::path;

fn main() {
    let input_data_path = path::Path::new("./data/");

    if !input_data_path.exists() {
        eprintln!("Data path - ie folder containing input files doesn't exists");
        return;
    }

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("not enough args..");
        return;
    }

    let day = match &args.get(1) {
        Some(val) => val.to_string(),
        _ => {
            eprintln!("No val!");
            return;
        }
    };

    let n_problem = match &args.get(2) {
        Some(val) => val.to_string(),
        _ => {
            eprintln!("No n_problem val!");
            return;
        }
    };

    let input_file_path = format!(
        "./data/input_{}.txt",
        if day.len() > 1 {
            day.clone()
        } else {
            format!("0{}", day.clone())
        }
    );

    println!("Using content from file: `{}`", input_file_path);

    let input_content = match std::fs::read_to_string(input_file_path) {
        Ok(input) => input,
        Err(error) => {
            eprintln!("Error reading the file!: {:?}", error);
            return;
        }
    };

    match select_day(&day, &n_problem, input_content) {
        None => {
            println!("The convination of day and problem number doesn't exist in the program...")
        }
        _ => return,
    }
}

fn select_day(day: &str, solution: &str, input_content: String) -> Option<()> {
    let (n_day, n_solution): (u16, u16) = match (day.parse(), solution.parse()) {
        (Ok(n_day), Ok(n_solution)) => (n_day, n_solution),
        errors => {
            println!("Error while parsing!: `{:?}`", errors);
            return None;
        }
    };

    match n_day {
        1 => solutions::Day01::solve(n_solution, input_content),
        2 => solutions::Day02::solve(n_solution, input_content),
        3 => solutions::Day03::solve(n_solution, input_content),
        4 => solutions::Day04::solve(n_solution, input_content),
        _ => todo!("NEED TO ADD THE DAY IN MAIN.rs you silly!"),
    }
    Some(())
}

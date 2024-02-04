mod solutions;
mod common;
use crate::common::Solution;

use std::env;

fn main() {

    let cmd_args = env::args();

    println!("{:?}", cmd_args);

    select_day("1", "1").expect("what..");
}

fn select_day(day: &str, solution: &str) -> Option<()>{
    let (n_day, n_solution): (u16, u16) = match (day.parse(), solution.parse()) {
        (Ok(n_day), Ok(n_solution)) => (n_day, n_solution),
        errors => {
            println!("Error while parsing!: `{:?}`", errors);
            return None
        }
    };

    match n_day {
        1 => solutions::Day01::solve(n_solution),
        _ => todo!()
    }
    Some(())
}

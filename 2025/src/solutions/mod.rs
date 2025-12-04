use crate::solver::Solver;
use regex::Regex;
use std::env;
use std::fs::{metadata, File};
use std::io::{self, Read};
use std::path::Path;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

fn read_file_to_string(filename: &str) -> io::Result<String> {
    let mut file = File::open(filename)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

pub fn run_solutions() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <input_file_name>", args[0]);
        std::process::exit(1);
    }
    let filepath = &args[1];
    if metadata(filepath).is_err() {
        eprintln!("Error: File '{}' does not exist.", filepath);
        std::process::exit(1);
    }
    let filename = Path::new(filepath).file_name().unwrap().to_str().unwrap();
    let mut day: u32 = 1;
    let re = Regex::new(r"input(\d+)\.txt").unwrap();
    if let Some(captures) = re.captures(filename) {
        if let Some(x_str) = captures.get(1) {
            let x: u32 = x_str.as_str().parse().unwrap();
            day = x;
        }
    }
    let input = read_file_to_string(filepath).unwrap();
    match day {
        1 => day01::Problem {}.solve(day, &input),
        2 => day02::Problem {}.solve(day, &input),
        3 => day03::Problem {}.solve(day, &input),
        4 => day04::Problem {}.solve(day, &input),
        5 => day05::Problem {}.solve(day, &input),
        _ => todo!(),
    }
}

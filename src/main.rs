/*
Guidelines for adding a new day:
- Source code for new day's puzzle should be in `src/day_x.rs`
- That code should have one public `main `function that can be called
    from here
- Add the module to the list below
- Utility functions can be broken out later
*/

#![feature(iter_next_chunk)]
use std::env;
use std::fs;

struct Config {
    year: i32,
    day: i32,
    test: bool,
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next(); // skip first arg
        let year = match args.next() {
            Some(year) => match year.parse::<i32>() {
                Ok(yr) => yr,
                Err(e) => panic!("Invalid Year Specified: {} - {}", year, e),
            },
            None => return Err("No year specified"),
        };
        let day = match args.next() {
            Some(day) => match day.parse::<i32>() {
                Ok(dy) => dy,
                Err(e) => panic!("Invalid Day Specified: {} - {}", day, e),
            },
            None => return Err("No day specified"),
        };

        // TODO: make this actually check something
        let test = args.next().is_some();

        Ok(Config { year, day, test })
    }
}

/*
   Read the input file, and return a vector of string slices.
   Remove the last elemenet if it's empty (generated by trailing newline)
*/
fn _read_input(file_path: &str) -> Vec<String> {
    let contents = fs::read_to_string(&file_path).unwrap();
    let mut lines: Vec<String> = contents.lines().map(|l| l.to_string()).collect();
    while !lines.is_empty() && lines[lines.len() - 1].is_empty() {
        lines.pop();
    }
    lines
}

fn main() {
    let config = Config::build(env::args()).unwrap();
    let test_marker = match config.test {
        true => ".test",
        false => "",
    };
    let input_path = format!("inputs/{}.{}{}", config.year, config.day, test_marker);
    println!("{input_path}");

    // let contents = read_input(&input_path);
    let contents = fs::read_to_string(&input_path).unwrap();
    match config.day {
        9 => day_9::main(contents),
        8 => day_8::main(contents),
        7 => day_7::main(contents),
        6 => day_6::main(contents),
        5 => day_5::main(contents),
        4 => day_4::main(contents),
        3 => day_3::main(contents),
        2 => day_2::main(contents),
        1 => day_1::main(contents),
        _ => panic!("Day not implemented!"),
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        assert_eq!(_read_input("inputs/2022.1.test").len(), 14);
    }
}
pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;
pub mod day_7;
pub mod day_8;
pub mod day_9;

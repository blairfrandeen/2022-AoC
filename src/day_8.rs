/*
AoC 2022 Day 8
How best to represent a grid in Rust?
*/
pub fn main(contents: String) {
    println!("Hello AoC!")
}

struct Grid {
    num_rows: u32,
    num_cols: u32,
    data: Vec<u8>,
}
fn parse_input(contents: String) -> Vec<u8> {
    let mut grid: Vec<u8> = Vec::new();
    for line in contents.lines() {
        let mut l: Vec<u8> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        grid.append(&mut l)
    }
    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let test_input = String::from("12345\n67890\n");
        assert_eq!(parse_input(test_input), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0]);
    }
}

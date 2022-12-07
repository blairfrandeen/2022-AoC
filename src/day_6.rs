// 4096 characters in input string

use std::collections::HashSet;

pub fn main(contents: String) {
    println!("Part 1: {}", find_marker(&contents, 4 as usize));
    println!("Part 2: {}", find_marker(&contents, 14 as usize));
}

fn find_marker(signal: &str, marker_size: usize) -> usize {
    let signal_chars: Vec<char> = signal.chars().collect();
    for i in 0..signal_chars.len() {
        let window = &signal_chars[i..i + marker_size];
        let signal_set: HashSet<&char> = HashSet::from_iter(window);
        // println!("{:?} - {}", window, signal_set.len());
        if signal_set.len() == marker_size {
            return i + marker_size;
        }
        // println!("{:?}", signal_set);
    }
    unreachable!()
}
#[cfg(test)]
mod tests {
    use super::*;

    fn test_cases_p1() -> Vec<(&'static str, usize)> {
        vec![
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ]
    }
    fn test_cases_p2() -> Vec<(&'static str, usize)> {
        vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ]
    }

    #[test]
    fn test_examples() {
        for example in test_cases_p1() {
            assert_eq!(find_marker(example.0, 4 as usize), example.1);
        }
    }

    #[test]
    fn test_p2() {
        for example in test_cases_p2() {
            assert_eq!(find_marker(example.0, 14 as usize), example.1);
        }
    }
}

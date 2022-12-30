use std::collections::HashMap;
use std::collections::VecDeque;

pub fn main(contents: String) {
    let mut monkeys: HashMap<String, i64> = HashMap::new();
    let mut unknowns: VecDeque<&str> = VecDeque::new();
    for line in contents.lines() {
        if let Some(entry) = parse_input(line) {
            monkeys.insert(entry.0, entry.1);
        } else {
            unknowns.push_front(line);
        }
    }
    while !unknowns.is_empty() {
        let next = unknowns.pop_back().expect("not empty");
        if let Some(entry) = parse_unknown(next, &monkeys) {
            monkeys.insert(entry.0, entry.1);
        } else {
            unknowns.push_front(next);
        }
    }
    println!("Part 1: {}", monkeys.get("root").expect("Root exists"));
    // println!("{:?}", monkeys);
    // println!("{:?}", unknowns);
}

fn parse_unknown(input: &str, monkeys: &HashMap<String, i64>) -> Option<(String, i64)> {
    let words: Vec<&str> = input.split_whitespace().collect();
    let key = words[0].replace(':', "");

    if monkeys.contains_key(words[1]) && monkeys.contains_key(words[3]) {
        let v1 = monkeys.get(words[1]).unwrap();
        let v2 = monkeys.get(words[3]).unwrap();
        match words[2] {
            "+" => Some((key, v1 + v2)),
            "-" => Some((key, v1 - v2)),
            "*" => Some((key, v1 * v2)),
            "/" => Some((key, v1 / v2)),
            _ => panic!("Bad Operator"),
        }
    } else {
        None
    }
}

fn parse_input(input: &str) -> Option<(String, i64)> {
    let words: Vec<&str> = input.split_whitespace().collect();
    match words.len() {
        2 => {
            let val = words[1].parse::<i64>().expect("Valid number input");
            let key = words[0].replace(':', "");
            Some((key, val))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unknown() {
        let mut monkeys: HashMap<String, i64> = HashMap::new();
        monkeys.insert("blair".to_string(), 5);
        monkeys.insert("kivo".to_string(), 8);
        let input = "layla: blair + kivo";
        assert_eq!(
            parse_unknown(input, &monkeys),
            Some(("layla".to_string(), 13))
        );
        let input_2 = "momo: blair + julia";
        assert_eq!(parse_unknown(input_2, &monkeys), None);
    }
    #[test]
    fn test_input() {
        assert_eq!(parse_input("dbpl: 5"), Some(("dbpl".to_string(), 5)));
        assert_eq!(parse_input("kivo: hun + gry"), None);
    }
}

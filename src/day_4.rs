/*
Given sections in list form, i.e.
14-89,17-25
On each line

Want to start with a parsing function that returns two tuples
*/
pub fn main(contents: String) {
    let part_1_ans = part_1(&contents);
    println!("Part 1: {part_1_ans}");
}

fn part_1(contents: &str) -> i32 {
    let mut sum = 0;
    for line in contents.split('\n') {
        if line.is_empty() {
            break;
        }
        if contains(parse_line(line)) {
            sum += 1;
        }
    }
    sum
}

fn parse_line(line: &str) -> Vec<i32> {
    line.trim()
        .split(|c| c == ',' || c == '-')
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn contains(assignment: Vec<i32>) -> bool {
    (assignment[0] <= assignment[2] && assignment[1] >= assignment[3])
        || (assignment[0] >= assignment[2] && assignment[1] <= assignment[3])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let line = "19-84,6-17\n";
        assert_eq!(parse_line(line), vec![19, 84, 6, 17]);
    }

    #[test]
    fn test_contains() {
        assert!(contains(vec![10, 20, 12, 18]));
        assert!(contains(vec![10, 20, 5, 25]));
        assert!(!contains(vec![10, 28, 5, 25]));
        assert!(!contains(vec![10, 20, 12, 48]));
    }
}

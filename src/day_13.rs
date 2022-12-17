// 149 pairs
// answer 4781 too low
// answer 5629 too high
pub fn main(contents: String) {
    let p = parse_pairs(contents);
    let mut part_1: u32 = 0;
    for index in 0..p.len() {
        let pair = p[index].clone();
        println!("{:?}", &pair);
        if compare(pair.0, pair.1) {
            println!("Index: {index}");
            part_1 += index as u32 + 1
        }
    }

    println!("Part 1: {part_1}"); // 4781 too low
}

fn parse_pairs(contents: String) -> Vec<(String, String)> {
    let mut lines = contents.lines();
    let mut pairs = vec![];
    while let Some(left) = lines.next() {
        if let Some(right) = lines.next() {
            pairs.push((left.to_string(), right.to_string()));
        }
        lines.next();
    }
    pairs
}

fn compare(left: String, right: String) -> bool {
    to_vec(left) < to_vec(right)
}

fn to_vec(input: String) -> Vec<u8> {
    input
        .split_terminator(|c| c == '[' || c == ']' || c == ',')
        // .filter(|s| !s.is_empty())
        .map(|s| s.trim().parse().unwrap_or(0))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_compare() {
        assert!(compare(
            "[1,1,30,1,1]".to_string(),
            "[1,1,50,1,1]".to_string()
        ));
        assert!(compare("[[1],[2,3,4]]".to_string(), "[[1],4]".to_string()));
        assert!(!compare("[9]".to_string(), "[[8,7,6]]".to_string()));
        assert!(!compare("[7,7,7,7]".to_string(), "[7,7,7]".to_string()));
        assert!(compare("[]".to_string(), "[3]".to_string()));
        assert!(compare(
            "[[4,4],4,4]".to_string(),
            "[[4,4],4,4,4]".to_string()
        ));
        assert!(!compare("[[[]]]".to_string(), "[[]]".to_string()));
        assert!(compare("[[]]".to_string(), "[[[]]]".to_string()));
        assert!(!compare(
            "[1,[2,[3,[4,[5,6,7]]]],8,9]".to_string(),
            "[1,[2,[3,[4,[5,6,0]]]],8,9]".to_string()
        ));
        assert_ne!(to_vec("[[]]".to_string()), to_vec("[[[]]]".to_string()));
    }

    #[test]
    fn test_vec() {
        assert_eq!(to_vec("[1,2,3,4,5]".to_string()), vec![0, 1, 2, 3, 4, 5]);
        assert_eq!(to_vec("[[]]".to_string()), vec![0, 0, 0, 0]);
    }
    #[test]
    fn test_pairs() {
        let inputs = indoc!(
            "
        good
        morning

        my
        sunshine
        "
        )
        .to_string();
        assert_eq!(parse_pairs(inputs.clone())[0].1, "morning".to_string());
        assert_eq!(parse_pairs(inputs.clone())[1].1, "sunshine".to_string());
    }
}

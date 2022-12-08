use std::collections::HashSet;

pub fn main(contents: String) {
    let part_1_total = part_1(&contents);
    println!("Part 1: {part_1_total}");
    let part_2_total = part_2(&contents);
    println!("Part 2: {part_2_total}");
}

fn part_1(contents: &str) -> u32 {
    let mut sum: u32 = 0;
    for line in contents.split('\n') {
        // TODO: Make this work without having to check for empty lines
        if line.is_empty() {
            break;
        }
        sum += get_priority(find_common(split_word(line))) as u32;
    }

    sum
}

fn part_2(contents: &str) -> u32 {
    let mut sum: u32 = 0;
    let mut lines = contents.split('\n');
    while let Ok(chunk) = lines.next_chunk::<3>() {
        sum += get_priority(find_common(chunk.to_vec())) as u32;
    }

    sum
}

fn find_common(words: Vec<&str>) -> char {
    let mut sets: Vec<HashSet<char>> = words.iter().map(|w| w.chars().collect()).collect();
    // println!("{:?}", sets);
    let (intersection, others) = sets.split_at_mut(1);
    let intersection = &mut intersection[0];
    for other in others {
        intersection.retain(|letter| other.contains(letter))
    }
    // println!("{:?}", intersection);
    *intersection.iter().next().unwrap()
}

fn split_word(word: &str) -> Vec<&str> {
    let half_len: usize = word.len() / 2;
    vec![&word[0..half_len], &word[half_len..]]
}

fn get_priority(item: char) -> usize {
    let ord = match item.is_uppercase() {
        true => 'A' as usize - 26,
        false => 'a' as usize,
    };
    item as usize - ord + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common() {
        assert_eq!(find_common(vec!["big", "log", "arg"]), 'g');
        assert_eq!(find_common(vec!["big", "log"]), 'g');
        assert_eq!(find_common(vec!["vJrwpWtwJgWr", "hcsFMMfFFhFp"]), 'p');
    }

    #[test]
    fn test_priority() {
        assert_eq!(1, get_priority('a'));
        assert_eq!(16, get_priority('p'));
        assert_eq!(38, get_priority('L'));
        assert_eq!(42, get_priority('P'));
    }

    #[test]
    fn test_split() {
        assert_eq!(vec!["ki", "vo"], split_word("kivo"));
        assert_eq!(
            vec!["vJrwpWtwJgWr", "hcsFMMfFFhFp"],
            split_word("vJrwpWtwJgWrhcsFMMfFFhFp")
        );
    }
}

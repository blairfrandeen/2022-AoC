use std::collections::HashSet;

pub fn main(contents: String) {
    println!("Hello AoC!");
    let mut sum: u32 = 0;
    for line in contents.split('\n') {
        if line.len() == 0 {
            break
        }
        sum += get_priority(find_common(split_word(line))) as u32;
    }
    println!("Part 1: {:?}", sum);

}

fn find_common(words: (&str, &str)) -> char {
    let w1: HashSet<char> = words.0.chars().collect();
    let w2: HashSet<char> = words.1.chars().collect();
    *w1.intersection(&w2).next().unwrap()
}


fn split_word(word: &str) -> (&str, &str) {
    let half_len: usize = word.len() / 2;
    (&word[0..half_len], &word[half_len..])
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
        assert_eq!(find_common(("big", "log")), 'g');
        assert_eq!(find_common(("vJrwpWtwJgWr", "hcsFMMfFFhFp")), 'p');
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
        assert_eq!(("ki","vo"), split_word("kivo"));
        assert_eq!(("vJrwpWtwJgWr", "hcsFMMfFFhFp"), split_word("vJrwpWtwJgWrhcsFMMfFFhFp"));
    }
}

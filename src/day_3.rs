pub fn main(contents: String) {
    println!("Hello AoC!")
}


fn split_word(word: &str) -> (&str, &str) {
    let half_len: usize = word.len() / 2;
    (&word[0..half_len], &word[half_len..])
}

fn get_priority(item: char) -> usize {
    item as usize// - 96
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority() {
        // assert_eq!(1, get_priority('a'));
        // assert_eq!(16, get_priority('p'));
        assert_eq!(38, get_priority('L'));
        assert_eq!(42, get_priority('P'));
    }

    #[test]
    fn test_split() {
        assert_eq!(("ki","vo"), split_word("kivo"));
        assert_eq!(("vJrwpWtwJgWr", "hcsFMMfFFhFp"), split_word("vJrwpWtwJgWrhcsFMMfFFhFp"));
    }
}

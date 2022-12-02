/*
Text input will be groups of numbers separated by newlines
Goal is to find elf with most calories, i.e. largest total between any two blank lines.
*/

pub fn main(contents: String) {
    let mut current_elf = 0;
    let mut elf_cal_counts: Vec<i32> = Vec::new();

    for cal in content_to_ints(contents) {
        current_elf += cal;
        if cal == 0 {
            // assume no elves carrying zero calorie snacks.
            // I mean come on what kind of elf drinks La Croix?
            elf_cal_counts.push(current_elf);
            current_elf = 0;
        }
    }
    elf_cal_counts.sort();
    elf_cal_counts.reverse();
    let top_3: i32 = elf_cal_counts[0..3].iter().sum();

    println!("Part 1: {:?}", elf_cal_counts[0]);
    println!("Part 2: {}", top_3);
}

fn content_to_ints(contents: String) -> Vec<i32> {
    let parsed_string = contents
        .split('\n')
        .map(|s| s.parse::<i32>().unwrap_or_default())
        .collect();
    match &contents.ends_with("\n") {
        false => parsed_string,
        true => {
            // last element will be zero (default). Ignore it.
            let n = parsed_string.len() - 1;
            parsed_string[..n].to_vec()
        }
    }
}
mod tests {
    use super::*;

    #[test]
    fn test_content_to_int() {
        // test with trailing newline
        let content = String::from("1\n2\n3\n");
        let ints = content_to_ints(content);
        assert_eq!(ints, vec![1, 2, 3]);

        // test without trailing newline
        let content = String::from("1\n2\n3");
        let ints = content_to_ints(content);
        assert_eq!(ints, vec![1, 2, 3]);
    }
}

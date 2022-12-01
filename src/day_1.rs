/*
Text input will be groups of numbers separated by newlines
Goal is to find elf with most calories, i.e. largest total between any two blank lines.
*/

pub fn main(contents: String) {
    let mut current_elf = 0;
    let mut elf_cal_counts: Vec<i32> = Vec::new();

    for line in contents.split('\n') {
        let cal = line.parse::<i32>().unwrap_or_default();
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

mod tests {
    // use super::*;

    #[test]
    fn test_42() {
        let answer = 42;
        assert_eq!(answer, 42)
    }
}

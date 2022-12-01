/*
Text input will be groups of numbers separated by newlines
Goal is to find elf with most calories, i.e. largest total between any two blank lines.
*/

pub fn main(contents: String) {
    let mut max_cal = 0;
    let mut current_elf = 0;
    for line in contents.split('\n') {
        let cal = line.parse::<i32>().unwrap_or_default();
        current_elf += cal;
        if cal == 0 {
            if current_elf > max_cal {
                max_cal = current_elf;
            }
            current_elf = 0;
        }
    }
    println!("{}", max_cal)
}

mod tests {
    // use super::*;

    #[test]
    fn test_42() {
        let answer = 42;
        assert_eq!(answer, 42)
    }
}

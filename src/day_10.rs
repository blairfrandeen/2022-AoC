const OFF: char = '🎄';
const ON: char = '🎅';
const LINE_LEN: usize = 40;

pub fn main(contents: String) {
    let target_cycles = vec![19, 59, 99, 139, 179, 219];
    let cycles = process_signal(contents);
    let part_1: i32 = target_cycles
        .iter()
        .map(|index| signal_strength(&cycles, *index))
        .sum();
    println!("Part 1: {part_1}");
    for i in 0..cycles.len() - 1 {
        if i % LINE_LEN == 0 {
            println!();
        }
        match render_pixel(&cycles, i) {
            true => print!("{ON}"),
            false => print!("{OFF}"),
        }
    }
}
fn render_pixel(cycles: &[i32], index: usize) -> bool {
    let row_index: usize = index % LINE_LEN;
    (cycles[index] - row_index as i32).abs() <= 1
}

fn parse_input(input: &str) -> Option<i32> {
    let mut inst = input.split_whitespace();
    inst.next();
    inst.next().map(|num| num.parse::<i32>().unwrap())
}

fn process_signal(signal: String) -> Vec<i32> {
    let mut cycles: Vec<i32> = Vec::new();
    let mut register: i32 = 1;
    cycles.push(register); // first cycle is 1
    for input in signal.lines() {
        if let Some(n) = parse_input(input) {
            cycles.push(register);
            register += n;
        };
        cycles.push(register);
    }
    cycles
}

fn signal_strength(cycles: &[i32], index: usize) -> i32 {
    cycles[index] * (index as i32 + 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_parse() {
        assert_eq!(parse_input("noop\n"), None);
        assert_eq!(parse_input("addx 3\n"), Some(3));
        assert_eq!(parse_input("addx -13\n"), Some(-13));
    }

    #[test]
    fn test_render() {
        let cycles = process_signal(fs::read_to_string("inputs/2022.10.test").unwrap());
        // lines are 40 chars long
        assert!(render_pixel(&cycles, 0));
        assert!(!render_pixel(&cycles, 2));
        assert!(!render_pixel(&cycles, 6));
        assert!(render_pixel(&cycles, 12));
        assert!(render_pixel(&cycles, 40));
        assert!(!render_pixel(&cycles, 43));
    }

    #[test]
    fn test_sample() {
        let cycles = process_signal(fs::read_to_string("inputs/2022.10.test").unwrap());
        assert_eq!(cycles[19], 21);
        assert_eq!(cycles[59], 19);
        assert_eq!(cycles[99], 18);
        assert_eq!(cycles[139], 21);
        assert_eq!(cycles[179], 16);
        assert_eq!(cycles[219], 18);
    }

    #[test]
    fn test_strength() {
        let cycles = process_signal(fs::read_to_string("inputs/2022.10.test").unwrap());
        assert_eq!(signal_strength(&cycles, 19), 420);
        assert_eq!(signal_strength(&cycles, 59), 1140);
        assert_eq!(signal_strength(&cycles, 99), 1800);
        assert_eq!(signal_strength(&cycles, 139), 2940);
        assert_eq!(signal_strength(&cycles, 179), 2880);
        assert_eq!(signal_strength(&cycles, 219), 3960);
    }
    #[test]
    fn test_process() {
        let signal = String::from("noop\naddx 3\naddx -5\n");
        assert_eq!(process_signal(signal), vec![1, 1, 1, 4, 4, -1]);
    }
}

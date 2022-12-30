// note: inputs are _not_ unique!
// my file has 5000 lines, 3638 unique numbers
use std::collections::VecDeque;

type Signal = VecDeque<(usize, i32)>;

pub fn main(contents: String) {
    let mut input = parse_input(contents);
    for index in 0..input.len() {
        let index_to_mix = &input
            .iter()
            .position(|p| p.0 == index)
            .expect("Valid signal");
        // print!("Mixing: {index_to_mix} - ");
        mix_element(&mut input, *index_to_mix);
    }
    // println!("{:?}", sig_to_vec(&input));
    let part_1 = after_zero(&input, 1000) + after_zero(&input, 2000) + after_zero(&input, 3000);
    println!("Part 1: {part_1}");
}
fn mix_element(signal: &mut Signal, index: usize) {
    // Perform mix operation in a given index in a signal
    let delta = signal[index].1;
    let next_index = circular_index(index, delta, signal.len() - 1);
    // println!("{:?}", sig_to_vec(&signal));
    // println!("Moving from {index} to {next_index}");
    if next_index >= index {
        signal.as_mut_slices().0[index..=next_index].rotate_left(1);
    } else {
        signal.as_mut_slices().0[next_index..=index].rotate_right(1);
    }
}

fn circular_index(
    start: usize, //starting index of element
    delta: i32,   // amount to shift forward ro back
    len: usize,   // length of signal
) -> usize // return new index of element shifted by delta
{
    let new_index = (start as i32 + delta).rem_euclid(len as i32) as usize;
    // new_index

    match new_index {
        0 => len, // make sure anything that gets moved to the 0th position
        // ends up at the end of the list
        _ => new_index,
    }
}

fn parse_input(contents: String) -> Signal {
    contents
        .lines()
        .enumerate()
        .map(|(index, element)| (index, element.parse::<i32>().expect("Valid input")))
        .collect()
}

fn after_zero(signal: &Signal, delta: i32) -> i32 {
    let zero_index = signal.iter().position(|p| p.1 == 0).expect(" a zero value");
    signal[circular_index(zero_index, delta, signal.len())].1
}
fn sig_to_vec(signal: &Signal) -> Vec<i32> {
    signal.iter().map(|i| i.1).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn signal_fixture() -> Signal {
        VecDeque::from([(0, 1), (1, 2), (2, -43), (3, 5)])
    }

    #[test]
    fn test_after_zero() {
        let test_input: Signal = parse_input("1\n2\n-3\n4\n0\n3\n-2".to_string());
        assert_eq!(after_zero(&test_input, 1000), 4);
        assert_eq!(after_zero(&test_input, 2000), -3);
        assert_eq!(after_zero(&test_input, 3000), 2);
    }

    #[test]
    fn test_mix() {
        let mix_sequence: Vec<Vec<i32>> = vec![
            vec![1, 2, -3, 3, -2, 0, 4], // start condition
            vec![2, 1, -3, 3, -2, 0, 4],
            vec![1, -3, 2, 3, -2, 0, 4],
            vec![1, 2, 3, -2, -3, 0, 4],
            vec![1, 2, -2, -3, 0, 3, 4],
            vec![1, 2, -3, 0, 3, 4, -2],
            vec![1, 2, -3, 0, 3, 4, -2],
            vec![1, 2, -3, 4, 0, 3, -2], // end condition
        ];
        let indices = vec![0, 0, 1, 2, 2, 3, 5]; // manual input
        let mut test_input: Signal =
            parse_input(include_str!("../inputs/2022.20.test").to_string());
        for i in 0..7 {
            mix_element(&mut test_input, indices[i]);
            assert_eq!(sig_to_vec(&test_input), mix_sequence[i + 1]);
        }
    }

    #[test]
    fn test_index() {
        assert_eq!(circular_index(7, 5, 11), 1);
        assert_eq!(circular_index(7, 1, 11), 8);
        assert_eq!(circular_index(1, 25, 11), 4);
        assert_eq!(circular_index(7, -2, 11), 5);
        assert_eq!(circular_index(7, -8, 11), 10);
    }

    #[test]
    fn test_parse() {
        let input = String::from("1\n2\n-43\n5\n");
        assert_eq!(parse_input(input), signal_fixture());
    }
}

// note: inputs are _not_ unique!
// my file has 5000 lines, 3638 unique numbers
use std::collections::VecDeque;

pub fn main(contents: String) {
    let input = parse_input(contents);
    println!("{:?}", &input);
    let input = mix(input, 1);
    println!("{:?}", &input);
}

fn circular_index(start: usize, delta: i32, len: usize) -> usize {
    let new_index = (start as i32 + delta).rem_euclid(len as i32) as usize;
    match new_index {
        0 => len,
        _ => new_index,
    }
}

fn mix(mut signal: VecDeque<i32>, index: usize) -> VecDeque<i32> {
    // let value = signal.remove(index).unwrap();
    let value = signal[index];
    let next_index = circular_index(index, value, signal.len() - 1);
    signal.as_mut_slices().0[index..=next_index].rotate_left(1);
    // signal.insert(circular_index(index, value, signal.len()), value);
    signal
}

fn parse_input(contents: String) -> VecDeque<i32> {
    contents
        .lines()
        .map(|l| l.parse::<i32>().expect("Valid input"))
        .collect()
}

fn after_zero(signal: &VecDeque<i32>, num: i32) -> i32 {
    let zero_index = signal.iter().position(|p| *p == 0).expect(" a zero value");
    signal[circular_index(zero_index, num, signal.len())]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_after_zero() {
        let v8: VecDeque<i32> = vec![1, 2, -3, 4, 0, 3, -2].into();
        assert_eq!(after_zero(&v8, 1000), 4);
        assert_eq!(after_zero(&v8, 2000), -3);
        assert_eq!(after_zero(&v8, 3000), 2);
    }

    #[test]
    fn test_mix() {
        let v1 = vec![1, 2, -3, 3, -2, 0, 4];
        //    <--
        let v2 = vec![2, 1, -3, 3, -2, 0, 4];
        //    <-------
        let v3 = vec![1, -3, 2, 3, -2, 0, 4];
        //        <----------
        let v4 = vec![1, 2, 3, -2, -3, 0, 4];
        //          <-----------
        let v5 = vec![1, 2, -2, -3, 0, 3, 4];
        //    <-------
        let v6 = vec![1, 2, -3, 0, 3, 4, -2];
        let v7 = vec![1, 2, -3, 0, 3, 4, -2];
        //              <------
        let v8 = vec![1, 2, -3, 4, 0, 3, -2];
        assert_eq!(mix(v1.into(), 0), v2);
        assert_eq!(mix(v2.into(), 0), v3);
        assert_eq!(mix(v3.into(), 1), v4);
        assert_eq!(mix(v4.into(), 2), v5);
        assert_eq!(mix(v5.into(), 2), v6);
        assert_eq!(mix(v6.clone().into(), 3), v6);
        assert_eq!(mix(v7.into(), 5), v8);
        println!("hi");
    }

    /*
    3 moves between 0 and 4:

    -2 moves between 4 and 1:

    0 does not move:
    1, 2, -3, 0, 3, 4, -2

    4 moves between -3 and 0:
    */
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
        assert_eq!(parse_input(input), vec![1, 2, -43, 5]);
    }
}

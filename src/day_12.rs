use crate::common::Grid;

const START: u8 = 'S' as u8;
const END: u8 = 'E' as u8;

pub fn main(contents: String) {
    let grid = Grid::build(contents);
    let start = linear_search(&grid.data, START).unwrap();
    let end = linear_search(&grid.data, END).unwrap();
    println!("Start at {:?}", grid.loc(start).unwrap());
    println!("End at {:?}", grid.loc(end).unwrap());
    println!("Hello AoC!")
}

fn next_moves(grid: Grid, index: usize) -> Vec<usize> {
    let mut moves: Vec<usize> = Vec::new();
    let (r, c) = grid.loc(index).unwrap();
    let value = grid.data[index];
    moves
}

fn linear_search(haystack: &Vec<u8>, needle: u8) -> Option<usize> {
    for index in 0..haystack.len() {
        if haystack[index] == needle {
            return Some(index);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let h = vec![1, 2, 3, 4, 5];
        assert_eq!(linear_search(&h, 3), Some(2));
        assert_eq!(linear_search(&h, 7), None);
    }
}

use crate::common::Grid;
use std::collections::HashSet;

const START: u8 = 'S' as u8;
const END: u8 = 'E' as u8;

pub fn main(contents: String) {
    let mut grid = Grid::build(contents);
    let start = linear_search(&grid.data, START).unwrap();
    grid.data[start] = 'a' as u8;
    let end = linear_search(&grid.data, END).unwrap();
    grid.data[end] = 'z' as u8;

    println!("Start at {:?}", grid.loc(start).unwrap());
    println!("End at {:?}", grid.loc(end).unwrap());
    println!("Next moves: {:?}", next_moves(&grid, start));
    let path = bfs(&grid, start, end, HashSet::new(), vec![]);
    println!("Path length: {}", path.len());
    println!("Path: {:?}", path);
}

fn bfs(
    grid: &Grid,
    current_position: usize,
    goal: usize,
    mut visited: HashSet<usize>,
    mut next_positions: Vec<usize>,
) -> HashSet<usize> {
    visited.insert(current_position);
    if current_position == goal {
        return visited;
    }
    for position in next_moves(&grid, current_position) {
        if !visited.contains(&position) {
            next_positions.push(position);
        }
    }
    if let Some(next_move) = next_positions.pop() {
        visited = bfs(&grid, next_move, goal, visited, next_positions);
    } else {
        panic!("Goal unreachable!");
    }
    visited
}

fn next_moves(grid: &Grid, index: usize) -> Vec<usize> {
    grid.neighbors_lateral(index)
        .unwrap()
        .into_iter()
        .filter(|h| (grid.data[index] as i32 - grid.data[*h] as i32).abs() <= 1)
        .collect()
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
    use indoc::indoc;
    fn mock_grid() -> Grid {
        let mock_input = indoc!(
            "
        123
        456
        789
        "
        )
        .to_string();
        Grid::build(mock_input)
    }

    #[test]
    fn test_next() {
        let g = mock_grid();
        assert_eq!(next_moves(&g, 0), vec![1]);
        assert_eq!(next_moves(&g, 4), vec![3, 5])
    }

    #[test]
    fn test_search() {
        let h = vec![1, 2, 3, 4, 5];
        assert_eq!(linear_search(&h, 3), Some(2));
        assert_eq!(linear_search(&h, 7), None);
    }
}

use crate::common::Grid;
use std::collections::HashSet;

const START: u8 = b'S';
const END: u8 = b'E';

pub fn main(contents: String) {
    let mut grid = Grid::build(contents);
    let start = grid.data.iter().position(|&x| x == START).unwrap();
    grid.data[start] = b'a';
    let end = grid.data.iter().position(|&x| x == END).unwrap();
    grid.data[end] = b'z';

    println!("Start at {:?}", grid.loc(start).unwrap());
    println!("End at {:?}", grid.loc(end).unwrap());
    println!("Grid Size: {} X {}", grid.num_rows, grid.num_cols);
    let path = bfs(&grid, start, end).unwrap();
    println!("Part 1: {}", path.len());
    let mut starting_points: Vec<usize> = Vec::new();
    for ind in 0..grid.data.len() {
        if grid.data[ind] == b'a' {
            starting_points.push(ind);
        }
    }
    let mut min_len: usize = grid.data.len();
    for point in starting_points {
        if let Some(path) = bfs(&grid, point, end) {
            if path.len() < min_len {
                min_len = path.len()
            }
        }
    }
    println!("Part 2: {min_len}");
}

fn bfs(grid: &Grid, start: usize, goal: usize) -> Option<Vec<usize>> {
    let mut visited: HashSet<usize> = HashSet::new();
    visited.insert(start);

    let mut to_visit = vec![];
    for mv in next_moves(grid, start) {
        to_visit.push((mv, vec![mv]));
        // println!("{:?}", to_visit);
    }

    while !to_visit.is_empty() {
        // println!("{:?}", to_visit);
        let (current_pos, current_path) = to_visit[0].clone();
        to_visit.remove(0); //.pop().unwrap();
        if visited.contains(&current_pos) {
            continue;
        }
        visited.insert(current_pos);
        if current_pos == goal {
            return Some(current_path);
        }
        for mv in next_moves(grid, current_pos) {
            if !visited.contains(&mv) {
                let mut new_path = current_path.clone();
                new_path.push(mv);
                to_visit.push((mv, new_path));
            }
        }
    }
    None
}

fn next_moves(grid: &Grid, index: usize) -> Vec<usize> {
    grid.neighbors_lateral(index)
        .unwrap()
        .into_iter()
        .filter(|h| (grid.data[*h] as i32 - grid.data[index] as i32) <= 1)
        .collect()
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
        assert_eq!(next_moves(&g, 4), vec![3, 5, 1])
    }

    #[test]
    fn test_search() {
        let h = vec![1, 2, 3, 4, 5];
        assert_eq!(linear_search(&h, 3), Some(2));
        assert_eq!(linear_search(&h, 7), None);
    }
}

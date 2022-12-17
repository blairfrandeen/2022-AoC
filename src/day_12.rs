use crate::common::Grid;
use std::collections::HashSet;

const START: u8 = 'S' as u8;
const END: u8 = 'E' as u8;

pub fn main(contents: String) {
    let mut grid = Grid::build(contents);
    // grid.disp();
    let start = linear_search(&grid.data, START).unwrap();
    grid.data[start] = 'a' as u8;
    let end = linear_search(&grid.data, END).unwrap();
    grid.data[end] = 'z' as u8;

    println!("Start at {:?}", grid.loc(start).unwrap());
    println!("End at {:?}", grid.loc(end).unwrap());
    println!("Grid Size: {} X {}", grid.num_rows, grid.num_cols);
    println!("Next moves: {:?}", next_moves(&grid, start));
    let path = bfs(&grid, start, end);
    println!("Path length: {}", path.len());
    println!("Path: {:?}", path);
    for index in path {
        grid.data[index] = '*' as u8;
    }
    grid.disp();
}

fn bfs(grid: &Grid, start: usize, goal: usize) -> Vec<usize> {
    let mut visited: HashSet<usize> = HashSet::new();
    visited.insert(start);

    let mut to_visit = vec![];
    for mv in next_moves(&grid, start) {
        to_visit.push((mv, vec![mv]));
        println!("{:?}", to_visit);
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
            return current_path;
        }
        for mv in next_moves(&grid, current_pos) {
            if !visited.contains(&mv) {
                let mut new_path = current_path.clone();
                new_path.push(mv);
                to_visit.push((mv, new_path));
            }
        }
    }
    panic!("Goal unreachable!");
}

fn next_moves(grid: &Grid, index: usize) -> Vec<usize> {
    grid.neighbors_lateral(index)
        .unwrap()
        .into_iter()
        .filter(|h| (grid.data[*h] as i32 - grid.data[index] as i32) <= 1)
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
        assert_eq!(next_moves(&g, 4), vec![3, 5, 1])
    }

    #[test]
    fn test_search() {
        let h = vec![1, 2, 3, 4, 5];
        assert_eq!(linear_search(&h, 3), Some(2));
        assert_eq!(linear_search(&h, 7), None);
    }
}

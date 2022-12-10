/*
AoC 2022 Day 8
How best to represent a grid in Rust?
*/

pub fn main(contents: String) {
    let grid = parse_input(contents);
    let mut num_visible: u32 = 0;
    let mut max_scenic_score: u32 = 0;
    for index in 0..grid.num_rows * grid.num_cols {
        let tree_scenic_score = scenic_score(&grid, index);
        if tree_scenic_score > max_scenic_score {
            max_scenic_score = tree_scenic_score;
        }
        if is_visible(&grid, index) {
            num_visible += 1;
        }
    }
    println!("Part 1: {num_visible}");
    println!("Part 2: {max_scenic_score}");
}

#[derive(PartialEq, Debug)]
struct Grid {
    num_rows: usize,
    num_cols: usize,
    data: Vec<u8>,
}

impl Grid {
    fn get(&self, row: usize, col: usize) -> u8 {
        let value: &u8 = &self.data[row * self.num_cols + col];
        *value
    }

    fn loc(&self, index: usize) -> (usize, usize) {
        let row: usize = &index / &self.num_cols;
        let col: usize = &index % &self.num_cols;
        (row, col)
    }
}

fn scenic_score(grid: &Grid, index: usize) -> u32 {
    let height: &u8 = &grid.data[index];
    let (row, col) = grid.loc(index);

    let mut n_row_before = 0; // LEFT
    for c in 1..=col {
        n_row_before += 1;
        if grid.get(row, col - c) >= *height {
            break;
        }
    }
    let mut n_row_after = 0; // RIGHT
    for c in col + 1..grid.num_cols {
        n_row_after += 1;
        if grid.get(row, c) >= *height {
            break;
        }
    }
    let mut n_col_before = 0; // UP
    for r in 1..=row {
        n_col_before += 1;
        if grid.get(row - r, col) >= *height {
            break;
        }
    }
    let mut n_col_after = 0; // DOWN
    for r in row + 1..grid.num_rows {
        n_col_after += 1;
        if grid.get(r, col) >= *height {
            break;
        }
    }
    // println!("LEFT: {n_row_before}"); //1
    // println!("RIGHT: {n_row_after}"); //2
    // println!("UP: {n_col_before}"); // 1
    // println!("DOWN: {n_col_after}"); // 2
    n_row_before * n_row_after * n_col_before * n_col_after
}

fn is_visible(grid: &Grid, index: usize) -> bool {
    let height: &u8 = &grid.data[index];
    let (row, col) = grid.loc(index);
    let mut row_before: bool = true;
    for c in 0..col {
        if grid.get(row, c) >= *height {
            row_before = false;
            break;
        }
    }

    let mut row_after: bool = true;
    for c in col + 1..grid.num_cols {
        if grid.get(row, c) >= *height {
            row_after = false;
            break;
        }
    }
    let mut col_before: bool = true;
    for r in 0..row {
        if grid.get(r, col) >= *height {
            col_before = false;
            break;
        }
    }
    let mut col_after: bool = true;
    for r in row + 1..grid.num_rows {
        if grid.get(r, col) >= *height {
            col_after = false;
            break;
        }
    }

    row_before || row_after || col_before || col_after
}

fn parse_input(contents: String) -> Grid {
    let mut grid: Vec<u8> = Vec::new();
    let mut num_rows: usize = 0;
    let mut num_cols: usize = 0;
    for line in contents.lines() {
        let mut l: Vec<u8> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        if num_cols != 0 && line.len() != num_cols {
            panic!("Unequal number of columns in input!")
        }
        num_cols = line.len();
        grid.append(&mut l);
        num_rows += 1;
    }
    Grid {
        num_rows,
        num_cols,
        data: grid,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn mock_grid() -> Grid {
        Grid {
            num_rows: 2,
            num_cols: 5,
            data: vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0],
        }
    }

    #[test]
    fn test_visible() {
        for n in 0..10 {
            assert!(is_visible(&mock_grid(), n));
        }
        let test_contents = fs::read_to_string("inputs/2022.8.test").unwrap();
        let test_grid = parse_input(test_contents);
        assert!(is_visible(&test_grid, 6));
        assert!(!is_visible(&test_grid, 12));
        assert!(!is_visible(&test_grid, 16));
        assert!(is_visible(&test_grid, 17));
        assert!(!is_visible(&test_grid, 18));
    }
    #[test]
    fn test_scenic() {
        let test_contents = fs::read_to_string("inputs/2022.8.test").unwrap();
        let test_grid = parse_input(test_contents);
        assert_eq!(scenic_score(&test_grid, 7), 4);
        assert_eq!(scenic_score(&test_grid, 17), 8);
    }

    #[test]
    fn test_loc() {
        let grid = mock_grid();
        assert_eq!(grid.loc(2), (0, 2));
        assert_eq!(grid.loc(8), (1, 3));
    }
    #[test]
    fn test_get() {
        let grid = mock_grid();
        assert_eq!(grid.get(0, 0), 1);
        assert_eq!(grid.get(0, 4), 5);
        assert_eq!(grid.get(1, 2), 8);
        assert_eq!(grid.get(1, 4), 0);
    }

    #[test]
    fn test_parse() {
        let test_input = String::from("12345\n67890\n");
        let test_grid = mock_grid();
        assert_eq!(parse_input(test_input), test_grid);
    }
}

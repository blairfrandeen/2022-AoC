use crate::common::Grid;

pub fn main(contents: String) {
    let grid = Grid::build(contents);
    let num_visible: u32 = (0..grid.data.len())
        .map(|x| is_visible(&grid, x) as u32)
        .sum();
    let max_scenic_score: u32 = (0..grid.data.len())
        .map(|x| scenic_score(&grid, x))
        .reduce(|max, item| if max >= item { max } else { item })
        .unwrap();
    println!("Part 1: {num_visible}");
    println!("Part 2: {max_scenic_score}");
}

fn scenic_score(grid: &Grid, index: usize) -> u32 {
    let height: &u8 = &grid.data[index];
    let (row, col) = grid.loc(index).unwrap();

    let mut n_row_before = 0; // LEFT
    for c in 1..=col {
        n_row_before += 1;
        if grid.get(row, col - c).unwrap() >= *height {
            break;
        }
    }
    let mut n_row_after = 0; // RIGHT
    for c in col + 1..grid.num_cols {
        n_row_after += 1;
        if grid.get(row, c).unwrap() >= *height {
            break;
        }
    }
    let mut n_col_before = 0; // UP
    for r in 1..=row {
        n_col_before += 1;
        if grid.get(row - r, col).unwrap() >= *height {
            break;
        }
    }
    let mut n_col_after = 0; // DOWN
    for r in row + 1..grid.num_rows {
        n_col_after += 1;
        if grid.get(r, col).unwrap() >= *height {
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
    let (row, col) = grid.loc(index).unwrap();
    let mut row_before: bool = true;
    for c in 0..col {
        if grid.get(row, c).unwrap() >= *height {
            row_before = false;
            break;
        }
    }

    let mut row_after: bool = true;
    for c in col + 1..grid.num_cols {
        if grid.get(row, c).unwrap() >= *height {
            row_after = false;
            break;
        }
    }
    let mut col_before: bool = true;
    for r in 0..row {
        if grid.get(r, col).unwrap() >= *height {
            col_before = false;
            break;
        }
    }
    let mut col_after: bool = true;
    for r in row + 1..grid.num_rows {
        if grid.get(r, col).unwrap() >= *height {
            col_after = false;
            break;
        }
    }

    row_before || row_after || col_before || col_after
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests;
    use std::fs;

    #[test]
    fn test_visible() {
        for n in 0..10 {
            assert!(is_visible(&tests::mock_grid(), n));
        }
        let test_contents = fs::read_to_string("inputs/2022.8.test").unwrap();
        let test_grid = Grid::build(test_contents);
        assert!(is_visible(&test_grid, 6));
        assert!(!is_visible(&test_grid, 12));
        assert!(!is_visible(&test_grid, 16));
        assert!(is_visible(&test_grid, 17));
        assert!(!is_visible(&test_grid, 18));
    }
    #[test]
    fn test_scenic() {
        let test_contents = fs::read_to_string("inputs/2022.8.test").unwrap();
        let test_grid = Grid::build(test_contents);
        assert_eq!(scenic_score(&test_grid, 7), 4);
        assert_eq!(scenic_score(&test_grid, 17), 8);
    }
}

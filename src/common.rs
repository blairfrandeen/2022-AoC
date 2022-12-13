#[derive(PartialEq, Debug)]
pub struct Grid {
    pub num_rows: usize,
    pub num_cols: usize,
    pub data: Vec<u8>,
}

impl Grid {
    pub fn get(&self, row: usize, col: usize) -> u8 {
        let value: &u8 = &self.data[row * self.num_cols + col];
        *value
    }

    pub fn loc(&self, index: usize) -> (usize, usize) {
        let row: usize = index / self.num_cols;
        let col: usize = index % self.num_cols;
        (row, col)
    }

    pub fn build(contents: String) -> Grid {
        let mut data: Vec<u8> = Vec::new();
        let mut num_rows: usize = 0;
        let mut num_cols: usize = 0;
        for line in contents.lines() {
            let mut l: Vec<u8> = line.chars().map(|c| c as u8).collect();
            if num_cols != 0 && line.len() != num_cols {
                panic!("Unequal number of columns in input!")
            }
            num_cols = line.len();
            data.append(&mut l);
            num_rows += 1;
        }
        Grid {
            num_rows,
            num_cols,
            data,
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn mock_grid() -> Grid {
        let digits: Vec<u8> = "1234567890".chars().map(|c| c as u8).collect();
        Grid {
            num_rows: 2,
            num_cols: 5,
            data: digits,
        }
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
        assert_eq!(grid.get(0, 0), 49);
        assert_eq!(grid.get(0, 4), 53);
        assert_eq!(grid.get(1, 2), 56);
        assert_eq!(grid.get(1, 4), 48);
    }

    #[test]
    fn test_grid() {
        // TODO: Fix failing test
        let test_input = String::from("12345\n67890\n");
        let test_grid = mock_grid();
        assert_eq!(Grid::build(test_input), test_grid);
    }
}

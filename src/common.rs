#[derive(PartialEq, Debug)]
pub struct Grid {
    pub num_rows: usize,
    pub num_cols: usize,
    pub data: Vec<u8>,
}

impl Grid {
    pub fn get(&self, row: usize, col: usize) -> Result<u8, &str> {
        // Get the value at a given `row` and `col` in the grid.
        if row > self.num_rows || col > self.num_cols {
            return Err("Row or column out of bounds");
        }
        let value: &u8 = &self.data[row * self.num_cols + col];
        Ok(*value)
    }

    pub fn loc(&self, index: usize) -> Result<(usize, usize), &str> {
        if index > self.data.len() {
            return Err("Index out of bounds");
        }
        let row: usize = index / self.num_cols;
        let col: usize = index % self.num_cols;
        Ok((row, col))
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
        assert_eq!(grid.loc(2), Ok((0, 2)));
        assert_eq!(grid.loc(8), Ok((1, 3)));
        assert_eq!(grid.loc(8888), Err("Index out of bounds"));
    }
    #[test]
    fn test_get() {
        let grid = mock_grid();
        assert_eq!(grid.get(0, 0), Ok(49));
        assert_eq!(grid.get(0, 4), Ok(53));
        assert_eq!(grid.get(1, 2), Ok(56));
        assert_eq!(grid.get(1, 4), Ok(48));
        assert_eq!(grid.get(1, 10), Err("Row or column out of bounds"));
        assert_eq!(grid.get(10, 1), Err("Row or column out of bounds"));
    }

    #[test]
    fn test_grid() {
        // TODO: Fix failing test
        let test_input = String::from("12345\n67890\n");
        let test_grid = mock_grid();
        assert_eq!(Grid::build(test_input), test_grid);
    }
}

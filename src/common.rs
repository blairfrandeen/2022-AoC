#[derive(PartialEq, Debug)]
pub struct Grid {
    pub num_rows: usize,
    pub num_cols: usize,
    pub data: Vec<u8>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    IndexError,
}

impl Grid {
    pub fn get(&self, row: usize, col: usize) -> Result<u8, Error> {
        // Get the value at a given `row` and `col` in the grid.
        if row > self.num_rows || col > self.num_cols {
            return Err(Error::IndexError);
        }
        let value: &u8 = &self.data[row * self.num_cols + col];
        Ok(*value)
    }

    pub fn loc(&self, index: usize) -> Result<(usize, usize), Error> {
        // Get the row and column of a given index
        if index > self.data.len() {
            return Err(Error::IndexError);
        }
        let row: usize = index / self.num_cols;
        let col: usize = index % self.num_cols;
        Ok((row, col))
    }

    pub fn ind(&self, row: usize, col: usize) -> Result<usize, Error> {
        // Get the index at a given `row` and `col` in the grid
        if row > self.num_rows || col > self.num_cols {
            return Err(Error::IndexError);
        }
        Ok(row * self.num_cols + col)
    }

    pub fn neighbors_lateral(&self, index: usize) -> Result<Vec<usize>, Error> {
        match self.loc(index) {
            Ok((row, col)) => {
                let mut neighbors: Vec<usize> = Vec::new();
                if col > 0 {
                    neighbors.push(index - 1);
                }
                if col < self.num_cols - 1 {
                    neighbors.push(index + 1);
                }
                if row > 0 {
                    neighbors.push(self.ind(row - 1, col).unwrap());
                }
                if row < self.num_rows - 1 {
                    neighbors.push(self.ind(row + 1, col).unwrap());
                }
                // behind, in front, above below
                Ok(neighbors)
            }
            Err(e) => Err(e),
        }
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
    fn test_neighbors() {
        let grid = mock_grid();
        assert_eq!(grid.neighbors_lateral(0), Ok(vec![1, 5]));
        assert_eq!(grid.neighbors_lateral(2), Ok(vec![1, 3, 7]));
        assert_eq!(grid.neighbors_lateral(7), Ok(vec![6, 8, 2]));
        assert_eq!(grid.neighbors_lateral(4), Ok(vec![3, 9]));
    }

    #[test]
    fn test_loc() {
        let grid = mock_grid();
        assert_eq!(grid.loc(2), Ok((0, 2)));
        assert_eq!(grid.loc(8), Ok((1, 3)));
        assert_eq!(grid.loc(8888), Err(Error::IndexError));
    }
    #[test]
    fn test_get() {
        let grid = mock_grid();
        assert_eq!(grid.get(0, 0), Ok(49));
        assert_eq!(grid.get(0, 4), Ok(53));
        assert_eq!(grid.get(1, 2), Ok(56));
        assert_eq!(grid.get(1, 4), Ok(48));
        assert_eq!(grid.get(1, 10), Err(Error::IndexError));
        assert_eq!(grid.get(10, 1), Err(Error::IndexError));
    }

    #[test]
    fn test_grid() {
        // TODO: Fix failing test
        let test_input = String::from("12345\n67890\n");
        let test_grid = mock_grid();
        assert_eq!(Grid::build(test_input), test_grid);
    }
}

use std::cmp;

const EMPTY: char = ' ';
const SAND: &str = "⛱";
const ROCK: &str = "⛰️";

pub fn main(contents: String) {
    let paths: Vec<Path> = contents.lines().map(|line| parse_path(line)).collect();
    println!(
        "Min/Max Coords: {} - {}",
        coord_min_max(&paths, cmp::min, Point::get_x),
        coord_min_max(&paths, cmp::max, Point::get_x)
    );
    let num_cols: usize = coord_min_max(&paths, cmp::max, Point::get_x) as usize;
    let min_col: usize = coord_min_max(&paths, cmp::min, Point::get_x) as usize;
    let min_row: usize = coord_min_max(&paths, cmp::min, Point::get_y) as usize;
    let num_rows: usize = 500 - min_row;
    let cells = vec![CaveCell::Rock; num_rows * num_cols];

    let mut cave = Cave {
        cells,
        num_rows,
        num_cols,
        min_col,
        min_row,
    };

    cave.display();

    // println!("{:?}", paths);
}

#[derive(PartialEq, Debug)]
struct Cave {
    cells: Vec<CaveCell>,
    num_rows: usize,
    num_cols: usize,
    min_col: usize,
    min_row: usize,
}

impl Cave {
    fn get_cell(&self, row: usize, col: usize) -> CaveCell {
        self.cells[row * self.num_cols + col]
    }

    fn get_rc(&self, index: usize) -> (usize, usize) {
        (index / self.num_cols, index % self.num_cols)
    }

    fn display(&self) {
        for row in self.min_row..self.num_rows {
            for col in self.min_col..self.num_cols {
                match &self.get_cell(row, col) {
                    CaveCell::Rock => print!("{}", ROCK),
                    CaveCell::Sand => print!("{}", SAND),
                    CaveCell::Empty => print!("{}", EMPTY),
                }
            }
            println!();
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum CaveCell {
    Empty,
    Rock,
    Sand,
}

#[derive(PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn get_x(&self) -> i32 {
        self.x
    }
    fn get_y(&self) -> i32 {
        self.y
    }
}

#[derive(PartialEq, Debug)]
struct Path {
    points: Vec<Point>,
}

fn coord_min_max(
    paths: &Vec<Path>,
    cmp_fn: impl Fn(i32, i32) -> i32,
    coordinate_fn: impl Fn(&Point) -> i32,
) -> i32 {
    // output the maximum x value of any point in the path
    let max: i32 = paths
        .iter()
        .map(|path| {
            path.points
                .iter()
                .map(|p| coordinate_fn(p))
                .reduce(|max, item| cmp_fn(max, item))
                .unwrap()
        })
        .reduce(|max, item| cmp_fn(max, item))
        .unwrap();
    max
}

fn parse_point(point: &str) -> Point {
    let values: Vec<i32> = point
        .split(',')
        .map(|p| p.parse::<i32>().expect("invalid input"))
        .collect();
    Point {
        x: values[0],
        y: values[1],
    }
}

fn parse_path(path: &str) -> Path {
    let points: Vec<Point> = path.split(" -> ").map(|point| parse_point(point)).collect();
    Path { points }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_min() {
        let p = vec![
            parse_path("498,4 -> 498,6 -> 496,6"),
            parse_path("503,4 -> 502,4 -> 502,9 -> 494,9"),
        ];
        assert_eq!(coord_min_max(&p, cmp::max), 503);
        assert_eq!(coord_min_max(&p, cmp::min), 494);
        // assert_eq!(p.x_min(), 496);
    }
    #[test]
    fn test_parse_path() {
        let input = "498,4 -> 498,6 -> 496,6";
        let path = parse_path(input);
        assert_eq!(path.points[0].x, 498);
        assert_eq!(path.points[2].y, 6);
    }
    #[test]
    fn test_parse_point() {
        assert_eq!(parse_point("123,45"), Point { x: 123, y: 45 });
    }
}

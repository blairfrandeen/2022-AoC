use std::cmp;

const EMPTY: char = '.';
const SAND: char = 'o'; //"⛱";
const ROCK: char = '#'; //"⛰️";

pub fn main(contents: String) {
    let paths: Vec<Path> = contents.lines().map(|line| parse_path(line)).collect();
    println!(
        "Min/Max Coords: {} - {}",
        coord_min_max(&paths, cmp::min, Point::col),
        coord_min_max(&paths, cmp::max, Point::col)
    );
    let num_cols: usize = coord_min_max(&paths, cmp::max, Point::col) as usize;
    let min_col: usize = coord_min_max(&paths, cmp::min, Point::col) as usize;
    let min_row: usize = 0; //coord_min_max(&paths, cmp::min, Point::row) as usize;
    let num_rows: usize = coord_min_max(&paths, cmp::max, Point::row) as usize;
    let cells = vec![CaveCell::Empty; (num_rows + 1) * (num_cols + 1)];

    let mut cave = Cave {
        cells,
        num_rows,
        num_cols,
        min_col,
        min_row,
    };
    for path in paths {
        cave.add_path(path);
    }

    cave.display();
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
    fn add_path(&mut self, path: Path) {
        let mut points = path.points.iter();
        let mut current_point = points.next().expect("need more than zero points");
        while let Some(next_point) = points.next() {
            let index = &self.get_index(current_point.r as usize, current_point.c as usize);
            self.cells[*index] = CaveCell::Rock;
        }
    }

    fn get_cell(&self, row: usize, col: usize) -> CaveCell {
        self.cells[self.get_index(row, col)]
    }

    fn get_index(&self, row: usize, col: usize) -> usize {
        row * self.num_cols + col
    }

    fn get_rc(&self, index: usize) -> (usize, usize) {
        (index / self.num_cols, index % self.num_cols)
    }

    fn display(&self) {
        for row in self.min_row..self.num_rows + 1 {
            for col in self.min_col..self.num_cols + 1 {
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
    c: i32, // column
    r: i32, // row
}

impl Point {
    fn row(&self) -> i32 {
        self.r
    }
    fn col(&self) -> i32 {
        self.c
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
        c: values[0],
        r: values[1],
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

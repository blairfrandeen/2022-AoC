use std::cmp;

const EMPTY: char = ' ';
const SAND: &str = "⛱";
const ROCK: &str = "⛰️";

pub fn main(contents: String) {
    let paths: Vec<Path> = contents.lines().map(|line| parse_path(line)).collect();
    println!(
        "Min/Max Coords: {} - {}",
        coord_min_max(&paths, cmp::min, Point::col),
        coord_min_max(&paths, cmp::max, Point::col)
    );
    let start = Point { c: 500, r: 0 };

    let mut cave_1 = Cave::build(paths.clone());
    let part_1 = cave_1.fill(&start);
    cave_1.display();
    println!("Part 1: {part_1}");

    let mut cave_2 = Cave::build(paths);
    let mut empty_row = vec![CaveCell::Empty; cave_2.num_cols + 1];
    let mut floor = vec![CaveCell::Rock; cave_2.num_cols + 1];
    cave_2.cells.append(&mut empty_row);
    cave_2.cells.append(&mut floor);
    cave_2.num_rows += 2;

    let part_2 = cave_2.fill(&start);
    cave_2.display();
    println!("Part 2: {part_2}");
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
    fn fill(&mut self, start: &Point) -> u32 {
        let mut num_sand = 0;
        while let Ok(_) = self.drop_sand(&start) {
            num_sand += 1;
        }
        num_sand
    }
    fn build(paths: Vec<Path>) -> Cave {
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
        cave
    }
    fn drop_sand(&mut self, start: &Point) -> Result<(), &str> {
        let mut sand_loc = Point {
            r: start.r,
            c: start.c,
        };
        let start_index = self.get_index(start.r as usize, start.c as usize);
        loop {
            if (sand_loc.r as usize) >= self.num_rows || self.cells[start_index] == CaveCell::Sand {
                return Err("Puzzle complete");
            }
            if self.get_cell(sand_loc.r as usize + 1, sand_loc.c as usize) == CaveCell::Empty {
                sand_loc.r += 1;
                continue;
            } else if self.get_cell(sand_loc.r as usize + 1, sand_loc.c as usize - 1)
                == CaveCell::Empty
            {
                sand_loc.r += 1;
                sand_loc.c -= 1;
                continue;
            } else if self.get_cell(sand_loc.r as usize + 1, sand_loc.c as usize + 1)
                == CaveCell::Empty
            {
                sand_loc.r += 1;
                sand_loc.c += 1;
                continue;
            }
            break;
        }
        let index = self.get_index(sand_loc.r as usize, sand_loc.c as usize);
        self.cells[index] = CaveCell::Sand;
        Ok(())
    }

    fn add_path(&mut self, path: Path) {
        let mut points = path.points.iter();
        let mut current_point = points.next().expect("need more than zero points");
        while let Some(next_point) = points.next() {
            for point in points_between(current_point, next_point) {
                let index = &self.get_index(point.r as usize, point.c as usize);
                self.cells[*index] = CaveCell::Rock;
            }
            current_point = next_point;
        }
    }

    fn get_cell(&self, row: usize, col: usize) -> CaveCell {
        self.cells[self.get_index(row, col)]
    }

    fn get_index(&self, row: usize, col: usize) -> usize {
        row * self.num_cols + col
    }

    fn display(&self) {
        for row in self.min_row..=self.num_rows {
            for col in self.min_col..=self.num_cols {
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

fn points_between(p1: &Point, p2: &Point) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::new();
    if p1.r == p2.r {
        for col in 0..=(p1.c - p2.c).abs() {
            let mut col = col;
            if p1.c > p2.c {
                col *= -1;
            }
            points.push(Point {
                c: p1.c + col,
                r: p1.r,
            });
        }
        // columns are different
    } else if p1.c == p2.c {
        for row in 0..=(p1.r - p2.r).abs() {
            let mut row = row;
            if p1.r > p2.r {
                row *= -1;
            }
            points.push(Point {
                c: p1.c,
                r: p1.r + row,
            });
        }
        // rows are different
    } else {
        println!("{:?} {:?}", p1, p2);
        panic!("rows and cols both different!");
    }
    points
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum CaveCell {
    Empty,
    Rock,
    Sand,
}

#[derive(PartialEq, Debug, Clone)]
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

#[derive(PartialEq, Debug, Clone)]
struct Path {
    points: Vec<Point>,
}

fn coord_min_max(
    paths: &Vec<Path>,
    cmp_fn: impl Fn(i32, i32) -> i32,
    coordinate_fn: impl Fn(&Point) -> i32,
) -> i32 {
    // output the min or max row or column for any set of paths
    let max: i32 = paths
        .iter()
        .map(|path| {
            path.points
                .iter()
                .map(|p| coordinate_fn(p))
                .reduce(|max, item| cmp_fn(max, item))
                .expect("Path should have points")
        })
        .reduce(|max, item| cmp_fn(max, item))
        .expect("Path vector should have paths.");
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
    fn test_between() {
        let p1 = Point { r: 5, c: 5 };
        let p2 = Point { r: 6, c: 5 };
        let pb = points_between(&p1, &p2);
        assert_eq!(pb.len(), 2);
        let p1 = Point { r: 5, c: 1 };
        let p2 = Point { r: 5, c: 4 };
        let pb = points_between(&p1, &p2);
        assert_eq!(pb.len(), 4);
        assert_eq!(pb[1].r, 5);
        assert_eq!(pb[1].c, 2);
        let p1 = Point { r: 5, c: 1 };
        let p2 = Point { r: 1, c: 1 };
        let pb = points_between(&p1, &p2);
        assert_eq!(pb.len(), 5);
        assert_eq!(pb[1].r, 4);
        assert_eq!(pb[4].r, 1);
    }

    #[test]
    fn test_max_min() {
        let p = vec![
            parse_path("498,4 -> 498,6 -> 496,6"),
            parse_path("503,4 -> 502,4 -> 502,9 -> 494,9"),
        ];
        assert_eq!(coord_min_max(&p, cmp::max, Point::col), 503);
        assert_eq!(coord_min_max(&p, cmp::min, Point::col), 494);
        // assert_eq!(p.x_min(), 496);
    }
    #[test]
    fn test_parse_path() {
        let input = "498,4 -> 498,6 -> 496,6";
        let path = parse_path(input);
        assert_eq!(path.points[0].c, 498);
        assert_eq!(path.points[2].r, 6);
    }
    #[test]
    fn test_parse_point() {
        assert_eq!(parse_point("123,45"), Point { c: 123, r: 45 });
    }
}

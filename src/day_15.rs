use nom::{bytes::complete::tag, bytes::complete::take_till, character::complete::i32, IResult};
use std::cmp::max;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq)]
struct SensorCoverage {
    p: Point,
    size: i32,
}

impl SensorCoverage {
    fn row_coverage(&self, y: i32) -> i32 {
        let row_diff = (y - self.p.y).abs();
        println!("RD: {row_diff}");
        max(0, (self.size - row_diff) * 2 + 1)
    }
}

pub fn main(contents: String) {
    let mut sensors: Vec<SensorCoverage> = Vec::new();
    let mut beacons: Vec<Point> = Vec::new();
    for line in contents.lines() {
        let (sensor, beacon) = parse_input(line);
        let size = manhattan_distance(&sensor, &beacon);
        let sensor_cov = SensorCoverage { p: sensor, size };
        sensors.push(sensor_cov);
        beacons.push(beacon);
    }
    println!("{:?}", sensors)
}

fn parse_input(input: &str) -> (Point, Point) {
    let (input, x1) = i32_after_eq(input).expect("expect valid input");
    let (input, y1) = i32_after_eq(input).expect("expect valid input");
    let (input, x2) = i32_after_eq(input).expect("expect valid input");
    let (_, y2) = i32_after_eq(input).expect("expect valid input");
    (Point { x: x1, y: y1 }, Point { x: x2, y: y2 })
}

fn manhattan_distance(p1: &Point, p2: &Point) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

fn i32_after_eq(s: &str) -> IResult<&str, i32> {
    let (input, _) = take_till(|c| c == '=')(s)?;
    let (input, _) = tag("=")(input)?;
    let (num, res) = i32(input)?;
    Ok((num, res))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cov() {
        let sc = SensorCoverage {
            p: Point { x: 8, y: 7 },
            size: 9,
        };
        assert_eq!(sc.row_coverage(7), 19);
        assert_eq!(sc.row_coverage(70), 0);
        assert_eq!(sc.row_coverage(-2), 1);
        assert_eq!(sc.row_coverage(13), 7);
    }
    #[test]
    fn test_md() {
        let p1 = Point { x: 0, y: 5 };
        let p2 = Point { x: 1, y: 2 };
        assert_eq!(manhattan_distance(&p1, &p2), 4);
    }
    #[test]
    fn test_parse() {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15\n";
        assert_eq!(parse_input(input).0.x, 2);
        assert_eq!(parse_input(input).1.y, 15);
    }
}

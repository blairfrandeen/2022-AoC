use nom::{bytes::complete::tag, bytes::complete::take_till, character::complete::i32, IResult};
use std::collections::HashSet;
use std::ops::RangeInclusive;

#[derive(Debug, Eq, PartialEq, Hash)]
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
    fn row_coverage(&self, y: i32) -> Option<RangeInclusive<i32>> {
        let row_diff = (y - self.p.y).abs();
        if row_diff > self.size {
            return None;
        }
        Some((self.p.x - (self.size - row_diff))..=(self.p.x + (self.size - row_diff)))
    }
}

pub fn main(contents: String) {
    let mut sensors: Vec<SensorCoverage> = Vec::new();
    let mut beacons: HashSet<Point> = HashSet::new();
    for line in contents.lines() {
        let (sensor, beacon) = parse_input(line);
        let size = manhattan_distance(&sensor, &beacon);
        let sensor_cov = SensorCoverage { p: sensor, size };
        sensors.push(sensor_cov);
        beacons.insert(beacon);
    }
    let part_1 = num_row_non_beacon(2_000_000, &sensors, &beacons);
    println!("Part 1: {part_1}");
}

fn num_row_non_beacon(row: i32, sensors: &Vec<SensorCoverage>, beacons: &HashSet<Point>) -> i32 {
    let mut row_cov: Vec<RangeInclusive<i32>> = Vec::new();
    for s in sensors {
        if let Some(cov) = s.row_coverage(row) {
            row_cov.push(cov);
        }
    }
    range_total_coverage(&merge_ranges(row_cov)) - beacons_per_row(&beacons, row)
}

fn beacons_per_row(beacons: &HashSet<Point>, row: i32) -> i32 {
    let mut num_beacons = 0;
    for b in beacons {
        if b.y == row {
            num_beacons += 1;
        }
    }
    num_beacons
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

fn range_total_span(ranges: &Vec<RangeInclusive<i32>>) -> i32 {
    ranges.last().unwrap().end() - ranges.first().unwrap().start()
}

fn range_total_coverage(ranges: &Vec<RangeInclusive<i32>>) -> i32 {
    let mut total_coverage = 0;
    for range in ranges {
        total_coverage += range.end() + 1 - range.start()
    }
    total_coverage
}
fn merge_ranges(mut ranges: Vec<RangeInclusive<i32>>) -> Vec<RangeInclusive<i32>> {
    // given a list of ranges, merge them into a minimal list of ranges
    let mut merged_ranges: Vec<RangeInclusive<i32>> = Vec::new();
    ranges.sort_by(|r1, r2| r1.start().cmp(&r2.start()));
    let mut range_iter = ranges.into_iter();
    merged_ranges.push(range_iter.next().expect("At least one range"));
    while let Some(next_range) = range_iter.next() {
        let current_range = merged_ranges.pop().expect("At least one range");
        // println!("{:?}, {:?}", current_range, next_range);
        if (next_range.start() - current_range.end() == 1)
            || (current_range.end() >= next_range.start()
                && next_range.end() >= current_range.end())
        {
            // partially overlapping ranges are merged
            // adjacent ranges are merged
            merged_ranges.push(*current_range.start()..=*next_range.end());
        } else if next_range.start() > current_range.end() {
            // separate ranges stay separate
            merged_ranges.push(current_range.clone());
            merged_ranges.push(next_range);
        } else if next_range.start() >= current_range.start()
            && next_range.end() <= current_range.end()
        {
            // one range consumes the other
            merged_ranges.push(current_range.clone());
        }
    }
    merged_ranges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coverage() {
        let r1: Vec<RangeInclusive<i32>> = vec![1..=4, 6..=7];
        assert_eq!(range_total_coverage(&r1), 6);
        let r2: Vec<RangeInclusive<i32>> = vec![-10..=5, 10..=20, 50..=70];
        assert_eq!(range_total_coverage(&r2), 48);
    }
    #[test]
    fn test_span() {
        let r1: Vec<RangeInclusive<i32>> = vec![1..=5, 3..=7];
        assert_eq!(range_total_span(&r1), 6);
        let r2: Vec<RangeInclusive<i32>> = vec![-10..=5, 10..=20, 50..=70];
        assert_eq!(range_total_span(&r2), 80);
    }
    #[test]
    fn test_merge_range() {
        // overlapping ranges
        let r1: Vec<RangeInclusive<i32>> = vec![1..=5, 3..=7];
        assert_eq!(merge_ranges(r1), vec![1..=7]);
        // non-overlapping ranges
        let r2: Vec<RangeInclusive<i32>> = vec![1..=5, 8..=10];
        assert_eq!(merge_ranges(r2), vec![1..=5, 8..=10]);
        // one range completely envelops the other
        let r3: Vec<RangeInclusive<i32>> = vec![1..=15, 8..=10];
        assert_eq!(merge_ranges(r3), vec![1..=15]);
        // a lot of ranges, should merge to single range
        let r4: Vec<RangeInclusive<i32>> = vec![1..=15, 8..=10, -5..=0, -20..=-2];
        assert_eq!(merge_ranges(r4), vec![-20..=15]);
        // odd number of ranges
        let r5: Vec<RangeInclusive<i32>> = vec![2..=3, -3..=5, 4..=9];
        assert_eq!(merge_ranges(r5), vec![-3..=9]);
        // direclty adjacent ranges
        let r6: Vec<RangeInclusive<i32>> = vec![1..=2, 2..=3, 3..=9];
        assert_eq!(merge_ranges(r6), vec![1..=9]);
    }
    #[test]
    fn test_cov() {
        let sc = SensorCoverage {
            p: Point { x: 8, y: 7 },
            size: 9,
        };
        assert_eq!(sc.row_coverage(7), Some(-1..=17));
        assert_eq!(sc.row_coverage(70), None);
        assert_eq!(sc.row_coverage(-2), Some(8..=8));
        assert_eq!(sc.row_coverage(13), Some(5..=11));
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

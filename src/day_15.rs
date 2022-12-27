use nom::{bytes::complete::tag, bytes::complete::take_till, character::complete::i32, IResult};
use std::cmp;
use std::collections::HashSet;
use std::ops::RangeInclusive;
use std::time;

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

fn build_sensor_beacon_map(input: String) -> (Vec<SensorCoverage>, HashSet<Point>) {
    let mut sensors: Vec<SensorCoverage> = Vec::new();
    let mut beacons: HashSet<Point> = HashSet::new();
    for line in input.lines() {
        let (sensor, beacon) = parse_input(line);
        let size = manhattan_distance(&sensor, &beacon);
        let sensor_cov = SensorCoverage { p: sensor, size };
        sensors.push(sensor_cov);
        beacons.insert(beacon);
    }
    (sensors, beacons)
}

fn part_1(search_row: i32, sensors: &Vec<SensorCoverage>, beacons: &HashSet<Point>) -> i32 {
    num_row_non_beacon(search_row, &sensors, &beacons, None)
}

fn part_2(
    limits: RangeInclusive<i32>,
    sensors: &Vec<SensorCoverage>,
    beacons: &HashSet<Point>,
) -> i64 {
    let ranges = split_range(limits.clone(), 10);
    for range in ranges {
        let start_row = *range.start();
        let end_row = *range.end();
        println!("Checking from {} to {}", &start_row, &end_row);
        for row in start_row..=end_row {
            let nrnb = num_row_non_beacon(row, &sensors, &beacons, Some(&limits));
            if nrnb == *limits.end() {
                let mut row_covered_ranges = get_row_coverage_ranges(row, &sensors);
                row_covered_ranges = truncate_ranges(row_covered_ranges, 0, *limits.end());
                row_covered_ranges = merge_ranges(row_covered_ranges);
                return tuning_frequency(row, row_covered_ranges);
            }
        }
    }
    unreachable!("Solution should exist");
}

pub fn main(contents: String) {
    let (sensors, beacons) = build_sensor_beacon_map(contents);
    println!("Part 1: {}", part_1(2_000_000, &sensors, &beacons));
    let start_time = time::Instant::now();
    println!("Part 2: {}", part_2(0..=4_000_000, &sensors, &beacons));
    let end_time = start_time.elapsed();
    println!("Part 2 completed in {} seconds.", end_time.as_secs_f32());
}

fn tuning_frequency(row: i32, covered_ranges: Vec<RangeInclusive<i32>>) -> i64 {
    row as i64 + 4_000_000 * (*covered_ranges[0].end() as i64 + 1)
}

fn get_row_coverage_ranges(row: i32, sensors: &Vec<SensorCoverage>) -> Vec<RangeInclusive<i32>> {
    let mut row_cov: Vec<RangeInclusive<i32>> = Vec::new();
    for s in sensors {
        if let Some(cov) = s.row_coverage(row) {
            row_cov.push(cov);
        }
    }
    row_cov
}
fn num_row_non_beacon(
    row: i32,
    sensors: &Vec<SensorCoverage>,
    beacons: &HashSet<Point>,
    limits: Option<&RangeInclusive<i32>>,
) -> i32 {
    let mut row_cov = get_row_coverage_ranges(row, sensors);
    if row_cov.is_empty() {
        return 0;
    }
    if let Some(lim) = limits {
        row_cov = truncate_ranges(row_cov, *lim.start(), *lim.end());
        return range_total_coverage(&merge_ranges(row_cov));
    }
    range_total_coverage(&merge_ranges(row_cov)) - beacons_per_row(beacons, row)
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

fn _range_total_span(ranges: &[RangeInclusive<i32>]) -> i32 {
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
    ranges.sort_by(|r1, r2| r1.start().cmp(r2.start()));
    let mut range_iter = ranges.into_iter();
    merged_ranges.push(range_iter.next().expect("At least one range"));
    for next_range in range_iter {
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

fn truncate_ranges(
    ranges: Vec<RangeInclusive<i32>>,
    min_bound: i32,
    max_bound: i32,
) -> Vec<RangeInclusive<i32>> {
    let mut truncated_ranges: Vec<RangeInclusive<i32>> = Vec::new();
    for range in ranges {
        // discard ranges that are completely out of bounds
        if range.end() < &min_bound || range.start() > &max_bound {
            continue;
        }
        let new_start = cmp::max(range.start(), &min_bound);
        let new_end = cmp::min(range.end(), &max_bound);
        truncated_ranges.push(*new_start..=*new_end);
    }
    truncated_ranges
}

fn split_range(range: RangeInclusive<i32>, num_splits: usize) -> Vec<RangeInclusive<i32>> {
    let span = range.end() - range.start();
    let mut splits: Vec<RangeInclusive<i32>> = Vec::new();
    let split_size: i32 = span / num_splits as i32;
    let mut start = *range.start();
    for _ in 0..num_splits {
        let split_end = start + split_size;
        splits.push(start..=split_end.clone());
        start = split_end;
    }
    splits
}
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    #[test]
    fn test_split_range() {
        assert_eq!(split_range(1..=100, 10).len(), 10);
        assert_eq!(split_range(1..=10, 3).len(), 3);
        assert_eq!(split_range(1..=10, 3)[2], 7..=10);
        assert_eq!(split_range(1..=16, 3).len(), 3);
        assert_eq!(split_range(1..=16, 3)[2], 11..=16);
    }

    #[test]
    fn test_part_1() {
        let input = include_str!("../inputs/2022.15.test").to_string();

        let (sensors, beacons) = build_sensor_beacon_map(input);
        assert_eq!(part_1(10, &sensors, &beacons), 26);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("../inputs/2022.15.test").to_string();
        let (sensors, beacons) = build_sensor_beacon_map(input);
        assert_eq!(part_2(0..=20, &sensors, &beacons), 56000011);
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let input = include_str!("../inputs/2022.15.test").to_string();
        let (sensors, beacons) = build_sensor_beacon_map(input);
        b.iter(|| part_2(0..=20, &sensors, &beacons));
    }

    #[test]
    fn test_truncate_ranges() {
        let r1: Vec<RangeInclusive<i32>> = vec![-1..=4, 6..=70];
        assert_eq!(truncate_ranges(r1, 0, 50), vec![0..=4, 6..=50]);
        // truncate both sides of a range
        let r2: Vec<RangeInclusive<i32>> = vec![-1..=44];
        assert_eq!(truncate_ranges(r2, 0, 10), vec![0..=10]);
        // discard ranges that are far out of bounds
        let r3: Vec<RangeInclusive<i32>> = vec![-10..=-5, 8..=10, 100..=300];
        assert_eq!(truncate_ranges(r3, 5, 15), vec![8..=10]);
    }
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
        assert_eq!(_range_total_span(&r1), 6);
        let r2: Vec<RangeInclusive<i32>> = vec![-10..=5, 10..=20, 50..=70];
        assert_eq!(_range_total_span(&r2), 80);
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

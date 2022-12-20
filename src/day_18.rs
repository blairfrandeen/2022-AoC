use std::cmp;
use std::collections::HashSet;

pub fn main(contents: String) {
    let points: HashSet<Point3D> = contents.lines().map(Point3D::build).collect();
    let obj = Object3D { points };
    let part_1 = obj.count_sides();
    println!("Part 1: {part_1}");
    let part_2: u32 = part_1
        - obj
            .voids()
            .into_iter()
            .map(|v| v.count_sides())
            .sum::<u32>();
    println!("Part 2: {:?}", part_2);
}

#[derive(Debug)]
struct Object3D {
    points: HashSet<Point3D>,
}

impl Object3D {
    fn voids(&self) -> Vec<Object3D> {
        let outside = self.extents().0.delta(-1, -1, -1);
        self.inverse()
            .chunks()
            .into_iter()
            .filter(|c| !c.points.contains(&outside))
            .collect()
    }

    fn chunks(&self) -> Vec<Object3D> {
        let mut chunks: Vec<Object3D> = Vec::new();
        let mut all_pts: Vec<Point3D> = self.points.clone().into_iter().collect();
        while !all_pts.is_empty() {
            let mut to_visit: Vec<Point3D> = Vec::new();
            let mut visited: Vec<Point3D> = Vec::new();
            let current_point = all_pts.pop().expect("at least one point");
            for neighbor in current_point.neighbors() {
                if !visited.contains(&neighbor)
                    && !to_visit.contains(&neighbor)
                    && all_pts.contains(&neighbor)
                {
                    to_visit.push(neighbor);
                }
            }
            visited.push(current_point);
            while let Some(current_point) = to_visit.pop() {
                for neighbor in current_point.neighbors() {
                    if !visited.contains(&neighbor)
                        && !to_visit.contains(&neighbor)
                        && all_pts.contains(&neighbor)
                    {
                        to_visit.push(neighbor);
                    }
                }
                visited.push(
                    all_pts.remove(all_pts.iter().position(|p| p == &current_point).unwrap()),
                );
            }
            let points: HashSet<Point3D> = visited.into_iter().collect();
            chunks.push(Object3D { points });
            // println!("{:?}", chunks);
            // println!();
        }
        chunks
    }
    fn inverse(&self) -> Object3D {
        let ext = self.extents();
        let min_ext = &ext.0.delta(-1, -1, -1);
        let max_ext = &ext.1.delta(1, 1, 1);
        // println!("{:?} {:?}", min_ext, max_ext);
        let mut inv_points = HashSet::new();
        for x in min_ext.x..=max_ext.x {
            for y in min_ext.y..=max_ext.y {
                for z in min_ext.z..=max_ext.z {
                    let new_point = Point3D { x, y, z };
                    if !self.points.contains(&new_point) {
                        inv_points.insert(new_point);
                    }
                }
            }
        }
        Object3D { points: inv_points }
    }

    fn extents(&self) -> (Point3D, Point3D) {
        (
            Point3D {
                x: self.extreme(Point3D::get_x, cmp::min),
                y: self.extreme(Point3D::get_y, cmp::min),
                z: self.extreme(Point3D::get_z, cmp::min),
            },
            Point3D {
                x: self.extreme(Point3D::get_x, cmp::max),
                y: self.extreme(Point3D::get_y, cmp::max),
                z: self.extreme(Point3D::get_z, cmp::max),
            },
        )
    }
    fn extreme(&self, coord: impl Fn(&Point3D) -> i32, cmp_fn: impl Fn(i32, i32) -> i32) -> i32 {
        self.points
            .iter()
            .map(coord)
            .reduce(|item, accum| cmp_fn(item, accum))
            .unwrap()
    }

    fn count_sides(&self) -> u32 {
        self.points
            .iter()
            .map(|p| 6 - count_adjacent(p, &self))
            .sum()
    }
}

fn count_adjacent(point: &Point3D, object: &Object3D) -> u32 {
    point
        .neighbors()
        .iter()
        .filter(|n| object.points.contains(n))
        .collect::<Vec<&Point3D>>()
        .len() as u32
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

impl Point3D {
    fn build(input: &str) -> Point3D {
        let coords: Vec<i32> = input
            .trim()
            .split(',')
            .map(|c| c.parse::<i32>().expect("input should be valid"))
            .collect();
        Point3D {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        }
    }

    fn delta(&self, dx: i32, dy: i32, dz: i32) -> Point3D {
        Point3D {
            x: self.x + dx,
            y: self.y + dy,
            z: self.z + dz,
        }
    }

    fn neighbors(&self) -> Vec<Point3D> {
        let deltas = vec![
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
            (0, 0, 1),
            (0, 0, -1),
        ];
        let neighbors: Vec<Point3D> = deltas
            .iter()
            .map(|d| Point3D {
                x: self.x + d.0,
                y: self.y + d.1,
                z: self.z + d.2,
            })
            .collect();
        neighbors
    }
    fn get_x(&self) -> i32 {
        self.x
    }
    fn get_y(&self) -> i32 {
        self.y
    }
    fn get_z(&self) -> i32 {
        self.z
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunks() {
        let p1 = Point3D::build("1,1,1");
        let p2 = Point3D::build("1,1,3");
        let p3 = Point3D::build("1,1,4");
        let p4 = Point3D::build("1,1,5");
        let o = Object3D {
            points: HashSet::from([p1, p2, p3, p4]),
        };
        assert_eq!(o.chunks().len(), 2);
    }

    #[test]
    fn test_inverse() {
        let o = Object3D {
            points: HashSet::from([Point3D::build("1,1,1")]),
        };
        assert_eq!(o.inverse().points.len(), 26);
        assert!(o.inverse().points.contains(&Point3D { x: 0, y: 1, z: 1 }));
        assert!(!o.inverse().points.contains(&Point3D { x: 5, y: 1, z: 1 }));
    }

    #[test]
    fn test_extreme() {
        let p1 = Point3D::build("1,1,1");
        let p2 = Point3D::build("1,7,2");
        let p3 = Point3D::build("3,8,0");
        let points = HashSet::from([p1, p2, p3]);
        let obj = Object3D { points };
        assert_eq!(obj.extreme(Point3D::get_y, std::cmp::max,), 8);
        assert_eq!(obj.extreme(Point3D::get_z, std::cmp::min,), 0);
    }
    #[test]
    fn test_adjacent() {
        let p1 = Point3D::build("1,1,1");
        let p2 = Point3D::build("1,1,2");
        let obj = Object3D {
            points: HashSet::from([p1, p2]),
        };
        assert_eq!(count_adjacent(&Point3D::build("1,1,1"), &obj), 1);
        assert_eq!(obj.count_sides(), 10);
        assert_eq!(
            obj.extents(),
            (Point3D { x: 1, y: 1, z: 1 }, Point3D { x: 1, y: 1, z: 2 })
        );
    }
    #[test]
    fn test_neighbors() {
        let p = Point3D::build("1,2,3");
        assert_eq!(p.neighbors().len(), 6);
        assert!(p.neighbors().contains(&Point3D::build("2,2,3")));
    }
    #[test]
    fn test_delta() {
        let p = Point3D::build("1,2,3");
        assert_eq!(p.delta(1, 0, -1), Point3D::build("2,2,2"));
    }
    #[test]
    fn test_build() {
        let input = "3,2,1\n";
        assert_eq!(Point3D::build(input), Point3D { x: 3, y: 2, z: 1 });
    }
}

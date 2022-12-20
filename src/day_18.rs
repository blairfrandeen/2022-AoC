pub fn main(contents: String) {
    let points: Vec<Point3D> = contents.lines().map(Point3D::build).collect();
    let part_1 = count_sides(&points);
    println!("Part 1: {part_1}");
}

fn count_sides(points: &Vec<Point3D>) -> u32 {
    points.iter().map(|p| 6 - count_adjacent(p, &points)).sum()
}

fn count_adjacent(point: &Point3D, object: &Vec<Point3D>) -> u32 {
    point
        .neighbors()
        .iter()
        .filter(|n| object.contains(n))
        .collect::<Vec<&Point3D>>()
        .len() as u32
}

#[derive(Debug, PartialEq)]
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjacent() {
        let p1 = Point3D::build("1,1,1");
        let p2 = Point3D::build("1,1,2");
        let points = vec![p1, p2];
        assert_eq!(count_adjacent(&points[0], &points), 1);
        assert_eq!(count_sides(&points), 10);
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

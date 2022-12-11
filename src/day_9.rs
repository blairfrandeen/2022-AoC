#![allow(unused)]
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn lead(&mut self, dir: char) {
        match dir {
            'U' => self.y += 1,
            'D' => self.y -= 1,
            'L' => self.x -= 1,
            'R' => self.x += 1,
            _ => panic!("Wrong direction!"),
        };
    }

    fn follow(&mut self, other: &Position) {
        if touching(self, other) {
            return ();
        }
        // self and other are in same row or column
        if self.x == other.x || self.y == other.y {
            if self.x - other.x > 1 {
                self.x -= 1; // Move left
            } else if other.x - self.x > 1 {
                self.x += 1; // Move right
            } else if self.y - other.y > 1 {
                self.y -= 1; // Move down
            } else if other.y - self.y > 1 {
                self.y += 1; // Move up
            }
        } else {
            // move diagonal
            if other.x > self.x && other.y > self.y {
                // Up and Right
                self.x += 1;
                self.y += 1;
            } else if other.x < self.x && other.y > self.y {
                // Up and Left
                self.x -= 1;
                self.y += 1;
            } else if other.x < self.x && other.y < self.y {
                // Down and Left
                self.x -= 1;
                self.y -= 1;
            } else if other.x > self.x && other.y < self.y {
                // Down and Right
                self.x += 1;
                self.y -= 1;
            }
        }
    }
}

fn touching(p1: &Position, p2: &Position) -> bool {
    !((p1.x - p2.x).abs() > 1 || (p1.y - p2.y).abs() > 1)
}

fn parse_instruction(input: &str) -> (char, u32) {
    let inst: Vec<&str> = input.trim().split(' ').collect();
    let dir: char = inst[0].parse().unwrap();
    let num: u32 = inst[1].parse().unwrap();
    (dir, num)
}

struct Rope {
    positions: Vec<Position>,
    length: usize,
}

impl Rope {
    fn build(length: usize) -> Rope {
        if length < 2 {
            panic!("Your rope is too short lol");
        }
        let mut positions = Vec::<Position>::new();
        for index in 0..length {
            let mut new_pos = Position { x: 0, y: 0 };
            positions.push(new_pos);
        }
        Rope { positions, length }
    }
}

pub fn main(contents: String) {
    let mut rope = Rope::build(10);
    let mut part_1: HashSet<Position> = HashSet::new();
    let mut part_2: HashSet<Position> = HashSet::new();
    for line in contents.lines() {
        // println!("{}", &line);
        let (dir, num) = parse_instruction(line);
        for _ in 0..num {
            rope.positions[0].lead(dir);
            for p in 1..rope.length {
                let next_knot = rope.positions[p - 1].clone();
                rope.positions[p].follow(&next_knot);
            }
            // println!("{:?} {:?}", &head, &tail);
            part_1.insert(rope.positions[1].clone());
            part_2.insert(rope.positions[9].clone());
        }
    }
    println!("Part 1: {}", part_1.len());
    println!("Part 2: {}", part_2.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "R 4\n";
        assert_eq!(parse_instruction(input), ('R', 4));
        assert_eq!(parse_instruction("L 5\n"), ('L', 5));
        assert_eq!(parse_instruction("L 15\n"), ('L', 15));
    }
    #[test]
    fn test_follow() {
        let mut p1 = Position { x: 0, y: 0 };
        let p3 = Position { x: 0, y: 2 };
        p1.follow(&p3);
        assert_eq!(p1, Position { x: 0, y: 1 });
        p1.follow(&p3); // should not move
        assert_eq!(p1, Position { x: 0, y: 1 });
        let p2 = Position { x: 1, y: 3 };
        p1.follow(&p2);
        assert_eq!(p1, Position { x: 1, y: 2 });
    }
    #[test]
    fn test_touch() {
        let p1 = Position { x: 0, y: 0 };
        let p2 = Position { x: 1, y: 1 };
        let p3 = Position { x: 0, y: 2 };
        let p4 = Position { x: 0, y: 0 };
        let p5 = Position { x: 1, y: 0 };
        assert!(touching(&p1, &p4));
        assert!(touching(&p1, &p2));
        assert!(!touching(&p1, &p3));
        assert!(touching(&p1, &p5));
        assert!(!touching(&p5, &p3));
    }
    #[test]
    fn test_lead() {
        let mut p = Position { x: 0, y: 0 };
        p.lead('U');
        assert_eq!(p.x, 0);
        assert_eq!(p.y, 1);
        p.lead('R');
        assert_eq!(p.x, 1);
        assert_eq!(p.y, 1);
    }
}

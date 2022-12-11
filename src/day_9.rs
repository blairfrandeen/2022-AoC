#![allow(unused)]
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
    fn follow(&mut self, other: Position) {
        if self.x - other.x > 1 {
            self.x -= 1;
        } else if other.x - self.x > 1 {
            self.x += 1;
        }
    }
}

fn touching(p1: &Position, p2: &Position) -> bool {
    !((p1.x - p2.x).abs() > 1 || (p1.y - p2.y).abs() > 1)
}

pub fn main(contents: String) {
    let mut head = Position { x: 0, y: 0 };
    let mut tail = Position { x: 0, y: 0 };
    println!("Hello AoC!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_touch() {
        let p1 = Position { x: 0, y: 0 };
        let p2 = Position { x: 1, y: 1 };
        let p3 = Position { x: 0, y: 2 };
        let p4 = Position { x: 0, y: 0 };
        assert!(touching(&p1, &p4));
        assert!(touching(&p1, &p2));
        assert!(!touching(&p1, &p3));
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

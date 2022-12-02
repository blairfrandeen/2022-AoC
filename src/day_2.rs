pub fn main(contents: String) {
    let mut total_score = 0;

    for line in contents.split("\n") {
        let (opponent_choice, your_choice) = match read_rps_line(line) {
            None => break,
            Some((c1, c2)) => (get_choice(c1), get_choice(c2)),
        };
        total_score += calculate_score(&your_choice, rps_winner(&your_choice, &opponent_choice));
    }
    println!("Part 1: {}", total_score);
}

#[derive(PartialEq, Debug)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

fn read_rps_line(line: &str) -> Option<(char, char)> {
    let mut chars = line.chars();
    let p1 = chars.next();
    if p1 == None {
        // if p1 == '\n' {
        None
    } else {
        chars.next();
        let p2 = chars.next().unwrap();
        Some((p1.unwrap(), p2))
    }
}

fn rps_winner(p1: &RPS, p2: &RPS) -> Option<RPS> {
    match (p1, p2) {
        (RPS::Rock, RPS::Paper) => Some(RPS::Paper),
        (RPS::Paper, RPS::Rock) => Some(RPS::Paper),
        (RPS::Rock, RPS::Scissors) => Some(RPS::Rock),
        (RPS::Scissors, RPS::Rock) => Some(RPS::Rock),
        (RPS::Paper, RPS::Scissors) => Some(RPS::Scissors),
        (RPS::Scissors, RPS::Paper) => Some(RPS::Scissors),
        _ => None,
    }
}

fn get_choice(key: char) -> RPS {
    match key {
        'A' => RPS::Rock,
        'X' => RPS::Rock,
        'B' => RPS::Paper,
        'Y' => RPS::Paper,
        'C' => RPS::Scissors,
        'Z' => RPS::Scissors,
        _ => panic!("Invalid Key!"),
    }
}

fn calculate_score(choice: &RPS, result: Option<RPS>) -> i32 {
    let score = match choice {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissors => 3,
    };
    match result {
        None => score + 3,
        Some(winning_choice) => {
            if &winning_choice == choice {
                score + 6
            } else {
                score
            }
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_choice() {
        assert_eq!(get_choice('X'), RPS::Rock);
    }

    #[test]
    fn test_read_line() {
        assert_eq!(read_rps_line("A X\n"), Some(('A', 'X')));
        assert_eq!(read_rps_line("\n"), None);
    }

    #[test]
    fn test_calculate() {
        assert_eq!(calculate_score(&RPS::Paper, Some(RPS::Paper)), 8);
        assert_eq!(calculate_score(&RPS::Rock, Some(RPS::Paper)), 1);
        assert_eq!(calculate_score(&RPS::Scissors, None), 6);
    }
    #[test]
    fn test_winner() {
        let p1 = RPS::Rock;
        let p2 = RPS::Scissors;
        let answer = rps_winner(&p1, &p2);
        assert_eq!(answer, Some(RPS::Rock));

        let p1 = RPS::Paper;
        let p2 = RPS::Scissors;
        let answer = rps_winner(&p1, &p2);
        assert_eq!(answer, Some(RPS::Scissors));

        let p1 = RPS::Paper;
        let p2 = RPS::Paper;
        let answer = rps_winner(&p1, &p2);
        assert_eq!(answer, None);
    }
}

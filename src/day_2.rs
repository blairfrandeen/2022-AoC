pub fn main(contents: String) {
    let part_1_score = part_1(&contents);
    println!("Part 1: {}", part_1_score);

    let part_2_score = part_2(&contents);
    println!("Part 2: {}", part_2_score);
}

fn part_1(contents: &String) -> i32 {
    let mut part_1_score = 0;
    for line in contents.split("\n") {
        let (opponent_choice, your_choice) = match read_rps_line(line) {
            None => break,
            Some((c1, c2)) => (get_choice(c1), get_choice(c2)),
        };

        part_1_score += calculate_score(&your_choice, rps_winner(&your_choice, &opponent_choice));
    }
    part_1_score
}

fn part_2(contents: &String) -> i32 {
    let mut part_2_score = 0;
    for line in contents.split("\n") {
        let (opponent_choice, desired_result) = match read_rps_line(line) {
            None => break,
            Some((c1, c2)) => (get_choice(c1), get_result(c2)),
        };
        let your_choice = get_desired_outcome(&opponent_choice, &desired_result);
        part_2_score += calculate_score(&your_choice, desired_result);
        // println!("Move should be {:?}", tmp_);
    }
    part_2_score
}

fn get_desired_outcome(opponent_choice: &RPS, desired_outcome: &GameResult) -> RPS {
    let options = vec![RPS::Rock, RPS::Paper, RPS::Scissors];
    for opt in options.into_iter() {
        if rps_winner(&opt, opponent_choice) == *desired_outcome {
            return opt;
        }
    }
    unreachable!("!")
}

#[derive(PartialEq, Debug)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

#[derive(PartialEq, Debug)]
enum GameResult {
    Win,
    Loss,
    Draw,
}

fn get_result(key: char) -> GameResult {
    match key {
        'X' => GameResult::Loss,
        'Y' => GameResult::Draw,
        'Z' => GameResult::Win,
        _ => panic!("Bad key!"),
    }
}

fn read_rps_line(line: &str) -> Option<(char, char)> {
    let mut chars = line.chars();
    let p1 = chars.next();
    if p1 == None {
        None
    } else {
        chars.next();
        let p2 = chars.next();
        if p2 == None {
            None
        } else {
            Some((p1.unwrap(), p2.unwrap()))
        }
    }
}

fn rps_winner(your_choice: &RPS, opponents_choice: &RPS) -> GameResult {
    if your_choice == opponents_choice {
        GameResult::Draw
    } else {
        match (your_choice, opponents_choice) {
            (RPS::Rock, RPS::Paper) => GameResult::Loss,
            (RPS::Paper, RPS::Rock) => GameResult::Win,
            (RPS::Rock, RPS::Scissors) => GameResult::Win,
            (RPS::Scissors, RPS::Rock) => GameResult::Loss,
            (RPS::Paper, RPS::Scissors) => GameResult::Loss,
            (RPS::Scissors, RPS::Paper) => GameResult::Win,
            _ => panic!("invalid choices!"),
        }
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

fn calculate_score(choice: &RPS, result: GameResult) -> i32 {
    let score = match choice {
        RPS::Rock => 1,
        RPS::Paper => 2,
        RPS::Scissors => 3,
    };
    match result {
        GameResult::Loss => score,
        GameResult::Draw => score + 3,
        GameResult::Win => score + 6,
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
        assert_eq!(calculate_score(&RPS::Paper, GameResult::Win), 8);
        assert_eq!(calculate_score(&RPS::Rock, GameResult::Loss), 1);
        assert_eq!(calculate_score(&RPS::Scissors, GameResult::Draw), 6);
    }
    #[test]
    fn test_winner() {
        let p1 = RPS::Rock;
        let p2 = RPS::Scissors;
        let answer = rps_winner(&p1, &p2);
        assert_eq!(answer, GameResult::Win);

        let p1 = RPS::Paper;
        let p2 = RPS::Scissors;
        let answer = rps_winner(&p1, &p2);
        assert_eq!(answer, GameResult::Loss);

        let p1 = RPS::Paper;
        let p2 = RPS::Paper;
        let answer = rps_winner(&p1, &p2);
        assert_eq!(answer, GameResult::Draw);
    }
}

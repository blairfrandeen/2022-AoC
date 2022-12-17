pub fn main(contents: String) {
    let part_1_score = part_1(&contents);
    println!("Part 1: {part_1_score}");

    let part_2_score = part_2(&contents);
    println!("Part 2: {part_2_score}");
}

fn part_1(contents: &str) -> i32 {
    let mut part_1_score = 0;
    for line in contents.split('\n') {
        let (opponent_choice, your_choice) = match read_rps_line(line) {
            None => break,
            Some((c1, c2)) => (get_choice(c1), get_choice(c2)),
        };

        part_1_score += calculate_score(&your_choice, rps_winner(&your_choice, &opponent_choice));
    }
    part_1_score
}

fn part_2(contents: &str) -> i32 {
    let mut part_2_score = 0;
    for line in contents.split('\n') {
        let (opponent_choice, desired_result) = match read_rps_line(line) {
            None => break,
            Some((c1, c2)) => (get_choice(c1), get_result(c2)),
        };
        let your_choice = get_desired_outcome(&opponent_choice, &desired_result);
        part_2_score += calculate_score(&your_choice, desired_result);
    }
    part_2_score
}

fn get_desired_outcome(opponent_choice: &Rps, desired_outcome: &GameResult) -> Rps {
    let options = vec![Rps::Rock, Rps::Paper, Rps::Scissors];
    for opt in options.into_iter() {
        if rps_winner(&opt, opponent_choice) == *desired_outcome {
            return opt;
        }
    }
    unreachable!("!")
}

#[derive(PartialEq, Debug)]
enum Rps {
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
    let chars: Vec<char> = line
        .split_whitespace()
        .map(|c| c.parse::<char>().unwrap())
        .collect();
    match chars.len() {
        2 => Some((chars[0], chars[1])),
        0 => None,
        _ => panic!("Too many characters!"),
    }
}

fn rps_winner(your_choice: &Rps, opponents_choice: &Rps) -> GameResult {
    if your_choice == opponents_choice {
        GameResult::Draw
    } else {
        match (your_choice, opponents_choice) {
            (Rps::Rock, Rps::Paper) => GameResult::Loss,
            (Rps::Paper, Rps::Rock) => GameResult::Win,
            (Rps::Rock, Rps::Scissors) => GameResult::Win,
            (Rps::Scissors, Rps::Rock) => GameResult::Loss,
            (Rps::Paper, Rps::Scissors) => GameResult::Loss,
            (Rps::Scissors, Rps::Paper) => GameResult::Win,
            _ => panic!("invalid choices!"),
        }
    }
}

fn get_choice(key: char) -> Rps {
    match key {
        'A' => Rps::Rock,
        'X' => Rps::Rock,
        'B' => Rps::Paper,
        'Y' => Rps::Paper,
        'C' => Rps::Scissors,
        'Z' => Rps::Scissors,
        _ => panic!("Invalid Key!"),
    }
}

fn calculate_score(choice: &Rps, result: GameResult) -> i32 {
    let score = match choice {
        Rps::Rock => 1,
        Rps::Paper => 2,
        Rps::Scissors => 3,
    };
    match result {
        GameResult::Loss => score,
        GameResult::Draw => score + 3,
        GameResult::Win => score + 6,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_choice() {
        assert_eq!(get_choice('X'), Rps::Rock);
    }

    #[test]
    fn test_read_line() {
        assert_eq!(read_rps_line("A X\n"), Some(('A', 'X')));
        assert_eq!(read_rps_line("\n"), None);
    }

    #[test]
    fn test_calculate() {
        assert_eq!(calculate_score(&Rps::Paper, GameResult::Win), 8);
        assert_eq!(calculate_score(&Rps::Rock, GameResult::Loss), 1);
        assert_eq!(calculate_score(&Rps::Scissors, GameResult::Draw), 6);
    }
    #[test]
    fn test_winner() {
        let p1 = Rps::Rock;
        let p2 = Rps::Scissors;
        let answer = rps_winner(&p1, &p2);
        assert_eq!(answer, GameResult::Win);

        let p1 = Rps::Paper;
        let p2 = Rps::Scissors;
        let answer = rps_winner(&p1, &p2);
        assert_eq!(answer, GameResult::Loss);

        let p1 = Rps::Paper;
        let p2 = Rps::Paper;
        let answer = rps_winner(&p1, &p2);
        assert_eq!(answer, GameResult::Draw);
    }
}

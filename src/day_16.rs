use nom::{
    bytes::complete::tag, bytes::complete::take_till, bytes::complete::take_while,
    character::complete::digit1, character::complete::i32, multi::separated_list1, sequence::tuple,
    IResult,
};
use std::collections::HashMap;
use std::rc::Rc;

type ValveNetwork = HashMap<String, Valve>;

#[derive(Debug)]
struct Valve {
    name: String,
    flow_rate: u32,
    is_open: bool,
    tunnels: Vec<String>,
}

impl Valve {
    fn build(input: &str) -> Valve {
        let (name, flow_rate, tunnels) = parse_input(input);
        Valve {
            name: name.to_string(),
            is_open: false,
            flow_rate,
            tunnels: tunnels.into_iter().map(|t| t.to_string()).collect(),
        }
    }
}

fn parse_input(input: &str) -> (&str, u32, Vec<&str>) {
    let res: IResult<&str, (&str, &str, &str, &str, &str, Vec<&str>)> = tuple((
        tag("Valve "),                          // first word
        take_till(|c: char| c.is_whitespace()), // valve name
        take_till(|c: char| c.is_digit(10)),
        take_while(|c: char| c.is_digit(10)), // flow rate
        take_till(|c: char| c.is_uppercase()),
        separated_list1(tag(", "), take_while(|c: char| c.is_uppercase())), // connected valves
    ))(input);
    let (_, name, _, flow_rate, _, tunnels) = res.unwrap().1;
    let flow_rate: u32 = flow_rate.parse::<u32>().expect("Valid flow rate");
    // println!("{} {} {:?}", name, flow_rate, tunnels);
    (name, flow_rate, tunnels)
}
pub fn main(contents: String) {
    println!("Hello AoC!");
    let mut network: ValveNetwork = ValveNetwork::new();
    for line in contents.lines() {
        let new_valve: Valve = Valve::build(line);
        network.insert(new_valve.name.clone(), new_valve);
    }
    let current_position = "AA"; // starting position
    println!("{:?}", network.get("AA"));
    println!("{:?}", get_next_moves("AA".to_string(), &network, 30));
}

#[derive(Debug)]
enum Move {
    OpenValve,
    NextCave(String),
}

fn max_pressure(current_position: String, network: &ValveNetwork, current_time: u32, current_pressure: u32) -> u32 {
    if let Some(next_moves) = get_next_moves(current_position.clone(), &network, current_time) {
        let mut results: Vec<u32> =  Vec::new();
        for move_ in next_moves.iter() {
            let p = match move_ {
                Move::OpenValve => max_pressure(
                    current_position.clone(),
                    &network,
                    current_time - 1,
                    (current_time - 1) * network.get(&current_position).unwrap().flow_rate
                ),
                Move::NextCave(cave) => max_pressure(cave.to_string(), &network, current_time - 1, current_pressure)
            };
            results.push(p);
        }
        0 // need to return max of result vec
    } else {
        current_pressure
    }
}

fn get_next_moves(current_position: String, network: &ValveNetwork, current_time: u32) -> Option<Vec<Move>> {
    if current_time == 0 {
        None
    } else {
        let current_cave = network.get(&current_position).unwrap();
        let mut moves: Vec<Move> = Vec::new();
        if current_cave.flow_rate > 0 && !current_cave.is_open {
            moves.push(Move::OpenValve)
        }
        for cave in current_cave.tunnels.iter() {
            moves.push(Move::NextCave((cave.clone())));
        }
        Some(moves)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valve() {
        let v = Valve::build("Valve AA has flow rate=10; tunnels lead to valves DD, II, BB");

        assert_eq!(v.name, "AA");
        assert_eq!(v.flow_rate,10);
        // assert_eq!(v.tunnels,vec![]);
    }

    #[test]
    fn test_parse_input() {
        let inp = "Valve AA has flow rate=10; tunnels lead to valves DD, II, BB";
        assert_eq!(parse_input(inp), ("AA", 10, vec!["DD", "II", "BB"]))
    }
}

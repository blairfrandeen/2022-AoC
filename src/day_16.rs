use nom::{
    bytes::complete::tag, bytes::complete::take_till, bytes::complete::take_while,
    multi::separated_list1, sequence::tuple, IResult,
};
use std::collections::HashMap;
use std::collections::HashSet;

type ValveNetwork = HashMap<String, Valve>;

#[derive(Debug, Clone)]
struct NetworkState {
    time_remaining: u32,
    current_position: String,          // current valve we are on
    open_valves: HashMap<String, u32>, // at what time was what valve opened?
}

#[derive(Debug, Clone)]
struct Valve {
    name: String,
    flow_rate: u32,
    // is_open: bool,
    tunnels: Vec<String>,
}

impl Valve {
    fn build(input: &str) -> Valve {
        let (name, flow_rate, tunnels) = parse_input(input);
        Valve {
            name: name.to_string(),
            // is_open: false,
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

/* NOTE: If not using full program from github.com/blairfrandeen/2022-AoC/
simply comment out the function definition below, and uncomment the following
two lines, replacing the '../inputs/2022.16' with the appropriate file. */
pub fn main(contents: String) {
    // pub fn main() {
    //     let contents = include_str!("../inputs/2022.16").to_string();
    let mut network: ValveNetwork = ValveNetwork::new();
    for line in contents.lines() {
        let new_valve: Valve = Valve::build(line);
        network.insert(new_valve.name.clone(), new_valve);
    }
    let current_position = "AA"; // starting position
                                 // println!("{:?}", network.get("AA"));
                                 // println!("{:?}", get_next_moves("AA".to_string(), &network, 30));
    let time_remaining = 20;
    let initial_state = NetworkState {
        time_remaining,
        current_position: current_position.to_string(),
        open_valves: HashMap::new(),
    };
    println!(
        "max pressure attainable: {:?}",
        max_pressure(initial_state, &network, 0)
    );
}

fn max_pressure(
    network_state: NetworkState,
    network: &ValveNetwork,
    current_pressure: u32,
    // path: String,
) -> u32 {
    if network_state.time_remaining == 0 {
        println!(
            "Time Remaining: {}, current pressure: {}, current position: {} Open valves: {:?}",
            &network_state.time_remaining,
            &current_pressure,
            &network_state.current_position,
            &network_state.open_valves,
        );
    }
    if let Some(next_moves) = get_next_moves(&network_state, &network) {
        let mut results: Vec<u32> = Vec::new();
        // println!("possible next moves: {:?}", next_moves);
        for move_ in next_moves.iter() {
            let mut new_state = network_state.clone();
            let next_time = network_state.time_remaining - 1;
            new_state.time_remaining = next_time;
            let p = match move_ {
                Move::OpenValve => {
                    // let mut new_network = network.clone();
                    // new_network.get_mut(&current_position).unwrap().is_open = true;
                    new_state
                        .open_valves
                        .insert(new_state.current_position.clone(), next_time);
                    // let next_time = network_state.time_remaining - 1;
                    // let flow_time = next_time;
                    max_pressure(
                        new_state.clone(),
                        &network,
                        (next_time
                            * network
                                .get(&network_state.current_position)
                                .unwrap()
                                .flow_rate)
                            + current_pressure,
                    )
                }
                Move::NextCave(cave) => {
                    new_state.current_position = cave.to_string();
                    max_pressure(new_state, &network, current_pressure)
                }
            };
            results.push(p);
        }
        results.sort();
        // println!("pressures found: {:?}", results);
        *results.last().unwrap()
    } else {
        current_pressure
    }
}

#[derive(Debug)]
enum Move {
    OpenValve,
    NextCave(String),
}

fn get_next_moves(
    network_state: &NetworkState,
    // current_position: String,
    network: &ValveNetwork,
    // current_time: u32,
) -> Option<Vec<Move>> {
    if network_state.time_remaining == 0 {
        None
    } else {
        let current_cave = network.get(&network_state.current_position).unwrap();
        let mut moves: Vec<Move> = Vec::new();
        if current_cave.flow_rate > 0 && !network_state.open_valves.contains_key(&current_cave.name)
        {
            moves.push(Move::OpenValve)
        }
        for cave in current_cave.tunnels.iter() {
            moves.push(Move::NextCave(cave.clone()));
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
        assert_eq!(v.flow_rate, 10);
        // assert_eq!(v.tunnels,vec![]);
    }

    #[test]
    fn test_parse_input() {
        let inp = "Valve AA has flow rate=10; tunnels lead to valves DD, II, BB";
        assert_eq!(parse_input(inp), ("AA", 10, vec!["DD", "II", "BB"]))
    }

    #[test]
    fn test_small_network_max() {
        let mut network: ValveNetwork = ValveNetwork::new();
        let a = Valve::build("Valve AA has flow rate=0; tunnels lead to valves BB, CC");
        let b = Valve::build("Valve BB has flow rate=2; tunnels lead to valves AA");
        let c = Valve::build("Valve CC has flow rate=15; tunnels lead to valves AA");
        network.insert("AA".to_string(), a);
        network.insert("BB".to_string(), b);
        network.insert("CC".to_string(), c);

        println!("{:?}", network);

        let initial_state = NetworkState {
            time_remaining: 6,
            current_position: "AA".to_string(),
            open_valves: HashMap::new(),
        };
        assert_eq!(
            (15 * 4) + (2 * 1),
            max_pressure(initial_state, &network, 0,)
        );

        // assert_eq!(v.tunnels,vec![]);
    }
}

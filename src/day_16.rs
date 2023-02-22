use nom::{
    bytes::complete::tag, bytes::complete::take_till, bytes::complete::take_while,
    multi::separated_list1, sequence::tuple, IResult,
};
use std::collections::HashMap;
use std::collections::HashSet;
use std::thread::current;

type ValveNetwork = HashMap<String, Valve>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct NetworkState {
    time_remaining: u32,
    current_position: String,         // current valve we are on
    open_valves: Vec<Option<String>>, // at what time was what valve opened?
}

impl NetworkState {
    fn pressure(&self, network: &ValveNetwork) -> u32 {
        let mut pressure = 0;
        for (i, val) in self.open_valves.iter().enumerate() {
            if let Some(name) = val {
                pressure += (i as u32) * network.get(name).unwrap().flow_rate
            }
        }
        pressure
    }

    fn remaining_potential(&self, network: &ValveNetwork) -> u32 {
        let mut potential = 0;
        if self.time_remaining <= 1 {
            // println!("{}", &potential);
            return potential;
        }
        let mut time_counter = self.time_remaining - 1;
        let mut valve_list: Vec<Valve> = network.clone().into_values().collect(); //.sort().reverse();
        valve_list.sort_by(|a, b| a.flow_rate.cmp(&b.flow_rate));
        valve_list.reverse();
        for valve in valve_list {
            if time_counter <= 1 {
                // println!("{}", &potential);
                return potential;
            }
            if !self.open_valves.contains(&Some(valve.name.clone())) {
                potential += valve.flow_rate * time_counter;
                time_counter -= 2;
            }
        }
        // println!("{}", &potential);
        potential
    }
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
    (name, flow_rate, tunnels)
}

/* NOTE: If not using full program from github.com/blairfrandeen/2022-AoC/
simply comment out the function definition below, and uncomment the following
two lines, replacing the '../inputs/2022.16' with the appropriate file. */
pub fn main(contents: String) {
    // let contents = include_str!("../test_input.txt").to_string();
    let mut network: ValveNetwork = ValveNetwork::new();
    for line in contents.lines() {
        let new_valve: Valve = Valve::build(line);
        network.insert(new_valve.name.clone(), new_valve);
    }
    let current_position = "AA"; // starting position
    let time_remaining = 30;
    let initial_state = NetworkState {
        time_remaining,
        current_position: current_position.to_string(),
        open_valves: vec![None; time_remaining as usize],
    };
    println!(
        "Max pressure release: {}",
        initial_state.remaining_potential(&network)
    );
    let mut visited = HashSet::<NetworkState>::new();
    println!(
        "max pressure attainable: {:?}",
        find_max_pressure(initial_state, &network, &mut visited, 0)
    );
}

// HashMap<NetworkState, u32>
// a->c->a->b->d->b->e
// a->c->a->b->e

// max(turn_on_a, visit_b, visit_c)
//     a
//    /  \
//   b    c
//  / \
// d   e

fn find_max_pressure(
    network_state: NetworkState,
    network: &ValveNetwork,
    visited: &mut HashSet<NetworkState>,
    max_pressure: u32,
    // path: String,
) -> u32 {
    if !visited.insert(network_state.clone()) {
        return max_pressure;
    }
    let current_pressure = network_state.pressure(network);
    // let rem = network_state.remaining_potential(&network);
    // println!("{} {} {}", max_pressure, current_pressure, rem);
    if network_state.remaining_potential(&network) == 0 {
        //+ current_pressure < max_pressure {
        // println!("blair's check worked! :)");
        return max_pressure;
    }
    let mut new_max_pressure = match current_pressure > max_pressure {
        true => current_pressure,
        false => max_pressure,
    };

    // if current_pressure < max_pressure {
    //     return max_pressure;
    // }
    /*

    println!(
        "Time Remaining: {}, current pressure: {}, current position: {} Open valves: {:?}",
        &network_state.time_remaining,
        &max_pressure,
        &network_state.current_position,
        &network_state.open_valves,
    );
    */
    if let Some(next_moves) = get_next_moves(&network_state, &network) {
        // let mut results: Vec<u32> = Vec::new();
        for move_ in next_moves.iter() {
            let new_state = execute_move(&network_state, move_);
            return find_max_pressure(new_state, &network, visited, new_max_pressure);
            //     if p > new_max_pressure {
            //         new_max_pressure = p;
            //         // results.push(p);
            //     }
        }
        new_max_pressure
        // results.sort();
        // println!("pressures found: {:?}", results);
        // *results.last().unwrap()
    } else {
        current_pressure
    }
}

fn execute_move(network_state: &NetworkState, move_: &Move) -> NetworkState {
    let mut new_state = network_state.clone();
    let time_remaining = new_state.time_remaining - 1;
    new_state.time_remaining = time_remaining;
    match move_ {
        Move::OpenValve => {
            new_state.open_valves[time_remaining as usize] =
                Some(new_state.current_position.clone());
        }
        Move::NextCave(cave) => {
            new_state.current_position = cave.to_string();
        }
    };
    new_state
}

#[derive(Debug)]
enum Move {
    OpenValve,
    NextCave(String),
}

fn get_next_moves(network_state: &NetworkState, network: &ValveNetwork) -> Option<Vec<Move>> {
    if network_state.time_remaining == 0 {
        None
    } else {
        let current_cave = network.get(&network_state.current_position).unwrap();
        let mut moves: Vec<Move> = Vec::new();
        if current_cave.flow_rate > 0
            && !network_state
                .open_valves
                .contains(&Some(current_cave.name.clone()))
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
            open_valves: vec![None; 6],
        };
        let mut visited = HashSet::<NetworkState>::new();
        assert_eq!(
            (15 * 4) + (2 * 1),
            find_max_pressure(initial_state, &network, &mut visited, 0,)
        );

        // assert_eq!(v.tunnels,vec![]);
    }
}

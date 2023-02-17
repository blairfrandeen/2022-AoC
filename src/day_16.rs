use nom::{
    bytes::complete::tag, bytes::complete::take_till, bytes::complete::take_while,
    character::complete::digit1, character::complete::i32, multi::separated_list1, sequence::tuple,
    IResult,
};
use std::collections::HashMap;
use std::rc::Rc;

type ValveNetwork = HashMap<String, Valve>;
struct Valve {
    name: String,
    flow_rate: u32,
    tunnels: Vec<Rc<Valve>>,
}

impl Valve {
    fn build(input: &str) -> Valve {
        let (name, flow_rate, tunnels) = parse_input(input);
        let tunnels: Vec<Rc<Valve>> = tunnels
            .into_iter()
            .map(|t| {
                Rc::new(Valve {
                    name: t.to_string(),
                    flow_rate: 0,
                    tunnels: vec![],
                })
            })
            .collect();
        Valve {
            name: name.to_string(),
            flow_rate,
            tunnels,
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
    println!("{} {} {:?}", name, flow_rate, tunnels);
    (name, flow_rate, tunnels)
}
pub fn main(contents: String) {
    println!("Hello AoC!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valve() {
        assert_eq!(
            Valve::build("Valve AA has flow rate=0; tunnels lead to valves DD, II, BB").name,
            "AA"
        );
    }
}

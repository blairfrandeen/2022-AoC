use std::collections::HashMap;

pub fn main(contents: String) {
    let mut monkeys = parse_monkeys(contents);
    for _ in 0..10000 {
        take_turn(&mut monkeys);
    }
    let mb = monkey_business(&monkeys);
    println!("Quantity of Monkey Business: {mb}");
}

struct Monkey {
    items: Vec<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    test: u32,
    test_true: usize,
    test_false: usize,
    num_inspections: u64,
}

fn take_turn(monkeys: &mut HashMap<usize, Monkey>) {
    let monkey_test_mod: u32 = monkeys
        .iter()
        .map(|(_, m)| m.test)
        .reduce(|accum, item| accum * item)
        .unwrap();
    for monkey_index in 0..monkeys.len() {
        let mut current_monkey = monkeys.get_mut(&monkey_index).unwrap();
        let mut thrown_true: Vec<u64> = Vec::new();
        let mut thrown_false: Vec<u64> = Vec::new();
        let true_index = current_monkey.test_true;
        let false_index = current_monkey.test_false;
        while let Some(mut item) = current_monkey.items.pop() {
            current_monkey.num_inspections += 1; // inspect item
                                                 // item = item % current_monkey.test as u64;
            item = (current_monkey.operation)(item.into());
            // item /= 3; // decrease worry
            item %= monkey_test_mod as u64; // decrease worry
            match item % current_monkey.test as u64 {
                0 => thrown_true.push(item),
                _ => thrown_false.push(item),
            };
        }
        thrown_true.reverse();
        thrown_false.reverse();
        let target_true = monkeys.get_mut(&true_index).unwrap();
        while let Some(item) = thrown_true.pop() {
            target_true.items.push(item);
        }
        let target_false = monkeys.get_mut(&false_index).unwrap();
        while let Some(item) = thrown_false.pop() {
            target_false.items.push(item);
        }
    }
}

fn monkey_business(monkeys: &HashMap<usize, Monkey>) -> u64 {
    let mut inspections: Vec<u64> = monkeys.iter().map(|(_, m)| m.num_inspections).collect();
    inspections.sort();
    inspections.reverse();
    inspections[0] as u64 * inspections[1] as u64
}

fn parse_monkeys(input: String) -> HashMap<usize, Monkey> {
    let mut monkeys: HashMap<usize, Monkey> = HashMap::new();
    let mut index: usize = 0;
    let mut input_stream = input.lines();
    loop {
        if input_stream.next().is_none() {
            break;
        }
        let items = parse_items(input_stream.next().unwrap());
        let operation = parse_operation(input_stream.next().unwrap());
        let test = parse_last(input_stream.next().unwrap());
        let test_true = parse_last(input_stream.next().unwrap()) as usize;
        let test_false = parse_last(input_stream.next().unwrap()) as usize;
        monkeys.insert(
            index,
            Monkey {
                items,
                operation,
                test,
                test_true,
                test_false,
                num_inspections: 0,
            },
        );
        index += 1;
        if input_stream.next().is_none() {
            break;
        }
    }

    monkeys
}

fn parse_items(line: &str) -> Vec<u64> {
    let mut items: Vec<u64> = Vec::new();
    let line_items: Vec<&str> = line
        .trim()
        .split(|c| c == ',' || c == ':' || c == ' ')
        .collect();
    for item in line_items {
        if let Ok(new_item) = item.parse::<u64>() {
            items.push(new_item);
        }
    }
    items
}

fn parse_operation(op: &str) -> Box<dyn Fn(u64) -> u64> {
    let mut line_items: Vec<&str> = op.trim().split_whitespace().collect();
    let constant = match line_items.pop().unwrap() {
        "old" => return Box::new(|x| x * x),
        num => num.parse::<u64>().unwrap(),
    };
    match line_items.pop().unwrap() {
        "*" => Box::new(move |x| x * constant),
        "+" => Box::new(move |x| x + constant),
        _ => panic!("Bad Operator!"),
    }
}

fn parse_last(line: &str) -> u32 {
    line.trim()
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_business() {
        let mut monkeys = parse_monkeys(mock_monkeys().to_string());
        monkeys.get_mut(&0).unwrap().num_inspections = 5;
        monkeys.get_mut(&1).unwrap().num_inspections = 7;
        assert_eq!(monkey_business(&monkeys), 35);
    }
    #[test]
    fn test_parse_monkeys() {
        let monkeys = parse_monkeys(mock_monkeys().to_string());
        assert_eq!(monkeys[&0].test, 13);
        assert_eq!(monkeys[&1].test_true, 7);
        assert_eq!(monkeys[&1].test_false, 4);
    }
    #[test]
    fn test_parse_last() {
        assert_eq!(parse_last("  Test: divisible by 19\n"), 19);
        assert_eq!(parse_last("    If true: throw to monkey 6\n"), 6);
        assert_eq!(parse_last("    If false: throw to monkey 1\n"), 1);
    }
    #[test]
    fn test_parse_items() {
        assert_eq!(
            parse_items("  Starting Items: 54, 65, 75, 74\n"),
            vec![54, 65, 75, 74]
        );
        assert_eq!(parse_items("  Starting Items: 74\n"), vec![74]);
        assert_eq!(parse_items("  Starting Items:\n"), vec![]);
    }

    #[test]
    fn test_parse_operation() {
        assert_eq!(parse_operation("  Operation: new = old * old\n")(7), 49);
        assert_eq!(parse_operation("  Operation: new = old * 3\n")(7), 21);
        assert_eq!(parse_operation("  Operation: new = old + 1\n")(7), 8);
    }

    fn mock_monkeys() -> String {
        let monkeys = indoc! {"
        Monkey 0:
          Starting items: 89, 73, 66, 57, 64, 80
          Operation: new = old * 3
          Test: divisible by 13
            If true: throw to monkey 6
            If false: throw to monkey 2

        Monkey 1:
          Starting items: 83, 78, 81, 55, 81, 59, 69
          Operation: new = old + 1
          Test: divisible by 3
            If true: throw to monkey 7
            If false: throw to monkey 4
            "}
        .to_string();
        monkeys
    }
}

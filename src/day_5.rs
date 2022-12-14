// 9 stacks
// ~500 instructions
// No more than ~20 crates moved at a time
#[derive(PartialEq, Debug)]
struct Instruction {
    qty: usize,
    from: usize,
    to: usize,
}

pub fn main(contents: String) {
    let mut stack_cols_p1: Vec<Vec<char>> = initialize_stacks(&contents);
    let mut stack_cols_p2 = stack_cols_p1.clone();
    let mut instructions = contents.lines();
    instructions.find(|&b| b.is_empty());
    for line in instructions {
        // execute the moves as they come
        stack_cols_p1 = execute_instruction(stack_cols_p1, parse_instruction(line), false);
        stack_cols_p2 = execute_instruction(stack_cols_p2, parse_instruction(line), true);
    }
    let message_p1 = assemble_message(stack_cols_p1);
    let message_p2 = assemble_message(stack_cols_p2);
    println!("Part 1: {message_p1}");
    println!("Part 2: {message_p2}");
}

fn initialize_stacks(contents: &str) -> Vec<Vec<char>> {
    let mut stack_rows: Vec<Vec<char>> = Vec::new();
    let mut stack_cols: Vec<Vec<char>> = Vec::new();
    let num_stacks: u32;

    for line in contents.lines() {
        if line.is_empty() {
            // first blank line denotes end of stacks
            num_stacks = stack_rows
                .pop() // last row is row of numbers
                .unwrap()
                .pop() // last element in the row
                .unwrap()
                .to_digit(10) // convert from char to digit
                .unwrap();
            println!("We have {num_stacks} stacks!");

            // Now we need to build the stacks
            // start by making empty stacks
            stack_cols = vec![vec![]; num_stacks as usize];
            while let Some(mut row) = stack_rows.pop() {
                // start on the bottom row
                for index in 0..num_stacks {
                    let next_box = row.pop().unwrap();
                    match next_box {
                        ' ' => continue, // blanks don't exist
                        _ => stack_cols[index as usize].push(next_box),
                    }
                }
            }
            stack_cols.reverse();
            break;
        } else {
            // if we haven't reached the blank, keep adding to the rows
            stack_rows.push(parse_row(line));
        }
    }
    stack_cols
}

fn parse_instruction(inst: &str) -> Instruction {
    let inst_n: Vec<&str> = inst.trim().split(' ').collect();
    Instruction {
        qty: inst_n[1].parse().unwrap(),
        from: inst_n[3].parse().unwrap(),
        to: inst_n[5].parse().unwrap(),
    }
}

fn parse_row(row_str: &str) -> Vec<char> {
    /*
    Need to figure out how to parse stacks
    Easiest way is to look through them from the bottom up
    Input reading function should save a vector of stack strings
    Once input function finds row of stack labels, it knows how
    many stacks we have.
    */
    let mut ind = -1;
    let mut row_chars: Vec<char> = row_str.chars().collect();
    row_chars.retain(|_| {
        ind += 1;
        ind % 4 == 1
    });
    row_chars
}

fn execute_instruction(
    mut stack_cols: Vec<Vec<char>>,
    inst: Instruction,
    part_2: bool,
) -> Vec<Vec<char>> {
    // let mut stack_cols = stack_cols.clone();
    let mut source_stack = stack_cols[inst.from - 1].clone();
    let mut target_stack = stack_cols[inst.to - 1].clone();
    let mut moved_stack = source_stack.split_off(source_stack.len() - inst.qty);

    if !part_2 {
        moved_stack.reverse()
    }
    // println!("{:?}", source_stack);
    target_stack.append(&mut moved_stack);

    stack_cols[inst.from - 1] = source_stack;
    stack_cols[inst.to - 1] = target_stack;
    stack_cols
}

fn assemble_message(mut crates: Vec<Vec<char>>) -> String {
    let mut message = String::new();
    for stack in crates.iter_mut() {
        // Unlikely that we will have any empty stacks
        // If we do, assume it's a space in the message
        message.push(stack.pop().unwrap_or(' '));
    }
    message
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_crates() -> Vec<Vec<char>> {
        vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]
    }

    fn sample_instruction() -> Instruction {
        Instruction {
            qty: 1,
            from: 2,
            to: 1,
        }
    }

    #[test]
    fn test_parse_row() {
        let row_str = "[L] [O] [L]\n";
        assert_eq!(parse_row(row_str), vec!['L', 'O', 'L']);
        let row_str = "    [L] [O] [L]     [H] [I]\n";
        assert_eq!(parse_row(row_str), vec![' ', 'L', 'O', 'L', ' ', 'H', 'I']);
    }

    #[test]
    fn test_parse_inst() {
        assert_eq!(
            parse_instruction("move 1 from 2 to 1\n"),
            sample_instruction(),
        )
    }

    #[test]
    fn test_execute() {
        let stacks = sample_crates();
        assert_eq!(
            execute_instruction(stacks.clone(), sample_instruction(), false),
            vec![vec!['Z', 'N', 'D'], vec!['M', 'C'], vec!['P']]
        );
        assert_eq!(
            execute_instruction(stacks.clone(), sample_instruction(), true),
            vec![vec!['Z', 'N', 'D'], vec!['M', 'C'], vec!['P']]
        );
        let inst = Instruction {
            qty: 3,
            from: 1,
            to: 3,
        };
        let stacks = vec![vec!['Z', 'N', 'D'], vec!['M', 'C'], vec!['P']];
        assert_eq!(
            execute_instruction(stacks.clone(), inst, false),
            vec![vec![], vec!['M', 'C'], vec!['P', 'D', 'N', 'Z']]
        );
    }

    #[test]
    fn test_assemble() {
        let stacks = sample_crates();
        assert_eq!(assemble_message(stacks), "NDP")
    }

    #[test]
    fn test_42() {
        let answer = 42;
        assert_eq!(answer, 42)
    }
}

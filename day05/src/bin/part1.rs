#[derive(Debug)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let (stacks_and_platforms_str, instructions_str) = input.split_once("\n\n").unwrap();
    let (stacks_str, platforms) = stacks_and_platforms_str.rsplit_once('\n').unwrap();

    // let (stack_no, line_no) = get_stack_no_and_line();
    let stack_no: u64 = platforms
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); stack_no as usize];
    for line in stacks_str.lines().rev() {
        let chars: Vec<char> = line.chars().collect();
        for i in (0..chars.len() - 1).step_by(4) {
            let maybe_letter = chars[i as usize + 1];
            if maybe_letter.is_whitespace() {
                continue;
            }
            stacks[i as usize / 4].push(maybe_letter);
        }
    }

    println!(" INIT {:?}", stacks);
    // parse instructions
    let mut instructions = Vec::new();
    for line in instructions_str.lines() {
        let rest = line.strip_prefix("move ").unwrap();
        let (amount, rest) = rest.split_once(" from ").unwrap();
        let (from, to) = rest.split_once(" to ").unwrap();
        let instruction = Instruction {
            amount: amount.parse().ok().unwrap(),
            from: from.parse::<usize>().ok().unwrap() - 1,
            to: to.parse::<usize>().ok().unwrap() - 1,
        };
        instructions.push(instruction);
    }

    println!("{:?}", instructions);

    for Instruction { amount, from, to } in instructions {
        for _ in 0..amount {
            let maybe_value = stacks[from].pop();
            if maybe_value.is_some() {
               stacks[to].push(maybe_value.unwrap());
            }
        }

    }

    println!(" AFTER {:?}", stacks);
    println!("SOLUTION");
    for stack in stacks {
        print!("{}", stack[stack.len() - 1])
    }
    println!("");
}

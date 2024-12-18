pub fn parse_input(input: &str) -> (u64, u64, u64, Vec<u64>) {
    let re = regex::Regex::new(r"Register A: ([0-9]+)\nRegister B: ([0-9]+)\nRegister C: ([0-9]+)\n\nProgram: ([0-7](?:,[0-7])*)").unwrap();
    let cap = re.captures(input).unwrap();
    (
        cap[1].parse::<u64>().unwrap(),
        cap[2].parse::<u64>().unwrap(),
        cap[3].parse::<u64>().unwrap(),
        cap[4]
            .split(",")
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>(),
    )
}

pub fn solve_part_1(input: &(u64, u64, u64, Vec<u64>)) -> String {
    solve(input, 0)
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn solve(input: &(u64, u64, u64, Vec<u64>), a_val: u64) -> Vec<u64> {
    let mut a = input.0;
    if a_val != 0 {
        a = a_val;
    }
    let mut b = input.1;
    let mut c = input.2;
    let instructions = &input.3;
    let mut ip: usize = 0;
    let mut output = Vec::new();
    while ip < instructions.len() {
        let instruction = instructions[ip];
        match instruction {
            // adv
            0 => {
                let operand = get_operand(instructions[ip + 1], a, b, c);
                a /= 1 << operand;
                ip += 2;
            }
            // bxl
            1 => {
                let operand = instructions[ip + 1];
                b ^= operand;
                ip += 2;
            }
            // bst
            2 => {
                let operand = get_operand(instructions[ip + 1], a, b, c);
                b = operand & 0b111;
                ip += 2;
            }
            // jnz {
            3 => {
                let operand = instructions[ip + 1];
                if a != 0 {
                    ip = operand as usize;
                } else {
                    ip += 2;
                }
            }
            // bxc
            4 => {
                b ^= c;
                ip += 2;
            }
            // out
            5 => {
                let operand = get_operand(instructions[ip + 1], a, b, c);
                output.push(operand & 0b111);
                ip += 2;
            }
            // bdv
            6 => {
                let operand = get_operand(instructions[ip + 1], a, b, c);
                b = a / (1 << operand);
                ip += 2;
            }
            // cdv
            7 => {
                let operand = get_operand(instructions[ip + 1], a, b, c);
                c = a / (1 << operand);
                ip += 2;
            }
            _ => panic!("Invalid instruction"),
        }
    }
    output
}

fn get_operand(instruction: u64, a: u64, b: u64, c: u64) -> u64 {
    match instruction {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => a,
        5 => b,
        6 => c,
        _ => panic!("Invalid instruction"),
    }
}

pub fn solve_part_2(input: &(u64, u64, u64, Vec<u64>)) -> u64 {
    let instructions = &input.3;
    let mut stack = Vec::new();
    for i in 0..128 {
        let output = solve(input, i);
        if vector_suffix_match(instructions, &output) {
            stack.push(i);
        }
    }

    while stack.len() > 0 {
        let n = stack.pop().unwrap();
        for i in 0..8 {
            let num = (n << 3) + i as u64;
            let output = solve(input, num);
            if vector_matches(instructions, &output) {
                return num;
            }
            if vector_suffix_match(instructions, &output) {
                stack.push(num);
            }
        }
    }
    0
}

fn vector_matches(v: &Vec<u64>, suffix: &Vec<u64>) -> bool {
    if v.len() != suffix.len() {
        return false;
    }
    for i in 0..suffix.len() {
        if v[i] != suffix[i] {
            return false;
        }
    }
    true
}

fn vector_suffix_match(v: &Vec<u64>, suffix: &Vec<u64>) -> bool {
    if v.len() < suffix.len() {
        return false;
    }
    for i in 0..suffix.len() {
        if v[v.len() - suffix.len() + i] != suffix[i] {
            return false;
        }
    }
    true
}

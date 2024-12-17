pub fn parse_input(input: &str) -> (i32, i32, i32, Vec<i32>) {
    let re = regex::Regex::new(r"Register A: ([0-9]+)\nRegister B: ([0-9]+)\nRegister C: ([0-9]+)\n\nProgram: ([0-7](?:,[0-7])*)").unwrap();
    let cap = re.captures(input).unwrap();
    (
        cap[1].parse::<i32>().unwrap(),
        cap[2].parse::<i32>().unwrap(),
        cap[3].parse::<i32>().unwrap(),
        cap[4]
            .split(",")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>(),
    )
}

pub fn solve_part_1(input: &(i32, i32, i32, Vec<i32>)) -> String {
    let mut a = input.0;
    let mut b = input.1;
    let mut c = input.2;
    let instructions = &input.3;
    let mut ip: usize = 0;
    let mut output = Vec::new();
    while ip < instructions.len() {
        let instruction = instructions[ip as usize];
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
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn get_operand(instruction: i32, a: i32, b: i32, c: i32) -> i32 {
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

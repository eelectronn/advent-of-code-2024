pub fn parse_input(input: &str) -> Vec<u128> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn solve_part_1(input: &Vec<u128>) -> u128 {
    let mut answer = 0;
    for i in input {
        let mut secret = *i;
        for _ in 0..2000 {
            secret = get_next(secret);
        }
        answer += secret
    }
    answer
}

fn get_next(curr: u128) -> u128 {
    let curr = ((curr * 64) ^ curr) % 16777216;
    let curr = ((curr / 32) ^ curr) % 16777216;
    ((curr * 2048) ^ curr) % 16777216
}

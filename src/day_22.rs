use std::collections::HashMap;

pub fn parse_input(input: &str) -> Vec<i64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

pub fn solve_part_2(input: &Vec<i64>) -> i64 {
    let mut seq_to_total_profit = HashMap::new();
    for i in input {
        let mut secret = *i;
        let mut prev_units_digit: i64 = secret % 10;
        let (mut s1, mut s2, mut s3, mut s4) = (0, 0, 0, 0);
        let mut seq_to_profit = HashMap::new();
        for i in 0..2000 {
            secret = get_next(secret);
            let units_digit: i64 = secret % 10;
            s1 = s2;
            s2 = s3;
            s3 = s4;
            s4 = units_digit - prev_units_digit;
            if i >= 3 {
                seq_to_profit.entry((s1, s2, s3, s4)).or_insert(units_digit);
            }
            prev_units_digit = units_digit;
        }
        for (seq, profit) in seq_to_profit {
            let curr = seq_to_total_profit.entry(seq).or_insert(0);
            *curr += profit;
        }
    }
    let mut max_profit = 0;
    for (_, profit) in seq_to_total_profit {
        if profit > max_profit {
            max_profit = profit;
        }
    }

    max_profit
}

pub fn solve_part_1(input: &Vec<i64>) -> i64 {
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

fn get_next(curr: i64) -> i64 {
    let curr = ((curr * 64) ^ curr) % 16777216;
    let curr = ((curr / 32) ^ curr) % 16777216;
    ((curr * 2048) ^ curr) % 16777216
}

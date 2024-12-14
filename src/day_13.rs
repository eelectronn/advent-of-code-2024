use regex::Regex;

#[derive(Debug, Clone)]
pub struct Line {
    a: i64,
    b: i64,
    c: i64,
}

pub fn parse_input(input: &str) -> Vec<(Line, Line)> {
    let re_button = Regex::new(r"^Button .: X\+([0-9]+), Y\+([0-9]+)$").unwrap();
    let re_prize = Regex::new(r"^Prize: X=([0-9]+), Y=([0-9]+)$").unwrap();
    input
        .split("\n\n")
        .map(|x| {
            let q = x.split("\n").collect::<Vec<&str>>();
            let cap = re_button.captures(q[0]).unwrap();
            let a1 = cap[1].parse::<i64>().unwrap();
            let a2 = cap[2].parse::<i64>().unwrap();
            let cap = re_button.captures(q[1]).unwrap();
            let b1 = cap[1].parse::<i64>().unwrap();
            let b2 = cap[2].parse::<i64>().unwrap();
            let cap = re_prize.captures(q[2]).unwrap();
            let c1 = -cap[1].parse::<i64>().unwrap();
            let c2 = -cap[2].parse::<i64>().unwrap();
            (
                Line {
                    a: a1,
                    b: b1,
                    c: c1,
                },
                Line {
                    a: a2,
                    b: b2,
                    c: c2,
                },
            )
        })
        .collect()
}

pub fn solve_part_1(input: &Vec<(Line, Line)>) -> u64 {
    let mut cost: u64 = 0;
    for (line1, line2) in input {
        let x = (line1.b * line2.c - line2.b * line1.c) as f64
            / (line1.a * line2.b - line2.a * line1.b) as f64;
        let y = (line1.a * line2.c - line2.a * line1.c) as f64
            / (line1.b * line2.a - line2.b * line1.a) as f64;
        if x.fract() == 0.0 && y.fract() == 0.0 {
            cost += 3 * (x as u64) + (y as u64);
        }
    }
    cost
}

pub fn solve_part_2(input: &Vec<(Line, Line)>) -> u64 {
    let mut input_copy = input.clone();
    for (line1, line2) in input_copy.iter_mut() {
        // Subtract 10000000000000 from c on both lines
        line1.c -= 10000000000000;
        line2.c -= 10000000000000;
    }
    solve_part_1(&input_copy)
}

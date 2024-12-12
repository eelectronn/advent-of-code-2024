use regex::Regex;

pub fn parse_input(input: &str) -> &str {
    return input;
}

pub fn solve_part_1(input: &str) -> i64 {
    let rg = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();

    let mut answer = 0;
    for cap in rg.captures_iter(input) {
        if let Some(matched) = cap.get(0) {
            let matched = matched.as_str();
            let nums = matched[4..matched.len() - 1]
                .split(",")
                .map(|x| x.parse::<i64>().expect("Item should parse to i64"))
                .collect::<Vec<i64>>();
            answer += nums[0] * nums[1];
        }
    }
    answer
}

pub fn solve_part_2(input: &str) -> i64 {
    let full_input = format!("do(){}", input);

    let rg = Regex::new(r"do\(\)|don't\(\)").unwrap();

    let mut matches = Vec::new();
    for cap in rg.captures_iter(full_input.as_str()) {
        if let Some(matched) = cap.get(0) {
            matches.push(matched);
        }
    }
    let mut answer = 0;
    for i in 0..matches.len() {
        let matched = matches[i];
        let start = matched.start();
        let end = if i + 1 < matches.len() {
            matches[i + 1].start()
        } else {
            input.len()
        };
        let matched = matched.as_str();
        if matched == "do()" {
            answer += solve_part_1(&input[start..end])
        }
    }
    answer
}

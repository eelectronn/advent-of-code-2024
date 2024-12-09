use std::vec;

pub fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(
            |line| {
                line
                    .split_whitespace()
                    .map(|item| item.parse().expect("Item should parse to i32"))
                    .collect()
            }
        )
        .collect()
}

pub fn solve_part_1(input: &Vec<Vec<i32>>) -> i32 {
    let mut answer = 0;
    for report in input {
        if is_safe(report) {
        answer += 1;
        }
    }
    answer
}

fn is_safe(report: &Vec<i32>) -> bool {
    if report.len() <= 1 {
        return false;
    }
    let first_diff = report[1] - report[0];
    
    for i in 1..report.len() {
        let diff = report[i] - report[i-1];

        if diff == 0 || diff.abs() > 3 || first_diff * diff < 0 {
            return false
        }
    }
    true
}

fn get_diff(report: &Vec<i32>) -> Vec<i32> {
    let mut diff = vec![0; report.len() - 1];
    for i in 1..report.len() {
        diff[i-1] = report[i] - report[i-1];
    }
    diff
}

pub fn solve_part_2(input: &Vec<Vec<i32>>) -> i32 {
    let mut answer = 0;
    for report in input {
        let mut diff = vec![0; report.len() -1 ];
        for i in 1..report.len() {
            diff[i-1] = report[i] - report[i-1];
        }
        let mut defects = Vec::new();
        for i in 0..diff.len() {
            if diff[i] == 0 || diff[i].abs() > 3 {
                defects.push(i);
            }
        }

        if defects.len() > 2 {
            continue
        }



    }
    answer
}

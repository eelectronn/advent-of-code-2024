pub fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_whitespace()
                .map(|item| item.parse().expect("Item should parse to i32"))
                .collect()
        })
        .collect()
}

pub fn solve_part_1(input: &Vec<Vec<i32>>) -> i32 {
    let mut answer = 0;
    for report in input {
        let diff = get_diff(report);
        if is_safe(&diff) {
            answer += 1;
        }
    }
    answer
}

fn is_safe(diff: &Vec<i32>) -> bool {
    if diff.len() == 0 {
        return false;
    }
    let first_diff = diff[0];
    for d in diff {
        // Opposite sign in diff means the the sequence is not always increasing/decreasing.
        // Product of two numbers with the opposite sign is always negative.
        if d * first_diff <= 0 || *d == 0 || d.abs() > 3 {
            return false;
        }
    }
    true
}

fn get_diff(report: &Vec<i32>) -> Vec<i32> {
    let mut diff = Vec::new();
    for i in 1..report.len() {
        diff.push(report[i] - report[i - 1]);
    }
    diff
}

pub fn solve_part_2(input: &Vec<Vec<i32>>) -> i32 {
    let mut answer = 0;
    for report in input {
        if is_safe_part_2(report) {
            answer += 1;
        }
    }
    answer
}

fn is_safe_part_2(report: &Vec<i32>) -> bool {
    if report.len() <= 1 {
        return true;
    }
    let diff = get_diff(report);

    let diff_copy = diff.clone();
    if is_safe_with_one_skip(&diff_copy) {
        return true;
    }

    let diff_reverse: Vec<i32> = diff.iter().rev().map(|d| *d).collect();
    if is_safe_with_one_skip(&diff_reverse) {
        return true;
    }

    false
}

fn is_safe_with_one_skip(diff_ref: &Vec<i32>) -> bool {
    let diff: Vec<i32> = if diff_ref[0] < 0 {
        diff_ref.iter().map(|d| -*d).collect()
    } else {
        diff_ref.iter().map(|d| *d).collect()
    };

    let mut i = 0;
    let mut skipped_once = false;
    while i < diff.len() {
        if 1 <= diff[i] && diff[i] <= 3 {
            i += 1;
        } else {
            if skipped_once {
                return false;
            } else {
                skipped_once = true;
                if i + 1 < diff.len() {
                    let combined_diff = diff[i] + diff[i + 1];
                    if 1 <= combined_diff && combined_diff <= 3 {
                        i += 2;
                    } else {
                        return false;
                    }
                } else {
                    i += 1;
                }
            }
        }
    }
    true
}

use std::iter;

pub fn parse_input(input: &str) -> Vec<((i64, i64), (i64, i64))> {
    let re = regex::Regex::new(r"p=(-?[0-9]+),(-?[0-9]+) v=(-?[0-9]+),(-?[0-9]+)").unwrap();
    input
        .lines()
        .map(|line| {
            let cap = re.captures(line).unwrap();
            (
                (cap[1].parse().unwrap(), cap[2].parse().unwrap()),
                (cap[3].parse().unwrap(), cap[4].parse().unwrap()),
            )
        })
        .collect()
}

pub fn solve(input: &Vec<((i64, i64), (i64, i64))>, iterations: i64) -> (u64, Vec<Vec<i32>>) {
    let y_len = 103;
    let x_len = 101;
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;
    let mut robotPositions = vec![vec![0; x_len]; y_len];
    for ((x, y), (vx, vy)) in input.iter() {
        // println!("{} {} {} {}", y, x, vy, vx);
        let f_y = (y + vy * iterations).rem_euclid(y_len as i64);
        let f_x = (x + vx * iterations).rem_euclid(x_len as i64);
        // println!("{} {}", f_y, f_x);
        robotPositions[f_y as usize][f_x as usize] += 1;
        if f_y < y_len as i64 / 2 {
            if f_x < x_len as i64 / 2 {
                q1 += 1;
            } else if f_x > x_len as i64 / 2 {
                q2 += 1;
            }
        } else if f_y > y_len as i64 / 2 {
            if f_x < x_len as i64 / 2 {
                q3 += 1;
            } else if f_x > x_len as i64 / 2 {
                q4 += 1;
            }
        }
    }
    (q1 * q2 * q3 * q4, robotPositions)
}

pub fn solve_part_1(input: &Vec<((i64, i64), (i64, i64))>) -> u64 {
    solve(input, 100).0
}

pub fn solve_part_2(input: &Vec<((i64, i64), (i64, i64))>) -> u64 {
    let (_, curr) = solve(input, 0);
    for i in 5000..30000 {
        let now = solve(input, i).1;
        if exist_filled_block(&now, 5) {
            println!("At iteration {}", i);
            print_output(&now);
        } else {
            // println!("No blank block found at iteration {}", i);
        }
    }
    0
}

pub fn exist_filled_block(v: &Vec<Vec<i32>>, block_len: usize) -> bool {
    for i in 0..v.len() - block_len {
        for j in 0..v[i].len() - block_len {
            let mut is_filled = true;
            for k in 0..block_len {
                for l in 0..block_len {
                    if v[i + k][j + l] == 0 {
                        is_filled = false;
                        break;
                    }
                }
            }
            if is_filled {
                return true;
            }
        }
    }
    false
}

fn exist_blank_block(v: &Vec<Vec<i32>>, block_len: usize) -> bool {
    for i in 0..v.len() - block_len {
        for j in 0..v[i].len() - block_len {
            let mut is_blank = true;
            for k in 0..block_len {
                for l in 0..block_len {
                    if v[i + k][j + l] != 0 {
                        is_blank = false;
                        break;
                    }
                }
            }
            if is_blank {
                return true;
            }
        }
    }
    false
}

fn compare_output(v1: &Vec<Vec<i32>>, v2: &Vec<Vec<i32>>) -> bool {
    for i in 0..v1.len() {
        for j in 0..v1[i].len() {
            if v1[i][j] != v2[i][j] {
                return false;
            }
        }
    }
    true
}

fn print_output(v: &Vec<Vec<i32>>) {
    for row in v.iter() {
        for cell in row.iter() {
            print!(
                "{}",
                if *cell > 0 {
                    "#".to_string()
                } else {
                    " ".to_string()
                }
            );
        }
        println!();
    }
}

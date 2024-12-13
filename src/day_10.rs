use std::collections::HashSet;

pub fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|item| item.to_string().parse().expect("Item should parse to i32"))
                .collect()
        })
        .collect()
}

pub fn solve_part_1(input: &Vec<Vec<usize>>) -> u64 {
    let mut map: Vec<Vec<(usize, usize)>> = vec![Vec::new(); 10];
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            map[input[y][x]].push((y, x));
        }
    }
    let y_size: i32 = input.len() as i32;
    let x_size: i32 = input[0].len() as i32;
    let mut dp = vec![vec![HashSet::new(); x_size as usize]; y_size as usize];
    for (y, x) in map[9].iter() {
        dp[*y][*x].insert((*y, *x));
    }
    for i in 1..10 {
        let p = 10 - i - 1;
        for (y, x) in map[p].iter() {
            let y_cast = *y as i32;
            let x_cast = *x as i32;
            for (n_y, n_x) in [
                (y_cast + 1, x_cast),
                (y_cast, x_cast + 1),
                (y_cast - 1, x_cast),
                (y_cast, x_cast - 1),
            ]
            .iter()
            {
                if *n_y >= 0
                    && *n_y < y_size
                    && *n_x >= 0
                    && *n_x < x_size
                    && input[*n_y as usize][*n_x as usize] == p + 1
                {
                    dp[*y][*x] = dp[*y][*x]
                        .union(&dp[*n_y as usize][*n_x as usize])
                        .cloned()
                        .collect();
                }
            }
        }
    }
    let mut count: u64 = 0;
    for (y, x) in map[0].iter() {
        count += dp[*y][*x].len() as u64
    }
    count
}

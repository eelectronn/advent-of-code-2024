use std::collections::HashSet;

#[derive(Clone, Copy)]
enum Dir {
    N,
    E,
    S,
    W,
}

pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

pub fn solve_part_1(input: &Vec<Vec<char>>) -> u64 {
    let dp = solve(input);
    let (s_y, s_x) = find_char(&input, 'S');
    dp[Dir::E as usize][s_y][s_x].0
}

pub fn solve_part_2(input: &Vec<Vec<char>>) -> u64 {
    let dp = solve(input);
    let (y, x) = find_char(&input, 'S');

    let mut stack = vec![(Dir::E, y, x)];
    let mut visited = HashSet::new();
    while stack.len() > 0 {
        let (heading, y, x) = stack.pop().unwrap();
        visited.insert((y, x));
        for (ny, nx) in dp[heading as usize][y][x].1.iter() {
            stack.push((get_direction((y, x), (*ny, *nx)), *ny, *nx));
        }
    }
    visited.len() as u64
}

fn solve(input: &Vec<Vec<char>>) -> Vec<Vec<Vec<(u64, HashSet<(usize, usize)>)>>> {
    let mut dp = vec![vec![vec![(std::u64::MAX, HashSet::new()); input[0].len()]; input.len()]; 4];

    let (e_y, e_x) = find_char(&input, 'E');
    dp[Dir::E as usize][e_y][e_x] = (0, HashSet::new());
    dp[Dir::N as usize][e_y][e_x] = (0, HashSet::new());
    dp[Dir::S as usize][e_y][e_x] = (0, HashSet::new());
    dp[Dir::W as usize][e_y][e_x] = (0, HashSet::new());

    let mut stack = vec![
        (Dir::E, e_y, e_x),
        (Dir::N, e_y, e_x),
        (Dir::S, e_y, e_x),
        (Dir::W, e_y, e_x),
    ];
    while stack.len() > 0 {
        let (landing_dir, y, x) = stack.pop().unwrap();
        let neighbours = get_neighbours(y, x, input);
        for neighbour_dir in &[Dir::E, Dir::N, Dir::S, Dir::W] {
            let (ny, nx) = neighbours[*neighbour_dir as usize];
            if (ny, nx) == (0, 0) {
                continue;
            }
            for curr_dir in &[Dir::E, Dir::N, Dir::S, Dir::W] {
                let cost = turning_cost(*curr_dir, opposite_dir(*neighbour_dir))
                    + 1
                    + turning_cost(opposite_dir(*neighbour_dir), landing_dir)
                    + dp[landing_dir as usize][y][x].0;
                let curr_cost = dp[*curr_dir as usize][ny][nx].0;
                if cost < curr_cost {
                    dp[*curr_dir as usize][ny][nx].0 = cost;
                    stack.push((*curr_dir, ny, nx));
                }
                if cost < curr_cost {
                    dp[*curr_dir as usize][ny][nx].1 = HashSet::new();
                    dp[*curr_dir as usize][ny][nx].1.insert((y, x));
                } else if cost == curr_cost {
                    dp[*curr_dir as usize][ny][nx].1.insert((y, x));
                }
            }
        }
    }
    dp
}

fn find_char(input: &Vec<Vec<char>>, c: char) -> (usize, usize) {
    for (i, row) in input.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col == c {
                return (i, j);
            }
        }
    }
    (0, 0)
}

fn get_neighbours(y: usize, x: usize, input: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut neighbours = vec![(0, 0); 4];
    if y > 0 && input[y - 1][x] != '#' {
        neighbours[Dir::N as usize] = (y - 1, x);
    }
    if y < input.len() - 1 && input[y + 1][x] != '#' {
        neighbours[Dir::S as usize] = (y + 1, x);
    }
    if x > 0 && input[y][x - 1] != '#' {
        neighbours[Dir::W as usize] = (y, x - 1);
    }
    if x < input[0].len() - 1 && input[y][x + 1] != '#' {
        neighbours[Dir::E as usize] = (y, x + 1);
    }
    neighbours
}

fn turning_cost(curr_dir: Dir, target_dir: Dir) -> u64 {
    match curr_dir {
        Dir::N => match target_dir {
            Dir::N => 0,
            Dir::E => 1000,
            Dir::S => 2000,
            Dir::W => 1000,
        },
        Dir::E => match target_dir {
            Dir::N => 1000,
            Dir::E => 0,
            Dir::S => 1000,
            Dir::W => 2000,
        },
        Dir::S => match target_dir {
            Dir::N => 2000,
            Dir::E => 1000,
            Dir::S => 0,
            Dir::W => 1000,
        },
        Dir::W => match target_dir {
            Dir::N => 1000,
            Dir::E => 2000,
            Dir::S => 1000,
            Dir::W => 0,
        },
    }
}

fn opposite_dir(dir: Dir) -> Dir {
    match dir {
        Dir::N => Dir::S,
        Dir::E => Dir::W,
        Dir::S => Dir::N,
        Dir::W => Dir::E,
    }
}

fn get_direction(source: (usize, usize), target: (usize, usize)) -> Dir {
    if source.0 > target.0 {
        Dir::N
    } else if source.0 < target.0 {
        Dir::S
    } else if source.1 > target.1 {
        Dir::W
    } else {
        Dir::E
    }
}

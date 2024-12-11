use std::collections::{hash_set, HashSet};

pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

pub fn solve_part_1(input: &Vec<Vec<char>>) -> usize {
    let mut grid = input.clone();
    let (mut y, mut x) = find_guard(&grid);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    visited.insert((y, x));
    loop {
        // Hit the wall
        if (grid[y][x] == '^' && y == 0) || (grid[y][x] == '>' && x == grid[y].len() - 1) || (grid[y][x] == 'v' && y == grid.len() - 1) || (grid[y][x] == '<' && x == 0) {
            break;
        }
        let (new_y, new_x) = match grid[y][x] {
            '^' => (y - 1, x),
            '>' => (y, x + 1),
            'v' => (y + 1, x),
            '<' => (y, x - 1),
            _ => (0, 0),
        };
        if grid[new_y][new_x] == '#' {
            grid[y][x] = match grid[y][x] {
                '^' => '>',
                '>' => 'v',
                'v' => '<',
                '<' => '^',
                _ => ' ',
            };
            continue;
        }
        grid[new_y][new_x] = grid[y][x];
        grid[y][x] = '.';
        y = new_y;
        x = new_x;
        visited.insert((y, x));
        // for row in &grid {
            // println!("{}", row.iter().collect::<String>());
        // }
        // println!();
        // Wait for user input
        // let mut input = String::new();
        // std::io::stdin().read_line(&mut input).unwrap();
    }
    visited.len()
}

fn find_guard(grid: &Vec<Vec<char>>) -> (usize, usize) {
    let guards: HashSet<char> = vec!['^', '>', 'v', '<'].into_iter().collect();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if guards.contains(&grid[y][x]) {
                return (y, x);
            }
        }
    }
    (0, 0)
}
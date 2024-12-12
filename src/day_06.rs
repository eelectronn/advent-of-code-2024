use std::{
    arch::x86_64::_CMP_NEQ_US,
    collections::{hash_set, HashMap, HashSet},
    hash::Hash,
};

pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

pub fn solve_part_1(input: &Vec<Vec<char>>) -> isize {
    let (answer, _) = find_path(input);
    answer
}

fn find_path(input: &Vec<Vec<char>>) -> (isize, HashMap<(usize, usize), char>) {
    let mut grid = input.clone();
    let (mut y, mut x) = find_guard(&grid);
    let mut visited: HashMap<(usize, usize), char> = HashMap::new();
    visited.insert((y, x), grid[y][x]);
    loop {
        // Hit the wall
        if (grid[y][x] == '^' && y == 0)
            || (grid[y][x] == '>' && x == grid[y].len() - 1)
            || (grid[y][x] == 'v' && y == grid.len() - 1)
            || (grid[y][x] == '<' && x == 0)
        {
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
            // print_and_wait(&grid);
            continue;
        }
        grid[new_y][new_x] = grid[y][x];
        grid[y][x] = '.';
        y = new_y;
        x = new_x;
        if let Some(&dir) = visited.get(&(y, x)) {
            if dir == grid[y][x] {
                return (-1, visited);
            }
        }
        visited.insert((y, x), grid[y][x]);
        // print_and_wait(&grid);
    }
    (visited.len() as isize, visited)
}

pub fn solve_part_2(input: &Vec<Vec<char>>) -> usize {
    let (guard_y, gurard_x) = find_guard(&input);
    let (_, visited) = find_path(input);
    let mut grid = input.clone();
    let mut count = 0;
    for (y, x) in visited.keys() {
        if (y, x) != (&guard_y, &gurard_x) {
            grid[*y][*x] = '#';
            let (l, _) = find_path(&grid);
            if l == -1 {
                count += 1;
            }
            grid[*y][*x] = '.';
        }
    }
    count
}

fn print_and_wait(grid: &Vec<Vec<char>>) {
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
    println!();
    // Wait for user input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
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

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
    let mut visited = vec![vec![' '; input[0].len()]; input.len()];
    find_path(&mut input.clone(), &mut visited);
    let mut count = 0;
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if visited[y][x] != ' ' {
                count += 1;
            }
        }
    }
    count
}

fn find_path(grid: &mut Vec<Vec<char>>, visited: &mut Vec<Vec<char>>) -> bool {
    let (mut y, mut x) = find_guard(&grid);
    visited[y][x] = grid[y][x];
    visited[y][x] = grid[y][x];
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
        if visited[y][x] == grid[y][x] {
            return false;
        }
        visited[y][x] = grid[y][x];
        // print_and_wait(&grid);
    }
    true
}

pub fn solve_part_2(input: &Vec<Vec<char>>) -> usize {
    let (guard_y, gurard_x) = find_guard(&input);
    let mut grid = input.clone();
    let mut visited = vec![vec![' '; input[0].len()]; input.len()];
    find_path(&mut grid, &mut visited);

    let visited_vec: Vec<(usize, usize)> = visited
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &cell)| cell != ' ')
                .map(move |(x, _)| (y, x))
        })
        .collect();

    empty_grid(&mut visited);

    reset_grid(&mut grid, input);
    let mut count = 0;
    for (y, x) in visited_vec {
        if (y, x) != (guard_y, gurard_x) {
            grid[y][x] = '#';
            let found = find_path(&mut grid, &mut visited);
            if !found {
                count += 1;
            }
            reset_grid(&mut grid, &input);
            empty_grid(&mut visited);
        }
    }
    count
}

fn reset_grid(grid: &mut Vec<Vec<char>>, input: &Vec<Vec<char>>) {
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            grid[y][x] = input[y][x];
        }
    }
}

fn empty_grid(grid: &mut Vec<Vec<char>>) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            grid[y][x] = ' ';
        }
    }
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

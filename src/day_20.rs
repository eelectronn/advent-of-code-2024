use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    vec,
};

pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

pub fn solve_part_1(input: &Vec<Vec<char>>) -> u64 {
    let (start_y, start_x) = find_char(&input, 'S').unwrap();
    let shortest = find_steps(&input, (start_y, start_x));
    let (end_y, end_x) = find_char(&input, 'E').unwrap();
    let shortest_initial = shortest[end_y][end_x];
    let mut tally = HashMap::new();
    for y in 0..shortest.len() {
        for x in 0..shortest[y].len() {
            if shortest[y][x] == -1 {
                continue;
            }
            let neighbours = find_multi_step_neighbours(&input, (y, x), 2);
            for (ny, nx) in neighbours {
                let new_shortest = shortest_initial - shortest[ny][nx] + shortest[y][x] + 2;
                if shortest_initial > new_shortest {
                    *tally.entry(shortest_initial - new_shortest).or_insert(0) += 1;
                }
            }
        }
    }

    let mut count = 0;
    for (k, v) in tally {
        if k >= 100 {
            count += v;
        }
    }

    count
}

fn find_steps(grid: &Vec<Vec<char>>, start: (usize, usize)) -> Vec<Vec<i32>> {
    let mut heap = BinaryHeap::new();
    heap.push((0 as i32, start));
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut shortest = vec![vec![-1; grid[0].len()]; grid.len()];
    while let Some((steps, (y, x))) = heap.pop() {
        if visited[y][x] {
            continue;
        }
        shortest[y][x] = -1 * steps;
        visited[y][x] = true;
        if x > 0 && grid[y][x - 1] != '#' {
            heap.push((steps - 1, (y, x - 1)));
        }
        if x < grid[0].len() - 1 && grid[y][x + 1] != '#' {
            heap.push((steps - 1, (y, x + 1)));
        }
        if y > 0 && grid[y - 1][x] != '#' {
            heap.push((steps - 1, (y - 1, x)));
        }
        if y < grid.len() - 1 && grid[y + 1][x] != '#' {
            heap.push((steps - 1, (y + 1, x)));
        }
    }
    shortest
}

fn find_char(input: &Vec<Vec<char>>, c: char) -> Option<(usize, usize)> {
    for (i, row) in input.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col == c {
                return Some((i, j));
            }
        }
    }
    None
}

fn find_multi_step_neighbours(
    grid: &Vec<Vec<char>>,
    curr: (usize, usize),
    steps: usize,
) -> Vec<(usize, usize)> {
    let mut neighbours = HashSet::new();
    if steps == 0 {
        return vec![(curr.0, curr.1)];
    }
    let (y, x) = curr;
    if x > 0 {
        for n in find_multi_step_neighbours(grid, (y, x - 1), steps - 1) {
            if grid[n.0][n.1] != '#' {
                neighbours.insert(n);
            }
        }
    }
    if x < grid[0].len() - 1 {
        for n in find_multi_step_neighbours(grid, (y, x + 1), steps - 1) {
            if grid[n.0][n.1] != '#' {
                neighbours.insert(n);
            }
        }
    }
    if y > 0 {
        for n in find_multi_step_neighbours(grid, (y - 1, x), steps - 1) {
            if grid[n.0][n.1] != '#' {
                neighbours.insert(n);
            }
        }
    }
    if y < grid.len() - 1 {
        for n in find_multi_step_neighbours(grid, (y + 1, x), steps - 1) {
            if grid[n.0][n.1] != '#' {
                neighbours.insert(n);
            }
        }
    }
    neighbours.into_iter().collect()
}

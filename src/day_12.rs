use std::{collections::HashMap, vec};

use crate::data_structures::DisjointSets;

#[derive(Clone)]
struct Side {
    top: bool,
    bottom: bool,
    left: bool,
    right: bool,
}

pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

fn make_disjoint_sets(input: &Vec<Vec<char>>) -> DisjointSets<(usize, usize)> {
    let mut ds = DisjointSets::new();
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            ds.union((y, x), (y, x));
            let neighbours: Vec<(usize, usize)> = [(-1, 0), (0, -1)]
                .iter()
                .map(|(dy, dx)| (y as i32 + dy, x as i32 + dx))
                .filter(|(ny, nx)| {
                    *ny >= 0 && *ny < input.len() as i32 && *nx >= 0 && *nx < input[0].len() as i32
                })
                .map(|(dy, dx)| (dy as usize, dx as usize))
                .collect();
            for (ny, nx) in neighbours {
                if input[ny][nx] == input[y][x] {
                    ds.union((ny, nx), (y, x));
                }
            }
        }
    }
    ds
}

fn create_grid_with_disjoint_sets(input: &Vec<Vec<char>>) -> Vec<Vec<usize>> {
    let mut ds = make_disjoint_sets(input);
    let mut grid = vec![vec![0; input[0].len()]; input.len()];
    let mut map: HashMap<(usize, usize), usize> = HashMap::new();
    let mut id = 1;
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            let p = ds.find_parent(&(y, x));
            if !map.contains_key(&p) {
                map.insert(p, id);
                id += 1;
            }
            grid[y][x] = *map.get(&p).unwrap();
        }
    }
    grid
}

pub fn solve_part_1(input: &Vec<Vec<char>>) -> u64 {
    let grid = create_grid_with_disjoint_sets(input);
    let mut area = HashMap::new();
    let mut perimeter = HashMap::new();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            *area.entry(grid[y][x]).or_insert(0) += 1;

            if y == 0 {
                *perimeter.entry(grid[y][x]).or_insert(0) += 1;
            }
            if y == grid.len() - 1 {
                *perimeter.entry(grid[y][x]).or_insert(0) += 1;
            }
            if x == 0 {
                *perimeter.entry(grid[y][x]).or_insert(0) += 1;
            }
            if x == grid[y].len() - 1 {
                *perimeter.entry(grid[y][x]).or_insert(0) += 1;
            }

            let neighbours: Vec<(usize, usize)> = [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .iter()
                .map(|(dy, dx)| (y as i32 + dy, x as i32 + dx))
                .filter(|(ny, nx)| {
                    *ny >= 0 && *ny < grid.len() as i32 && *nx >= 0 && *nx < grid[0].len() as i32
                })
                .map(|(dy, dx)| (dy as usize, dx as usize))
                .collect();

            for (ny, nx) in neighbours {
                if grid[ny][nx] != grid[y][x] {
                    *perimeter.entry(grid[y][x]).or_insert(0) += 1;
                }
            }
        }
    }
    let mut answer = 0;
    for (k, v) in perimeter.iter() {
        answer += area.get(k).unwrap() * v;
    }
    answer
}

pub fn solve_part_2(input: &Vec<Vec<char>>) -> usize {
    let disjoin_grid = create_grid_with_disjoint_sets(input);
    // Padde the grid with -1 cells
    let mut grid = vec![vec![0; disjoin_grid[0].len() + 2]; disjoin_grid.len() + 2];
    for y in 0..disjoin_grid.len() {
        for x in 0..disjoin_grid[y].len() {
            grid[y + 1][x + 1] = disjoin_grid[y][x];
        }
    }
    let mut area = HashMap::new();
    let mut number_of_sides = HashMap::new();
    let mut sides = vec![
        vec![
            Side {
                top: false,
                bottom: false,
                left: false,
                right: false,
            };
            grid[0].len()
        ];
        grid.len()
    ];
    for y in 1..grid.len() - 1 {
        for x in 1..grid[y].len() - 1 {
            *area.entry(grid[y][x]).or_insert(0) += 1;
            let mut side = Side {
                top: true,
                bottom: true,
                left: true,
                right: true,
            };
            let mut side_for_count = Side {
                top: true,
                bottom: true,
                left: true,
                right: true,
            };
            if grid[y - 1][x] == grid[y][x] {
                side.top = false;
                side_for_count.top = false;
                if sides[y - 1][x].left {
                    side_for_count.left = false;
                }
                if sides[y - 1][x].right {
                    side_for_count.right = false;
                }
            }
            if grid[y + 1][x] == grid[y][x] {
                side.bottom = false;
                side_for_count.bottom = false;
                if sides[y + 1][x].left {
                    side_for_count.left = false;
                }
                if sides[y + 1][x].right {
                    side_for_count.right = false;
                }
            }
            if grid[y][x - 1] == grid[y][x] {
                side.left = false;
                side_for_count.left = false;
                if sides[y][x - 1].top {
                    side_for_count.top = false;
                }
                if sides[y][x - 1].bottom {
                    side_for_count.bottom = false;
                }
            }
            if grid[y][x + 1] == grid[y][x] {
                side.right = false;
                side_for_count.right = false;
                if sides[y][x + 1].top {
                    side_for_count.top = false;
                }
                if sides[y][x + 1].bottom {
                    side_for_count.bottom = false;
                }
            }

            sides[y][x] = side;

            if side_for_count.top {
                *number_of_sides.entry(grid[y][x]).or_insert(0) += 1;
            }
            if side_for_count.bottom {
                *number_of_sides.entry(grid[y][x]).or_insert(0) += 1;
            }
            if side_for_count.left {
                *number_of_sides.entry(grid[y][x]).or_insert(0) += 1;
            }
            if side_for_count.right {
                *number_of_sides.entry(grid[y][x]).or_insert(0) += 1;
            }
        }
    }
    let mut answer = 0;
    for (k, v) in number_of_sides.iter() {
        answer += area.get(k).unwrap() * v;
    }
    answer
}

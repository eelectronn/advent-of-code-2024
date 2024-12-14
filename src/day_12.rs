use std::{collections::HashMap, vec};

use crate::data_structures::DisjointSets;

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

use std::collections::{HashMap, HashSet};

pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

pub fn solve_part_1(input: &Vec<Vec<char>>) -> usize {
    let mut antenna_locations = HashMap::new();
    let mut antinode_locations = HashSet::new();
    let y_size = input.len();
    let x_size = input[0].len();
    for y in 0..y_size {
        for x in 0..x_size {
            if input[y][x] != '.' {
                antenna_locations
                    .entry(input[y][x])
                    .or_insert_with(Vec::new)
                    .push((y, x));
            }
        }
    }
    println!("{:?}", antenna_locations);
    for (_, locations) in antenna_locations.iter() {
        for (y1, x1) in locations.iter() {
            for (y2, x2) in locations.iter() {
                if y1 == y2 && x1 == x2 {
                    continue;
                }
                let (y1, x1, y2, x2) = (*y1 as isize, *x1 as isize, *y2 as isize, *x2 as isize);
                let (diff_y, diff_x) = (y2 - y1, x2 - x1);
                let (node_1_y, node_1_x) = (y1 - diff_y, x1 - diff_x);
                let (node_2_y, node_2_x) = (y2 + diff_y, x2 + diff_x);

                for node in [(node_1_y, node_1_x), (node_2_y, node_2_x)].iter() {
                    if node.0 >= 0
                        && node.1 >= 0
                        && node.0 < y_size as isize
                        && node.1 < x_size as isize
                    {
                        antinode_locations.insert((node.0 as usize, node.1 as usize));
                    }
                }
            }
        }
    }
    antinode_locations.len()
}

pub fn solve_part_2(input: &Vec<Vec<char>>) -> usize {
    let mut antenna_locations = HashMap::new();
    let mut antinode_locations = HashSet::new();
    let y_size = input.len();
    let x_size = input[0].len();
    for y in 0..y_size {
        for x in 0..x_size {
            if input[y][x] != '.' {
                antenna_locations
                    .entry(input[y][x])
                    .or_insert_with(Vec::new)
                    .push((y, x));
            }
        }
    }
    println!("{:?}", antenna_locations);
    for (_, locations) in antenna_locations.iter() {
        for (y1, x1) in locations.iter() {
            for (y2, x2) in locations.iter() {
                if y1 == y2 && x1 == x2 {
                    continue;
                }
                let (y1, x1, y2, x2) = (*y1 as isize, *x1 as isize, *y2 as isize, *x2 as isize);
                let (diff_y, diff_x) = (y2 - y1, x2 - x1);
                let (mut node_y, mut node_x) = (y1 + diff_y, x1 + diff_x);
                while (node_y >= 0
                    && node_x >= 0
                    && node_y < y_size as isize
                    && node_x < x_size as isize)
                {
                    antinode_locations.insert((node_y as usize, node_x as usize));
                    node_y += diff_y;
                    node_x += diff_x;
                }
                let (mut node_y, mut node_x) = (y2 - diff_y, x2 - diff_x);
                while (node_y >= 0
                    && node_x >= 0
                    && node_y < y_size as isize
                    && node_x < x_size as isize)
                {
                    antinode_locations.insert((node_y as usize, node_x as usize));
                    node_y -= diff_y;
                    node_x -= diff_x;
                }
            }
        }
    }
    antinode_locations.len()
}

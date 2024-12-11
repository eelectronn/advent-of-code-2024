use std::collections::{HashMap, HashSet};

pub fn parse_input(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    (
        input
            .lines()
            .filter(|line| line.contains("|"))
            .map(|line| {
                let parts = line
                .split("|")
                .map(|part| part.parse().expect("Item should be parsable to i32"))
                .collect::<Vec<i32>>();
                (parts[0], parts[1])
            })
            .collect::<Vec<(i32, i32)>>(),
        input
            .lines()
            .filter(|line| line.contains(","))
            .map(|line| {
                line
                .split(",")
                .map(|part| part.parse().expect("Item should be parsable to i32"))
                .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>()
    )
}

pub fn solve_part_1(input: &(Vec<(i32, i32)>, Vec<Vec<i32>>)) -> i32 {

    let neighbour_pair_set: HashSet<(i32, i32)> = input.0.iter().cloned().collect();

    let mut answer = 0;
    'paths: for path in &input.1 {
        for i in 0..path.len() - 1 {
            if !neighbour_pair_set.contains(&(path[i], path[i + 1])) {
                continue 'paths;
            }
        }
        answer += path[path.len()/2];
    }
    answer
}

pub fn solve_part_2(input: &(Vec<(i32, i32)>, Vec<Vec<i32>>)) -> i32 {
    let neighbour_pair_set: HashSet<(i32, i32)> = input.0.iter().cloned().collect();
    let mut answer = 0;
    for path in &input.1 {
        let mut path = path.clone();
        path.sort_by(|a, b| {
            if neighbour_pair_set.contains(&(a.clone(), b.clone())) {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });
        answer += path[path.len()/2];
    }
    answer - solve_part_1(input)
}

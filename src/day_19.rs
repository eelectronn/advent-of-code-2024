use std::collections::{HashMap, HashSet};

pub fn parse_input(input: &str) -> (Vec<String>, Vec<String>) {
    let lines: Vec<String> = input
        .lines()
        .filter(|x| !x.is_empty())
        .map(|x| x.to_string())
        .collect();
    (
        lines[0].split(",").map(|x| x.trim().to_string()).collect(),
        lines[1..].to_vec(),
    )
}

pub fn solve_part_1(input: &(Vec<String>, Vec<String>)) -> usize {
    let (parts, queries) = input;
    let parts: HashSet<String> = HashSet::from_iter(parts.iter().cloned());

    let mut count = 0;
    for query in queries {
        if curse(query, &parts, 0, &mut HashMap::new()) > 0 {
            count += 1;
        }
    }
    count
}

pub fn solve_part_2(input: &(Vec<String>, Vec<String>)) -> usize {
    let (parts, queries) = input;
    let parts: HashSet<String> = HashSet::from_iter(parts.iter().cloned());

    let mut count = 0;
    for query in queries {
        count += curse(query, &parts, 0, &mut HashMap::new());
    }
    count
}

fn curse(
    query: &str,
    parts: &HashSet<String>,
    index: usize,
    memo: &mut HashMap<usize, usize>,
) -> usize {
    if memo.contains_key(&index) {
        return memo[&index];
    }
    if index == query.len() {
        return 1;
    }
    let mut count = 0;
    for i in index..=query.len() {
        if parts.contains(&query[index..i]) {
            count += curse(query, parts, i, memo);
        }
    }
    memo.insert(index, count);
    count
}

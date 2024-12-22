use std::{collections::HashMap, usize::MAX, vec};

pub fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn solve_part_1(input: &Vec<&str>) -> usize {
    let numpad_map: HashMap<char, (usize, usize)> = HashMap::from_iter([
        ('7', (0, 0)),
        ('8', (0, 1)),
        ('9', (0, 2)),
        ('4', (1, 0)),
        ('5', (1, 1)),
        ('6', (1, 2)),
        ('1', (2, 0)),
        ('2', (2, 1)),
        ('3', (2, 2)),
        ('#', (3, 0)),
        ('0', (3, 1)),
        ('A', (3, 2)),
    ]);
    let dirpad_map: HashMap<char, (usize, usize)> = HashMap::from_iter([
        ('#', (0, 0)),
        ('^', (0, 1)),
        ('A', (0, 2)),
        ('<', (1, 0)),
        ('v', (1, 1)),
        ('>', (1, 2)),
    ]);
    let numpad_paths = find_pairwise_paths(&numpad_map);
    let dirpad_paths = find_pairwise_paths(&dirpad_map);

    let mut answer = 0;
    for q in input {
        let line: Vec<char> = q.chars().collect();
        let num = line[0..line.len() - 1]
            .iter()
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        answer += num * forward(&numpad_paths, &dirpad_paths, q.to_string(), 3, true).len();
    }

    answer
}

fn forward(
    numpad_paths: &HashMap<(char, char), Vec<(char, char, usize, usize)>>,
    dirpad_paths: &HashMap<(char, char), Vec<(char, char, usize, usize)>>,
    seq: String,
    steps: usize,
    use_numpad: bool,
) -> String {
    if steps == 0 {
        return seq;
    }
    let mut unique_chars = Vec::new();
    let mut char_freq = HashMap::new();
    for k in seq.chars() {
        if char_freq.contains_key(&k) {
            let counter = char_freq.get_mut(&k).unwrap();
            *counter += 1;
        } else {
            unique_chars.push(k);
            char_freq.insert(k, 1);
        }
    }
    let mut curr = 'A';
    let mut buttons_with_unique = Vec::new();
    let paths = if use_numpad {
        numpad_paths
    } else {
        dirpad_paths
    };
    for k in &unique_chars {
        let mut l = MAX;
        let mut answer = "".to_string();
        for path in paths.get(&(curr, *k)).unwrap() {
            let (dir_1, dir_2, dir_1_steps, dir_2_steps) = path;
            let potential_answer = forward(
                numpad_paths,
                dirpad_paths,
                format!(
                    "{}{}A",
                    vec![dir_1; *dir_1_steps].into_iter().collect::<String>(),
                    vec![dir_2; *dir_2_steps].into_iter().collect::<String>()
                ),
                steps - 1,
                false,
            );
            if potential_answer.len() < l {
                l = potential_answer.len();
                answer = potential_answer;
            }
        }
        buttons_with_unique.push(answer);
        curr = *k;
    }
    let mut full_answer = Vec::new();
    for i in 0..unique_chars.len() {
        full_answer.push(buttons_with_unique[i].clone());
        let freq = char_freq.get(&unique_chars[i]).unwrap();
        full_answer.push(vec!['A'; *freq - 1].into_iter().collect::<String>());
    }

    full_answer.into_iter().collect::<String>()
}

fn find_pairwise_paths(
    buttons_map: &HashMap<char, (usize, usize)>,
) -> HashMap<(char, char), Vec<(char, char, usize, usize)>> {
    let mut paths = HashMap::new();
    let forbidden_cell = buttons_map.get(&'#').unwrap();
    let forbidden_cell = (forbidden_cell.0 as i32, forbidden_cell.1 as i32);
    for (k1, v1) in buttons_map {
        for (k2, v2) in buttons_map {
            if k1 == k2 || *k1 == '#' || *k2 == '#' {
                continue;
            }
            let diff = (v2.0 as i32 - v1.0 as i32, v2.1 as i32 - v1.1 as i32);
            let vertical_dir = if diff.0 < 0 { '^' } else { 'v' };
            let horizontal_dir = if diff.1 < 0 { '<' } else { '>' };
            if diff.0 == 0 || diff.1 == 0 {
                paths.insert(
                    (*k1, *k2),
                    vec![(
                        vertical_dir,
                        horizontal_dir,
                        diff.0.abs() as usize,
                        diff.1.abs() as usize,
                    )],
                );
                continue;
            }
            let potential_corner_1 = (v1.0 as i32 + diff.0, v1.1 as i32);
            let potential_corner_2 = (v1.0 as i32, v1.1 as i32 + diff.1);
            if forbidden_cell == potential_corner_1 {
                paths.insert(
                    (*k1, *k2),
                    vec![(
                        horizontal_dir,
                        vertical_dir,
                        diff.1.abs() as usize,
                        diff.0.abs() as usize,
                    )],
                );
            } else if forbidden_cell == potential_corner_2 {
                paths.insert(
                    (*k1, *k2),
                    vec![(
                        vertical_dir,
                        horizontal_dir,
                        diff.0.abs() as usize,
                        diff.1.abs() as usize,
                    )],
                );
            } else {
                paths.insert(
                    (*k1, *k2),
                    vec![
                        (
                            horizontal_dir,
                            vertical_dir,
                            diff.1.abs() as usize,
                            diff.0.abs() as usize,
                        ),
                        (
                            vertical_dir,
                            horizontal_dir,
                            diff.0.abs() as usize,
                            diff.1.abs() as usize,
                        ),
                    ],
                );
            }
        }
    }
    paths
}

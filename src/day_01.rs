use itertools::Itertools;

pub fn solve_part_2(first: &Vec<i32>, second: &Vec<i32>) -> i32 {
    let count = second.iter().counts();
    let mut answer = 0;
    for i in first {
        if count.contains_key(&i) {
            answer += i * *count.get(&i).unwrap() as i32;
        }
    }
    answer
}

pub fn solve_part_1(first: &Vec<i32>, second: &Vec<i32>) -> i32 {
    let mut first_sorted = first.clone();
    first_sorted.sort();
    let mut second_sorted = second.clone();
    second_sorted.sort();
    let mut answer = 0;
    for (i, j) in first_sorted.iter().zip(second_sorted.iter()) {
        answer += (i-j).abs();
    }
    answer
}

pub fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(
            |line| {
                let items: Vec<i32> = line
                    .split_whitespace()
                    .map(|item| item.parse().expect("Item should parse to i32"))
                    .collect();
                (items[0], items[1])
            }
        )
        .unzip()
}

pub fn parse_input(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}

pub fn solve_part_1(input: &Vec<&str>) -> u64 {
    solve(input, 25)
}

pub fn solve_part_2(input: &Vec<&str>) -> u64 {
    solve(input, 75)
}

fn solve(input: &Vec<&str>, iterations: usize) -> u64 {
    let mut curr: Vec<(String, u64)> = input.iter().map(|&s| (s.to_string(), 1)).collect();
    for _ in 0..iterations {
        let mut next = Vec::new();
        for (i, f) in &curr {
            if i == "0" {
                next.push(("1".to_string(), *f));
            } else if i.len() % 2 == 0 {
                next.push((i[0..i.len() / 2].parse::<u64>().unwrap().to_string(), *f));
                next.push((
                    i[i.len() / 2..i.len()].parse::<u64>().unwrap().to_string(),
                    *f,
                ));
            } else {
                next.push((format!("{}", i.parse::<u64>().unwrap() * 2024), *f));
            }
        }
        next.sort_by(|a, b| a.0.cmp(&b.0));
        let mut combined_next = Vec::new();
        let mut num: String = "-1".to_string();
        let mut freq = 0;
        for (i, f) in next {
            if i == num {
                freq += f;
            } else {
                if num != "-1" {
                    combined_next.push((num.to_string(), freq));
                }
                num = i;
                freq = f;
            }
        }
        combined_next.push((num.to_string(), freq));
        curr = combined_next;
    }
    curr.iter().map(|(_, f)| f).sum()
}

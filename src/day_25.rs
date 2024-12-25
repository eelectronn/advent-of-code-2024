pub fn parse_input(input: &str) -> Vec<Vec<Vec<char>>> {
    input
        .split("\n\n")
        .map(|x| {
            x.lines()
                .map(|x| x.chars().collect())
                .collect::<Vec<Vec<char>>>()
        })
        .collect()
}

pub fn solve_part_1(input: &Vec<Vec<Vec<char>>>) -> usize {
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    for obj in input {
        let mut o = vec![-1; 5];
        for y in 0..obj.len() {
            for x in 0..obj[y].len() {
                match obj[y][x] {
                    '#' => o[x] += 1,
                    _ => (),
                }
            }
        }
        if obj[0][0] == '#' {
            locks.push(o);
        } else {
            keys.push(o);
        }
    }
    let mut count = 0;
    for key in &keys {
        'locks: for lock in &locks {
            for i in 0..5 {
                if key[i] + lock[i] >= 6 {
                    continue 'locks;
                }
            }
            count += 1;
        }
    }
    count
}

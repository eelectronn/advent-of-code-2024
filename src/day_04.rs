use regex::Regex;

pub fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

pub fn solve_part_1(input: &Vec<Vec<char>>) -> usize {
    let mut horizontal = Vec::new();
    let mut vertical = Vec::new();
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            horizontal.push(input[y][x]);
            vertical.push(input[x][y]);
        }
        horizontal.push(' ');
        vertical.push(' ');
    }

    let horizontal: String = horizontal.into_iter().collect();
    let vertical: String = vertical.into_iter().collect();
    println!("{}", horizontal);
    println!("{}", vertical);

    let y_len = input.len();
    let x_len = input[0].len();
    let sum = x_len + y_len - 2;
    let mut minor_diagonal = Vec::new();
    let mut major_diagonal = Vec::new();
    for l in 0..=sum {
        for y in 0..=l {
            let x = l - y;
            if x < x_len && y < y_len {
                minor_diagonal.push(input[y][x]);
                major_diagonal.push(input[y][x_len - x - 1]);
            }
        }
        minor_diagonal.push(' ');
        major_diagonal.push(' ');
    }

    let minor_diagonal: String = minor_diagonal.into_iter().collect();
    let major_diagonal: String = major_diagonal.into_iter().collect();
    println!("{}", minor_diagonal);
    println!("{}", major_diagonal);

    let rg_xmas = Regex::new(r"XMAS").unwrap();
    let rg_samx = Regex::new(r"SAMX").unwrap();
    let mut count = 0;
    for line in vec![horizontal, vertical, minor_diagonal, major_diagonal] {
        count += rg_xmas.find_iter(&line).count();
        count += rg_samx.find_iter(&line).count();
    }

    count
}

pub fn solve_part_2(input: &Vec<Vec<char>>) -> usize {
    let mut count = 0;
    for y in 0..input.len() - 2 {
        for x in 0..input[y].len() - 2 {
            let major_diagonal = format!(
                "{}{}{}",
                input[y][x],
                input[y + 1][x + 1],
                input[y + 2][x + 2]
            );
            let minor_diagonal = format!(
                "{}{}{}",
                input[y][x + 2],
                input[y + 1][x + 1],
                input[y + 2][x]
            );
            if (major_diagonal == "MAS" || major_diagonal == "SAM")
                && (minor_diagonal == "MAS" || minor_diagonal == "SAM")
            {
                count += 1;
            }
        }
    }
    count
}

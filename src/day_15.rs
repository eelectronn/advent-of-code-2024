use std::collections::HashSet;

pub fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<char>) {
    let (grid, moves) = input.split_once("\n\n").unwrap();
    (
        grid.lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>(),
        moves
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .flat_map(|line| line)
            .collect::<Vec<char>>(),
    )
}

pub fn solve_part_1(input: &(Vec<Vec<char>>, Vec<char>)) -> usize {
    let (grid, moves) = input;
    let mut grid = grid.clone();
    let mut player_cords = find_player(&grid);

    for &dir in moves {
        player_cords = step(&mut grid, player_cords, dir);
    }

    let mut answer = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'O' {
                answer += 100 * y + x;
            }
        }
    }

    answer
}

pub fn solve_part_2(input: &(Vec<Vec<char>>, Vec<char>)) -> usize {
    let (grid, moves) = input;
    let mut grid = enlarge_grid(grid);
    let mut player_cords = find_player(&grid);

    for &dir in moves {
        match dir {
            '^' | 'v' => player_cords = step_vertical(&mut grid, player_cords, dir),
            _ => player_cords = step(&mut grid, player_cords, dir),
        }
    }

    let mut answer = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '[' {
                answer += 100 * y + x;
            }
        }
    }

    answer
}

fn find_player(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == '@' {
                return (y, x);
            }
        }
    }
    (0, 0)
}

fn step_vertical(
    grid: &mut Vec<Vec<char>>,
    player_cords: (usize, usize),
    dir: char,
) -> (usize, usize) {
    let (py, px) = player_cords;
    let (dy, dx) = match dir {
        '^' => (-1, 0),
        'v' => (1, 0),
        _ => (0, 0),
    };
    let (oy, ox) = (py as i32, px as i32);
    let mut to_move = vec![HashSet::from([(oy, ox)])];
    loop {
        let mut to_move_next = HashSet::new();
        for (y, x) in &to_move[to_move.len() - 1] {
            let (ny, nx) = (y + dy, x + dx);
            match grid[ny as usize][nx as usize] {
                '#' => {
                    return (oy as usize, ox as usize);
                }
                '[' => {
                    to_move_next.insert((ny, nx));
                    to_move_next.insert((ny, nx + 1));
                }
                ']' => {
                    to_move_next.insert((ny, nx));
                    to_move_next.insert((ny, nx - 1));
                }
                _ => {}
            }
        }
        if to_move_next.is_empty() {
            break;
        }
        to_move.push(to_move_next);
    }
    to_move.reverse();
    for layer in to_move {
        for (y, x) in layer {
            let (ny, nx) = (y + dy, x + dx);
            grid[ny as usize][nx as usize] = grid[y as usize][x as usize];
            grid[y as usize][x as usize] = '.';
            if grid[ny as usize][nx as usize] == '@' {
                return (ny as usize, nx as usize);
            }
        }
    }
    (0, 0)
}

fn step(grid: &mut Vec<Vec<char>>, player_cords: (usize, usize), dir: char) -> (usize, usize) {
    let (py, px) = player_cords;
    let (dy, dx) = match dir {
        '^' => (-1, 0),
        'v' => (1, 0),
        '<' => (0, -1),
        '>' => (0, 1),
        _ => (0, 0),
    };
    let (oy, ox) = (py as i32, px as i32);
    let (mut ny, mut nx) = (oy + dy, ox + dx);
    let mut found = false;
    while ny >= 0 && ny < grid.len() as i32 && nx >= 0 && nx < grid[0].len() as i32 {
        if grid[ny as usize][nx as usize] == '#' {
            break;
        }
        if grid[ny as usize][nx as usize] == '.' {
            found = true;
            break;
        }
        (ny, nx) = (ny + dy, nx + dx);
    }
    let (mut p_final_y, mut p_final_x) = (oy as usize, ox as usize);
    while found && (ny != oy || nx != ox) {
        let (ey, ex) = (ny - dy, nx - dx);
        grid[ny as usize][nx as usize] = grid[ey as usize][ex as usize];
        if grid[ny as usize][nx as usize] == '@' {
            (p_final_y, p_final_x) = (ny as usize, nx as usize);
        }
        grid[ey as usize][ex as usize] = '.';
        (ny, nx) = (ey, ex);
    }
    (p_final_y, p_final_x)
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
    println!();
}

fn enlarge_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_gird = Vec::with_capacity(grid.len());
    for row in grid {
        let mut new_row = Vec::with_capacity(row.len() * 2);
        for cell in row {
            match cell {
                '@' => {
                    new_row.push('@');
                    new_row.push('.');
                }
                'O' => {
                    new_row.push('[');
                    new_row.push(']');
                }
                _ => {
                    new_row.push(*cell);
                    new_row.push(*cell);
                }
            }
        }
        new_gird.push(new_row);
    }
    new_gird
}

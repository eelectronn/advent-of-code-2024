pub fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<char>) {
    let (maze, moves) = input.split_once("\n\n").unwrap();
    (
        maze.lines()
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
    let (maze, moves) = input;
    let mut maze = maze.clone();
    let mut player_cords = find_player(&maze);

    for &dir in moves {
        player_cords = step(&mut maze, player_cords, dir);
        // println!("After moving {}", dir);
        // print_maze(&maze);
    }

    let mut answer = 0;
    for y in 0..maze.len() {
        for x in 0..maze[0].len() {
            if maze[y][x] == 'O' {
                answer += 100 * y + x;
            }
        }
    }

    answer
}

fn find_player(maze: &Vec<Vec<char>>) -> (usize, usize) {
    for (y, row) in maze.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell == '@' {
                return (y, x);
            }
        }
    }
    (0, 0)
}

fn step(maze: &mut Vec<Vec<char>>, player_cords: (usize, usize), dir: char) -> (usize, usize) {
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
    while ny >= 0 && ny < maze.len() as i32 && nx >= 0 && nx < maze[0].len() as i32 {
        if maze[ny as usize][nx as usize] == '#' {
            break;
        }
        if maze[ny as usize][nx as usize] == '.' {
            found = true;
            break;
        }
        (ny, nx) = (ny + dy, nx + dx);
    }
    let (mut p_final_y, mut p_final_x) = (oy as usize, ox as usize);
    while found && (ny != oy || nx != ox) {
        let (ey, ex) = (ny - dy, nx - dx);
        maze[ny as usize][nx as usize] = maze[ey as usize][ex as usize];
        if maze[ny as usize][nx as usize] == '@' {
            (p_final_y, p_final_x) = (ny as usize, nx as usize);
        }
        maze[ey as usize][ex as usize] = '.';
        (ny, nx) = (ey, ex);
    }
    (p_final_y, p_final_x)
}

fn print_maze(maze: &Vec<Vec<char>>) {
    for row in maze {
        println!("{}", row.iter().collect::<String>());
    }
    println!();
}

use std::collections::BinaryHeap;

pub fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .filter(|x| !x.is_empty())
        // Comma separated integers in lines
        .map(|x| {
            let mut parts = x.split(",");
            (
                parts.next().unwrap().parse::<usize>().unwrap(),
                parts.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect()
}

pub fn solve_part_1(input: &Vec<(usize, usize)>) -> i32 {
    let l = 71;
    let num_of_bytes = 1024;

    let mut grid = vec![vec!['.'; l]; l];
    for i in 0..num_of_bytes {
        let (x, y) = input[i];
        grid[y][x] = '#';
    }

    find_steps(&grid, (0, 0), (l - 1, l - 1))
}

pub fn solve_part_2(input: &Vec<(usize, usize)>) -> String {
    let l = 71;
    let num_of_bytes = 1024;

    let mut lower = num_of_bytes as usize;
    let mut upper = input.len();
    while upper - lower > 5 {
        let mid = (lower + upper) / 2;
        let mut grid = vec![vec!['.'; l]; l];
        for i in 0..=mid {
            let (x, y) = input[i];
            grid[y][x] = '#';
        }
        let steps = find_steps(&grid, (0, 0), (l - 1, l - 1));
        if steps == -1 {
            upper = mid + 1;
        } else {
            lower = mid;
        }
    }

    for i in lower..upper {
        let mut grid = vec![vec!['.'; l]; l];
        for j in 0..i {
            let (x, y) = input[j];
            grid[y][x] = '#';
        }
        let steps = find_steps(&grid, (0, 0), (l - 1, l - 1));
        if steps == -1 {
            return format!("{},{}", input[i - 1].0, input[i - 1].1);
        }
    }
    "-1,-1".to_string()
}

fn find_steps(grid: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> i32 {
    let mut heap = BinaryHeap::new();
    heap.push((0 as i32, start));
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut answer = -1;
    while let Some((steps, (y, x))) = heap.pop() {
        if x == end.1 && y == end.0 {
            answer = -1 * steps;
            break;
        }
        if visited[y][x] {
            continue;
        }
        visited[y][x] = true;
        if x > 0 && grid[y][x - 1] == '.' {
            heap.push((steps - 1, (y, x - 1)));
        }
        if x < grid[0].len() - 1 && grid[y][x + 1] == '.' {
            heap.push((steps - 1, (y, x + 1)));
        }
        if y > 0 && grid[y - 1][x] == '.' {
            heap.push((steps - 1, (y - 1, x)));
        }
        if y < grid.len() - 1 && grid[y + 1][x] == '.' {
            heap.push((steps - 1, (y + 1, x)));
        }
    }

    answer
}

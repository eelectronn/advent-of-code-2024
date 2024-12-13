pub fn parse_input(input: &str) -> Vec<usize> {
    input
        .trim()
        .chars()
        .map(|c| format!("{}", c).parse().unwrap())
        .collect()
}

pub fn solve_part_1(input: &Vec<usize>) -> i64 {
    let mut disk: Vec<i32> = Vec::new();
    let mut file_blocks = Vec::new();
    let mut count = 0;
    for i in 0..input.len() {
        if i % 2 == 0 {
            disk.extend_from_slice(&vec![(i / 2) as i32; input[i]]);
            file_blocks.extend_from_slice(&vec![(i / 2) as i32; input[i]]);
            count += input[i];
        } else {
            disk.extend_from_slice(&vec![-1 as i32; input[i]]);
        }
    }
    file_blocks.reverse();

    let mut corrected_disk = Vec::with_capacity(count);
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    while k < count {
        if disk[i] != -1 {
            corrected_disk.push(disk[i]);
        } else {
            corrected_disk.push(file_blocks[j]);
            j += 1;
        }
        i += 1;
        k += 1;
    }

    let mut answer: i64 = 0;
    for (i, file_id) in corrected_disk.iter().enumerate() {
        answer += i as i64 * *file_id as i64;
    }

    answer
}

pub fn solve_part_2(input: &Vec<usize>) -> i64 {
    let mut disk = Vec::new();
    let mut file_blocks = Vec::new();
    for i in 0..input.len() {
        if i % 2 == 0 {
            disk.push(((i / 2) as i32, input[i]));
            file_blocks.push(((i / 2) as i32, input[i]));
        } else {
            disk.push((-1, input[i]));
        }
    }
    file_blocks.reverse();

    for (id, file_size) in file_blocks.iter() {
        let mut next_iteration = Vec::new();
        let mut moved = false;
        for (disk_id, disk_size) in disk.iter() {
            if *disk_id != -1 {
                if id != disk_id {
                    next_iteration.push((*disk_id, *disk_size));
                    continue;
                }
                if moved {
                    next_iteration.push((-1, *disk_size));
                } else {
                    next_iteration.push((*disk_id, *disk_size));
                    if *disk_id == *id {
                        moved = true;
                    }
                }
                continue;
            }
            if file_size <= disk_size && !moved {
                next_iteration.push((*id, *file_size));
                moved = true;
                if *disk_size - *file_size > 0 {
                    next_iteration.push((-1, *disk_size - *file_size));
                }
            } else {
                next_iteration.push((-1, *disk_size));
            }
        }
        disk = next_iteration;
    }
    let mut answer: i64 = 0;
    let mut i = 0;
    for (id, size) in disk.iter() {
        if *id == -1 {
            i += *size;
            continue;
        }
        for j in 0..*size {
            answer += i as i64 * *id as i64;
            i += 1;
        }
    }
    answer
}

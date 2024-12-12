pub fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let items: Vec<&str> = line.split(':').collect();
            let target = items[0].parse().unwrap();
            let items = items[1]
                .trim()
                .split(' ')
                .map(|item| item.parse().unwrap())
                .collect();
            (target, items)
        })
        .collect()
}

pub fn solve_part_1(input: &Vec<(u64, Vec<u64>)>) -> u64 {
    let mut answer = 0;
    for (target, nums) in input {
        if curse_part_1(*target, nums[0], nums, 1) {
            answer += target;
        }
    }
    answer
}

fn curse_part_1(target: u64, actual: u64, nums: &Vec<u64>, index: usize) -> bool {
    if index == nums.len() {
        return target == actual;
    }
    curse_part_1(target, actual + nums[index], nums, index + 1)
        || curse_part_1(target, actual * nums[index], nums, index + 1)
}

pub fn solve_part_2(input: &Vec<(u64, Vec<u64>)>) -> u64 {
    let mut answer = 0;
    for (target, nums) in input {
        if curse_part_2(*target, nums[0], nums, 1) {
            answer += target;
        }
    }
    answer
}

fn curse_part_2(target: u64, actual: u64, nums: &Vec<u64>, index: usize) -> bool {
    if index == nums.len() {
        return target == actual;
    }
    curse_part_2(target, actual + nums[index], nums, index + 1)
        || curse_part_2(target, actual * nums[index], nums, index + 1)
        || curse_part_2(
            target,
            format!("{}{}", actual, nums[index]).parse().unwrap(),
            nums,
            index + 1,
        )
}

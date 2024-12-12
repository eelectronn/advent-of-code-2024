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
        if curse(*target, nums[0], nums, 1) {
            answer += target;
        }
    }
    answer
}

fn curse(target: u64, actual: u64, nums: &Vec<u64>, index: usize) -> bool {
    if index == nums.len() {
        return target == actual;
    }
    curse(target, actual + nums[index], nums, index + 1)
        || curse(target, actual * nums[index], nums, index + 1)
}

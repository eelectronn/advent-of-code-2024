mod input;
mod day_01;

fn main() {
    dotenv::dotenv().ok();
    input::get_input("1");
    let input = input::read_input("1");
    let (first, second) = day_01::parse_input(&input);
    println!("Answer part 1: {}", day_01::solve_part_1(&first, &second));
    println!("Answer part 2: {}", day_01::solve_part_2(&first, &second));
}

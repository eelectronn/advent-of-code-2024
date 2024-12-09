mod input;
// mod day_01;
mod day_02;

fn main() {
    dotenv::dotenv().ok();

    // let day = "2";

    // input::get_input("1");
    // let input = input::read_input("1");
    // let (first, second) = day_01::parse_input(&input);
    // println!("Answer part 1: {}", day_01::solve_part_1(&first, &second));
    // println!("Answer part 2: {}", day_01::solve_part_2(&first, &second));

    let input = input::read_input("2");
    let parsed_input = day_02::parse_input(&input);
    println!("Answer part 1: {}", day_02::solve_part_1(&parsed_input));
}

mod input;
mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;

fn main() {
    dotenv::dotenv().ok();

    // input::get_input("5");
    // let input = input::read_input("1");
    // let (first, second) = day_01::parse_input(&input);
    // println!("Answer part 1: {}", day_01::solve_part_1(&first, &second));
    // println!("Answer part 2: {}", day_01::solve_part_2(&first, &second));

    // let input = input::read_input("2");
    // let parsed_input = day_02::parse_input(&input);
    // println!("Answer part 1: {}", day_02::solve_part_1(&parsed_input));
    // println!("Answer part 2: {}", day_02::solve_part_2(&parsed_input));

    // let input = input::read_input("3");
    // let parsed_input = day_03::parse_input(&input);
    // println!("Answer part 1: {}", day_03::solve_part_1(&parsed_input));
    // println!("Answer part 2: {}", day_03::solve_part_2(&parsed_input));

    // let input = input::read_input("4");
    // let parsed_input = day_04::parse_input(&input);
    // println!("Answer part 1: {}", day_04::solve_part_1(&parsed_input));
    // println!("Answer part 2: {}", day_04::solve_part_2(&parsed_input));

    let input = input::read_input("5");
    let parsed_input = day_05::parse_input(&input);
    println!("Answer part 1: {}", day_05::solve_part_1(&parsed_input));
}

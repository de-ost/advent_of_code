// https://adventofcode.com/2023/day/2

mod part_1;
mod part_2;

fn main() {
    let input = include_str!("../resources/input.txt");

    let result_1 = part_1::result(input);
    let result_2 = part_2::result(input);

    println!("Result of part 1: {}", result_1);
    println!("Result of part 2: {}", result_2);
}

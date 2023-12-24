// https://adventofcode.com/2023/day/2

mod part_1;

fn main() {
    let input = include_str!("../resources/input.txt");
    let result = part_1::result(input);
    println!("Result of part 1: {}", result);
}

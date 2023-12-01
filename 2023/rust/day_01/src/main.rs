// https://adventofcode.com/2023/day/1

mod part1;
mod part2;

use part1::solve_part_1;
use part2::solve_part_2;

fn main() {
    let input = include_str!("../resources/input.txt");
    let ans_part_1 = solve_part_1(&input);
    let ans_part_2 = solve_part_2(&input);

    println!("Answer part 1: {}", ans_part_1);
    println!("Answer part 2: {}", ans_part_2);
}

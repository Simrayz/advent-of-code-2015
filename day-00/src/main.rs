use day_00::*;

fn main() {
    let input = include_str!("../input1.txt");
    println!("Part1: {}", part1::process(input).unwrap());
    println!("Part2: {}", part2::process(input).unwrap());
}

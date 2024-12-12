use day_03::*;

fn main() {
    let input = include_str!("../input2.txt");
    println!("Part1: {}", part1::process(input).unwrap());
    println!("Part2: {}", part2::process(input).unwrap());
}

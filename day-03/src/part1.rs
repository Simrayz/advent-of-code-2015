use std::collections::HashSet;

use crate::move_in_direction;

pub fn process(input: &str) -> miette::Result<String> {
    let mut unique_houses = HashSet::<(i32, i32)>::from([(0, 0)]);

    let final_position = input.chars().fold((0, 0), |pos, c| {
        let new_position = move_in_direction(pos, c);
        unique_houses.insert(new_position);
        new_position
    });

    let unique_house_count = unique_houses.len();

    println!("Final position: {:?}", final_position);

    Ok(unique_house_count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process_part1() {
        let input = include_str!("../input1.txt");
        let result = process(input).unwrap();
        assert_eq!("4", result);
    }
}

use std::collections::HashSet;

use crate::move_in_direction;

pub fn process(input: &str) -> miette::Result<String> {
    let (santa_moves, robo_moves) = input.chars().enumerate().fold(
        (Vec::new(), Vec::new()),
        |(mut santa, mut robo), (idx, c)| {
            if idx % 2 == 0 {
                santa.push(c);
            } else {
                robo.push(c);
            }
            (santa, robo)
        },
    );

    let mut santa_houses = solve_for_moves(santa_moves);
    let robo_houses = solve_for_moves(robo_moves);
    santa_houses.extend(robo_houses);

    Ok(santa_houses.len().to_string())
}

fn solve_for_moves(moves: Vec<char>) -> HashSet<(i32, i32)> {
    let mut unique_houses = HashSet::<(i32, i32)>::from([(0, 0)]);
    moves.into_iter().fold((0, 0), |pos, c| {
        let new_position = move_in_direction(pos, c);
        unique_houses.insert(new_position);
        new_position
    });
    return unique_houses;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process_part1() {
        let input = include_str!("../input1.txt");
        let result = process(input).unwrap();
        assert_eq!("", result);
    }
}

pub mod part1;
pub mod part2;

pub fn move_in_direction((row, col): (i32, i32), direction: char) -> (i32, i32) {
    match direction {
        '^' => (row - 1, col),
        'v' => (row + 1, col),
        '>' => (row, col + 1),
        '<' => (row, col - 1),
        _ => (row, col),
    }
}

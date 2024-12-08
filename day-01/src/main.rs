fn main() {
    let input: &str = include_str!("../input.txt");
    let mut basement_idx = 0;
    let mut basement_found = false;
    let mut current_level = 0;

    let level = input.chars().enumerate().find(|(idx, c)| {
        if *c == '(' {
            current_level += 1;
        } else {
            current_level -= 1;
        }

        if current_level < 0 && !basement_found {
            basement_found = true;
            basement_idx = *idx;
        }

        return basement_found;
    });

    println!("Current level: {}", basement_idx);

    println!("Final level: {:?}", level);
}

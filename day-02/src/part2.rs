use crate::get_package_dimension;

pub fn process(input: &str) -> miette::Result<String> {
    let package_dimensions = get_package_dimension(input);

    let total_ribbon = package_dimensions
        .iter()
        .map(|dims| get_ribbon_length(dims[0], dims[1], dims[2]))
        .sum::<u32>();

    Ok(total_ribbon.to_string())
}

fn get_ribbon_length(l: u32, w: u32, h: u32) -> u32 {
    let smallest_perimeter = 2 * vec![l + w, w + h, h + l].into_iter().min().unwrap();
    let volume = l * w * h;
    return smallest_perimeter + volume;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process_part2() {
        let input = include_str!("../input0.txt");
        let result = process(input).unwrap();
        assert_eq!("48", result);
    }
}

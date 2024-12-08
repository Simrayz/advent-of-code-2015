use crate::get_package_dimension;

pub fn process(input: &str) -> miette::Result<String> {
    let package_dimensions = get_package_dimension(input);

    let total_wrapping_paper = package_dimensions
        .iter()
        .map(|dims| get_wrapping_paper(dims[0], dims[1], dims[2]))
        .sum::<u32>();

    Ok(total_wrapping_paper.to_string())
}

fn get_wrapping_paper(l: u32, w: u32, h: u32) -> u32 {
    let required = (2 * l * w) + (2 * w * h) + (2 * h * l);
    let slack = vec![w * h, l * h, l * w].into_iter().min().unwrap();
    return required + slack;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_get_wrapping_paper() {
        assert_eq!(get_wrapping_paper(2, 3, 4), 58);
        assert_eq!(get_wrapping_paper(1, 1, 10), 43);
    }

    #[test_log::test]
    fn test_process_part1() {
        let input = include_str!("../input0.txt");
        let result = process(input).unwrap();
        assert_eq!("101", result);
    }
}

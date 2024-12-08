pub fn process(input: &str) -> miette::Result<String> {
    Ok("".to_string())
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

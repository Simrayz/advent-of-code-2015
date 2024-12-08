pub fn process(_input: &str) -> miette::Result<String> {
    Ok("".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test() {
        assert_eq!(process("").unwrap(), "");
    }
}

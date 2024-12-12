pub fn process(input: &str) -> miette::Result<String> {
    let nice_count: Vec<&str> = input
        .lines()
        .filter(|line| is_nice(&line.to_string()))
        .collect();

    Ok(nice_count.len().to_string())
}

pub fn is_nice(word: &str) -> bool {
    let word = word.to_string();

    if ["ab", "cd", "pq", "xy"]
        .iter()
        .any(|pair| word.contains(pair))
    {
        return false;
    }

    let mut vowel_count = 0;
    for c in word.chars() {
        if "aeiou".contains(c) {
            vowel_count += 1;
        }
    }

    if vowel_count < 3 {
        return false;
    }

    let mut is_nice = false;
    for i in 0..word.len() - 1 {
        if word.chars().nth(i).unwrap() == word.chars().nth(i + 1).unwrap() {
            is_nice = true;
            break;
        }
    }

    return is_nice;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_is_nice() {
        assert!(is_nice(&"ugknbfddgicrmopn".to_string()));
        assert!(is_nice(&"aaa".to_string()));
    }

    #[test_log::test]
    fn test_is_naughty() {
        assert!(!is_nice(&"jchzalrnumimnmhp".to_string()));
    }

    #[test_log::test]
    fn test_process_1() {
        let input = include_str!("../input1.txt");
        let result = process(input).unwrap();
        assert_eq!("2", result);
    }
}

pub fn process(input: &str) -> miette::Result<String> {
    let mut nice_count = 0;
    for word in input.lines() {
        let mut twice = false;
        for i in 0..word.len() - 3 {
            let a = word.chars().nth(i).unwrap();
            let b = word.chars().nth(i + 1).unwrap();

            for j in i + 2..word.len() - 1 {
                let c = word.chars().nth(j).unwrap();
                let d = word.chars().nth(j + 1).unwrap();

                if a == c && b == d {
                    twice = true;
                    break;
                }
            }
            if twice {
                break;
            }
        }
        if !twice {
            continue;
        }

        for i in 0..input.len() - 2 {
            if word.chars().nth(i).unwrap() == word.chars().nth(i + 2).unwrap() {
                nice_count += 1;
                break;
            }
        }
    }

    Ok(nice_count.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process_1() {
        let input = include_str!("../input3.txt");
        let result = process(input).unwrap();
        assert_eq!("2", result);
    }
}

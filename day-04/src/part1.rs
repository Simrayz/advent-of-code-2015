pub fn process(input: &str) -> miette::Result<String> {
    let data = input.to_string();
    let mut key = 0;

    loop {
        let key_str = key.to_string();
        let mut x = data.clone();

        x.push_str(&key_str);
        let digest = md5::compute(x);

        if format!("{digest:x}").starts_with("000000") {
            // println!("{}\t{:?}", key, digest);
            println!("{key}");
            break;
        }

        key += 1;
    }

    Ok(key.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_log::test]
    fn test_process_1() {
        let input = "abcdef";
        let result = process(input).unwrap();
        assert_eq!("609043", result);
    }
}

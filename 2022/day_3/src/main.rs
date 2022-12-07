#![feature(custom_test_frameworks)]

fn main() {
    let filename = "day_3/src/input.txt";
    let lines = fileutils::lines_from_file(filename);
    let list = "aA";
    println!(
        "{:?}",
        list.chars()
            .map(map_to_value)
            .map(|i| i.unwrap())
            .collect::<Vec<u8>>()
    );
}

fn map_to_value(c: char) -> Option<u8> {
    const START_VALUE_LOWERCASE: u8 = 1;
    const START_VALUE_UPPERCASE: u8 = 27;
    match c {
        c if c.is_alphabetic() && c.is_ascii_uppercase() => {
            Some(c as u8 - 'A' as u8 + START_VALUE_UPPERCASE)
        }
        c if c.is_alphabetic() && c.is_ascii_lowercase() => {
            Some(c as u8 - 'a' as u8 + START_VALUE_LOWERCASE)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::*;

    #[test_case('a', Some(1); "lowercase a")]
    #[test_case('b', Some(2); "lowercase b")]
    #[test_case('z', Some(26); "lowercase z")]
    #[test_case('A', Some(27); "uppercase A")]
    #[test_case('Z', Some(52); "uppercase Z")]
    fn values_are_correct(c: char, expected: Option<u8>) {
        let result = map_to_value(c);
        assert_eq!(result, expected);
    }
}

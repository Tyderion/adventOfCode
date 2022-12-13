fn main() {
    println!("Hello, world!");
}

fn part1<T: AsRef<str>>(lines: Vec<T>) -> u32{
    0
}

#[cfg(test)]
mod tests {

    use super::*;

    static EXAMPLE: [&str; 5] = ["30373", "25512", "65332", "33549", "35390"];

    #[test]
    fn example_case_part1() {
        let result = part1(EXAMPLE.to_vec());
        assert_eq!(result, 21);
    }
}

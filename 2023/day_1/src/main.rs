pub fn main() {
    let filename = "day_1/src/input.txt";
    let _lines = fileutils::safe_lines_from_file(filename);
    println!("Hello world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const EXAMPLE_INPUT: [&str; 0] = [];
}

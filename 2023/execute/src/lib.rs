use std::fmt::Display;

pub fn load_and_execute<T, U>(
    filename: impl AsRef<str>,
    part1: fn(lines: &Vec<String>) -> T,
    part2: fn(lines: &Vec<String>) -> U,
) where
    T: Display,
    U: Display,
{
    let input = fileutils::safe_lines_from_file(filename.as_ref());
    let part1_result = match input {
        None => panic!("No input received"),
        Some(ref lines) => part1(lines),
    };
    let part2_result = match input {
        None => panic!("No input received"),
        Some(ref lines) => part2(lines),
    };
    println!("Part 1: {}", part1_result);
    println!("Part 2: {}", part2_result);
}

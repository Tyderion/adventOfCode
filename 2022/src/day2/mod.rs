
use crate::fileutils;

fn compute_rps_results(lines: Vec<String>) -> i32 {
    0
}

#[allow(dead_code)]
pub fn run() {
    let filename = "./src/day2/input.txt";
    let lines = fileutils::lines_from_file(filename);

    let result = compute_rps_results(lines);
    println!("Result: {:?} Points", result)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rps_results_are_computed_correctly() {
        let result = compute_rps_results(["A Y","B X","C _Z"].map(String::from).to_vec());
        assert_eq!(result, 15);
    }
}
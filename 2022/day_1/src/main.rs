use std::cmp::Reverse;

pub fn main() {
    let filename = "day_1/src/input.txt";
    let lines = fileutils::lines_from_file(filename);
    let grouped_calories = compute_calories(lines);
    println!("Max: {:?}", grouped_calories.iter().max().unwrap());

    let top3 = grouped_calories.iter().take(3).sum::<i32>();
    println!("Top 3 carry: {:?}", top3);
}

fn compute_calories(lines: Vec<String>) -> Vec<i32> {
    let mut grouped_calories = lines.iter().fold(
        vec![0],
        |mut result, current| {
            match current {
                _ if current.is_empty() => result.push(0),
                weight => *result.last_mut().unwrap() += weight.parse::<i32>().unwrap(),
            }
            result
        },
    );
    grouped_calories.sort_by_key(|n| Reverse(*n));
    grouped_calories
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calorie_are_computed_and_ordered_correctly() {
        let result = compute_calories(["1000","2000","3000","","4000","","5000","6000","","7000","8000","9000","","10000"].map(String::from).to_vec());
        assert_eq!(result, vec![24000, 11000, 10000, 6000, 4000]);
    }
}
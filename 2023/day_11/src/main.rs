pub fn main() {
    execute::load_and_execute("day_11/src/input.txt", part1, part2);
}

#[derive(Debug)]
struct Galaxy {
    row: usize,
    col: usize,
}

impl Galaxy {
    fn distance(&self, other: &Galaxy) -> usize {
        self.col.abs_diff(other.col) + self.row.abs_diff(other.row)
    }
}

fn compute_distances(lines: &Vec<impl AsRef<str>>, empty_space_size: usize) -> usize {
    let mut rows = vec![false; lines.len()];
    let mut cols = vec![false; lines.first().unwrap().as_ref().chars().count()];
    let mut galaxies = lines
        .iter()
        .enumerate()
        .flat_map(|(row, l)| {
            l.as_ref().chars().enumerate().filter_map(move |(col, c)| {
                if c == '#' {
                    Some(Galaxy { row: row, col: col })
                } else {
                    None
                }
            })
        })
        .collect::<Vec<_>>();
    galaxies.iter().for_each(|g| {
        rows[g.row] = true;
        cols[g.col] = true;
    });

    rows.iter()
        .enumerate()
        .filter_map(|(row, galaxy_present)| if !galaxy_present { Some(row) } else { None })
        .rev()
        .for_each(|row| {
            galaxies
                .iter_mut()
                .filter(|g| g.row > row)
                .for_each(|galaxy| {
                    galaxy.row += empty_space_size - 1;
                });
        });

    cols.iter()
        .enumerate()
        .filter_map(|(col, galaxy_present)| if !galaxy_present { Some(col) } else { None })
        .rev()
        .for_each(|col| {
            galaxies
                .iter_mut()
                .filter(|g| g.col > col)
                .for_each(|galaxy| {
                    galaxy.col += empty_space_size - 1;
                });
        });

    let mut sum = 0 as usize;
    for (i, galaxy_1) in galaxies.iter().enumerate() {
        for galaxy_2 in galaxies[i + 1..].iter() {
            sum += galaxy_1.distance(galaxy_2);
        }
    }
    sum
}

fn part1(lines: &Vec<impl AsRef<str>>) -> usize {
    compute_distances(lines, 2)
}

fn part2(lines: &Vec<impl AsRef<str>>) -> usize {
    compute_distances(lines, 1000000 as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT1: [&str; 10] = [
        "...#......",
        ".......#..",
        "#.........",
        "..........",
        "......#...",
        ".#........",
        ".........#",
        "..........",
        ".......#..",
        "#...#.....",
    ];

    #[test]
    fn example_case_part1() {
        let result = compute_distances(
            &EXAMPLE_INPUT1.iter().map(|x| String::from(*x)).collect(),
            2 as usize,
        );
        assert_eq!(result, 374);
    }

    #[test]
    fn example_case_part2() {
        let result = compute_distances(
            &EXAMPLE_INPUT1.iter().map(|x| String::from(*x)).collect(),
            10 as usize,
        );
        assert_eq!(result, 1030);
    }

    #[test]
    fn example_case_part2_2() {
        let result = compute_distances(
            &EXAMPLE_INPUT1.iter().map(|x| String::from(*x)).collect(),
            100 as usize,
        );
        assert_eq!(result, 8410);
    }
}

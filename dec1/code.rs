const TARGET: u64 = 2020;

#[allow(dead_code)]
fn report_pair(numbers: &[u64]) -> Result<(u64, u64), impl std::fmt::Debug> {
    let mut seen = std::collections::HashSet::new();
    for number in numbers {
        if seen.contains(&(TARGET - number)) {
            return Ok((*number, (TARGET - number)));
        }
        seen.insert(number);
    }

    Err("input doesn't contain what you are looking for...")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufRead;

    #[test]
    fn example() {
        let numbers = vec![1721, 979, 366, 299, 675, 1456];

        assert_eq!(
            report_pair(&numbers).expect("failed to find two numbers that sum to 2020"),
            (299, 1721),
        );

        println!("Answer is: {}", 299 * 1721);
    }

    #[test]
    fn real() {
        let numbers = std::io::BufReader::new(std::fs::File::open("input.txt").unwrap())
            .lines()
            .flatten()
            .map(|i| i.parse().unwrap())
            .collect::<Vec<_>>();

        let pair = report_pair(&numbers).expect("failed to find two numbers that sum to 2020");

        println!("Answer is: {}", pair.0 * pair.1);
    }
}

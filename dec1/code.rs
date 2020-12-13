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

#[allow(dead_code)]
fn report_tripple(numbers: &[u64]) -> Result<(u64, u64, u64), impl std::fmt::Debug> {
    let mut seen = std::collections::HashSet::new();
    for number in numbers {
        for number_second in numbers {
            if let Some(look_for) = TARGET.checked_sub(number + number_second) {
                if seen.contains(&look_for) {
                    return Ok((*number, *number_second, look_for));
                }
            }
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

        println!("Example Answer is: {}", 299 * 1721);

        let triple = report_tripple(&numbers).expect("failed to find three numbers that sum to 2020");
        dbg!(triple);
        println!("Example Answer to part two is is: {}", 979 * 366 * 366);
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

        let triple = report_tripple(&numbers).expect("failed to find three numbers that sum to 2020");
        dbg!(triple);
        println!("Answer to part two is is: {}", triple.0 * triple.1 * triple.2);
    }
}

#[allow(dead_code)]
fn count_valid(passwords: impl Iterator<Item = String>) -> usize {
    passwords
        .filter(|pwd_line| {
            match pwd_line
                .split(|c| c == ' ' || c == '-' || c == ':')
                .collect::<Vec<_>>()
                .as_slice()
            {
                [from, to, r#char, _, password] => {
                    let from = from.parse::<usize>().unwrap();
                    let to = to.parse::<usize>().unwrap();

                    let count = password
                        .chars()
                        .filter(|c| *c == r#char.chars().next().unwrap())
                        .count();

                    count <= to && count >= from
                }
                got => panic!("malformed input: {:?}", got),
            }
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufRead;

    #[test]
    fn example() {
        let passwords = vec![
            "1-3 a: abcde".to_string(),
            "1-3 b: cdefg".to_string(),
            "2-9 c: ccccccccc".to_string(),
        ];

        let n = count_valid(passwords.into_iter());

        println!("Answer is: {}", n);
    }

    #[test]
    fn real() {
        let mut passwords = std::io::BufReader::new(std::fs::File::open("input.txt").unwrap())
            .lines()
            .flatten();

        let n = count_valid(&mut passwords);

        println!("Answer is: {}", n);
    }
}

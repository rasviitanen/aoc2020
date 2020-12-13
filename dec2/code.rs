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

#[allow(dead_code)]
fn count_valid_part_two(mut passwords: impl Iterator<Item = String>) -> usize {
    passwords
     .by_ref()
        .filter(|pwd_line| {
            match pwd_line
                .split(|c| c == ' ' || c == '-' || c == ':')
                .collect::<Vec<_>>()
                .as_slice()
            {
                [pos1, pos2, r#char, _, password] => {
                    let pos1= pos1.parse::<usize>().unwrap();
                    let pos2 = pos2.parse::<usize>().unwrap();

                    let look_for = r#char.chars().nth(0).unwrap();
                    let (a, b) = (password.chars().nth(pos1-1).map(|c| c == look_for), password.chars().nth(pos2-1).map(|c| c == look_for));
                    
                    a.unwrap_or(false) ^ b.unwrap_or(false)
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

        let n = count_valid(passwords.clone().into_iter());

        println!("Example Answer is: {}", n);

        let n = count_valid_part_two(passwords.into_iter());

        println!("Example Answer to part two is: {}", n);
    }

    #[test]
    fn real() {
        {
            let mut passwords = std::io::BufReader::new(std::fs::File::open("input.txt").unwrap())
                .lines()
                .flatten();
    
            let n = count_valid(&mut passwords);
    
            println!("Answer is: {}", n);
        }
        {
            let mut passwords = std::io::BufReader::new(std::fs::File::open("input.txt").unwrap())
                .lines()
                .flatten();
    
            let n = count_valid_part_two(&mut passwords);
    
            println!("Answer to part two is: {}", n);
        }
    }
}

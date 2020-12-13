fn is_valid(passport_string: &str) -> bool {
    let mut want = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .collect::<std::collections::HashSet<_>>();

    for (key, _value) in passport_string.split_whitespace().map(|x| x.split_at(3)) {
        want.remove(&key);
    }

    want.is_empty()
}

fn is_valid_part_two(passport_string: &str) -> bool {
    let mut want = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .collect::<std::collections::HashSet<_>>();

    for (key, value) in passport_string.split_whitespace().map(|x| x.split_at(3)) {
        let value = value.trim_start_matches(':');
        let format_is_ok = match key {
            "byr" => { (1920..=2002).contains(&value.parse().unwrap_or(0)) }
            "iyr" => { (2010..=2020).contains(&value.parse().unwrap_or(0)) }
            "eyr" => { (2010..=2030).contains(&value.parse().unwrap_or(0)) }
            "hgt" => { 
                if let Some(digits) = value.strip_suffix("cm") {
                    (150..=193).contains(&digits.parse::<u32>().unwrap_or(0))
                } else if let Some(digits) = value.strip_suffix("in") {
                    (59..=76).contains(&digits.parse::<u32>().unwrap_or(0))
                } else {
                    false
                }
            }
            "hcl" => {
                value.len() == 7 && value.chars().next() == Some('#')
            }
            "ecl" => {
                match value {
                    "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                    _ => false,
                }
            }
            "pid" => {
                value.len() == 9 && value.parse::<u32>().is_ok()
            }
            _ => false,
        };

        if format_is_ok {
            want.remove(&key);
        }
    }

    want.is_empty()
}

#[allow(dead_code)]
enum Strategy {
    PartOne,
    PartTwo,
}

#[allow(dead_code)]
fn process_passports(passport_lines: impl Iterator<Item = String>, strategy: Strategy) -> usize {
    let mut valid = 0;
    let mut passport_lines = passport_lines.peekable();
    while let Some(_) = passport_lines.peek() {
        let is_valid = match strategy {
            Strategy::PartOne => is_valid(
                &passport_lines
                    .by_ref()
                    .take_while(|line| line != "")
                    .collect::<Vec<_>>()
                    .join(" "),
            ),
            Strategy::PartTwo => is_valid_part_two(
                &passport_lines
                    .by_ref()
                    .take_while(|line| line != "")
                    .collect::<Vec<_>>()
                    .join(" "),
            ),
        };

        if is_valid {
            valid += 1;
        }
    }

    valid
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufRead;

    #[test]
    fn example() {
        let terrain = vec![
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
            byr:1937 iyr:2017 cid:147 hgt:183cm"
                .to_string(),
            "".to_string(),
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
            hcl:#cfa07d byr:1929"
                .to_string(),
            "".to_string(),
            "hcl:#ae17e1 iyr:2013
            eyr:2024
            ecl:brn pid:760753108 byr:1931
            hgt:179cm"
                .to_string(),
            "".to_string(),
            "hcl:#cfa07d eyr:2025 pid:166559648
            iyr:2011 ecl:brn hgt:59in"
                .to_string(),
        ];

        let n = process_passports(terrain.into_iter(), Strategy::PartOne);

        println!("Answer is: {}", n);
    }

    #[test]
    fn real() {
        { // Part one
            let mut terrain = std::io::BufReader::new(std::fs::File::open("input.txt").unwrap())
                .lines()
                .flatten();
    
            let n = process_passports(&mut terrain, Strategy::PartOne);
    
            println!("Answer is: {}", n);
        }

        { // Part two
            let mut terrain = std::io::BufReader::new(std::fs::File::open("input.txt").unwrap())
            .lines()
            .flatten();
    
            let n = process_passports(&mut terrain, Strategy::PartTwo);
    
            println!("Answer to part two is: {}", n);
        }
    }
}

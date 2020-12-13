use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
enum Instruction {
    Jmp(i32),
    Acc(i32),
    Nop(i32),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (op, val) = s.split_at(3);
        let op = op.trim();
        let val = val.trim();

        let val = if let Some(val) = val.strip_prefix('+') {
            val.parse::<i32>().unwrap()
        } else if let Some(val) = val.strip_prefix('-') {
            -val.parse::<i32>().unwrap()
        } else {
            panic!("syntax error");
        };

        let instruction = match op {
            "jmp" => Instruction::Jmp(val),
            "acc" => Instruction::Acc(val),
            "nop" => Instruction::Nop(val),
            _ => panic!("syntax error"),
        };

        Ok(instruction)
    }
}

#[allow(dead_code)]
fn halting(input: impl Iterator<Item = String>) -> i32 {
    let mut acc = 0;
    let mut input = input
        .map(|i| (false, Instruction::from_str(&i).expect("syntax error")))
        .collect::<Vec<(bool, Instruction)>>();

    let mut idx = 0i32;
    loop {
        match &mut input[idx as usize] {
            (true, _) => return acc,
            (visited, Instruction::Jmp(val)) => {
                *visited = true;
                idx += *val;
            }
            (visited, Instruction::Acc(val)) => {
                *visited = true;
                acc += *val;
                idx += 1;
            }
            (visited, Instruction::Nop(_)) => {
                *visited = true;
                idx += 1;
            }
        }
    }
}

#[allow(dead_code)]
fn halting_part_two(input: impl Iterator<Item = String>) -> i32 {
    let mut tried = std::collections::HashSet::new();
    let input = input
        .map(|i| (false, Instruction::from_str(&i).expect("syntax error")))
        .collect::<Vec<(bool, Instruction)>>();

    'search: loop {
        let mut acc = 0;
        let mut idx = 0i32;
        let mut changed_instruction_idx = None;
        let mut input = input.clone();
        for _ in 0..3 {
            loop {
                if idx == input.len() as i32 {
                    return acc;
                }

                if let Some(changed_instruction_idx) = changed_instruction_idx {
                    if idx == changed_instruction_idx {
                        match &mut input[idx as usize] {
                            (true, _) => continue 'search,
                            (visited, Instruction::Jmp(_)) => {
                                *visited = true;
                                idx += 1;
                            }
                            (visited, Instruction::Acc(val)) => {
                                *visited = true;
                                acc += *val;
                                idx += 1;
                            }
                            (visited, Instruction::Nop(val)) => {
                                *visited = true;
                                idx += *val;
                            }
                        }
                    }
                }

                match &mut input[idx as usize] {
                    (true, _) => continue 'search,
                    (visited, Instruction::Jmp(val)) => {
                        if changed_instruction_idx.is_none() && !tried.contains(&idx) {
                            changed_instruction_idx = Some(idx);
                            tried.insert(idx);
                            continue;
                        }
                        *visited = true;
                        idx += *val;
                    }
                    (visited, Instruction::Acc(val)) => {
                        *visited = true;
                        acc += *val;
                        idx += 1;
                    }
                    (visited, Instruction::Nop(_)) => {
                        if changed_instruction_idx.is_none() && !tried.contains(&idx) {
                            changed_instruction_idx = Some(idx);
                            tried.insert(idx);
                            continue;
                        };
                        *visited = true;
                        idx += 1;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufRead;

    #[test]
    fn example() {
        let input = vec![
            "nop +0".to_string(),
            "acc +1".to_string(),
            "jmp +4".to_string(),
            "acc +3".to_string(),
            "jmp -3".to_string(),
            "acc -99".to_string(),
            "acc +1".to_string(),
            "jmp -4".to_string(),
            "acc +6".to_string(),
        ];

        let n = halting(input.clone().into_iter());

        println!("Example answer is: {:?}", n);

        let n = halting_part_two(input.into_iter());

        println!("Example answer part two is: {:?}", n);
    }

    #[test]
    fn real() {
        {
            let mut input = std::io::BufReader::new(std::fs::File::open("input.txt").unwrap())
                .lines()
                .flatten();

            let n = halting(&mut input);

            println!("Answer is: {:?}", n);
        }
        {
            let mut input = std::io::BufReader::new(std::fs::File::open("input.txt").unwrap())
                .lines()
                .flatten();

            let n = halting_part_two(&mut input);

            println!("Answer to part two is: {:?}", n);
        }
    }
}

#[allow(dead_code)]
fn get_sum(items: impl Iterator<Item = String>) -> usize {
    let mut items = items.peekable();
    let mut sum = 0;
    while let Some(_) = items.peek() {
        let answers = items.by_ref().take_while(|line| line != "");

        let mut hs = std::collections::HashSet::new();
        for answer in answers {
            answer.chars().for_each(|c| {
                hs.insert(c);
            });
        }

        sum += hs.len();
    }
    sum
}

#[allow(dead_code)]
fn get_sum_part_two(items: impl Iterator<Item = String>) -> usize {
    let mut items = items.peekable();
    let mut sum = 0;
    while let Some(_) = items.peek() {
        let answers = items.by_ref().take_while(|line| line != "");

        let mut hm = std::collections::HashMap::new();

        let mut total_people = 0;
        for answer in answers {
            total_people += 1;
            answer.chars().for_each(|c| {
                *hm.entry(c).or_insert(0) += 1;
            });
        }

        for (_, count) in hm {
            if count == total_people {
                sum += 1;
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufRead;

    #[test]
    fn example() {
        let input = vec![
            "abc".to_string(),
            "".to_string(),
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "".to_string(),
            "ab".to_string(),
            "ac".to_string(),
            "".to_string(),
            "a".to_string(),
            "a".to_string(),
            "a".to_string(),
            "a".to_string(),
            "".to_string(),
            "b".to_string(),
        ];

        let n = get_sum(input.clone().into_iter());

        println!("Example answer is: {:?}", n);

        let n = get_sum_part_two(input.into_iter());

        println!("Example answer to part two is: {:?}", n);
    }

    #[test]
    fn real() {
        {
            let mut input = std::io::BufReader::new(std::fs::File::open("input.txt").unwrap())
                .lines()
                .flatten();
    
            let n = get_sum(&mut input);
    
            println!("Answer is: {:?}", n);
        }
        {
            let mut input = std::io::BufReader::new(std::fs::File::open("input.txt").unwrap())
            .lines()
            .flatten();
        
            let n = get_sum_part_two(&mut input);
    
            println!("Answer to part two is: {:?}", n);
        }

    }
}

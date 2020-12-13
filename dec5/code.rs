#[allow(dead_code)]
fn get_seats(boarding_passes: impl Iterator<Item = String>) -> impl Iterator<Item = u32> {
    boarding_passes.map(|pass| {
        let (mut row_start, mut row_end) = (0, 127);
        let (mut col_start, mut col_end) = (0, 8);

        for c in pass.chars() {
            match c {
                'F' => {
                    row_end -= (row_end - row_start) / 2;
                }
                'B' => {
                    let rounding = if (row_end - row_start) % 2 == 0 { 0 } else { 1 };
                    row_start += (row_end - row_start) / 2 + rounding;
                }
                'L' => {
                    col_end -= (col_end - col_start) / 2;
                }
                'R' => {
                    let rounding = if (col_end - col_start) % 2 == 0 { 0 } else { 1 };
                    col_start += (col_end - col_start) / 2 + rounding;
                }
                inp => panic!("bad input {}", inp),
            }
        }
        row_start * 8 + col_start
    })
}

#[allow(dead_code)]
fn get_seat_part_two(boarding_passes: impl Iterator<Item = String>) -> u32 {
    let occupied_seats = get_seats(boarding_passes)
        .map(|seat| {
            (
                (seat & 0b1111_1111_1111_1000) / 8,
                seat & 0b0000_0000_0000_0111,
            )
        })
        .fold(std::collections::HashMap::new(), |mut hm, it| {
            hm.entry(it.0).or_insert(Vec::new()).push(it.1);
            hm
        });

    for (row, occupied) in &occupied_seats {
        if *row == 0 || *row == 127 {
            continue;
        }

        if occupied.len() == 7 {
            if let Some(_occupied @ 8..=8) = occupied_seats.get(&(row - 1)).map(|x| x.len()) {
                if let Some(_occupied @ 8..=8) = occupied_seats.get(&(row + 1)).map(|x| x.len()) {
                    for col in 0..7 {
                        if !occupied.contains(&col) {
                            return row * 8 + col;
                        }
                    }
                }
            }
        }
    }

    panic!("oh no");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufRead;

    #[test]
    fn example() {
        let boardingpasses = vec![
            "BFFFBBFRRR".to_string(),
            "FFFBBBFRRR".to_string(),
            "BBFFBBFRLL".to_string(),
        ];

        let n = get_seats(boardingpasses.clone().into_iter()).max();

        println!("Answer is: {:?}", n);
    }

    #[test]
    fn real() {
        {
            let mut boardingpasses =
                std::io::BufReader::new(std::fs::File::open("input.txt").unwrap())
                    .lines()
                    .flatten();

            let n = get_seats(&mut boardingpasses).max();

            println!("Answer is: {:?}", n);
        }
        {
            let boardingpasses = std::io::BufReader::new(std::fs::File::open("input.txt").unwrap())
                .lines()
                .flatten();
            let n = get_seat_part_two(boardingpasses.into_iter());

            println!("Answer to part two is: {:?}", n);
        }
    }
}

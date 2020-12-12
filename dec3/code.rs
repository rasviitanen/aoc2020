#[allow(dead_code)]
fn count_trees(terrain: impl Iterator<Item = String>) -> usize {
    let mut trees = 0;
    let mut x = 0;
    let slope = 3;

    for row in terrain {
        let len = row.len();
        let standing_on = row.chars().nth(x % len);

        if let Some('#') = standing_on {
            trees += 1;
        }

        x += slope;
    }

    trees
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufRead;

    #[test]
    fn example_repeating() {
        let terrain = vec![
            "..##.......".to_string(),
            "#...#...#..".to_string(),
            ".#....#..#.".to_string(),
            "..#.#...#.#".to_string(),
            ".#...##..#.".to_string(),
            "..#.##.....".to_string(),
            ".#.#.#....#".to_string(),
            ".#........#".to_string(),
            "#.##...#...".to_string(),
            "#...##....#".to_string(),
            ".#..#...#.#".to_string(),
        ];

        let n = count_trees(terrain.into_iter());

        println!("Answer is: {}", n);
    }

    #[test]
    fn example() {
        let terrain = vec![
            "..##.........##.........##.........##.........##.........##.......".to_string(),
            "#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..".to_string(),
            ".#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.".to_string(),
            "..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#".to_string(),
            ".#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.".to_string(),
            "..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....".to_string(),
            ".#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#".to_string(),
            ".#........#.#........#.#........#.#........#.#........#.#........#".to_string(),
            "#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...".to_string(),
            "#...##....##...##....##...##....##...##....##...##....##...##....#".to_string(),
            ".#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#".to_string(),
        ];

        let n = count_trees(terrain.into_iter());

        println!("Answer is: {}", n);
    }

    #[test]
    fn real() {
        let mut terrain = std::io::BufReader::new(std::fs::File::open("input.txt").unwrap())
            .lines()
            .flatten();

        let n = count_trees(&mut terrain);

        println!("Answer is: {}", n);
    }
}

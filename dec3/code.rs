#[allow(dead_code)]
fn count_trees(terrain: impl Iterator<Item = String>, slope: (usize, usize)) -> usize {
    let mut trees = 0;
    let mut x = 0;
    let (slope_x, slope_y) = slope;

    for row in terrain.step_by(slope_y) {
        let len = row.len();
        let standing_on = row.chars().nth(x % len);

        if let Some('#') = standing_on {
            trees += 1;
        }

        x += slope_x;
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

        let n = count_trees(terrain.clone().into_iter(), (3, 1));

        println!("Answer is: {}", n);

        let n: usize = [
            count_trees(terrain.clone().into_iter(), (1, 1)),
            count_trees(terrain.clone().into_iter(), (3, 1)),
            count_trees(terrain.clone().into_iter(), (5, 1)),
            count_trees(terrain.clone().into_iter(), (7, 1)),
            count_trees(terrain.clone().into_iter(), (1, 2)),
        ]
        .iter()
        .product();

        println!("Answer to part two is: {}", n);
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

        let n = count_trees(terrain.into_iter(), (3, 1));

        println!("Answer is: {}", n);
    }

    #[test]
    fn real() {
        let mut terrain = std::io::BufReader::new(std::fs::File::open("input.txt").unwrap())
            .lines()
            .flatten();

        let n = count_trees(&mut terrain, (3, 1));

        println!("Answer is: {}", n);

        let n: usize = [
            count_trees(
                &mut std::io::BufReader::new(std::fs::File::open("input.txt").unwrap())
                    .lines()
                    .flatten(),
                (1, 1),
            ),
            count_trees(
                &mut std::io::BufReader::new(std::fs::File::open("input.txt").unwrap())
                    .lines()
                    .flatten(),
                (3, 1),
            ),
            count_trees(
                &mut std::io::BufReader::new(std::fs::File::open("input.txt").unwrap())
                    .lines()
                    .flatten(),
                (5, 1),
            ),
            count_trees(
                &mut std::io::BufReader::new(std::fs::File::open("input.txt").unwrap())
                    .lines()
                    .flatten(),
                (7, 1),
            ),
            count_trees(
                &mut std::io::BufReader::new(std::fs::File::open("input.txt").unwrap())
                    .lines()
                    .flatten(),
                (1, 2),
            ),
        ]
        .iter()
        .product();

        println!("Answer to part two is: {}", n);
    }
}

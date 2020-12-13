use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

fn hash_bag(adj: &str, color: &str) -> usize {
    let mut hasher = DefaultHasher::new();
    (adj, color).hash(&mut hasher);
    hasher.finish() as usize
}

#[allow(dead_code)]
fn calc_bags(input: impl Iterator<Item = String>) -> usize {
    let mut reachable = std::collections::HashMap::new();

    for bag in input {
        match bag.split_whitespace().collect::<Vec<_>>().as_slice() {
            [_, _, _, "contain", "no", ..] => { /* oh well.. */ }
            [bag_a_adj, bag_a_color, "bags", "contain", bags @ ..] => {
                let mut bags = bags.iter();
                while let (Some(_bag_n), Some(bag_adj), Some(bag_color)) =
                    (bags.next(), bags.next(), bags.next())
                {
                    reachable
                        .entry(hash_bag(bag_adj, bag_color))
                        .or_insert_with(Vec::new)
                        .push(hash_bag(bag_a_adj, bag_a_color));
                    bags.next(); // pop "bags,""
                }
            }
            _ => panic!("incorrect input assumptions ya donkey"),
        }
    }

    fn recursive_search(
        begin_at: usize,
        can_contain: &mut HashSet<usize>,
        reachable: &mut HashMap<usize, Vec<usize>>,
    ) {
        if let Some(this_bag_can_reach) = reachable.remove(&begin_at) {
            for reachable_bag in this_bag_can_reach {
                can_contain.insert(reachable_bag);
                recursive_search(reachable_bag, can_contain, reachable);
            }
        }
    }

    let mut bag_candidates = std::collections::HashSet::new();
    recursive_search(
        hash_bag("shiny", "gold"),
        &mut bag_candidates,
        &mut reachable,
    );

    bag_candidates.len()
}

#[allow(dead_code)]
fn calc_bags_part_two(input: impl Iterator<Item = String>) -> usize {
    let mut contains = std::collections::HashMap::new();
    let mut contain_counts = std::collections::HashMap::new(); // { bag: {contained_bag: count, ...}, ... }

    for bag in input {
        match bag.split_whitespace().collect::<Vec<_>>().as_slice() {
            [_, _, _, "contain", "no", ..] => { /* oh well.. */ }
            [bag_a_adj, bag_a_color, "bags", "contain", bags @ ..] => {
                let mut bags = bags.iter();
                while let (Some(bag_n), Some(bag_adj), Some(bag_color)) =
                    (bags.next(), bags.next(), bags.next())
                {
                    contains
                        .entry(hash_bag(bag_a_adj, bag_a_color))
                        .or_insert_with(Vec::new)
                        .push(hash_bag(bag_adj, bag_color));
                    contain_counts
                        .entry(hash_bag(bag_a_adj, bag_a_color))
                        .or_insert_with(HashMap::new)
                        .insert(
                            hash_bag(bag_adj, bag_color),
                            bag_n.parse::<usize>().unwrap(),
                        );
                    bags.next(); // pop "bags,""
                }
            }
            _ => panic!("incorrect input assumptions ya donkey"),
        }
    }

    let mut bags_to_search = std::collections::LinkedList::new();
    let mut total = 0;
    bags_to_search.push_back((1, hash_bag("shiny", "gold")));
    while let Some((multiplier, bag_hash)) = bags_to_search.pop_front() {
        if let Some(bag) = contains.get(&bag_hash) {
            for inner_bag in bag {
                let count = contain_counts
                    .get(&bag_hash)
                    .and_then(|b| b.get(&inner_bag))
                    .unwrap();
                total += multiplier * count;
                bags_to_search.push_back((multiplier * *count, *inner_bag));
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufRead;

    #[test]
    fn example() {
        let input = vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string(),
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string(),
            "bright white bags contain 1 shiny gold bag.".to_string(),
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".to_string(),
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string(),
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".to_string(),
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_string(),
            "faded blue bags contain no other bags.".to_string(),
            "dotted black bags contain no other bags.".to_string(),
        ];

        let n = calc_bags(input.clone().into_iter());

        println!("Example answer is: {:?}", n);

        let n = calc_bags_part_two(input.clone().into_iter());

        println!("Example answer part two is: {:?}", n);
    }

    #[test]
    fn real() {
        {
            let mut input = std::io::BufReader::new(std::fs::File::open("input.txt").unwrap())
                .lines()
                .flatten();

            let n = calc_bags(&mut input);

            println!("Answer is: {:?}", n);
        }
        {
            let mut input = std::io::BufReader::new(std::fs::File::open("input.txt").unwrap())
                .lines()
                .flatten();

            let n = calc_bags_part_two(&mut input);

            println!("Answer part two is: {:?}", n);
        }
    }
}

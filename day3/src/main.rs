use itertools::Itertools;
use std::collections::HashSet;

fn priority_from_item(item: &char) -> u32 {
    match *item {
        'a'..='z' => *item as u32 - 'a' as u32 + 1,
        'A'..='Z' => *item as u32 - 'A' as u32 + 27,
        _ => panic!(),
    }
}

fn main() {
    let input = include_str!("input");

    let total: u32 = input
        .lines()
        .map(|line| {
            let length = line.chars().count();
            let mut chars = line.chars();
            chars
                .by_ref()
                .take(length / 2)
                .collect::<HashSet<char>>()
                .intersection(&chars.collect())
                .map(priority_from_item)
                .sum::<u32>()
        })
        .sum();

    println!("Part 1: {}", total);

    let total: u32 = input
        .lines()
        .collect_vec()
        .chunks(3)
        .map(|groups| {
            groups
                .iter()
                .map(|group| group.chars().by_ref().collect::<HashSet<char>>())
                .reduce(|acc, e| &acc & &e)
                .unwrap()
                .iter()
                .map(priority_from_item)
                .next()
                .unwrap()
        })
        .sum();

    println!("Part 2: {}", total);
}

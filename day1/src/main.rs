use itertools::Itertools;

fn main() {
    let input = include_str!("input");

    let inventories = input.split("\n\n")
        .map(|inventory| {
            inventory
                .lines()
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .sorted()
        .rev()
        .collect_vec();

    println!("Part 1: {}", inventories.first().unwrap());

    println!("Part 2: {}", inventories.iter().take(3).sum::<u32>());
}

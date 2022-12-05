use std::collections::BTreeMap;

use itertools::Itertools;
use regex::Regex;

fn main() {
    let input = include_str!("input");

    let regex = Regex::new(r"(?m)^\s*([0-9]\s*)+\s*$").unwrap();

    let (crates, instructions) = regex.split(input).collect_tuple().unwrap();

    let mut stacks = BTreeMap::<usize, Vec<char>>::new();
    for line in crates.lines() {
        for (index, id) in line.chars().enumerate() {
            let index = index + 3;
            if index % 4 == 0 && id != ' ' {
                stacks.entry(index / 4).or_default().insert(0, id)
            }
        }
    }

    let stacks_copy = stacks.clone();

    let instructions_regex =
        Regex::new(r"move (?P<amount>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();

    for line in instructions.lines() {
        for (amount, from, to) in
            instructions_regex
                .captures_iter(line)
                .map(|captures| -> (usize, usize, usize) {
                    (
                        captures["amount"].parse().unwrap(),
                        captures["from"].parse().unwrap(),
                        captures["to"].parse().unwrap(),
                    )
                })
        {
            let from_stack = stacks.get_mut(&from).unwrap();
            let grabbed = &mut from_stack.split_off(from_stack.len() - amount);
            grabbed.reverse();
            let to_stack = stacks.get_mut(&to).unwrap();
            to_stack.append(grabbed);
        }
    }

    let output = stacks
        .iter_mut()
        .map(|(_id, stack)| stack.pop().unwrap())
        .join("");

    println!("Part 1: {}", output);

    let mut stacks = stacks_copy;

    for line in instructions.lines() {
        for (amount, from, to) in
            instructions_regex
                .captures_iter(line)
                .map(|captures| -> (usize, usize, usize) {
                    (
                        captures["amount"].parse().unwrap(),
                        captures["from"].parse().unwrap(),
                        captures["to"].parse().unwrap(),
                    )
                })
        {
            let from_stack = stacks.get_mut(&from).unwrap();
            let grabbed = &mut from_stack.split_off(from_stack.len() - amount);
            let to_stack = stacks.get_mut(&to).unwrap();
            to_stack.append(grabbed);
        }
    }

    let output = stacks
        .iter_mut()
        .map(|(_id, stack)| stack.pop().unwrap())
        .join("");

    println!("Part 2: {}", output);
}

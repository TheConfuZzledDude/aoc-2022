use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

use itertools::Itertools;

fn main() {
    let input = include_str!("input");

    let mut current_path = PathBuf::new();
    let mut tree = BTreeMap::new();

    for line in input
        .lines()
        .map(|line| line.split_whitespace().collect_vec())
    {
        match line[0] {
            "$" => {
                if line[1] == "cd" {
                    match line[2] {
                        ".." => {
                            current_path.pop();
                        }
                        dir => {
                            current_path.push(dir);
                            tree.insert(current_path.clone(), 0usize);
                        }
                    }
                }
            }
            "dir" => {}
            size => {
                for ancestor in current_path.ancestors() {
                    *tree.get_mut(ancestor).unwrap() += size.parse::<usize>().unwrap();
                }
            }
        }
    }

    let total = tree.values().fold(
        0,
        |acc, &size| if size <= 100_000 { acc + size } else { acc },
    );

    println!("Part 1: {total}");

    let unused = 70_000_000 - tree.get(Path::new("/")).unwrap();
    let required = 30_000_000 - unused;

    let smallest = tree
        .values()
        .filter(|&&size| size >= required)
        .k_smallest(1)
        .next()
        .unwrap();

    println!("Part 2: {smallest}");
}

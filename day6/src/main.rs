use itertools::Itertools;

fn main() {
    let input = include_str!("input");

    let position = input
        .chars()
        .collect_vec()
        .windows(4)
        .position(|code| code.iter().all_unique()).unwrap() + 4;

    println!("Part 1: {}", position);

    let position = input
        .chars()
        .collect_vec()
        .windows(14)
        .position(|code| code.iter().all_unique()).unwrap() + 14;

    println!("Part 2: {}", position);
}

use itertools::Itertools;

fn main() {
    let input = include_str!("input");

    let position = input
        .chars()
        .collect_vec()
        .windows(4)
        .find_position(|code| code.iter().all_unique()).unwrap().0 + 4;

    println!("Part 1: {}", position);

    let position = input
        .chars()
        .collect_vec()
        .windows(14)
        .find_position(|code| code.iter().all_unique()).unwrap().0 + 14;

    println!("Part 2: {}", position);
}

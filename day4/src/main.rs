fn main() {
    let input = include_str!("input");

    let total = input
        .lines()
        .filter(|line| {
            let mut pair = line.split(',').map(|assignment| {
                let mut assignment = assignment
                    .split('-')
                    .map(str::parse::<u32>)
                    .map(Result::unwrap);

                assignment.next().unwrap()..=assignment.next().unwrap()
            });

            let first = pair.next().unwrap();
            let second = pair.next().unwrap();

            first.contains(second.start()) && first.contains(second.end())
                || second.contains(first.start()) && second.contains(first.end())
        })
        .count();

    println!("Part 1: {total}");

    let total = input
        .lines()
        .filter(|line| {
            let mut pair = line.split(',').map(|assignment| {
                let mut assignment = assignment
                    .split('-')
                    .map(str::parse::<u32>)
                    .map(Result::unwrap);

                assignment.next().unwrap()..=assignment.next().unwrap()
            });

            let first = pair.next().unwrap();
            let second = pair.next().unwrap();

            first.contains(second.start())
                || first.contains(second.end())
                || second.contains(first.start())
                || second.contains(first.end())
        })
        .count();

    println!("Part 2: {total}");
}

use std::ops::Rem;

use itertools::Itertools;

fn main() {
    let input = include_str!("input");

    let mut history = Vec::new();
    let mut crt = Vec::new();
    let mut cycle = 0i32;
    let mut x = 1i32;

    macro_rules! process_cycle {
        () => {
            history.push(x);
            crt.push(if cycle.rem(40).abs_diff(x) <= 1 {
                '#'
            } else {
                '.'
            });
            cycle += 1;
        };
    }

    for instruction in input.lines() {
        let mut instruction = instruction.split_whitespace();
        match instruction.next().unwrap() {
            "noop" => {
                process_cycle!();
            }
            "addx" => {
                process_cycle!();
                process_cycle!();
                x += instruction.next().unwrap().parse::<i32>().unwrap();
            }
            _ => panic!(),
        };
    }

    let total: i32 = [20, 60, 100, 140, 180, 220]
        .into_iter()
        .map(|idx| idx * history[usize::try_from(idx).unwrap() - 1])
        .sum();

    println!("Part 1: {total}");

    println!("Part 2:");

    for line in crt.chunks(40) {
        let line = line.iter().join("");
        println!("{line}");
    }
}

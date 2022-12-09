#![feature(array_windows)]

use std::collections::HashSet;

use itertools::Itertools;
use maplit::hashset;
use nalgebra::{coordinates::XY, vector, Vector2};

fn main() {
    let input = include_str!("input");

    let moves = input.lines().map(|command| {
        let (direction, amount) = command.split_whitespace().collect_tuple().unwrap();

        (
            amount.parse::<i32>().unwrap(),
            match direction {
                "U" => vector![0, 1],
                "D" => vector![0, -1],
                "R" => vector![1, 0],
                "L" => vector![-1, 0],
                _ => panic!("Not a valid direction"),
            },
        )
    });

    let mut visited: HashSet<Vector2<i32>> = hashset! { vector![0,0] };

    let [mut head, mut tail] = [vector![0, 0i32]; 2];

    for (amount, direction) in moves.clone() {
        for _ in 0..amount {
            head += direction;

            let XY { x, y } = *(head - tail);

            if x.abs().max(y.abs()) > 1 {
                tail += vector![x.signum(), y.signum()];
            }

            visited.insert(tail);
        }
    }

    let total = visited.len();

    println!("Part 1: {total}");

    let mut visited: HashSet<Vector2<i32>> = hashset! { vector![0,0] };

    let mut knots = [vector![0, 0i32]; 10];

    for (amount, direction) in moves {
        for _ in 0..amount {
            knots[0] += direction;

            for (idx, [head, tail]) in knots.clone().array_windows().enumerate() {
                let XY { x, y } = *(head - tail);

                if x.abs().max(y.abs()) > 1 {
                    knots[idx + 1] += vector![x.signum(), y.signum()];
                }
            }

            visited.insert(knots[9]);
        }
    }

    let total = visited.len();

    println!("Part 2: {total}");
}

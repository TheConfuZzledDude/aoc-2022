use itertools::Itertools;
use ndarray::prelude::*;

fn visibility_scanner(tallest: &mut Option<usize>, tree_height: usize) -> usize {
    match tallest {
        Some(height) if tree_height <= *height => 0,
        _ => {
            *tallest = Some(tree_height);
            1
        }
    }
}

fn main() {
    let input = include_str!("input");

    let mut lines = input.lines().peekable();
    let length = lines.peek().unwrap().chars().count();

    let array = Array::from_shape_vec(
        [length; 2],
        lines
            .flat_map(|line| line.chars().map(|c| c.to_string().parse().unwrap()))
            .collect(),
    )
    .unwrap();

    let (horizontal, vertical) = [array.rows(), array.columns()]
        .into_iter()
        .map(|lanes| {
            Array::from_shape_vec(
                [length; 2],
                lanes
                    .into_iter()
                    .flat_map(|lane| {
                        let (vis_left, vis_right) = [lane, lane.slice_move(s![..;-1])]
                            .iter()
                            .map(|lane| {
                                lane.iter()
                                    .copied()
                                    .scan(None, |tallest, tree_height| {
                                        Some(visibility_scanner(tallest, tree_height))
                                    })
                                    .collect::<Array1<_>>()
                            })
                            .collect_tuple()
                            .unwrap();
                        vis_left | vis_right.slice_move(s![..;-1])
                    })
                    .collect(),
            )
            .unwrap()
        })
        .collect_tuple()
        .unwrap();

    let total = (horizontal | vertical.reversed_axes()).sum();

    println!("Part 1: {total}");

    let total: usize = array
        .indexed_iter()
        .map(|((x, y), &treehouse_height)| {
            [
                array.slice(s![x, ..y; -1]),
                array.slice(s![x, y + 1..]),
                array.slice(s![..x; -1, y]),
                array.slice(s![x + 1.., y]),
            ]
            .into_iter()
            .map(|trees| {
                trees
                    .into_iter()
                    .scan(false, |stop, &tree_height| {
                        if *stop {
                            None
                        } else {
                            if tree_height >= treehouse_height {
                                *stop = true;
                            }
                            Some(())
                        }
                    })
                    .count()
            })
            .product()
        })
        .max()
        .unwrap();

    println!("Part 2: {total}");
}

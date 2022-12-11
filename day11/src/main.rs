use std::{
    collections::VecDeque,
    mem::take,
    ops::{Add, Mul},
};

use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Mul,
}

#[derive(Debug, Clone, Copy)]
enum Operand {
    Old,
    Val(i64),
}

#[derive(Debug, Clone, Copy)]
struct Operation {
    operator: Operator,
    operand: Operand,
}

#[derive(Debug, Clone, Copy)]
struct Test {
    divisor: i64,
    true_idx: usize,
    false_idx: usize,
}

#[derive(Debug, Clone)]
struct Monkey {
    operation: Operation,
    inventory: VecDeque<i64>,
    test: Test,
}

fn main() {
    let input = include_str!("input");

    let item_regex = Regex::new(r"(?m)^\s*Starting items: (?P<items>\d.*)$").unwrap();
    let operation_regex =
        Regex::new(r"(?m)^\s*Operation: new = old (?P<operator>[\+\*]) (?P<operand>(old)|(\d+))$")
            .unwrap();

    let test_regex = Regex::new(
        r"Test: divisible by (?P<divisor>\d+)
    If true: throw to monkey (?P<true_idx>\d+)
    If false: throw to monkey (?P<false_idx>\d+)",
    )
    .unwrap();

    let mut monkeys = input
        .split("\n\n")
        .map(|monkey| {
            let inventory = item_regex.captures(monkey).unwrap()["items"]
                .split(", ")
                .map(|item| item.parse().unwrap())
                .collect();

            let operation_captures = operation_regex.captures(monkey).unwrap();
            let operator = match &operation_captures["operator"] {
                "+" => Operator::Add,
                "*" => Operator::Mul,
                _ => panic!(),
            };
            let operand = match &operation_captures["operand"] {
                "old" => Operand::Old,
                val => Operand::Val(val.parse().unwrap()),
            };
            let operation = Operation { operator, operand };

            let test_captures = test_regex.captures(monkey).unwrap();

            let test = Test {
                divisor: test_captures["divisor"].parse().unwrap(),
                true_idx: test_captures["true_idx"].parse().unwrap(),
                false_idx: test_captures["false_idx"].parse().unwrap(),
            };

            Monkey { operation, inventory, test }
        })
        .collect::<Vec<_>>();

    let monkeys_clone = monkeys.clone();

    let mut inspect_counters = [0; 8];

    for _ in 0..20 {
        for monkey_idx in 0..monkeys.len() {
            for item in take(&mut monkeys[monkey_idx].inventory) {
                inspect_counters[monkey_idx] += 1;
                let monkey = &monkeys[monkey_idx];
                let value = match monkey.operation.operator {
                    Operator::Add => Add::add,
                    Operator::Mul => Mul::mul,
                }(
                    item,
                    match monkey.operation.operand {
                        Operand::Old => item,
                        Operand::Val(x) => x,
                    },
                ) / 3;

                let to_idx = match value % monkey.test.divisor {
                    0 => monkey.test.true_idx,
                    _ => monkey.test.false_idx,
                };

                monkeys[to_idx].inventory.push_back(value);
            }
        }
    }

    let total: i64 = inspect_counters.iter().sorted().rev().take(2).product();

    println!("Part 1: {total}");

    let mut monkeys = monkeys_clone;
    let mut inspect_counters = [0; 8];

    let lcm: i64 = monkeys.iter().map(|monkey| monkey.test.divisor).product();

    for _ in 0..10000 {
        for monkey_idx in 0..monkeys.len() {
            for item in take(&mut monkeys[monkey_idx].inventory) {
                inspect_counters[monkey_idx] += 1;
                let monkey = &monkeys[monkey_idx];
                let value = match monkey.operation.operator {
                    Operator::Add => Add::add,
                    Operator::Mul => Mul::mul,
                }(
                    item % lcm,
                    match monkey.operation.operand {
                        Operand::Old => item,
                        Operand::Val(x) => x,
                    },
                );

                let to_idx = match value % monkey.test.divisor {
                    0 => monkey.test.true_idx,
                    _ => monkey.test.false_idx,
                };

                monkeys[to_idx].inventory.push_back(value);
            }
        }
    }

    let total: i64 = inspect_counters.iter().sorted().rev().take(2).product();
    println!("Part 2: {total}");
}

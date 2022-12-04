use std::cmp::Ordering;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self.value() - other.value()).rem_euclid(3) {
            0 => Some(Ordering::Equal),
            1 => Some(Ordering::Greater),
            2 => Some(Ordering::Less),
            _ => None,
        }
    }
}

impl Shape {
    pub fn value(self) -> i32 {
        self as i32
    }

    pub fn from_value(val: i32) -> Shape {
        match val {
            1 => Self::Rock,
            2 => Self::Paper,
            3 => Self::Scissors,
            _ => panic!(),
        }
    }

    pub fn score(&self, opponent: Self) -> i32 {
        self.value()
            + match self.partial_cmp(&opponent).unwrap() {
                Ordering::Greater => 6,
                Ordering::Equal => 3,
                Ordering::Less => 0,
            }
    }

    pub fn from_opponent_choice(choice: char) -> Shape {
        match choice {
            'A' => Self::Rock,
            'B' => Self::Paper,
            'C' => Self::Scissors,
            _ => panic!("Invalid opponent choice"),
        }
    }

    pub fn from_player_choice(choice: char) -> Shape {
        match choice {
            'X' => Self::Rock,
            'Y' => Self::Paper,
            'Z' => Self::Scissors,
            _ => panic!("Invalid player choice"),
        }
    }

    pub fn from_result(opponent: Shape, outcome: char) -> Shape {
        match outcome {
            'X' => Shape::from_value((opponent.value() - 2).rem_euclid(3) + 1),
            'Y' => opponent,
            'Z' => Shape::from_value(opponent.value() % 3 + 1),
            _ => panic!(),
        }
    }
}

fn main() {
    let input = include_str!("input");

    let total: i32 = input
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let opponent = Shape::from_opponent_choice(chars.next().unwrap());
            Shape::from_player_choice(chars.nth(1).unwrap()).score(opponent)
        })
        .sum();

    println!("Part 1: {}", total);

    let total: i32 = input
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let opponent = Shape::from_opponent_choice(chars.next().unwrap());
            let choice = Shape::from_result(opponent, chars.nth(1).unwrap());
            choice.score(opponent)
        })
        .sum();

    println!("Part 2: {}", total);
}

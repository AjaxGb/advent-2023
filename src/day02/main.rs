use std::{num::NonZeroU32, str::FromStr};
use thiserror::Error;

use advent_2023::simple_parse;

#[derive(Debug, Default, Clone, Copy)]
struct Colors<T> {
    red: T,
    green: T,
    blue: T,
}

impl<T> Colors<T> {
    pub fn zip<U>(self, other: Colors<U>) -> Colors<(T, U)> {
        Colors {
            red: (self.red, other.red),
            green: (self.green, other.green),
            blue: (self.blue, other.blue),
        }
    }

    pub fn map<F, U>(self, op: F) -> Colors<U>
    where
        F: Fn(T) -> U,
    {
        Colors {
            red: op(self.red),
            green: op(self.green),
            blue: op(self.blue),
        }
    }

    pub fn into_array(self) -> [T; 3] {
        let Colors { red, green, blue } = self;
        [red, green, blue]
    }
}

impl<T> IntoIterator for Colors<T> {
    type Item = T;

    type IntoIter = std::array::IntoIter<T, 3>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_array().into_iter()
    }
}

#[derive(Debug, Error)]
enum ParseCountsError {
    #[error("text was not formatted correctly")]
    WrongFormat,
    #[error("count was invalid")]
    InvalidCount(#[from] std::num::ParseIntError),
    #[error("color was invalid")]
    InvalidColor,
    #[error("color was used more than once")]
    DuplicateColor,
}

impl FromStr for Colors<u32> {
    type Err = ParseCountsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for entry in s.split(", ") {
            let (count, color) = entry.split_once(' ').ok_or(Self::Err::WrongFormat)?;
            let count: NonZeroU32 = count.parse()?;

            let slot = match color {
                "red" => &mut red,
                "green" => &mut green,
                "blue" => &mut blue,
                _ => return Err(Self::Err::InvalidColor),
            };
            if *slot != 0 {
                return Err(Self::Err::DuplicateColor);
            }
            *slot = count.get();
        }
        Ok(Self { red, green, blue })
    }
}

fn main() {
    let p1_actual = Colors::<u32> {
        red: 12,
        green: 13,
        blue: 14,
    };

    let mut result_p1 = 0;
    let mut result_p2 = 0;

    for line in include_str!("input.txt").lines() {
        let (game_id, draws) = simple_parse!(line => "Game ", parse u32, ": ", str).unwrap();

        let mut p1_possible = true;
        let mut p2_min = Colors::<u32>::default();

        for seen in draws.split("; ") {
            let seen: Colors<u32> = seen.parse().unwrap();

            p1_possible = p1_possible
                && seen
                    .zip(p1_actual)
                    .into_iter()
                    .all(|(seen, actual)| seen <= actual);
            p2_min = p2_min.zip(seen).map(|(a, b)| a.max(b));
        }

        if p1_possible {
            result_p1 += game_id;
        }

        let p2_power: u32 = p2_min.into_iter().product();
        result_p2 += p2_power;
    }

    println!("Part 1: {result_p1}");
    println!("Part 2: {result_p2}");
}

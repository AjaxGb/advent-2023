use std::collections::HashMap;

use regex::Regex;

struct Num {
    value: u32,
    min_x: i32,
    max_x: i32,
    y: i32,
}

impl Num {
    pub fn check_border<F>(&self, mut check: F)
    where
        F: FnMut(i32, i32),
    {
        let border_min_x = self.min_x - 1;
        let border_max_x = self.max_x + 1;
        let y = self.y;
        for x in border_min_x..=border_max_x {
            check(x, y - 1);
            check(x, y + 1);
        }
        check(border_min_x, y);
        check(border_max_x, y);
    }
}

struct Part {
    symbol: char,
    labels: Vec<u32>,
}

fn main() {
    let part_or_num = Regex::new(r"\d+|[^.]").unwrap();

    let input = include_str!("input.txt");

    let mut nums = Vec::new();
    let mut parts = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        let y = y as i32;
        for item in part_or_num.find_iter(line) {
            if item.as_str().starts_with(|c: char| c.is_ascii_digit()) {
                let value = item.as_str().parse().unwrap();
                let min_x = item.start() as i32;
                let max_x = item.end() as i32 - 1;
                nums.push(Num {
                    value,
                    min_x,
                    max_x,
                    y,
                });
            } else {
                let x = item.start() as i32;
                parts.insert(
                    (x, y),
                    Part {
                        symbol: item.as_str().parse().unwrap(),
                        labels: Vec::new(),
                    },
                );
            }
        }
    }

    let mut result_p1 = 0;

    for num in nums {
        let mut has_part = false;
        num.check_border(|x, y| {
            if let Some(part) = parts.get_mut(&(x, y)) {
                has_part = true;
                part.labels.push(num.value);
            }
        });
        if has_part {
            result_p1 += num.value;
        }
    }

    println!("Part 1: {result_p1}");

    let mut result_p2 = 0;

    for part in parts.into_values() {
        if let ('*', [a, b]) = (part.symbol, part.labels.as_slice()) {
            result_p2 += a * b;
        }
    }

    println!("Part 2: {result_p2}");
}

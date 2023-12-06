use std::collections::HashMap;

use regex::Regex;

struct Num {
    value: u32,
    min_x: i32,
    max_x: i32,
    y: i32,
}

impl Num {
    pub fn check_border<F>(&self, check: F) -> bool
    where
        F: Fn(i32, i32) -> bool,
    {
        let border_min_x = self.min_x - 1;
        let border_max_x = self.max_x + 1;
        let y = self.y;
        for x in border_min_x..=border_max_x {
            if check(x, y - 1) || check(x, y + 1) {
                return true;
            }
        }
        check(border_min_x, y) || check(border_max_x, y)
    }
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
                parts.insert((x, y), item.as_str());
            }
        }
    }

    let result_p1: u32 = nums
        .iter()
        .filter(|n| n.check_border(|x, y| parts.contains_key(&(x, y))))
        .map(|n| n.value)
        .sum();
    println!("Part 1: {result_p1}");
}

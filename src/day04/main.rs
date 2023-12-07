use std::collections::{binary_heap::PeekMut, BinaryHeap};

use advent_2023::simple_parse;

fn parse_nums(text: &str) -> impl Iterator<Item = u32> + '_ {
    text.split_ascii_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
}

#[derive(Debug, Eq)]
pub struct CopyRun {
    copies: u32,
    end: usize,
}

impl Ord for CopyRun {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.end.cmp(&other.end).reverse()
    }
}

impl PartialOrd for CopyRun {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for CopyRun {
    fn eq(&self, other: &Self) -> bool {
        self.end == other.end
    }
}

pub struct CopyRuns {
    curr_card: usize,
    curr_copies: u32,
    ends: BinaryHeap<CopyRun>,
}

impl CopyRuns {
    pub fn new() -> Self {
        CopyRuns {
            curr_card: 0,
            curr_copies: 1,
            ends: BinaryHeap::new(),
        }
    }

    pub fn add_run(&mut self, len: usize) {
        if len > 0 {
            self.ends.push(CopyRun {
                copies: self.curr_copies,
                end: self.curr_card + len,
            });
            self.curr_copies += self.curr_copies;
        }
    }

    pub fn curr_copies(&self) -> u32 {
        self.curr_copies
    }

    pub fn next_card(&mut self) {
        self.curr_card += 1;
        while let Some(next_end) = self.ends.peek_mut() {
            if next_end.end >= self.curr_card {
                break;
            }
            self.curr_copies -= PeekMut::pop(next_end).copies;
        }
    }
}

fn main() {
    let mut result_p1 = 0;
    let mut result_p2 = 0;

    let mut copy_runs = CopyRuns::new();

    for line in include_str!("input.txt").lines() {
        let (winning, mine) = simple_parse!(line => "Card ", _, ": ", str, " | ", str).unwrap();
        let winning: Vec<u32> = parse_nums(winning).collect();
        let num_matches = parse_nums(mine).filter(|n| winning.contains(n)).count();

        if let Some(p_shift) = num_matches.checked_sub(1) {
            let points = 1u32 << p_shift;
            result_p1 += points;
        }

        result_p2 += copy_runs.curr_copies();

        copy_runs.add_run(num_matches);
        copy_runs.next_card();
    }

    println!("Part 1: {result_p1}");
    println!("Part 2: {result_p2}");
}

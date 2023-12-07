use std::convert::identity;

use advent_2023::simple_parse;

#[derive(Debug)]
struct MapRange {
    pub out_start: u64,
    pub in_start: u64,
    pub len: u64,
}

impl MapRange {
    pub const fn try_map(&self, num: u64) -> Option<u64> {
        if let Some(offset) = num.checked_sub(self.in_start) {
            if offset < self.len {
                return Some(self.out_start + offset);
            }
        }
        None
    }
}

#[derive(Debug)]
struct Mapping {
    ranges: Vec<MapRange>,
}

impl Mapping {
    pub fn new() -> Self {
        Self { ranges: vec![] }
    }

    pub fn add_range(&mut self, range: MapRange) {
        let index = self
            .ranges
            .binary_search_by_key(&range.in_start, |r| r.in_start)
            .unwrap_err();
        self.ranges.insert(index, range);
    }

    pub fn try_map(&self, num: u64) -> Option<u64> {
        let index = self
            .ranges
            .binary_search_by_key(&num, |r| r.in_start + r.len - 1)
            .unwrap_or_else(identity);
        self.ranges.get(index)?.try_map(num)
    }
}

fn parse_mapping(text: &str) -> Mapping {
    let mut mapping = Mapping::new();
    for line in text.lines().skip(1) {
        let (out_start, in_start, len) =
            simple_parse!(line => parse u64, ' ', parse u64, ' ', parse u64).unwrap();
        mapping.add_range(MapRange {
            out_start,
            in_start,
            len,
        });
    }
    mapping
}

fn apply_mappings(mappings: &[Mapping], mut num: u64) -> u64 {
    for mapping in mappings {
        num = mapping.try_map(num).unwrap_or(num);
    }
    num
}

fn main() {
    let mut input = include_str!("input.txt").split("\n\n");
    let seeds = input
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split(' ')
        .map(|n| n.parse::<u64>().expect(n));

    let mappings: Vec<_> = input.map(parse_mapping).collect();

    let result_p1 = seeds.map(|s| apply_mappings(&mappings, s)).min().unwrap();

    println!("Part 1: {result_p1}");
}

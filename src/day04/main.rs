use advent_2023::simple_parse;

fn parse_nums(text: &str) -> impl Iterator<Item = u32> + '_ {
    text.split_ascii_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
}

fn main() {
    let mut result_p1 = 0;

    for line in include_str!("input.txt").lines() {
        let (winning, mine) = simple_parse!(line => "Card ", _, ": ", str, " | ", str).unwrap();
        let winning: Vec<u32> = parse_nums(winning).collect();
        let num_matches = parse_nums(mine).filter(|n| winning.contains(n)).count();
        if let Some(p_shift) = num_matches.checked_sub(1) {
            let points = 1u32 << p_shift;
            result_p1 += points;
        }
    }

    println!("Part 1: {result_p1}");
}

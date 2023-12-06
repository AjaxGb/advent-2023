fn first_digit_p1<T: Iterator<Item = char>>(iter: T) -> u32 {
    iter.filter(char::is_ascii_digit)
        .next()
        .unwrap()
        .to_digit(10)
        .unwrap()
}

const NAMES_P2: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn first_digit_p2<'a>(mut text: &'a str) -> u32 {
    loop {
        if let Some(n) = text.chars().next().unwrap().to_digit(10) {
            return n;
        }
        if let Some(index) = NAMES_P2.iter().position(|n| text.starts_with(n)) {
            return index as u32 + 1;
        }
        text = &text[1..];
    }
}

fn last_digit_p2<'a>(mut text: &'a str) -> u32 {
    loop {
        if let Some(n) = text.chars().rev().next().unwrap().to_digit(10) {
            return n;
        }
        if let Some(index) = NAMES_P2.iter().position(|n| text.ends_with(n)) {
            return index as u32 + 1;
        }
        text = &text[..text.len() - 1];
    }
}

fn main() {
    let input = include_str!("input.txt");

    let mut p1_sum = 0;
    let mut p2_sum = 0;

    for line in input.lines() {
        // P1
        {
            let a = first_digit_p1(line.chars());
            let b = first_digit_p1(line.chars().rev());
            p1_sum += a * 10 + b;
        }

        // P2
        {
            let a = first_digit_p2(line);
            let b = last_digit_p2(line);
            p2_sum += a * 10 + b;
        }
    }

    println!("Part 1: {}", p1_sum);
    println!("Part 2: {}", p2_sum);
}

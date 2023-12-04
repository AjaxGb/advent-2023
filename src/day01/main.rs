fn first_digit<T: Iterator<Item = char>>(iter: T) -> u32 {
    iter.filter(char::is_ascii_digit)
        .next()
        .unwrap()
        .to_digit(10)
        .unwrap()
}

fn main() {
    let input = include_str!("input.txt");
    let value: u32 = input
        .lines()
        .map(|line| {
            let a = first_digit(line.chars());
            let b = first_digit(line.chars().rev());
            a * 10 + b
        })
        .sum();
    println!("Part 1: {}", value);
}

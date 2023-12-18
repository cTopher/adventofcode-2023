use adventofcode_2023::day_12;
use std::time::Instant;

const INPUT: &str = include_str!("day_12/input.txt");

fn main() {
    let now = Instant::now();
    let _ = day_12::part_2(INPUT);
    println!("{} ms", now.elapsed().as_millis());
}

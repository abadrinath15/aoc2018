use std::collections;
use std::fs;

fn main() {
    let input = fs::read_to_string(r"input/input.txt").expect("File not found");
    part1(&input);
    part2(&input, 0);
}

fn part1(input_str: &str) -> () {
    let mut result = 0;
    for freq in input_str.lines() {
        result += freq.parse::<i32>().unwrap();
    }

    println!("{}", result);
}

fn part2(input_str: &str, mut start_freq: i32) -> () {
    let mut seen_numbers = collections::HashSet::<i32>::new();
    loop {
        for freq in input_str.lines() {
            start_freq += freq.parse::<i32>().unwrap();
            if seen_numbers.contains(&start_freq) {
                println!("{}", start_freq);
                return;
            }
            seen_numbers.insert(start_freq);
        }
    }
}

use itertools::Itertools;
use std::collections;
use std::fs;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

fn main() -> Result<()> {
    let input_str = fs::read_to_string(r"input/input.txt")?;
    part1(&input_str);
    part2(&input_str);
    Ok(())
}

fn part1(input: &str) {
    let mut twos = 0;
    let mut threes = 0;
    for line in input.lines() {
        let mut word_map = collections::HashMap::new();
        for letter in line.chars() {
            *word_map.entry(letter).or_insert(0) += 1;
        }
        let mut curr_two = 0;
        let mut curr_three = 0;
        for val in word_map.values() {
            match *val {
                2 => {
                    if curr_two == 0 {
                        curr_two += 1;
                    };
                }
                3 => {
                    if curr_three == 0 {
                        curr_three += 1;
                    };
                }
                _ => (),
            };
        }
        twos += curr_two;
        threes += curr_three;
    }
    println!("{}", twos * threes);
}

fn one_letter_dif(word1: &str, word2: &str) -> Option<usize> {
    let mut num_dif = 0;
    let mut ind_dif_1 = 0;
    for (ind, (char1, char2)) in word1.chars().zip(word2.chars()).enumerate() {
        if char1 != char2 {
            num_dif += 1;
            ind_dif_1 = ind;
            if num_dif > 1 {
                return None;
            }
        }
    }
    Some(ind_dif_1)
}

fn part2(input: &str) {
    let perms = input.lines().permutations(2);
    for perm in perms {
        if let Some(result) = one_letter_dif(perm[0], perm[1]) {
            println!("{}{}", &perm[0][..result], &perm[0][(result + 1)..]);
        }
    }
}

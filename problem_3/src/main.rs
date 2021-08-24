use itertools::Itertools;
use std::collections::HashMap;
use std::error;
use std::fs;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
struct Claim {
    left_coord: i32,
    top_coord: i32,
    width: i32,
    height: i32,
}

impl Claim {
    fn claimed_squares(self) -> Vec<String> {
        let mut sqs = vec![];
        for i in 0..self.height {
            for j in 0..self.width {
                sqs.push(format!("{}x{}", self.left_coord + j, self.top_coord + i));
            }
        }
        sqs
    }
}

fn str_to_claim(claim_str: &str) -> Result<Claim> {
    let claim_comps = claim_str.split(' ').collect::<Vec<&str>>();
    let (claim_left_coord, claim_top_coord) = claim_comps[2]
        .split(',')
        .collect_tuple()
        .ok_or("Not enough coordinates!")?;
    let (claim_top_coord, _) = claim_top_coord
        .split(':')
        .collect_tuple()
        .ok_or("Top coord didin't match specificed pattern")?;
    let (claim_width, claim_height) = claim_comps[3]
        .split('x')
        .collect_tuple()
        .ok_or("Not enough rectangle dimensions")?;
    Ok(Claim {
        left_coord: claim_left_coord.parse::<i32>()?,
        top_coord: claim_top_coord.parse::<i32>()?,
        width: claim_width.parse::<i32>()?,
        height: claim_height.parse::<i32>()?,
    })
}

fn part1(strclaim_file: &str) -> Result<()> {
    let mut claims_ledger = HashMap::new();
    for claim_str in strclaim_file.lines() {
        let claim = str_to_claim(claim_str)?;
        for square in claim.claimed_squares() {
            *claims_ledger.entry(square).or_insert(0) += 1;
        }
    }
    println!("{}", claims_ledger.values().filter(|&x| *x > 1).count());
    Ok(())
}

fn main() -> Result<()> {
    let claim_file = fs::read_to_string(r"input/input.txt")?;
    part1(&claim_file)
}

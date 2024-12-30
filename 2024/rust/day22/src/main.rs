#![feature(test)]
#![feature(iter_map_windows)]
extern crate test;

use anyhow::Result;
use common::Answer;
use std::{
    collections::{HashMap, HashSet},
    io,
};

pub fn main() -> Result<()> {
    let stdin = io::read_to_string(io::stdin())?;
    println!("part1: {}", part_one(&stdin)?);
    println!("part2: {}", part_two(&stdin)?);
    Ok(())
}

pub fn part_one(input: &str) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_one(&input)
}

pub fn part_two(input: &str) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_two(&input)
}

struct Input {
    secret_nums: Vec<i128>,
}

fn parse_input(input: &str) -> Result<Input> {
    // example to collect Vec<Result<T, E>> to Result<Vec<T>, E>
    let secret_nums: Vec<i128> = input
        .trim()
        .lines()
        .map(|l| i128::from_str_radix(l, 10))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Input { secret_nums })
}

fn mix_prune(num: &mut i128, mix: i128) {
    const PRUNE: i128 = 16777216;
    *num ^= mix;
    *num %= PRUNE;
}

fn next_secret(secret: i128) -> i128 {
    let mut next_secret = secret;
    let mul64 = next_secret << 6;
    mix_prune(&mut next_secret, mul64);
    let div32 = next_secret >> 5;
    mix_prune(&mut next_secret, div32);
    let mul2024 = next_secret << 11;
    mix_prune(&mut next_secret, mul2024);
    next_secret
}

fn nth_secret(n: usize, secret: &i128) -> i128 {
    let mut nth_secret = *secret;
    for _i in 0..n {
        nth_secret = next_secret(nth_secret);
    }
    nth_secret
}

fn gen_n_secrets(n: usize, secret: &i128) -> Vec<i128> {
    let mut nth_secret = *secret;
    let mut secrets = Vec::new();
    for _i in 0..n + 1 {
        secrets.push(nth_secret);
        nth_secret = next_secret(nth_secret);
    }
    secrets
}

fn solve_one(input: &Input) -> Result<Answer> {
    let Input { secret_nums } = input;
    let answer = secret_nums.iter().map(|s| nth_secret(2000, s)).sum();
    Ok(Answer::Num(answer))
}

fn solve_two(input: &Input) -> Result<Answer> {
    let Input { secret_nums } = input;
    let buyers: Vec<Buyer> = secret_nums.iter().map(|s| Buyer::from_secret(*s)).collect();
    let answer = Buyer::best_sequence(&buyers);
    Ok(Answer::Num(answer.0))
}

struct Buyer {
    // secrets: Vec<i128>,
    // prices: Vec<i128>,
    // deltas: Vec<i128>,
    sequence_prices: HashMap<[i128; 4], i128>,
}

impl Buyer {
    fn from_secret(secret: i128) -> Self {
        let secrets = gen_n_secrets(2000, &secret);
        let prices: Vec<i128> = secrets.iter().map(|s| s % 10).collect();
        let deltas: Vec<i128> = prices.windows(2).map(|w| w[1] - w[0]).collect();
        let mut sequence_prices = HashMap::new();
        for (s, price) in deltas.windows(4).zip(prices.iter().skip(4)) {
            let k = [s[0], s[1], s[2], s[3]];
            if !sequence_prices.contains_key(&k) {
                sequence_prices.insert(k, *price);
            }
        }
        Self {
            // secrets,
            // prices,
            // deltas,
            sequence_prices,
        }
    }
    fn get_price(&self, sequence: &[i128; 4]) -> i128 {
        *self.sequence_prices.get(sequence).unwrap_or(&0)
    }
    fn best_sequence(buyers: &[Buyer]) -> (i128, String) {
        let mut check_sequences: HashSet<[i128; 4]> = HashSet::new();
        for buyer in buyers.iter() {
            check_sequences.extend(buyer.sequence_prices.keys());
        }
        let (most_bananas, best_sequence) = check_sequences
            .iter()
            .map(|s| (buyers.iter().map(|b| b.get_price(s)).sum::<i128>(), s))
            .max_by(|a, b| a.0.cmp(&b.0))
            .unwrap();
        // println!("sequence: {:?}, bananas: {}", best_sequence, most_bananas);
        (
            most_bananas,
            best_sequence
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
                .join(","),
        )
    }
}

// Quickly obtain answers by running
// cargo test one [-r]
// cargo test two [-r]
#[cfg(test)]
mod day22_tests {
    use super::*;
    use common::test_utils::*;
    use std::sync::LazyLock;
    use test::Bencher;

    static TEST: LazyLock<String> = local_file!("test");
    static TEST2: LazyLock<String> = local_file!("test2");
    static INPUT: LazyLock<String> = local_file!("input");

    #[test]
    fn test_next_secret() {
        let mut secret = 123;
        let mut actual = Vec::new();
        for _i in 0..10 {
            secret = next_secret(secret);
            actual.push(secret);
        }
        let expected = vec![
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_one() -> Result<()> {
        let answer = super::part_one(&TEST)?;
        assert_eq!(answer, Answer::Num(37327623));
        Ok(())
    }
    fn part_one_impl() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(16299144133));
        Ok(())
    }
    #[bench]
    fn part_one(b: &mut Bencher) {
        part_one_impl().expect("Error");
        b.iter(|| part_one_impl())
    }
    #[test]
    fn test_two() -> Result<()> {
        let answer = super::part_two(&TEST2)?;
        assert_eq!(answer, Answer::Num(23));
        Ok(())
    }
    fn part_two_impl() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(answer, Answer::Num(1896));
        Ok(())
    }
    #[bench]
    fn part_two(b: &mut Bencher) {
        part_two_impl().expect("Error");
        b.iter(|| part_two_impl())
    }
}

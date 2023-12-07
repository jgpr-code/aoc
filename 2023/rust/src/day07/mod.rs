use super::common::*;
use anyhow::Result;
use std::{cmp::Ordering, collections::HashMap};

pub fn part_one(input: &str) -> Result<Answer> {
    let mut input = parse_input(input)?;
    solve_one(&mut input)
}

pub fn part_two(input: &str) -> Result<Answer> {
    let mut input = parse_input(input)?;
    solve_two(&mut input)
}

#[derive(Debug)]
struct Input {
    plays: Vec<(Hand, i128)>,
}
#[derive(Debug, Eq)]
struct Hand {
    cards: Vec<char>,
}

impl Hand {
    // 0 -> high card
    // 1 -> one pair
    // 2 -> two pair
    // 3 -> three of kind
    // 4 -> full house
    // 5 -> four of kind
    // 6 -> five of kind
    fn get_kind(&self) -> usize {
        let mut counter: HashMap<char, usize> = HashMap::new();
        for card in self.cards.iter() {
            counter.entry(*card).and_modify(|c| *c += 1).or_insert(1);
        }
        let mut counts: Vec<usize> = counter.iter().map(|e| *e.1).collect();
        counts.sort_unstable();
        let len = counts.len();
        if len == 1 {
            // five pairs
            assert_eq!(counts[0], 5);
            return 6;
        }
        let h = counts[len - 1];
        let s = counts[len - 2];
        match (h, s) {
            (4, 1) => 5,
            (3, 2) => 4,
            (3, 1) => 3,
            (2, 2) => 2,
            (2, 1) => 1,
            (1, 1) => 0,
            _ => unreachable!(),
        }
    }
    fn numeric_card(card: &char) -> usize {
        match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            c => c.to_digit(10).unwrap() as usize,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_kind = self.get_kind();
        let other_kind = other.get_kind();
        if self_kind < other_kind {
            return Ordering::Less;
        } else if self_kind > other_kind {
            return Ordering::Greater;
        }
        for (s, o) in self.cards.iter().zip(other.cards.iter()) {
            let sn = Self::numeric_card(s);
            let on = Self::numeric_card(o);
            if sn < on {
                return Ordering::Less;
            } else if sn > on {
                return Ordering::Greater;
            }
        }
        return Ordering::Equal;
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

fn parse_input(input: &str) -> Result<Input> {
    let mut plays = Vec::new();
    for line in input.lines() {
        let (cards, bid) = line.split_once(" ").unwrap();
        let bid = bid.trim().parse().unwrap();
        let cards = cards.trim().chars().collect();
        plays.push((Hand { cards }, bid));
    }
    Ok(Input { plays })
}

fn solve_one(input: &mut Input) -> Result<Answer> {
    let Input { ref mut plays } = input;
    println!("{:?}", plays);
    for (hand, bid) in plays.iter() {
        println!("{:?}:{} rank: {}", hand, bid, hand.get_kind());
    }
    plays.sort_by(|a, b| a.0.cmp(&b.0));
    println!("{:?}", plays);
    let mut total_winnings = 0;
    for (i, play) in plays.iter().enumerate() {
        total_winnings += (i + 1) as i128 * play.1;
    }
    Ok(Answer::Num(total_winnings))
}

fn solve_two(input: &mut Input) -> Result<Answer> {
    let Input { ref mut plays } = input;
    println!("{:?}", plays);
    Ok(Answer::Num(0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::LazyLock;
    use test::Bencher;

    static TEST: LazyLock<String> = local_file!("test.txt");
    static INPUT: LazyLock<String> = local_file!("input.txt");

    #[test]
    fn test_one() -> Result<()> {
        let answer = super::part_one(&TEST)?;
        assert_eq!(answer, Answer::Num(1));
        Ok(())
    }
    #[test]
    fn part_one() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(1));
        Ok(())
    }
    #[test]
    fn test_two() -> Result<()> {
        let answer = super::part_two(&TEST)?;
        assert_eq!(answer, Answer::Num(2));
        Ok(())
    }
    #[test]
    fn part_two() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(answer, Answer::Num(2));
        Ok(())
    }

    #[bench]
    fn bench_part_one(b: &mut Bencher) {
        b.iter(|| part_one())
    }
    #[bench]
    fn bench_part_two(b: &mut Bencher) {
        b.iter(|| part_two())
    }
}

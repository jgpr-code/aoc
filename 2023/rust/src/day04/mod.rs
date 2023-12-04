use super::common::*;
use anyhow::Result;
use std::{collections::HashSet, num::ParseIntError};

pub fn part_one(input: &str) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_one(&input)
}

pub fn part_two(input: &str) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_two(&input)
}

struct Input {
    cards: Vec<Card>,
}
struct Card {
    id: usize,
    winners: Vec<usize>,
    numbers: Vec<usize>,
}

fn parse_input(input: &str) -> Result<Input> {
    let lines = input.lines();
    let mut cards = Vec::new();
    for line in lines {
        let (card, rest) = line.split_once(":").unwrap();
        let card = card.trim();
        let rest = rest.trim();
        let a = card.split(" ").filter(|s| s.len() > 0).collect::<Vec<_>>();
        let id = a[1].parse::<usize>().unwrap();
        let (winners, numbers) = rest.split_once("|").unwrap();
        let winners = winners.trim();
        let numbers = numbers.trim();
        let winners = winners
            .split(" ")
            .filter(|s| s.len() > 0)
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let numbers = numbers
            .split(" ")
            .filter(|s| s.len() > 0)
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        cards.push(Card {
            id,
            winners,
            numbers,
        });
    }
    Ok(Input { cards })
}

fn solve_one(input: &Input) -> Result<Answer> {
    let Input { cards } = input;
    let mut total = 0;
    for card in cards {
        let mut winner_count = 0;
        let winning_set: HashSet<&usize> = HashSet::from_iter(card.winners.iter());
        for num in card.numbers.iter() {
            if winning_set.contains(&num) {
                winner_count += 1;
            }
        }
        if winner_count > 0 {
            let card_worth = 2_i32.pow(winner_count - 1);
            println!("{}", card_worth);
            total += card_worth
        }
    }
    Ok(Answer::Num(total as i128))
}

fn solve_two(input: &Input) -> Result<Answer> {
    let Input { cards } = input;
    let mut total = 0;
    for card in cards {
        let mut winner_count = 0;
        let winning_set: HashSet<&usize> = HashSet::from_iter(card.winners.iter());
        for num in card.numbers.iter() {
            if winning_set.contains(&num) {
                winner_count += 1;
            }
        }
        if winner_count > 0 {
            total += 2_i32.pow(winner_count - 1);
        }
    }
    Ok(Answer::Num(total as i128))
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
        assert_eq!(answer, Answer::Num(8));
        Ok(())
    }
    #[test]
    fn part_one() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(23235));
        Ok(())
    }
    #[test]
    fn test_two() -> Result<()> {
        let answer = super::part_two(&TEST)?;
        assert_eq!(answer, Answer::Num(30));
        Ok(())
    }
    #[test]
    fn part_two() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(answer, Answer::Num(55));
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

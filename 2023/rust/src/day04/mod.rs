use super::common::*;
use anyhow::Result;
use std::collections::{HashSet, VecDeque};

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
    _winners: Vec<usize>,
    _numbers: Vec<usize>,
    matching: usize,
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
        let mut matching = 0;
        let winning_set: HashSet<&usize> = HashSet::from_iter(winners.iter());
        for num in numbers.iter() {
            if winning_set.contains(&num) {
                matching += 1;
            }
        }
        cards.push(Card {
            id,
            _winners: winners,
            _numbers: numbers,
            matching,
        });
    }
    Ok(Input { cards })
}

fn solve_one(input: &Input) -> Result<Answer> {
    let Input { cards } = input;
    let mut total = 0;
    for card in cards {
        if card.matching > 0 {
            let card_worth = 2_i32.pow(card.matching as u32 - 1);
            println!("{}", card_worth);
            total += card_worth
        }
    }
    Ok(Answer::Num(total as i128))
}

fn solve_two(input: &Input) -> Result<Answer> {
    let Input { cards } = input;
    let mut mapper: Vec<Vec<usize>> = Vec::new();
    mapper.push(Vec::new()); // 1 indexed
    let mut queue = VecDeque::new();
    for card in cards {
        let mut winning: Vec<usize> = Vec::new();
        for offset in 1..=card.matching {
            winning.push(card.id + offset);
        }
        mapper.push(winning);
        queue.push_back(card.id);
    }
    let mut counts = vec![0; queue.len() + 1];

    while let Some(id) = queue.pop_front() {
        counts[id] += 1;
        for next in mapper[id].iter() {
            queue.push_back(*next);
        }
    }
    let total = counts.iter().sum();
    Ok(Answer::Num(total))
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
        assert_eq!(answer, Answer::Num(13));
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
        assert_eq!(answer, Answer::Num(5920640));
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

use super::common::*;
use anyhow::Result;
use std::num::ParseIntError;

pub fn part_one(input: &str) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_one(&input)
}

pub fn part_two(input: &str) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_two(&input)
}

struct Input {
    sequences: Vec<Vec<i128>>,
}

fn parse_input(input: &str) -> Result<Input> {
    let sequences = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse::<i128>())
                .collect::<Result<Vec<_>, ParseIntError>>()
        })
        .collect::<Result<Vec<_>, ParseIntError>>()?;
    //println!("{:?}", sequences);
    Ok(Input { sequences })
}

fn seq_pyramide(seq: &Vec<i128>) -> Vec<Vec<i128>> {
    //println!("{:?}", seq);
    let mut all_seqs = Vec::new();
    all_seqs.push(seq.clone());
    loop {
        let current_seq = all_seqs.last().unwrap();
        let mut next_seq = vec![0 as i128; current_seq.len() - 1];
        let mut found = false;
        for (i, ab) in current_seq.windows(2).enumerate() {
            let next_elem = ab[1] - ab[0];
            if !found && next_elem != 0 {
                found = true
            }
            next_seq[i] = next_elem;
        }
        all_seqs.push(next_seq);
        if !found {
            break;
        }
    }
    //println!("{:?}", all_seqs);
    all_seqs
}

fn next_val(seq_pyramide: &Vec<Vec<i128>>) -> i128 {
    seq_pyramide.iter().map(|s| s.last().unwrap()).sum()
}

fn prev_val(seq_pyramide: &Vec<Vec<i128>>) -> i128 {
    let mut last_add = 0;
    for c in seq_pyramide
        .iter()
        .rev()
        .skip(1)
        .map(|v| v.first().unwrap())
    {
        last_add = c - last_add;
    }
    last_add
}

fn solve_one(input: &Input) -> Result<Answer> {
    let Input { sequences } = input;
    let mut sum = 0;
    for seq in sequences.iter() {
        let seq_pyra = seq_pyramide(seq);
        sum += next_val(&seq_pyra);
    }
    Ok(Answer::Num(sum))
}

fn solve_two(input: &Input) -> Result<Answer> {
    let Input { sequences } = input;
    let mut sum = 0;
    for seq in sequences.iter() {
        let seq_pyra = seq_pyramide(seq);
        let pval = prev_val(&seq_pyra);
        // println!("{:?}", pval);
        sum += pval
    }
    Ok(Answer::Num(sum))
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
        assert_eq!(answer, Answer::Num(114));
        Ok(())
    }
    #[test]
    fn part_one() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(1702218515));
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
        assert_eq!(answer, Answer::Num(925));
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

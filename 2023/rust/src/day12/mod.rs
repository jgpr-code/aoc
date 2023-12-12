use std::collections::VecDeque;

use super::common::*;
use anyhow::Result;

pub fn part_one(input: &str) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_one(&input)
}

pub fn part_two(input: &str) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_two(&input)
}

struct Input {
    springs: Vec<Vec<char>>,
    records: Vec<Vec<usize>>,
}

fn parse_input(input: &str) -> Result<Input> {
    let mut springs = Vec::new();
    let mut records = Vec::new();
    for line in input.lines() {
        let (springs_line, records_line) = line.split_once(" ").unwrap();
        let springs_line = springs_line.trim().chars().collect();
        springs.push(springs_line);
        let records_line = records_line
            .split(",")
            .map(|s| s.trim().parse().unwrap())
            .collect();
        records.push(records_line);
    }
    Ok(Input { springs, records })
}

fn solve_line(springs: &[char], records: &[usize]) -> usize {
    // for each ? either can be . or replaced with group and progress
    let mut count = 0;
    solve_helper(springs, records, 0, &mut count);
    count
}

fn solve_helper(springs: &[char], records: &[usize], spos: usize, count: &mut usize) {
    // println!("{:?}, {:?}, {}, {}", springs, records, spos, count);
    if spos == springs.len() && !is_invalid(springs, records) {
        let springs_string: String = springs.iter().collect();
        // println!("{}", springs_string);
        *count += 1;
        return;
    }
    if springs[spos] != '?' {
        solve_helper(springs, records, spos + 1, count)
    } else {
        let mut next_springs: Vec<char> = springs.iter().map(|&c| c).collect();
        next_springs[spos] = '.';
        if !is_invalid(&next_springs, records) {
            solve_helper(&next_springs, records, spos + 1, count);
        }
        next_springs[spos] = '#';
        if !is_invalid(&next_springs, records) {
            solve_helper(&next_springs, records, spos + 1, count);
        }
    }
}

fn is_invalid(springs: &[char], records: &[usize]) -> bool {
    let cheat_springs: String = springs.iter().collect();
    let cheat_springs = format!(".{}.", cheat_springs);
    let cheats: Vec<char> = cheat_springs.chars().collect();
    let record_sum = records.iter().sum();
    let count_hashtag = count_c(&cheat_springs, '#');
    let count_questionmark = count_c(&cheat_springs, '?');
    if count_hashtag + count_questionmark < record_sum {
        return true;
    }
    if count_hashtag > record_sum {
        return true;
    }
    if !scan(&cheat_springs, records) {
        return true;
    }
    if count_questionmark == 0 {
        let re_finished_group = regex!(r"(#+)");
        // println!("checking {}", cheat_springs);
        let caps: Vec<&str> = re_finished_group
            .captures_iter(&cheat_springs)
            .map(|c| c.get(1).unwrap())
            .filter(|c| cheats[c.start() - 1] == '.' && cheats[c.end()] == '.')
            .map(|c| c.as_str())
            .collect();
        if caps.len() > records.len() {
            return true;
        }
        // println!("caps: {:?}", caps);
        for (i, cap) in caps.into_iter().enumerate().rev() {
            // println!("{}: captured {} (record {})", i, cap, records[i]);
            if cap.len() != records[i] {
                return true;
            }
        }
    }
    false
}

fn scan(springs: &str, records: &[usize]) -> bool {
    let springs: Vec<char> = springs.chars().collect();
    let mut connected = 0;
    let mut ri = 0;
    for i in 0..springs.len() {
        let c = springs[i];
        if c == '?' {
            return true;
        } else if c == '#' {
            connected += 1;
        } else if connected != 0 {
            if ri >= records.len() || connected != records[ri] {
                return false;
            }
            connected = 0;
            ri += 1;
        }
    }
    return true;
}

fn count_c(line: &str, c: char) -> usize {
    line.chars().filter(|&lc| lc == c).count()
}

fn solve_one(input: &Input) -> Result<Answer> {
    let Input { springs, records } = input;
    let mut sum = 0;
    for (springs, records) in springs.iter().zip(records.iter()) {
        sum += solve_line(&springs, &records);
    }
    Ok(Answer::Num(sum as i128))
}

fn solve_two(input: &Input) -> Result<Answer> {
    let Input { springs, records } = input;
    let mut sum = 0;
    for (springs, records) in springs.iter().zip(records.iter()) {
        let springs: Vec<char> = springs
            .into_iter()
            .cycle()
            .take(springs.len() * 5)
            .map(|&c| c)
            .collect();
        let records: Vec<usize> = records
            .into_iter()
            .cycle()
            .take(records.len() * 5)
            .map(|&c| c)
            .collect();
        let answer = solve_line(&springs, &records);
        sum += solve_line(&springs, &records);
        let sprstr = String::from_iter(springs.iter());
        println!("solved {}, {:?} = {}", sprstr, records, answer);
    }
    Ok(Answer::Num(sum as i128))
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
        assert_eq!(answer, Answer::Num(21));
        Ok(())
    }
    #[test]
    fn part_one() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(6871));
        Ok(())
    }
    #[test]
    fn test_two() -> Result<()> {
        let answer = super::part_two(&TEST)?;
        assert_eq!(answer, Answer::Num(14));
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

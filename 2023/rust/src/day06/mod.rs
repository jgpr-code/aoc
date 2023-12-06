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
    races: Vec<Race>,
}
struct Race {
    time: i128,
    distance: i128,
}

impl Race {
    // math: try different slope and offset and see if at time we are larger than distance
    // hd*x - hd^2
    fn hold_distance(time: i128, hold: i128) -> i128 {
        hold * (time - hold)
    }

    fn possible_wins(&self) -> i128 {
        let mut wins = 0;
        for hold in 1..self.time {
            let distance = Self::hold_distance(self.time, hold);
            //println!("{}->{}", hold, distance);
            if distance > self.distance {
                wins += 1;
            }
        }
        println!("wins: {}", wins);
        wins
    }

    // hold * (x - hold) > y => solve by hold
}

fn parse_input(input: &str) -> Result<Input> {
    let (times, distances) = input.split_once("\r\n").unwrap();
    let times: Vec<i128> = times
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();
    let distances: Vec<i128> = distances
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();

    assert_eq!(times.len(), distances.len());
    let races = times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| Race { time, distance })
        .collect();

    Ok(Input { races })
}

fn solve_one(input: &Input) -> Result<Answer> {
    let Input { races } = input;
    let answer = races.iter().map(|r| r.possible_wins()).product();
    Ok(Answer::Num(answer))
}

fn solve_two(input: &Input) -> Result<Answer> {
    let Input { races } = input;
    let (time_str, distance_str) =
        races
            .iter()
            .fold((String::from(""), String::from("")), |acc, race| {
                (
                    format!("{}{}", acc.0, race.time),
                    format!("{}{}", acc.1, race.distance),
                )
            });
    // let time = 56977875;
    // let distance = 546192711311139;
    let race = Race {
        time: time_str.parse().unwrap(),
        distance: distance_str.parse().unwrap(),
    };

    Ok(Answer::Num(race.possible_wins()))
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
        assert_eq!(answer, Answer::Num(288));
        Ok(())
    }
    #[test]
    fn part_one() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(1624896));
        Ok(())
    }
    #[test]
    fn test_two() -> Result<()> {
        let answer = super::part_two(&TEST)?;
        assert_eq!(answer, Answer::Num(71503));
        Ok(())
    }
    #[test]
    fn part_two() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(answer, Answer::Num(32583852));
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

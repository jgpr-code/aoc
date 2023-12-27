use std::collections::HashMap;

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
        // let springs_string: String = springs.iter().collect();
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
            return ri >= records.len() || connected <= records[ri];
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
    // println!("springs {:?}, records {:?}", springs, records);

    return ri >= records.len();
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

fn solve_line_two(springs: &[char], records: &[usize]) -> usize {
    let mut memo = HashMap::new();
    let rem = springs.iter().map(|&c| c).collect();
    let mat: Vec<char> = Vec::new();
    let des_len = springs.len();
    solve_help(&mut memo, rem, records, 0, mat, des_len)
}
fn solve_help(
    memo: &mut HashMap<(String, usize), usize>,
    rem: Vec<char>,
    records: &[usize],
    rpos: usize,
    mat: Vec<char>,
    des_len: usize,
) -> usize {
    // let matstr = String::from_iter(mat.iter());
    let remstr = String::from_iter(rem.iter());
    // println!(
    //     "calling mat={} rem={} rec={:?}",
    //     matstr,
    //     remstr,
    //     &records[rpos..]
    // );
    if let Some(count) = memo.get(&(remstr.clone(), rpos)) {
        // println!(
        //     "{} || {},{}({:?}) = {}",
        //     matstr.clone(),
        //     remstr.clone(),
        //     rpos,
        //     &records[rpos..],
        //     count
        // );
        return *count;
    }
    if rpos >= records.len() {
        if rem.contains(&'#') {
            return 0; // invalid
        } else {
            let matstr = String::from_iter(mat.iter());
            let remstr = String::from_iter(rem.iter());
            // println!("{} || {}", matstr, remstr);
            let complete_str = format!("{}{}", matstr, remstr).replace("?", ".");
            assert_eq!(des_len, complete_str.len());
            if scan(&complete_str, &records) {
                // println!("{} complete", complete_str);
                return 1;
            } else {
                return 0;
            }
        }
    }
    // try placing .####. if records is 4
    // e.g. .?????.##
    // plac .####..## next with ..##
    // plac ..####.## next with .##
    // plac ...###### not possible! done
    let place = format!(".{}.", "#".repeat(records[rpos]));
    let place: Vec<char> = place.chars().collect();
    let mut count: usize = 0;
    for i in 0..rem.len() {
        let mut valid = true;
        for j in 0..place.len() {
            if i + j >= rem.len() || !(rem[i + j] == '?' || rem[i + j] == place[j]) {
                valid = false;
                break;
            }
        }
        if valid {
            let start_rem = i + place.len() - 1;
            let mut matadd = Vec::from_iter(rem[0..start_rem].iter().map(|&c| c));
            for j in 0..place.len() - 1 {
                matadd[i + j] = place[j];
            }
            let mut next_rem = Vec::from_iter(rem[start_rem..].iter().map(|&c| c));
            assert_ne!(next_rem[0], '#');
            next_rem[0] = '.'; // important because it might have been ? before
            let mut next_mat = mat.clone();
            next_mat.append(&mut matadd);

            // println!(
            //     "{}: {}, {}, {}, {}",
            //     i,
            //     String::from_iter(rem.iter()),
            //     String::from_iter(place.iter()),
            //     String::from_iter(next_mat.iter()),
            //     String::from_iter(next_rem.iter())
            // );
            // check whether matstr+remstr can still be valid
            let next_matstr =
                String::from_iter(next_mat.iter().map(|&c| if c == '?' { '.' } else { c }));
            let current = format!("{}{}", next_matstr, String::from_iter(next_rem.iter()));
            let ok =
                scan(&format!("{}.", next_matstr), &records[..rpos + 1]) && scan(&current, records);
            // println!("{}  {}", current, ok);
            if ok {
                // println!(
                //     "{}: {}, {}, {}, {}",
                //     i,
                //     String::from_iter(rem.iter()),
                //     String::from_iter(place.iter()),
                //     String::from_iter(next_mat.iter()),
                //     String::from_iter(next_rem.iter())
                // );
                let add = solve_help(memo, next_rem.clone(), records, rpos + 1, next_mat, des_len);
                //memo.insert((String::from_iter(next_rem.iter()), rpos + 1), add);
                count += add;
            }
        }
    }

    let current = format!(
        "{}{}",
        String::from_iter(mat.iter().map(|&c| if c == '?' { '.' } else { c })),
        String::from_iter(rem.iter())
    );
    let ok = scan(&current, records);
    if ok {
        // println!(
        //     "inserting ({}, {}({:?})) = {}",
        //     remstr,
        //     rpos,
        //     &records[rpos..],
        //     count
        // );
        memo.insert((remstr, rpos), count);
    }
    count
}

//

fn solve_two(input: &Input) -> Result<Answer> {
    let Input { springs, records } = input;
    let mut sum = 0;
    for (springs, records) in springs.iter().zip(records.iter()).rev() {
        let mut rsprings = Vec::new();
        rsprings.push('.'); // makes processing later easier!
        for i in 0..5 {
            rsprings.append(&mut springs.clone());
            if i != 4 {
                rsprings.push('?')
            }
        }
        rsprings.push('.'); // makes processing later easier!
        let records: Vec<usize> = records
            .into_iter()
            .cycle()
            .take(records.len() * 5)
            .map(|&c| c)
            .collect();
        let answer = solve_line_two(&rsprings, &records);
        sum += answer;
        let sprstr = String::from_iter(rsprings.iter());
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
        assert_eq!(answer, Answer::Num(525152));
        Ok(())
    }
    // #[test]
    // fn test_solve_two() -> Result<()> {
    //     let a: Vec<char> = ".?###??????????###????????.".chars().collect();
    //     let ans = super::solve_line_two(&a[..], &[3, 2, 1, 3, 2, 1]);
    //     assert_eq!(ans, 1234);
    //     Ok(())
    // }
    // #[test]
    // fn aaa() -> Result<()> {
    //     let a: Vec<char> = ".??????????###????????.".chars().collect();
    //     let ans = super::solve_line_two(&a[..], &[3, 2, 1, 3]);
    //     assert_eq!(ans, 1234);
    //     Ok(())
    // }
    // #[test]
    // fn bbb() -> Result<()> {
    //     let a: Vec<char> = ".#?.?.#??#???#.?.???#?.?.#??#???#.?.???#?.?.#??#???#.?.???#?.?.#??#???#.?.???#?.?.#??#???#.?.??.".chars().collect();
    //     let ans = super::solve_line_two(
    //         &a[..],
    //         &[
    //             2, 1, 1, 2, 2, 2, 1, 1, 2, 2, 2, 1, 1, 2, 2, 2, 1, 1, 2, 2, 2, 1, 1, 2, 2,
    //         ],
    //     );
    //     assert_eq!(ans, 1234);
    //     Ok(())
    // }
    // #[test]
    // fn ccc() -> Result<()> {
    //     let a: Vec<char> = ".#?.?.#??#???#.?.???#?.?.#??#???#.?.??.".chars().collect();
    //     let ans = super::solve_line_two(&a[..], &[2, 1, 1, 2, 2, 2, 1, 1, 2, 2]);
    //     assert_eq!(ans, 1234);
    //     Ok(())
    // }
    // #[test]
    // fn ddd() -> Result<()> {
    //     let a: Vec<char> = ".???.?.?.?.?.###.???????.".chars().collect();
    //     let ans = super::solve_line_two(&a[..], &[2, 3, 2, 1]);
    //     assert_eq!(ans, 1234);
    //     Ok(())
    // }

    // #[test]
    // fn eee() -> Result<()> {
    //     let a: Vec<char> = ".?#???#.?.??.".chars().collect();
    //     let ans = super::solve_line_two(&a[..], &[1, 2, 2]);
    //     assert_eq!(ans, 1234);
    //     Ok(())
    // }
    #[test]
    fn part_two() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(answer, Answer::Num(2043098029844));
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

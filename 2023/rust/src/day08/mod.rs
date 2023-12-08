use super::common::*;
use anyhow::Result;
use std::collections::HashMap;

pub fn part_one(input: &str) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_one(&input)
}

pub fn part_two(input: &str) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_two(&input)
}

struct Input {
    directions: Vec<char>,
    nodes: HashMap<String, (String, String)>,
}

fn parse_input(input: &str) -> Result<Input> {
    let (directions, nodes_str) = input.split_once("\r\n\r\n").unwrap();
    let directions = directions.trim().chars().collect::<Vec<_>>();
    let re = regex!(r"(\w+) = \((\w+), (\w+)\)");
    let mut nodes = HashMap::new();
    for node in nodes_str.lines() {
        let caps = re.captures(node).unwrap();
        nodes.insert(
            String::from(&caps[1]),
            (String::from(&caps[2]), String::from(&caps[3])),
        );
    }

    Ok(Input { directions, nodes })
}

fn solve_one(input: &Input) -> Result<Answer> {
    let mut current = String::from("AAA");
    let mut directions = input.directions.iter().cycle();
    let mut steps = 0;
    loop {
        if current == "ZZZ" {
            break;
        }
        let dir = directions.next().unwrap();
        let (left, right) = input.nodes.get(&current).unwrap();
        match dir {
            'L' => current = left.clone(),
            'R' => current = right.clone(),
            _ => unreachable!(),
        }
        steps += 1;
    }
    Ok(Answer::Num(steps))
}

fn solve_two(input: &Input) -> Result<Answer> {
    let start_nodes = input
        .nodes
        .iter()
        .map(|(k, _)| k.clone())
        .filter(|s| s.ends_with("A"))
        .collect::<Vec<_>>();

    let answer = start_nodes
        .iter()
        .map(|n| {
            let ((_offset, loop_size), loop_offsets) = det_cycle(input, n.as_str());
            assert_eq!(loop_size, *loop_offsets.last().unwrap()); // otherwise lcm won't work
            loop_size
        })
        .fold(1, |acc, x| num::integer::lcm(acc, x));
    Ok(Answer::Num(answer as i128))
}

// returns all steps in cycle where at_node ends with "Z" and it returns first the offset until we reach the cycle
fn det_cycle(input: &Input, node: &str) -> ((usize, usize), Vec<usize>) {
    let len = input.directions.len();
    let mut directions = input.directions.iter().cycle();
    let mut at_dir = 0;
    let mut at_node = node;
    let mut was_at: HashMap<(usize, &str), usize> = HashMap::new();
    let mut at_end = Vec::new();
    let first_in_loop;
    let mut last_in_loop = 0;
    let mut steps = 0;
    loop {
        if at_node.ends_with("Z") {
            at_end.push(steps);
        }
        if was_at.contains_key(&(at_dir, at_node)) {
            first_in_loop = *was_at.get(&(at_dir, at_node)).unwrap();
            break;
        } else {
            was_at.insert((at_dir, at_node), steps);
        }
        let dir = directions.next().unwrap();
        let (left, right) = input.nodes.get(at_node).unwrap();
        match dir {
            'L' => at_node = left.as_str(),
            'R' => at_node = right.as_str(),
            _ => unreachable!(),
        }
        at_dir = (at_dir + 1) % len;
        last_in_loop = steps;
        steps += 1;

        // wolfram alpha eq (would work in the almost general case where for every ghost only one goal is reached during a loop)
        // 2 + n*21883 + 21881 ==  3 + m*16897 + 16894 == 5 + k*20221 + 20216 == 2 + l*16343 + 16341 == 2 + o*11911 + 11909 == 2 + p*18559 + 18557, n>=0,m>=0,k>=0,l>=0,o>=0,p>=0
        // n = 756916486
        // ans = 2 + 756916486*21883 + 21881
    }
    ((first_in_loop, last_in_loop - first_in_loop + 1), at_end)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::LazyLock;
    use test::Bencher;

    static TEST: LazyLock<String> = local_file!("test.txt");
    static TEST2: LazyLock<String> = local_file!("test2.txt");
    static INPUT: LazyLock<String> = local_file!("input.txt");
    static INPUT_LUCIE: LazyLock<String> = local_file!("input_lucie.txt");

    #[test]
    fn test_one() -> Result<()> {
        let answer = super::part_one(&TEST)?;
        assert_eq!(answer, Answer::Num(6));
        Ok(())
    }
    #[test]
    fn part_one() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(16897));
        Ok(())
    }
    #[test]
    fn test_two() -> Result<()> {
        let answer = super::part_two(&TEST2)?;
        assert_eq!(answer, Answer::Num(6));
        Ok(())
    }
    #[test]
    fn part_two() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(answer, Answer::Num(16563603485021));
        Ok(())
    }
    #[test]
    fn part_two_lucie() -> Result<()> {
        let answer = super::part_two(&INPUT_LUCIE)?;
        assert_eq!(answer, Answer::Num(17972669116327));
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

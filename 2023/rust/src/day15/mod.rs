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
    sequence: Vec<String>,
}

fn parse_input(input: &str) -> Result<Input> {
    let sequence: Vec<String> = input.split(",").map(|s| String::from(s)).collect();
    Ok(Input { sequence })
}

// Determine the ASCII code for the current character of the string.
// Increase the current value by the ASCII code you just determined.
// Set the current value to itself multiplied by 17.
// Set the current value to the remainder of dividing itself by 256.
fn hash(step: &str) -> usize {
    let mut curr = 0;
    for c in step.chars() {
        let cu32 = c as u32;
        curr += cu32 as usize;
        curr *= 17;
        curr %= 256;
    }
    curr
}

fn solve_one(input: &Input) -> Result<Answer> {
    let Input { sequence } = input;
    let mut sum = 0;
    for seq in sequence {
        sum += hash(&seq);
    }
    Ok(Answer::Num(sum as i128))
}

//The result of running the HASH algorithm on the label indicates the correct box for that step

// If there is already a lens in the box with the same label, replace the old lens with the new lens: remove the old lens and put the new lens in its place, not moving any other lenses in the box.
// If there is not already a lens in the box with the same label, add the lens to the box immediately behind any lenses already in the box. Don't move any of the other lenses when you do this. If there aren't any lenses in the box, the new lens goes all the way to the front of the box.

#[derive(Clone)]
struct Lens {
    label: String,
    focal: usize,
}

fn solve_two(input: &Input) -> Result<Answer> {
    let Input { sequence } = input;
    let mut lenses: Vec<Vec<Lens>> = vec![Vec::new(); 256];
    let re_add = regex!(r"(\w+)=(\d)");
    let re_sub = regex!(r"(\w+)-");
    for seq in sequence {
        if re_add.is_match(seq) {
            let caps = re_add.captures(seq).unwrap();
            let label = String::from(caps.get(1).unwrap().as_str());
            let target_box = hash(&label);
            let focal = caps.get(2).unwrap().as_str().parse().unwrap();
            let mut present = false;
            for lens in lenses[target_box].iter_mut() {
                if lens.label == label {
                    present = true;
                    lens.focal = focal;
                }
            }
            if !present {
                lenses[target_box].push(Lens { label, focal });
            }
        } else if re_sub.is_match(seq) {
            let caps = re_sub.captures(seq).unwrap();
            let label = String::from(caps.get(1).unwrap().as_str());
            let target_box = hash(&label);
            lenses[target_box].retain(|l| l.label != label);
        } else {
            panic!("unknown command {}", seq);
        }
    }
    // focus power
    let mut focus_power = 0;
    for (i, boxi) in lenses.iter().enumerate() {
        let box_mul = i + 1;
        for (j, l) in boxi.iter().enumerate() {
            let slot_mul = j + 1;
            focus_power += box_mul * slot_mul * l.focal;
        }
    }
    Ok(Answer::Num(focus_power as i128))
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
        assert_eq!(answer, Answer::Num(1320));
        Ok(())
    }
    #[test]
    fn part_one() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(517315));
        Ok(())
    }
    #[test]
    fn test_two() -> Result<()> {
        let answer = super::part_two(&TEST)?;
        assert_eq!(answer, Answer::Num(145));
        Ok(())
    }
    #[test]
    fn part_two() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(answer, Answer::Num(247763));
        Ok(())
    }
    #[test]
    fn test_hash() {
        let answer = hash("HASH");
        assert_eq!(answer, 52);
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

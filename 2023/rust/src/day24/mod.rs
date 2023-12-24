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

// min x, y at least 200000000000000 at most 400000000000000 => i128

struct Input {
    hailstones: Vec<Hailstone>,
}

impl Input {
    fn xy_crossings_in_area(&self, min: f64, max: f64) -> usize {
        let mut crossings = 0;
        let len = self.hailstones.len();
        for i in 0..len {
            let a = &self.hailstones[i];
            for j in i + 1..len {
                let b = &self.hailstones[j];
                let intersection = a.intersect_xy(&b);
                // println!("a = {:?}", a);
                // println!("b = {:?}", b);
                match intersection {
                    Intersection::None => {
                        // println!("no intersection");
                    }
                    Intersection::PastPoint(_, (a, b)) => {
                        // match (a, b) {
                        //     (true, true) => println!("intersection in past for both"),
                        //     (true, false) => println!("intersection in past for A"),
                        //     (false, true) => println!("intersection in past for B"),
                        //     _ => panic!("this should never happen, intersection is not in past"),
                        // }
                    }
                    Intersection::Point(p) => {
                        // println!("intersection at {:?}", p);
                        if min <= p.0 && p.0 <= max && min <= p.1 && p.1 <= max {
                            crossings += 1;
                        }
                    }
                }
            }
        }
        crossings
    }
}

#[derive(Debug)]
struct Hailstone {
    pos: (i128, i128, i128),
    delta: (i128, i128, i128),
}

// d* are never 0!
// but n*(dx, dy) = (da, db)
// n*dx = da => n = da/dx
// (da/dx)*dy = db => da*dy = db*dx => then only intersect if (x, y) + n * (dx, dy) = (a, b)
// x + n*dx = a => n = (a-x)/dx
// y + n*dy = b => n = (b-y)/dy
// => (a-x)/dx =!= (b-y)/dy => (a-x)*dy =!= (b-y)*dx

// (x, y) + n * (dx, dy) = (a, b) + m * (da, db)
// x + n*dx = a + m*da => n = (a + m*da - x) / dx = a/dx + m*da/dx - x/dx
// y + n*dy = b + m*db => y/db + a*dy/(dx*db) - x*dy/(dx*db) - b/db = m - m*(da*dy)/(dx*db) = m*(1-(da*dy)/(dx*db))

// m = (y/db + a*dy/(dx*db) - x*dy/(dx*db) - b/db) / (1-(da*dy)/(dx*db))
//   = (y - b)*dx/(dx*db - da*dy) + (a*dy - x*dy)/(dx*db - da*dy)
//   = (y*dx - b*dx + a*dy - x*dy) / (dx*db - da*dy)

enum Intersection {
    None,
    PastPoint((f64, f64), (bool, bool)),
    Point((f64, f64)),
}

impl Hailstone {
    fn intersect_xy(&self, other: &Hailstone) -> Intersection {
        let (x, y) = (self.pos.0 as f64, self.pos.1 as f64);
        let (dx, dy) = (self.delta.0 as f64, self.delta.1 as f64);
        let (a, b) = (other.pos.0 as f64, other.pos.1 as f64);
        let (da, db) = (other.delta.0 as f64, other.delta.1 as f64);
        let div = dx * db - da * dy;
        if div == 0.0 {
            if (a - x) * dy == (b - y) * dx {
                // intersect at ?
                panic!("not implemented for now");
            } else {
                // parallel -> no intersection
                return Intersection::None;
            }
        }
        let m = ((y - b) * dx + (a - x) * dy) / div;
        let ix = a + m * da;
        let iy = b + m * db;
        let n = (ix - x) / dx;
        if m < 0.0 || n < 0.0 {
            Intersection::PastPoint((ix, iy), (m < 0.0, n < 0.0))
        } else {
            Intersection::Point((ix, iy))
        }
    }
}

fn parse_input(input: &str) -> Result<Input> {
    let mut hailstones = Vec::new();
    for line in input.lines() {
        let (posstr, deltastr) = line.split_once("@").unwrap();
        let pos: Vec<i128> = posstr
            .trim()
            .split(",")
            .map(|s| s.trim().parse().unwrap())
            .collect();
        let delta: Vec<i128> = deltastr
            .trim()
            .split(",")
            .map(|s| s.trim().parse().unwrap())
            .collect();
        hailstones.push(Hailstone {
            pos: (pos[0], pos[1], pos[2]),
            delta: (delta[0], delta[1], delta[2]),
        });
    }
    Ok(Input { hailstones })
}

fn solve_one(input: &Input) -> Result<Answer> {
    let sum = input.xy_crossings_in_area(7.0, 27.0);
    println!("sum = {}", sum);
    let answer = input.xy_crossings_in_area(200000000000000.0, 400000000000000.0);
    Ok(Answer::Num(answer as i128))
}

fn solve_two(input: &Input) -> Result<Answer> {
    let sum = input.xy_crossings_in_area(7.0, 27.0);
    println!("sum = {}", sum);
    Ok(Answer::Num(-1))
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
        assert_eq!(answer, Answer::Num(-1));
        Ok(())
    }
    #[test]
    fn part_one() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(-1));
        Ok(())
    }
    #[test]
    fn test_two() -> Result<()> {
        let answer = super::part_two(&TEST)?;
        assert_eq!(answer, Answer::Num(-1));
        Ok(())
    }
    #[test]
    fn part_two() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(answer, Answer::Num(-1));
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

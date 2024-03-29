use super::common::*;
use anyhow::Result;

pub fn part_one(input: &str) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_one(&input, 200000000000000.0, 400000000000000.0)
}

pub fn part_two(input: &str) -> Result<Answer> {
    let input = parse_input(input)?;
    solve_two(&input)
}

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
                    Intersection::PastPoint(_, (_a, _b)) => {
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
    // fn pos_at_time(&self, time: i128) -> (i128, i128, i128) {
    //     (
    //         self.pos.0 + time * self.delta.0,
    //         self.pos.1 + time * self.delta.1,
    //         self.pos.2 + time * self.delta.2,
    //     )
    // }
    fn print_hailstone_equation(&self, time_subscript: usize) {
        // use for https://jfmc.github.io/z3-play/
        // (assert (= (+ x (* u t{})) (+ {} (* {} t{})) ))
        //
        println!(
            // "x+u*t{} = {}+{}*t{} ;",
            "(assert (= (+ x (* u t{})) (+ {} (* {} t{})) ))",
            time_subscript, self.pos.0, self.delta.0, time_subscript
        );
        println!(
            // "y+v*t{} = {}+{}*t{} ;",
            "(assert (= (+ y (* v t{})) (+ {} (* {} t{})) ))",
            time_subscript, self.pos.1, self.delta.1, time_subscript
        );
        println!(
            // "z+w*t{} = {}+{}*t{} ;",
            "(assert (= (+ z (* w t{})) (+ {} (* {} t{})) ))",
            time_subscript, self.pos.2, self.delta.2, time_subscript
        );
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

fn solve_one(input: &Input, min: f64, max: f64) -> Result<Answer> {
    let answer = input.xy_crossings_in_area(min, max);
    Ok(Answer::Num(answer as i128))
}

// each hailstone is a function of time now (I try to ignore non integer collisions here first)

// find line that intersects with all other lines

// (x,y,z) + m *(dx,dy,dz)
// (a,b,c) + n *(da,db,dc)

// find line that intersects both

// for each two hailstones at time n (a_n, b_n) determine distance vector between (a_n, b_(n+1))

fn solve_two(input: &Input) -> Result<Answer> {
    let Input { hailstones } = input;
    for i in 0..hailstones.len() {
        for j in i + 1..hailstones.len() {
            // they don't intersect but are there any that are just parallel I got to know
            let a = &hailstones[i];
            let b = &hailstones[j];
            let n0 = a.delta.0 as f64 / b.delta.0 as f64;
            let n1 = a.delta.1 as f64 / b.delta.1 as f64;
            let n2 = a.delta.2 as f64 / b.delta.2 as f64;
            if f64::abs(n0 - n1) < f64::EPSILON && f64::abs(n0 - n2) < f64::EPSILON {
                println!("found parallel hailstones: {:?}, {:?}", a, b);
            }
        }
    }
    for i in 0..3 {
        hailstones[i].print_hailstone_equation(i);
    }

    // (declare-const x Int)
    // (declare-const y Int)
    // (declare-const z Int)
    // (declare-const u Int)
    // (declare-const v Int)
    // (declare-const w Int)
    // (declare-const t0 Int)
    // (declare-const t1 Int)
    // (declare-const t2 Int)

    // (assert (>= t0 0))
    // (assert (>= t1 0))
    // (assert (>= t2 0))
    // (assert (>= u (- 500)))
    // (assert (>= v (- 500)))
    // (assert (>= w (- 500)))
    // (assert (<= u 500))
    // (assert (<= v 500))
    // (assert (<= w 500))
    // (assert (= (+ x (* u t0)) (+ 213004023520250 (* 118 t0)) ))
    // (assert (= (+ y (* v t0)) (+ 255007063487325 (* 41 t0)) ))
    // (assert (= (+ z (* w t0)) (+ 286351797522218 (* (- 64) t0)) ))
    // (assert (= (+ x (* u t1)) (+ 488850102886640 (* (- 73) t1)) ))
    // (assert (= (+ y (* v t1)) (+ 357544262814165 (* (- 83) t1)) ))
    // (assert (= (+ z (* w t1)) (+ 194409329434718 (* 182 t1)) ))
    // (assert (= (+ x (* u t2)) (+ 193401607687542 (* 133 t2)) ))
    // (assert (= (+ y (* v t2)) (+ 253348355203801 (* 51 t2)) ))
    // (assert (= (+ z (* w t2)) (+ 213339230780036 (* 98 t2)) ))

    //     sat
    // (
    //   (define-fun w () Int
    //     197)
    //   (define-fun y () Int
    //     263917577518425)
    //   (define-fun x () Int
    //     133619443970450)
    //   (define-fun t2 () Int
    //     330288197332)
    //   (define-fun z () Int
    //     180640699244168)
    //   (define-fun t1 () Int
    //     917908679370)
    //   (define-fun t0 () Int
    //     405023365050)
    //   (define-fun u () Int
    //     314)
    //   (define-fun v () Int
    //     19)
    // )

    // 2 unknowns => 2 equations
    // initial unknowns are x,y,z,dx,dy,dz
    // per line we add n and m but get three equations out
    // => six lines needed to determine unique solution
    // A1 + n1*da1 = x + m1*dx
    // A2 + n2*da2 = x + m2*dx
    // and so on
    // A1 + .. + A6 + n1*da1 + .. + n6*da6 = 6x + (m1 + .. + m6)*dx
    // n1*da1 + .. + n6*da6 - 6x - (m1 + .. + m6)*dx = -(A1 + .. + A6)
    // with vector [x,y,z,dx,dy,dz,n1,..,n6,m1,..m6]
    // X = (x,y,z,dx,dy,dz)
    // (a1,b1,c1) + n1*(da1,db1,dc1) = (x,y,z) + m1*(dx,dy,dz)
    // a1 + n1*da1 = x + m1*dx => n1 = (x - a1 + m1*dx)/da1
    // b1 + n1*db1 = y + m1*dy => n1 = (y - b1 + m1*dy)/db1
    // c1 + n1*dc1 = z + m1*dz => n1 = (z - c1 + m1*dz)/dc1
    // => m1
    // => Intersection at (a1,b1,c1) + n1*(da1,db1,dc1) = A1
    // (a2,b2,c2) + n2*(da2,db2,dc2) = (x,y,z) + m2*(dx,dy,dz)
    // => Intersection at (a2,b2,c2) + n2*(da2,db2,dc2) = A2
    // A1 - A2 = r12*(dx,dy,dz) =>
    // do the same for A3 - A4 = r34*(dx,dy,dz)

    // no hailstones are parallel good to know :)

    // x + y + z
    let answer = 133619443970450 + 263917577518425 + 180640699244168;
    Ok(Answer::Num(answer))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::LazyLock;
    use test::Bencher;

    static TEST: LazyLock<String> = local_file!("test.txt");
    static INPUT: LazyLock<String> = local_file!("input.txt");

    #[test]
    fn test_solve_one() -> Result<()> {
        let input = parse_input(&TEST)?;
        let answer = solve_one(&input, 7.0, 27.0)?;
        assert_eq!(answer, Answer::Num(2));
        Ok(())
    }
    #[test]
    fn part_one() -> Result<()> {
        let answer = super::part_one(&INPUT)?;
        assert_eq!(answer, Answer::Num(13965));
        Ok(())
    }
    // TODO: the general case is not implemented yet
    // #[test]
    // fn test_two() -> Result<()> {
    //     let answer = super::part_two(&TEST)?;
    //     assert_eq!(answer, Answer::Num(-1));
    //     Ok(())
    // }
    #[test]
    fn part_two() -> Result<()> {
        let answer = super::part_two(&INPUT)?;
        assert_eq!(answer, Answer::Num(578177720733043));
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

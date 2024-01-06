use sky::readfile;
use std::ops::RangeInclusive;

// Several concepts for part II cribbed from /u/TheZigerionScammer in Reddit's r/adventofcode

type Number = i64;

const TEST: RangeInclusive<Number> = 200000000000000..=400000000000000;

type Coord = (Number, Number, Number);
type Velocity = (Number, Number, Number);

#[derive(Copy, Clone, Debug)]
struct Hailstone {
    start: Coord,
    velocity: Velocity,
}

#[derive(Copy, Clone, Debug)]
enum Intersect {
    Never,
    Always,
    At(f64, f64),
}

impl Hailstone {
    fn hit_at_2d(&self, other: &Self) -> Intersect {
        // Can't handle verticals, no real inputs seem to have zero velocity on any axis
        assert_ne!(self.velocity.0, 0);
        assert_ne!(other.velocity.0, 0);

        fn two_d(any: (Number, Number, Number)) -> (f64, f64) {
            (any.0 as f64, any.1 as f64)
        }

        let (sxs, sys) = two_d(self.start);
        let (oxs, oys) = two_d(other.start);
        let (sxv, syv) = two_d(self.velocity);
        let (oxv, oyv) = two_d(other.velocity);

        let a = syv / sxv;
        let b = oyv / oxv;
        let c = sys - ((sxs * syv) / sxv);
        let d = oys - ((oxs * oyv) / oxv);

        if a == b {
            if c == d {
                // 2D Overlap (but not necessarily in time)
                return Intersect::Always;
            } else {
                // 2D Parallel
                return Intersect::Never;
            }
        }
        let x = (d - c) / (a - b);
        let y = a * x + c;

        Intersect::At(x, y)
    }

    fn hits(&self, other: &Self) -> bool {
        match self.hit_at_2d(other) {
            Intersect::Never => false,
            Intersect::Always => {
                // Check both stones are in the test area in the future?
                // Neither example nor our input do this
                panic!("We don't handle 2D overlap, need to test time overlap")
            }
            Intersect::At(x, y) => {
                let x = x as Number;
                let y = y as Number;
                // Check the hit happens in the test area
                if !TEST.contains(&x) || !TEST.contains(&y) {
                    return false;
                }

                // Check the hit happens in the future for both stones
                if x > self.start.0 && self.velocity.0 < 0 {
                    return false;
                }
                if x < self.start.0 && self.velocity.0 > 0 {
                    return false;
                }
                if x > other.start.0 && other.velocity.0 < 0 {
                    return false;
                }
                if x < other.start.0 && other.velocity.0 > 0 {
                    return false;
                }
                true
            }
        }
    }
}

fn triple(s: &str) -> (Number, Number, Number) {
    let v: Vec<Number> = s
        .split(", ")
        .filter_map(|s| s.trim().parse().ok())
        .collect();
    assert_eq!(v.len(), 3);
    (v[0], v[1], v[2])
}

use std::str::FromStr;
impl FromStr for Hailstone {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((start, velocity)) = s.split_once(" @ ") else {
            return Err("No @ symbol separating position from velocity");
        };
        let start = triple(start);
        let velocity = triple(velocity);
        Ok(Hailstone { start, velocity })
    }
}

pub fn a() {
    let ctxt = readfile("24");
    let mut stones: Vec<Hailstone> = ctxt.lines().filter_map(|s| s.parse().ok()).collect();

    let mut hits = 0;
    while let Some(stone) = stones.pop() {
        for other in stones.iter() {
            if stone.hits(other) {
                hits += 1;
            }
        }
    }
    println!("{hits} intersections occur in the test area on future paths");
}

fn velocity(stones: &[Hailstone]) -> Velocity {
    let mut sorted = Vec::new();
    sorted.extend_from_slice(stones);
    sorted.sort_unstable_by_key(|stone| stone.velocity.0);

    let mut vx: Option<Number> = None;
    'outer: for guess in -1000..=1000 {
        let mut old: Option<Hailstone> = None;
        for stone in sorted.iter() {
            let vel = stone.velocity.0;
            if old.is_some() && old.unwrap().velocity.0 == vel {
                let dist_diff = stone.start.0 - old.unwrap().start.0;
                let vel_diff = guess - vel;
                if vel_diff != 0 && dist_diff % vel_diff != 0 {
                    continue 'outer;
                }
            }
            old = Some(*stone);
        }
        vx = Some(guess);
        break 'outer;
    }

    sorted.sort_unstable_by_key(|stone| stone.velocity.1);

    let mut vy: Option<Number> = None;
    'outer: for guess in -1000..=1000 {
        let mut old: Option<Hailstone> = None;
        for stone in sorted.iter() {
            let vel = stone.velocity.1;
            if old.is_some() && old.unwrap().velocity.1 == vel {
                let dist_diff = stone.start.1 - old.unwrap().start.1;
                let vel_diff = guess - vel;
                if vel_diff != 0 && dist_diff % vel_diff != 0 {
                    continue 'outer;
                }
            }
            old = Some(*stone);
        }
        vy = Some(guess);
        break 'outer;
    }

    sorted.sort_unstable_by_key(|stone| stone.velocity.2);

    let mut vz: Option<Number> = None;
    'outer: for guess in -1000..=1000 {
        let mut old: Option<Hailstone> = None;
        for stone in sorted.iter() {
            let vel = stone.velocity.2;
            if old.is_some() && old.unwrap().velocity.2 == vel {
                let dist_diff = stone.start.2 - old.unwrap().start.2;
                let vel_diff = guess - vel;
                if vel_diff != 0 && dist_diff % vel_diff != 0 {
                    continue 'outer;
                }
            }
            old = Some(*stone);
        }
        vz = Some(guess);
        break 'outer;
    }

    let vx = vx.expect("There should be a single plausible X velocity");
    let vy = vy.expect("There should be a single plausible Y velocity");
    let vz = vz.expect("There should be a single plausible Z velocity");
    (vx, vy, vz)
}

#[derive(Copy, Clone, Debug)]
struct Rational {
    numerator: i128,
    denominator: i128,
}

impl Rational {
    fn new(n: Number) -> Self {
        Self {
            numerator: n.into(),
            denominator: 1,
        }
    }

    fn whole(self) -> Number {
        let i = self.numerator / self.denominator;
        let check = i * self.denominator;
        assert_eq!(check, self.numerator);
        i.try_into()
            .expect("Whole values should fit in the Number type")
    }

    fn simplify(mut numerator: i128, mut denominator: i128) -> Self {
        let divisor = num::Integer::gcd(&numerator, &denominator);
        numerator /= divisor;
        denominator /= divisor;

        Self {
            numerator,
            denominator,
        }
    }

    fn subtract(self, other: Self) -> Self {
        let numerator = self.numerator * other.denominator - other.numerator * self.denominator;
        let denominator = self.denominator * other.denominator;
        Self::simplify(numerator, denominator)
    }

    fn multiply(self, other: Self) -> Self {
        let numerator = self.numerator * other.numerator;
        let denominator = self.denominator * other.denominator;
        Self::simplify(numerator, denominator)
    }

    fn divide(self, other: Self) -> Self {
        let numerator = self.numerator * other.denominator;
        let denominator = self.denominator * other.numerator;
        Self::simplify(numerator, denominator)
    }
}

fn position(stones: &[Hailstone], vel: Velocity) -> Coord {
    let mut modified = Vec::new();
    modified.extend_from_slice(stones);
    for stone in modified.iter_mut() {
        stone.velocity.0 -= vel.0;
        stone.velocity.1 -= vel.1;
        stone.velocity.2 -= vel.2;
    }

    let mut last = modified.pop().expect("None of this works without stones");
    while last.velocity.0 == 0 {
        last = modified
            .pop()
            .expect("Surely not all the stones have X velocity 0");
    }

    for stone in modified {
        // Skip stones with no X velocity after modification
        if stone.velocity.0 == 0 {
            continue;
        }

        fn two_d(any: (Number, Number, Number)) -> (Rational, Rational) {
            (Rational::new(any.0), Rational::new(any.1))
        }

        let (sxs, sys) = two_d(stone.start);
        let (oxs, oys) = two_d(last.start);
        let (sxv, syv) = two_d(stone.velocity);
        let (oxv, oyv) = two_d(last.velocity);

        let a = syv.divide(sxv);
        let b = oyv.divide(oxv);
        let c = sys.subtract(sxs.multiply(syv).divide(sxv));
        let d = oys.subtract(oxs.multiply(oyv).divide(oxv));

        let x = d.subtract(c).divide(a.subtract(b)).whole();

        let time = (x - stone.start.0) / stone.velocity.0;
        let px = stone.start.0 + (time * stone.velocity.0);
        let py = stone.start.1 + (time * stone.velocity.1);
        let pz = stone.start.2 + (time * stone.velocity.2);
        return (px, py, pz);
    }

    panic!("Stones exhausted without discovering a solution");
}

pub fn b() {
    let ctxt = readfile("24");
    let stones: Vec<Hailstone> = ctxt.lines().filter_map(|s| s.parse().ok()).collect();
    let vel = velocity(&stones);
    let pos = position(&stones, vel);
    println!(
        "3D velocities are {vel:?}. 3D start position is {pos:?}. Therefore puzzle answer is {}",
        pos.0 + pos.1 + pos.2
    );
}

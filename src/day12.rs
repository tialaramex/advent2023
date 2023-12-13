use sky::readfile;

type Number = u64;
type Bits = u128;

const OP: u8 = b'.';
const DAMAGED: u8 = b'#';
const UNKNOWN: u8 = b'?';

#[derive(Copy, Clone, Debug)]
struct Arrangement {
    // Condition bits are zero if known operation, one if known damaged or if unknown
    condition: Bits,
    // Mask bits indicate condition is actually unknown
    mask: Bits,
    done: usize,
    count: Number,
}

#[derive(Debug)]
enum Outcome {
    Fail,
    Success,
    Onward(Arrangement),
    Split(Arrangement, Arrangement),
}

impl Arrangement {
    fn new(report: &str) -> Self {
        let mut condition: Bits = 0;
        let mut mask: Bits = 0;
        for b in report.bytes().rev() {
            match b {
                OP => {
                    condition <<= 1;
                    mask <<= 1;
                }
                DAMAGED => {
                    condition = (condition << 1) | 1;
                    mask <<= 1;
                }
                UNKNOWN => {
                    condition = (condition << 1) | 1;
                    mask = (mask << 1) | 1;
                }
                _ => panic!("Invalid damage report: {report}"),
            }
        }
        Self {
            condition,
            mask,
            done: 0,
            count: 0,
        }
    }

    fn step(mut self, correct: &[Number]) -> Outcome {
        while self.condition > 0 && self.mask & 1 == 0 {
            // Known state resolve
            self.mask >>= 1;
            let damaged = self.condition & 1;
            self.condition >>= 1;

            if damaged == 1 {
                // Damaged
                if self.done == correct.len() || self.count == correct[self.done] {
                    return Outcome::Fail;
                } else {
                    self.count += 1;
                }
            } else {
                // Operational
                if self.count == 0 {
                    continue;
                }

                if self.count == correct[self.done] {
                    self.done += 1;
                    self.count = 0;
                    continue;
                }

                return Outcome::Fail;
            }
        }

        if self.condition == 0 && self.mask == 0 {
            // Run out of bits, either fail or success
            if (self.count == 0 && self.done == correct.len())
                || (self.done == correct.len() - 1 && self.count == correct[self.done])
            {
                return Outcome::Success;
            } else {
                return Outcome::Fail;
            }
        }

        self.mask >>= 1;
        self.condition >>= 1;

        if self.count == 0 {
            if self.done < correct.len() {
                let other = self;
                self.count += 1;
                return Outcome::Split(self, other);
            } else {
                return Outcome::Onward(self);
            }
        }

        if self.count == correct[self.done] {
            // Enough damaged, stop
            self.done += 1;
            self.count = 0;
        } else {
            // Ongoing damaged springs
            self.count += 1;
        }

        Outcome::Onward(self)
    }
}

fn ordinary(line: &str) -> Number {
    let (springs, nums) = line
        .split_once(' ')
        .expect("Each line should have a space in it");
    let nums: Vec<Number> = nums
        .split(',')
        .map(|n| n.parse::<Number>().expect("Should be an integer"))
        .collect();
    let mut v = Vec::new();
    v.push(Arrangement::new(springs));
    let mut count = 0;
    while let Some(arr) = v.pop() {
        match arr.step(&nums) {
            Outcome::Fail => {}
            Outcome::Success => count += 1,
            Outcome::Onward(arr) => {
                v.push(arr);
            }
            Outcome::Split(one, two) => {
                v.push(one);
                v.push(two);
            }
        }
    }
    count
}

use std::collections::HashMap;
type Memo = HashMap<(Bits, usize, Number), Number>;

fn working(arr: Arrangement, correct: &[Number], memo: &mut Memo) -> Number {
    if let Some(&answer) = memo.get(&(arr.mask, arr.done, arr.count)) {
        return answer;
    }
    match arr.step(correct) {
        Outcome::Fail => {
            memo.insert((arr.mask, arr.done, arr.count), 0);
            0
        }
        Outcome::Success => {
            memo.insert((arr.mask, arr.done, arr.count), 1);
            1
        }
        Outcome::Onward(arr) => working(arr, correct, memo),
        Outcome::Split(one, two) => {
            let answer = working(one, correct, memo) + working(two, correct, memo);
            memo.insert((arr.mask, arr.done, arr.count), answer);
            answer
        }
    }
}

fn revised(line: &str) -> Number {
    let (springs, nums) = line
        .split_once(' ')
        .expect("Each line should have a space in it");
    let nums: Vec<Number> = nums
        .split(',')
        .map(|n| n.parse::<Number>().expect("Should be an integer"))
        .collect();

    let mut correct = nums.clone();
    for _ in 0..4 {
        correct.extend_from_slice(&nums);
    }

    let more = format!("{springs}?{springs}?{springs}?{springs}?{springs}");
    let mut memo = HashMap::new();
    working(Arrangement::new(&more), &correct, &mut memo)
}

pub fn a() {
    let ctxt = readfile("12");
    let mut sum = 0;
    for line in ctxt.lines() {
        let count = ordinary(line);
        sum += count;
    }
    println!("Sum of counts of different arrangements is: {sum}");
}

pub fn b() {
    let ctxt = readfile("12");
    let mut sum = 0;
    for line in ctxt.lines() {
        let count = revised(line);
        sum += count;
    }
    println!("After unfolding, now sum of counts is: {sum}");
}

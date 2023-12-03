use sky::map::Map;
use sky::readfile;

type Number = u32;

#[derive(Copy, Clone, Default, Eq, PartialEq)]
enum Code {
    #[default]
    Empty,
    Digit(Number),
    Symbol(char),
}

impl From<char> for Code {
    fn from(ch: char) -> Self {
        match ch {
            '.' => Self::Empty,
            '0' => Self::Digit(0),
            '1' => Self::Digit(1),
            '2' => Self::Digit(2),
            '3' => Self::Digit(3),
            '4' => Self::Digit(4),
            '5' => Self::Digit(5),
            '6' => Self::Digit(6),
            '7' => Self::Digit(7),
            '8' => Self::Digit(8),
            '9' => Self::Digit(9),
            ch => Self::Symbol(ch),
        }
    }
}

type Schematic = Map<Code>;

use std::fmt::{Display, Formatter};

impl Display for Code {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Empty => f.write_str("."),
            Self::Digit(n) => f.write_str(&format!("{n}")),
            Self::Symbol(ch) => f.write_str(&format!("{ch}")),
        }
    }
}

pub fn a() {
    let ctxt = readfile("03");
    let schematic: Schematic = ctxt.value().parse().unwrap();
    let numbers = schematic.find(|code| matches!(code, Code::Digit(_)));
    let mut sum = 0;
    for (x, y) in numbers {
        // Subsequent digits don't count, move on
        if matches!(schematic.read(x - 1, y), Some(Code::Digit(_))) {
            continue;
        }
        let Some(Code::Digit(mut n)) = schematic.read(x, y) else {
            panic!("({x},{y}) should be a digit but it is not");
        };
        let mut width = 1;
        while let Some(Code::Digit(d)) = schematic.read(x + width, y) {
            width += 1;
            n = (n * 10) + d;
        }
        // Check for symbols
        if matches!(schematic.read(x - 1, y), Some(Code::Symbol(_))) {
            sum += n;
            continue;
        }
        if matches!(schematic.read(x + width, y), Some(Code::Symbol(_))) {
            sum += n;
            continue;
        }
        for step in -1..=width {
            if matches!(schematic.read(x + step, y - 1), Some(Code::Symbol(_))) {
                sum += n;
                continue;
            }
            if matches!(schematic.read(x + step, y + 1), Some(Code::Symbol(_))) {
                sum += n;
                continue;
            }
        }
    }
    println!("Sum of all part numbers is: {sum}");
}

fn read_number(schematic: &Schematic, mut x: isize, y: isize) -> Number {
    // Find start
    while let Some(Code::Digit(_)) = schematic.read(x - 1, y) {
        x -= 1;
    }
    let mut n = 0;
    // Read left-to-right
    while let Some(Code::Digit(d)) = schematic.read(x, y) {
        x += 1;
        n = (n * 10) + d;
    }
    n
}

fn check_one(schematic: &Schematic, x: isize, y: isize) -> Option<Number> {
    let code = schematic.read(x, y).unwrap_or_default();
    match code {
        Code::Digit(_) => Some(read_number(schematic, x, y)),
        _ => None,
    }
}

fn check_gear(schematic: &Schematic, x: isize, y: isize) -> Option<Number> {
    let mut nums: Vec<Number> = Vec::new();

    // left
    match check_one(schematic, x - 1, y) {
        Some(n) => nums.push(n),
        None => {}
    }

    // right
    match check_one(schematic, x + 1, y) {
        Some(n) => nums.push(n),
        None => {}
    }

    // above
    match check_one(schematic, x, y - 1) {
        Some(n) => nums.push(n),
        None => {
            match check_one(schematic, x - 1, y - 1) {
                Some(n) => nums.push(n),
                None => {}
            }
            match check_one(schematic, x + 1, y - 1) {
                Some(n) => nums.push(n),
                None => {}
            }
        }
    }

    // below
    match check_one(schematic, x, y + 1) {
        Some(n) => nums.push(n),
        None => {
            match check_one(schematic, x - 1, y + 1) {
                Some(n) => nums.push(n),
                None => {}
            }
            match check_one(schematic, x + 1, y + 1) {
                Some(n) => nums.push(n),
                None => {}
            }
        }
    }

    if nums.len() == 2 {
        Some(nums.into_iter().product())
    } else {
        None
    }
}

pub fn b() {
    let ctxt = readfile("03");
    let schematic: Schematic = ctxt.value().parse().unwrap();
    // Find the gears, start with '*'
    let possible = schematic.find(|code| matches!(code, Code::Symbol('*')));
    let mut sum = 0;
    for (x, y) in possible {
        let Some(ratio) = check_gear(&schematic, x, y) else {
            continue;
        };
        sum += ratio;
    }
    println!("Sum of all gear ratios is: {sum}");
}

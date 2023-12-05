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
            d @ '0'..='9' => Self::Digit(d.to_digit(10).unwrap()),
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
    if let Some(n) = check_one(schematic, x - 1, y) {
        nums.push(n);
    }

    // right
    if let Some(n) = check_one(schematic, x + 1, y) {
        nums.push(n);
    }

    // above
    if let Some(n) = check_one(schematic, x, y - 1) {
        nums.push(n);
    } else {
        if let Some(n) = check_one(schematic, x - 1, y - 1) {
            nums.push(n);
        }
        if let Some(n) = check_one(schematic, x + 1, y - 1) {
            nums.push(n);
        }
    }

    // below
    if let Some(n) = check_one(schematic, x, y + 1) {
        nums.push(n);
    } else {
        if let Some(n) = check_one(schematic, x - 1, y + 1) {
            nums.push(n);
        }
        if let Some(n) = check_one(schematic, x + 1, y + 1) {
            nums.push(n);
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

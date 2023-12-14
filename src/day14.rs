use sky::map::Map;
use sky::readfile;

type Number = u32;

#[derive(Copy, Clone, Default, Eq, PartialEq)]
enum Rock {
    #[default]
    Empty,
    Round,
    Cube,
}

impl From<char> for Rock {
    fn from(ch: char) -> Self {
        match ch {
            '.' => Self::Empty,
            'O' => Self::Round,
            '#' => Self::Cube,
            _ => panic!("{ch} is not a Rock"),
        }
    }
}

type Dish = Map<Rock>;

use std::fmt::{Display, Formatter};

impl Display for Rock {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Empty => f.write_str("."),
            Self::Round => f.write_str("O"),
            Self::Cube => f.write_str("#"),
        }
    }
}

use std::collections::HashSet;
type History = HashSet<String>;

fn tilt_north(mut dish: Dish) -> Dish {
    for x in dish.x() {
        let mut stop: isize = 0;

        for y in dish.y() {
            match dish.read(x, y).unwrap_or_default() {
                Rock::Empty => {}
                Rock::Round => {
                    if stop < y {
                        // Roll
                        dish.write(x, stop, Rock::Round);
                        dish.write(x, y, Rock::Empty);
                    }
                    stop += 1;
                }
                Rock::Cube => {
                    stop = y + 1;
                }
            }
        }
    }
    dish
}

fn tilt_west(mut dish: Dish) -> Dish {
    for y in dish.y() {
        let mut stop: isize = 0;

        for x in dish.x() {
            match dish.read(x, y).unwrap_or_default() {
                Rock::Empty => {}
                Rock::Round => {
                    if stop < x {
                        // Roll
                        dish.write(stop, y, Rock::Round);
                        dish.write(x, y, Rock::Empty);
                    }
                    stop += 1;
                }
                Rock::Cube => {
                    stop = x + 1;
                }
            }
        }
    }
    dish
}

fn tilt_south(mut dish: Dish) -> Dish {
    let &edge = dish.y().end();
    for x in dish.x() {
        let mut stop: isize = edge;

        for y in dish.y().rev() {
            match dish.read(x, y).unwrap_or_default() {
                Rock::Empty => {}
                Rock::Round => {
                    if stop > y {
                        // Roll
                        dish.write(x, stop, Rock::Round);
                        dish.write(x, y, Rock::Empty);
                    }
                    stop -= 1;
                }
                Rock::Cube => {
                    stop = y - 1;
                }
            }
        }
    }
    dish
}

fn tilt_east(mut dish: Dish) -> Dish {
    let &edge = dish.x().end();
    for y in dish.y() {
        let mut stop: isize = edge;

        for x in dish.x().rev() {
            match dish.read(x, y).unwrap_or_default() {
                Rock::Empty => {}
                Rock::Round => {
                    if stop > x {
                        // Roll
                        dish.write(stop, y, Rock::Round);
                        dish.write(x, y, Rock::Empty);
                    }
                    stop -= 1;
                }
                Rock::Cube => {
                    stop = x - 1;
                }
            }
        }
    }
    dish
}

fn cycle(dish: Dish) -> Dish {
    tilt_east(tilt_south(tilt_west(tilt_north(dish))))
}

fn load(dish: Dish) -> isize {
    let height = dish.y().end() + 1;
    dish.find(|r| r == Rock::Round)
        .iter()
        .map(|(_, y)| height - y)
        .sum()
}

pub fn a() {
    let ctxt = readfile("14");
    let dish: Dish = ctxt.value().parse().unwrap();
    let dish = tilt_north(dish);
    let total = load(dish);
    println!("Total load for the North support beams after tilting North is: {total}");
}

const BILLION: Number = 1_000_000_000;

pub fn b() {
    let ctxt = readfile("14");
    let mut dish: Dish = ctxt.value().parse().unwrap();

    let mut history: History = HashSet::new();
    let mut first: Option<Number> = None;
    for k in 0..1000 {
        let img = dish.to_string();
        if history.contains(&img) {
            first = Some(k);
            break;
        }
        history.insert(img);
        dish = cycle(dish);
    }
    history.clear();
    let Some(first) = first else {
        panic!("Pattern doesn't repeat at all in first 1000 cycles");
    };
    let mut repeats: Option<Number> = None;
    for k in 0..1000 {
        let img = dish.to_string();
        if history.contains(&img) {
            repeats = Some(k);
            break;
        }
        history.insert(img);
        dish = cycle(dish);
    }
    let Some(repeats) = repeats else {
        panic!("Somehow the repetition stopped working");
    };

    let loops = (BILLION - first) / repeats;
    let extra = BILLION - first - repeats * loops;
    for _ in 0..extra {
        dish = cycle(dish);
    }
    let total = load(dish);
    println!("Total load for the North support beams after {BILLION} iterations is: {total}");
}

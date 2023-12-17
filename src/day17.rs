use sky::map::Map;
use sky::readfile;

type Number = u32;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Block {
    loss: u8,
}

impl From<char> for Block {
    fn from(ch: char) -> Self {
        match ch {
            d @ '1'..='9' => Self {
                loss: d.to_digit(10).unwrap().try_into().unwrap(),
            },
            _ => panic!("Impossible symbol {ch}"),
        }
    }
}

type Factory = Map<Block>;

use std::fmt::{Display, Formatter};

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_fmt(format_args!("{}", self.loss))?;
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Direction {
    Horiz,
    Vert,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Best {
    lr: Option<Number>, // Starting from here but wanting to go left or right, best loss
    ud: Option<Number>, // Starting from here but now up or down
}

impl Best {
    fn best_of_the_best(&self) -> Option<Number> {
        match (self.lr, self.ud) {
            (None, None) => None,
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (Some(a), Some(b)) => Some(std::cmp::min(a, b)),
        }
    }
}

type Step = (isize, isize, Direction);

fn maybe_update(paths: &mut Map<Best>, next: &mut Vec<Step>, step: Step, loss: Number) {
    let (x, y, direction) = step;
    let mut best = paths.read(x, y).unwrap_or_default();
    match direction {
        Direction::Vert => {
            if best.ud.is_none() || best.ud.unwrap() > loss {
                best.ud = Some(loss);
                paths.write(x, y, best);
            } else {
                return;
            }
        }
        Direction::Horiz => {
            if best.lr.is_none() || best.lr.unwrap() > loss {
                best.lr = Some(loss);
                paths.write(x, y, best);
            } else {
                return;
            }
        }
    }
    next.push((x, y, direction));
}

fn least(map: &Factory) -> Number {
    let &right = map.x().end();
    let &bottom = map.y().end();

    let mut paths: Map<Best> = Map::new();
    paths.write(
        0,
        0,
        Best {
            lr: Some(0),
            ud: Some(0),
        },
    );

    let mut todo: Vec<Step> = vec![(0, 0, Direction::Horiz), (0, 0, Direction::Vert)];

    while !todo.is_empty() {
        let mut next: Vec<Step> = Vec::new();
        for (x, y, dir) in todo {
            match dir {
                Direction::Horiz => {
                    let Some(initial) = paths.read(x, y).and_then(|best| best.lr) else {
                        panic!("Trying to apply horizontal move from a position with no known best loss, {x} {y}");
                    };

                    let mut loss = initial;
                    for dx in 1..=3 {
                        let x = x + dx;
                        if x > right {
                            break;
                        }
                        loss += map.read(x, y).expect("Map out of bounds").loss as Number;
                        maybe_update(&mut paths, &mut next, (x, y, Direction::Vert), loss);
                    }

                    let mut loss = initial;
                    for dx in 1..=3 {
                        let x = x - dx;
                        if x < 0 {
                            break;
                        }
                        loss += map.read(x, y).expect("Map out of bounds").loss as Number;
                        maybe_update(&mut paths, &mut next, (x, y, Direction::Vert), loss);
                    }
                }
                Direction::Vert => {
                    let Some(initial) = paths.read(x, y).and_then(|best| best.ud) else {
                        panic!("Trying to apply vertical move from a position with no known best loss, {x} {y}");
                    };

                    let mut loss = initial;
                    for dy in 1..=3 {
                        let y = y + dy;
                        if y > bottom {
                            break;
                        }
                        loss += map.read(x, y).expect("Map out of bounds").loss as Number;
                        maybe_update(&mut paths, &mut next, (x, y, Direction::Horiz), loss);
                    }

                    let mut loss = initial;
                    for dy in 1..=3 {
                        let y = y - dy;
                        if y < 0 {
                            break;
                        }
                        loss += map.read(x, y).expect("Map out of bounds").loss as Number;
                        maybe_update(&mut paths, &mut next, (x, y, Direction::Horiz), loss);
                    }
                }
            }
        }
        todo = next;
    }
    let dest = paths
        .read(right, bottom)
        .expect("Should have identified a best route");
    dest.best_of_the_best().unwrap()
}

pub fn a() {
    let ctxt = readfile("17");
    let map: Factory = ctxt.value().parse().unwrap();
    let best = least(&map);
    println!("Least loss is: {best}");
}

fn ultra(map: &Factory) -> Number {
    let &right = map.x().end();
    let &bottom = map.y().end();

    let mut paths: Map<Best> = Map::new();
    paths.write(
        0,
        0,
        Best {
            lr: Some(0),
            ud: Some(0),
        },
    );

    let mut todo: Vec<Step> = vec![(0, 0, Direction::Horiz), (0, 0, Direction::Vert)];

    while !todo.is_empty() {
        let mut next: Vec<Step> = Vec::new();
        for (x, y, dir) in todo {
            match dir {
                Direction::Horiz => {
                    let Some(initial) = paths.read(x, y).and_then(|best| best.lr) else {
                        panic!("Trying to apply horizontal move from a position with no known best loss, {x} {y}");
                    };

                    let mut loss = initial;
                    for dx in 1..=10 {
                        let x = x + dx;
                        if x > right {
                            break;
                        }
                        loss += map.read(x, y).expect("Map out of bounds").loss as Number;
                        if dx < 4 {
                            continue;
                        }
                        maybe_update(&mut paths, &mut next, (x, y, Direction::Vert), loss);
                    }

                    let mut loss = initial;
                    for dx in 1..=10 {
                        let x = x - dx;
                        if x < 0 {
                            break;
                        }
                        loss += map.read(x, y).expect("Map out of bounds").loss as Number;
                        if dx < 4 {
                            continue;
                        }
                        maybe_update(&mut paths, &mut next, (x, y, Direction::Vert), loss);
                    }
                }
                Direction::Vert => {
                    let Some(initial) = paths.read(x, y).and_then(|best| best.ud) else {
                        panic!("Trying to apply vertical move from a position with no known best loss, {x} {y}");
                    };

                    let mut loss = initial;
                    for dy in 1..=10 {
                        let y = y + dy;
                        if y > bottom {
                            break;
                        }
                        loss += map.read(x, y).expect("Map out of bounds").loss as Number;
                        if dy < 4 {
                            continue;
                        }
                        maybe_update(&mut paths, &mut next, (x, y, Direction::Horiz), loss);
                    }

                    let mut loss = initial;
                    for dy in 1..=10 {
                        let y = y - dy;
                        if y < 0 {
                            break;
                        }
                        loss += map.read(x, y).expect("Map out of bounds").loss as Number;
                        if dy < 4 {
                            continue;
                        }
                        maybe_update(&mut paths, &mut next, (x, y, Direction::Horiz), loss);
                    }
                }
            }
        }
        todo = next;
    }
    let dest = paths
        .read(right, bottom)
        .expect("Should have identified a best route");
    dest.best_of_the_best().unwrap()
}

pub fn b() {
    let ctxt = readfile("17");
    let map: Factory = ctxt.value().parse().unwrap();
    let best = ultra(&map);
    println!("Least loss with ultra crucible is: {best}");
}

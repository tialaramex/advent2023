use sky::map::Map;
use sky::readfile;
use std::collections::HashSet;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
enum Element {
    #[default]
    Empty,
    MirrorLeft,
    MirrorRight,
    SplitVert,
    SplitHoriz,
}

impl From<char> for Element {
    fn from(ch: char) -> Self {
        match ch {
            '.' => Self::Empty,
            '/' => Self::MirrorLeft,
            '\\' => Self::MirrorRight,
            '-' => Self::SplitHoriz,
            '|' => Self::SplitVert,
            _ => panic!("Impossible symbol {ch}"),
        }
    }
}

type Contraption = Map<Element>;

use std::fmt::{Display, Formatter};

impl Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Empty => f.write_str("."),
            Self::MirrorLeft => f.write_str("/"),
            Self::MirrorRight => f.write_str("\\"),
            Self::SplitHoriz => f.write_str("-"),
            Self::SplitVert => f.write_str("|"),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Beam {
    x: isize,
    y: isize,
    direction: Direction,
}

impl Beam {
    fn step(&mut self) {
        match self.direction {
            Direction::Up => {
                self.y -= 1;
            }
            Direction::Down => {
                self.y += 1;
            }
            Direction::Left => {
                self.x -= 1;
            }
            Direction::Right => {
                self.x += 1;
            }
        }
    }

    fn turn(&mut self, mirror: Element) {
        let d = match (mirror, self.direction) {
            (Element::MirrorLeft, Direction::Up) => Direction::Right,
            (Element::MirrorLeft, Direction::Down) => Direction::Left,
            (Element::MirrorLeft, Direction::Left) => Direction::Down,
            (Element::MirrorLeft, Direction::Right) => Direction::Up,

            (Element::MirrorRight, Direction::Up) => Direction::Left,
            (Element::MirrorRight, Direction::Down) => Direction::Right,
            (Element::MirrorRight, Direction::Left) => Direction::Up,
            (Element::MirrorRight, Direction::Right) => Direction::Down,
            _ => panic!("Impossible mirror configuration"),
        };
        self.direction = d;
    }

    fn split(self) -> (Self, Self) {
        match self.direction {
            Direction::Up | Direction::Down => (
                Self {
                    x: self.x,
                    y: self.y,
                    direction: Direction::Left,
                },
                Self {
                    x: self.x + 1,
                    y: self.y,
                    direction: Direction::Right,
                },
            ),
            Direction::Left | Direction::Right => (
                Self {
                    x: self.x,
                    y: self.y,
                    direction: Direction::Up,
                },
                Self {
                    x: self.x,
                    y: self.y + 1,
                    direction: Direction::Down,
                },
            ),
        }
    }
}

fn basic(input: &Contraption) -> usize {
    energized(
        input,
        Beam {
            x: 0,
            y: 0,
            direction: Direction::Right,
        },
    )
}

fn energized(input: &Contraption, start: Beam) -> usize {
    let mut done: HashSet<Beam> = HashSet::new();
    let mut out: HashSet<(isize, isize)> = HashSet::new();
    let mut todo: Vec<Beam> = Vec::new();
    todo.push(start);
    let &right = input.x().end();
    let &bottom = input.y().end();
    while let Some(mut beam) = todo.pop() {
        while let Some(element) = input.read(beam.x, beam.y) {
            if beam.x < 0 || beam.y < 0 || beam.x > right || beam.y > bottom {
                break;
            }
            if done.contains(&beam) {
                break;
            }
            done.insert(beam);
            out.insert((beam.x, beam.y));
            match (element, beam.direction) {
                (Element::MirrorLeft | Element::MirrorRight, _) => {
                    beam.turn(element);
                }
                (Element::SplitHoriz, Direction::Up | Direction::Down) => {
                    let (left, right) = beam.split();
                    beam = left;
                    todo.push(right);
                }
                (Element::SplitVert, Direction::Left | Direction::Right) => {
                    let (up, down) = beam.split();
                    beam = up;
                    todo.push(down);
                }
                _ => {}
            }
            beam.step();
        }
    }

    out.len()
}

pub fn a() {
    let ctxt = readfile("16");
    let map: Contraption = ctxt.value().parse().unwrap();
    let count = basic(&map);
    println!("{count} tiles end up energized");
}

pub fn b() {
    let ctxt = readfile("16");
    let map: Contraption = ctxt.value().parse().unwrap();

    let &right = map.x().end();
    let &bottom = map.y().end();
    let mut best = 0;
    for x in map.x() {
        let count = energized(
            &map,
            Beam {
                x,
                y: 0,
                direction: Direction::Down,
            },
        );
        if count > best {
            best = count;
        }
        let count = energized(
            &map,
            Beam {
                x,
                y: bottom,
                direction: Direction::Up,
            },
        );
        if count > best {
            best = count;
        }
    }
    for y in map.y() {
        let count = energized(
            &map,
            Beam {
                x: 0,
                y,
                direction: Direction::Right,
            },
        );
        if count > best {
            best = count;
        }
        let count = energized(
            &map,
            Beam {
                x: right,
                y,
                direction: Direction::Left,
            },
        );
        if count > best {
            best = count;
        }
    }

    println!("Most tiles energized is: {best}");
}

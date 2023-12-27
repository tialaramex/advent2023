use sky::map::Map;
use sky::readfile;

type Number = u32;

type Coord = (u32, u32, u32);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Direction {
    Vertical,
    X,
    Y,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Brick {
    from: Coord,
    to: Coord,
}

impl Brick {
    fn kind(&self) -> Direction {
        if self.from.0 != self.to.0 {
            Direction::X
        } else if self.from.1 != self.to.1 {
            Direction::Y
        } else {
            Direction::Vertical
        }
    }
}

use std::cmp::Ordering;
impl Ord for Brick {
    fn cmp(&self, other: &Self) -> Ordering {
        // Hopefully this ordering is enough for real inputs
        (self.from.2, self.from.0, self.from.1).cmp(&(other.from.2, other.from.0, other.from.1))
    }
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn coord(s: &str) -> Coord {
    let coords: Vec<Number> = s.split(',').filter_map(|n| n.parse().ok()).collect();

    assert_eq!(coords.len(), 3);
    (coords[0], coords[1], coords[2])
}

use std::str::FromStr;
impl FromStr for Brick {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((from, to)) = s.split_once('~') else {
            return Err("Bricks should have ~ separator");
        };
        let from: Coord = coord(from);
        let to: Coord = coord(to);

        Ok(Brick { from, to })
    }
}

type Height = Map<Option<(usize, Number)>>;

fn safe(bricks: &[Brick]) -> usize {
    let mut heights: Height = Map::new();

    let mut needed: Vec<usize> = Vec::with_capacity(bricks.len());
    for (k, brick) in bricks.iter().enumerate() {
        match brick.kind() {
            Direction::X => {
                let bottom = brick.from.2;
                let y = brick.from.1;
                let mut support = 0;
                let mut solo: Option<usize> = None;
                for x in brick.from.0..=brick.to.0 {
                    match heights.read(x as isize, y as isize).unwrap_or_default() {
                        None => {}
                        Some((which, h)) => {
                            if h > bottom {
                                panic!("Impossible height {h} versus {brick:?}");
                            } else if h > support {
                                support = h;
                                solo = Some(which);
                            // Supported in multiple places, by different bricks
                            } else if h == support && Some(which) != solo {
                                solo = None;
                            }
                        }
                    }
                }
                if let Some(b) = solo {
                    // Mustn't disintegrate the block below
                    needed.push(b);
                }
                for x in brick.from.0..=brick.to.0 {
                    heights.write(x as isize, y as isize, Some((k, support + 1)));
                }
            }
            Direction::Y => {
                let bottom = brick.from.2;
                let x = brick.from.0;
                let mut support = 0;
                let mut solo: Option<usize> = None;
                for y in brick.from.1..=brick.to.1 {
                    match heights.read(x as isize, y as isize).unwrap_or_default() {
                        None => {}
                        Some((which, h)) => {
                            if h > bottom {
                                panic!("Impossible height {h} versus {brick:?}");
                            } else if h > support {
                                support = h;
                                solo = Some(which);
                            // Supported in multiple places, by different bricks
                            } else if h == support && Some(which) != solo {
                                solo = None;
                            }
                        }
                    }
                }
                if let Some(b) = solo {
                    // Mustn't disintegrate the block below
                    needed.push(b);
                }
                for y in brick.from.1..=brick.to.1 {
                    heights.write(x as isize, y as isize, Some((k, support + 1)));
                }
            }
            Direction::Vertical => {
                let bottom = brick.from.2;
                let x = brick.from.0;
                let y = brick.from.1;
                let current = heights.read(x as isize, y as isize).unwrap_or_default();
                let now = match current {
                    None => 1,
                    Some((which, h)) => {
                        if h > bottom {
                            panic!("Impossible height {h} versus {brick:?}");
                        } else {
                            // Mustn't disintegrate the block below
                            needed.push(which);
                            h + 1
                        }
                    }
                };
                let extra = brick.to.2 - brick.from.2;
                heights.write(x as isize, y as isize, Some((k, now + extra)));
            }
        }
    }
    needed.sort_unstable();
    needed.dedup();
    bricks.len() - needed.len()
}

pub fn a() {
    let ctxt = readfile("22");
    let mut snapshot: Vec<Brick> = Vec::new();
    for line in ctxt.lines() {
        let brick: Brick = line.parse().expect("Not a brick");
        snapshot.push(brick);
    }
    // Must sort snapshot first
    snapshot.sort();
    let ok = safe(&snapshot);
    println!("{ok} bricks can be safely chosen");
}

fn what_if(bricks: &[Brick], except: &Brick) -> usize {
    let mut with: Map<Number> = Map::new();
    let mut without: Map<Number> = Map::new();

    let mut dropped = 0;
    for brick in bricks.iter() {
        match brick.kind() {
            Direction::X => {
                let y = brick.from.1;
                let mut with_support = 0;
                let mut without_support = 0;
                for x in brick.from.0..=brick.to.0 {
                    let h = with.read(x as isize, y as isize).unwrap_or_default();
                    if h > with_support {
                        with_support = h;
                    }
                    let h = without.read(x as isize, y as isize).unwrap_or_default();
                    if h > without_support {
                        without_support = h;
                    }
                }
                if with_support != without_support {
                    dropped += 1;
                }
                for x in brick.from.0..=brick.to.0 {
                    with.write(x as isize, y as isize, with_support + 1);
                    if brick != except {
                        without.write(x as isize, y as isize, without_support + 1);
                    }
                }
            }
            Direction::Y => {
                let x = brick.from.0;
                let mut with_support = 0;
                let mut without_support = 0;
                for y in brick.from.1..=brick.to.1 {
                    let h = with.read(x as isize, y as isize).unwrap_or_default();
                    if h > with_support {
                        with_support = h;
                    }
                    let h = without.read(x as isize, y as isize).unwrap_or_default();
                    if h > without_support {
                        without_support = h;
                    }
                }
                if with_support != without_support {
                    dropped += 1;
                }
                for y in brick.from.1..=brick.to.1 {
                    with.write(x as isize, y as isize, with_support + 1);
                    if brick != except {
                        without.write(x as isize, y as isize, without_support + 1);
                    }
                }
            }
            Direction::Vertical => {
                let x = brick.from.0;
                let y = brick.from.1;
                let with_now = with.read(x as isize, y as isize).unwrap_or_default() + 1;
                let without_now = without.read(x as isize, y as isize).unwrap_or_default() + 1;
                if with_now != without_now {
                    dropped += 1;
                }
                let extra = brick.to.2 - brick.from.2;
                with.write(x as isize, y as isize, with_now + extra);
                if brick != except {
                    without.write(x as isize, y as isize, without_now + extra);
                }
            }
        }
    }
    dropped
}

pub fn b() {
    let ctxt = readfile("22");
    let mut snapshot: Vec<Brick> = Vec::new();
    for line in ctxt.lines() {
        let brick: Brick = line.parse().expect("Not a brick");
        snapshot.push(brick);
    }
    // Must sort snapshot first
    snapshot.sort();
    let mut total = 0;
    for brick in snapshot.iter() {
        total += what_if(&snapshot, brick);
    }
    println!("{total}");
}

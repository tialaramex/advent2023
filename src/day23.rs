use sky::map::Map;
use sky::readfile;

type Number = u32;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
enum Tile {
    #[default]
    Forest,
    Path,
    SlopeUp,
    SlopeRight,
    SlopeDown,
    SlopeLeft,
}

impl From<char> for Tile {
    fn from(ch: char) -> Self {
        match ch {
            '#' => Self::Forest,
            '.' => Self::Path,
            '^' => Self::SlopeUp,
            '>' => Self::SlopeRight,
            'v' => Self::SlopeDown,
            '<' => Self::SlopeLeft,
            _ => panic!("{ch} should not appear in a trail map"),
        }
    }
}

type Trail = Map<Tile>;

use std::fmt::{Display, Formatter};

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Forest => f.write_str("#"),
            Self::Path => f.write_str("."),
            Self::SlopeUp => f.write_str("^"),
            Self::SlopeRight => f.write_str(">"),
            Self::SlopeDown => f.write_str("v"),
            Self::SlopeLeft => f.write_str("<"),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn follow(map: &Trail, from: (isize, isize), mut heading: Direction) -> (Number, isize, isize) {
    let (mut x, mut y) = from;
    let mut steps = 1;

    loop {
        let up = map.read(x, y - 1).unwrap_or_default();
        let right = map.read(x + 1, y).unwrap_or_default();
        let down = map.read(x, y + 1).unwrap_or_default();
        let left = map.read(x - 1, y).unwrap_or_default();

        match (heading, up, right, down, left) {
            (Direction::Up, Tile::Path, Tile::Forest, _, Tile::Forest) => {
                y -= 1;
            }
            (Direction::Up, Tile::Forest, Tile::Path, _, Tile::Forest) => {
                x += 1;
                heading = Direction::Right;
            }
            (Direction::Up, Tile::Forest, Tile::Forest, _, Tile::Path) => {
                x -= 1;
                heading = Direction::Left;
            }
            (Direction::Up, Tile::SlopeUp, Tile::Forest, _, Tile::Forest) => {
                y -= 2;
                break;
            }
            (Direction::Up, Tile::Forest, Tile::SlopeRight, _, Tile::Forest) => {
                x += 2;
                break;
            }
            (Direction::Up, Tile::Forest, Tile::Forest, _, Tile::SlopeLeft) => {
                x -= 2;
                break;
            }

            (Direction::Right, Tile::Forest, Tile::Path, Tile::Forest, _) => {
                x += 1;
            }
            (Direction::Right, Tile::Path, Tile::Forest, Tile::Forest, _) => {
                y -= 1;
                heading = Direction::Up;
            }
            (Direction::Right, Tile::Forest, Tile::Forest, Tile::Path, _) => {
                y += 1;
                heading = Direction::Down;
            }
            (Direction::Right, Tile::Forest, Tile::SlopeRight, Tile::Forest, _) => {
                x += 2;
                break;
            }
            (Direction::Right, Tile::SlopeUp, Tile::Forest, Tile::Forest, _) => {
                y -= 2;
                break;
            }
            (Direction::Right, Tile::Forest, Tile::Forest, Tile::SlopeDown, _) => {
                y += 2;
                break;
            }

            (Direction::Down, _, Tile::Forest, Tile::Path, Tile::Forest) => {
                y += 1;
            }
            (Direction::Down, _, Tile::Path, Tile::Forest, Tile::Forest) => {
                x += 1;
                heading = Direction::Right;
            }
            (Direction::Down, _, Tile::Forest, Tile::Forest, Tile::Path) => {
                x -= 1;
                heading = Direction::Left;
            }
            (Direction::Down, _, Tile::Forest, Tile::SlopeDown, Tile::Forest) => {
                y += 2;
                break;
            }
            (Direction::Down, _, Tile::SlopeRight, Tile::Forest, Tile::Forest) => {
                x += 2;
                break;
            }
            (Direction::Down, _, Tile::Forest, Tile::Forest, Tile::SlopeLeft) => {
                x -= 2;
                break;
            }

            (Direction::Left, Tile::Forest, _, Tile::Forest, Tile::Path) => {
                x -= 1;
            }
            (Direction::Left, Tile::Path, _, Tile::Forest, Tile::Forest) => {
                y -= 1;
                heading = Direction::Up;
            }
            (Direction::Left, Tile::Forest, _, Tile::Path, Tile::Forest) => {
                y += 1;
                heading = Direction::Down;
            }
            (Direction::Left, Tile::Forest, _, Tile::Forest, Tile::SlopeLeft) => {
                x -= 2;
                break;
            }
            (Direction::Left, Tile::SlopeUp, _, Tile::Forest, Tile::Forest) => {
                y -= 2;
                break;
            }
            (Direction::Left, Tile::Forest, _, Tile::SlopeDown, Tile::Forest) => {
                y += 2;
                break;
            }

            (Direction::Down, _, Tile::Forest, Tile::Forest, Tile::Forest) => {
                // Exit
                y += 1;
                break;
            }

            _ => {
                panic!("{x},{y}: {heading:?}  U:{up:?} R:{right:?} D:{down:?} L:{left:?} !!! ({steps})");
            }
        }
        steps += 1;
    }
    (steps, x, y)
}

#[derive(Copy, Clone, Debug)]
struct Route {
    from: (isize, isize),
    to: (isize, isize),
    distance: Number,
}

use std::collections::HashSet;
fn routes(map: &Trail) -> Vec<Route> {
    let mut done = HashSet::new();
    let mut r = Vec::new();
    let mut nodes: Vec<(Direction, isize, isize)> = vec![(Direction::Down, 1, -1)];

    while let Some((heading, ox, oy)) = nodes.pop() {
        let (x, y) = match heading {
            Direction::Up => (ox, oy - 1),
            Direction::Right => (ox + 1, oy),
            Direction::Down => (ox, oy + 1),
            Direction::Left => (ox - 1, oy),
        };
        let (distance, x, y) = follow(map, (x, y), heading);
        let route = Route {
            from: (ox, oy),
            to: (x, y),
            distance,
        };
        r.push(route);
        if done.contains(&(x, y)) {
            continue;
        }
        done.insert((x, y));
        if map.read(x, y - 1).unwrap_or_default() == Tile::SlopeUp {
            nodes.push((Direction::Up, x, y));
        }
        if map.read(x + 1, y).unwrap_or_default() == Tile::SlopeRight {
            nodes.push((Direction::Right, x, y));
        }
        if map.read(x, y + 1).unwrap_or_default() == Tile::SlopeDown {
            nodes.push((Direction::Down, x, y));
        }
        if map.read(x - 1, y).unwrap_or_default() == Tile::SlopeLeft {
            nodes.push((Direction::Left, x, y));
        }
    }
    r
}

use std::collections::HashMap;
fn journeys(r: &[Route]) -> Vec<Number> {
    let mut totals: Vec<Number> = Vec::new();
    let mut graph: HashMap<(isize, isize), Vec<(Number, isize, isize)>> = HashMap::new();
    for route in r {
        let v = graph.entry(route.from).or_default();
        v.push((route.distance, route.to.0, route.to.1));
    }

    let mut attempts: Vec<(Number, isize, isize)> = vec![(0, 1, -1)];
    while let Some(attempt) = attempts.pop() {
        let (steps, x, y) = attempt;
        let Some(onward) = graph.get(&(x, y)) else {
            totals.push(steps - 3);
            continue;
        };
        for (extra, x, y) in onward {
            attempts.push((steps + extra + 2, *x, *y));
        }
    }
    totals.sort_unstable();
    totals
}

pub fn a() {
    let ctxt = readfile("23");
    let trail: Trail = ctxt.value().parse().unwrap();
    let r = routes(&trail);
    let totals = journeys(&r);
    let best = totals.last().expect("At least one possible route should exist");
    println!("The longest hike is {best} steps long");
}

fn remove_slopes(trail: &mut Trail) {
    for (x, y) in trail.find(|t| {
        matches!(
            t,
            Tile::SlopeUp | Tile::SlopeRight | Tile::SlopeDown | Tile::SlopeLeft
        )
    }) {
        trail.write(x, y, Tile::Path);
    }
}

fn new_follow(
    map: &Trail,
    from: (isize, isize),
    mut heading: Direction,
) -> (Number, isize, isize, Direction) {
    let (mut x, mut y) = from;
    let mut steps = 1;

    loop {
        let up = map.read(x, y - 1).unwrap_or_default();
        let right = map.read(x + 1, y).unwrap_or_default();
        let down = map.read(x, y + 1).unwrap_or_default();
        let left = map.read(x - 1, y).unwrap_or_default();

        match (heading, up, right, down, left) {
            (Direction::Up, Tile::Path, Tile::Forest, _, Tile::Forest) => {
                y -= 1;
            }
            (Direction::Up, Tile::Forest, Tile::Path, _, Tile::Forest) => {
                x += 1;
                heading = Direction::Right;
            }
            (Direction::Up, Tile::Forest, Tile::Forest, _, Tile::Path) => {
                x -= 1;
                heading = Direction::Left;
            }
            (
                Direction::Up,
                Tile::Path | Tile::Forest,
                Tile::Path | Tile::Forest,
                _,
                Tile::Path | Tile::Forest,
            ) => {
                break;
            }

            (Direction::Right, Tile::Forest, Tile::Path, Tile::Forest, _) => {
                x += 1;
            }
            (Direction::Right, Tile::Path, Tile::Forest, Tile::Forest, _) => {
                y -= 1;
                heading = Direction::Up;
            }
            (Direction::Right, Tile::Forest, Tile::Forest, Tile::Path, _) => {
                y += 1;
                heading = Direction::Down;
            }
            (
                Direction::Right,
                Tile::Path | Tile::Forest,
                Tile::Path | Tile::Forest,
                Tile::Path | Tile::Forest,
                _,
            ) => {
                break;
            }

            (Direction::Down, _, Tile::Forest, Tile::Path, Tile::Forest) => {
                y += 1;
            }
            (Direction::Down, _, Tile::Path, Tile::Forest, Tile::Forest) => {
                x += 1;
                heading = Direction::Right;
            }
            (Direction::Down, _, Tile::Forest, Tile::Forest, Tile::Path) => {
                x -= 1;
                heading = Direction::Left;
            }
            (Direction::Down, _, Tile::Forest, Tile::Forest, Tile::Forest) => {
                break;
            }

            // Maybe consolidate ?
            (
                Direction::Down,
                _,
                Tile::Path | Tile::Forest,
                Tile::Path | Tile::Forest,
                Tile::Path | Tile::Forest,
            ) => {
                break;
            }

            (Direction::Left, Tile::Forest, _, Tile::Forest, Tile::Path) => {
                x -= 1;
            }
            (Direction::Left, Tile::Path, _, Tile::Forest, Tile::Forest) => {
                y -= 1;
                heading = Direction::Up;
            }
            (Direction::Left, Tile::Forest, _, Tile::Path, Tile::Forest) => {
                y += 1;
                heading = Direction::Down;
            }
            (
                Direction::Left,
                Tile::Path | Tile::Forest,
                _,
                Tile::Path | Tile::Forest,
                Tile::Path | Tile::Forest,
            ) => {
                break;
            }

            _ => {
                panic!("{x},{y}: {heading:?}  U:{up:?} R:{right:?} D:{down:?} L:{left:?} !!! ({steps})");
            }
        }
        steps += 1;
    }
    (steps, x, y, heading)
}

fn new_routes(map: &Trail) -> Vec<Route> {
    let mut done = HashSet::new();
    let mut r = Vec::new();
    let mut nodes: Vec<(Direction, isize, isize)> = vec![(Direction::Down, 1, -1)];

    while let Some((heading, ox, oy)) = nodes.pop() {
        let (x, y) = match heading {
            Direction::Up => (ox, oy - 1),
            Direction::Right => (ox + 1, oy),
            Direction::Down => (ox, oy + 1),
            Direction::Left => (ox - 1, oy),
        };
        let (distance, x, y, heading) = new_follow(map, (x, y), heading);
        let route = Route {
            from: (ox, oy),
            to: (x, y),
            distance,
        };
        r.push(route);
        if done.contains(&(x, y)) {
            continue;
        }
        done.insert((x, y));
        if heading != Direction::Down && map.read(x, y - 1).unwrap_or_default() == Tile::Path {
            nodes.push((Direction::Up, x, y));
        }
        if heading != Direction::Left && map.read(x + 1, y).unwrap_or_default() == Tile::Path {
            nodes.push((Direction::Right, x, y));
        }
        if heading != Direction::Up && map.read(x, y + 1).unwrap_or_default() == Tile::Path {
            nodes.push((Direction::Down, x, y));
        }
        if heading != Direction::Right && map.read(x - 1, y).unwrap_or_default() == Tile::Path {
            nodes.push((Direction::Left, x, y));
        }
    }
    r
}

// Bit mask
type NodeId = u64;
// Bit masks
type NodeIds = u64;

type NewGraph = HashMap<NodeId, Vec<(Number, NodeId)>>;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Wanderer {
    steps: Number,
    at: NodeId,
    seen: NodeIds,
}

impl Wanderer {
    fn new(steps: Number, at: NodeId, seen: NodeIds) -> Self {
        Self { steps, at, seen }
    }

    fn next(&self, graph: &NewGraph) -> Vec<Self> {
        let mut v = Vec::new();

        let onward = graph
            .get(&self.at)
            .expect("Every Node ID should logically be in the graph");
        for (extra, to) in onward {
            if to & self.seen == 0 {
                // Not been here yet, so try that
                v.push(Wanderer {
                    steps: self.steps + extra,
                    at: *to,
                    seen: self.seen | to,
                });
            }
        }
        v
    }

    /// _Maximum_ steps from initial until predicate is true
    fn furthest(start: NodeId, end: NodeId, graph: &NewGraph) -> Number {
        if start == end {
            return 0;
        }

        let mut most_steps = 0;
        let mut best: HashMap<(NodeId, NodeIds), Number> = HashMap::new();
        let mut options: Vec<Self> = Vec::new();
        // Cheating with a 1 here
        options.push(Self::new(1, start, start));

        while !options.is_empty() {
            let mut tomorrow: Vec<Self> = Vec::new();
            while let Some(opt) = options.pop() {
                let steps = best.entry((opt.at, opt.seen)).or_default();
                if *steps < opt.steps {
                    if opt.at == end && most_steps < opt.steps {
                        most_steps = opt.steps;
                    }
                    *steps = opt.steps;
                    for nxt in opt.next(graph) {
                        tomorrow.push(nxt);
                    }
                }
            }
            options = tomorrow;
        }
        most_steps
    }
}

fn new_journeys(r: &[Route]) -> Number {
    let mut lookup: HashMap<(isize, isize), NodeId> = HashMap::new();
    let mut graph: NewGraph = HashMap::new();
    let mut next_node_id = 1;

    for route in r {
        let from = *lookup.entry(route.from).or_insert_with(|| {
            let node_id = next_node_id;
            next_node_id <<= 1;
            node_id
        });
        let to = *lookup.entry(route.to).or_insert_with(|| {
            let node_id = next_node_id;
            next_node_id <<= 1;
            node_id
        });

        let v = graph.entry(from).or_default();
        v.push((route.distance, to));
        let v = graph.entry(to).or_default();
        v.push((route.distance, from));
    }
    let start = *lookup
        .get(&(1, -1))
        .expect("The route should start from 1, -1");

    let mut end = start;
    let mut downest = 0;
    for k in lookup.keys() {
        if k.1 > downest {
            downest = k.1;
            end = *lookup.get(k).unwrap();
        }
    }

    Wanderer::furthest(start, end, &graph) - 2
}

pub fn b() {
    let ctxt = readfile("23");
    let mut trail: Trail = ctxt.value().parse().unwrap();
    remove_slopes(&mut trail);
    let rts = new_routes(&trail);
    let steps = new_journeys(&rts);
    println!("Longest surprisingly dry hike is {steps} steps");
}

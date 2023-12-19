use sky::map::Map;
use sky::readfile;

type Number = i64;

#[derive(Copy, Clone, Default, Eq, PartialEq)]
enum Cube {
    #[default]
    Ground,
    Trench,
}

type Lagoon = Map<Cube>;

use std::fmt::{Debug, Formatter};

impl Debug for Cube {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Ground => f.write_str("."),
            Self::Trench => f.write_str("#"),
        }
    }
}

// Flood fill
fn flood(lagoon: &mut Lagoon, start: (isize, isize)) {
    let mut queue = vec![start];
    while let Some((x, y)) = queue.pop() {
        match lagoon.read(x, y).unwrap_or_default() {
            Cube::Ground => {
                lagoon.write(x, y, Cube::Trench);
                queue.push((x + 1, y));
                queue.push((x - 1, y));
                queue.push((x, y + 1));
                queue.push((x, y - 1));
            }
            Cube::Trench => {}
        }
    }
}

fn inside(lagoon: &Lagoon, y: isize) -> (isize, isize) {
    for x in lagoon.x() {
        if let Some(Cube::Trench) = lagoon.read(x, y) {
            if let Some(Cube::Trench) = lagoon.read(x, y + 1) {
                return (x + 1, y + 1);
            } else {
                return (x + 1, y - 1);
            }
        }
    }

    panic!("There are apparently no trenches in this lagoon");
}

pub fn a() {
    let ctxt = readfile("18");
    let mut lagoon: Lagoon = Map::new();
    let mut x = 0;
    let mut y = 0;
    lagoon.write(x, y, Cube::Trench);
    for line in ctxt.lines() {
        let (dir, rest) = line
            .split_once(' ')
            .expect("Should be at least one space per line");
        let (dist, _) = rest.split_once(' ').expect("Should be two spaces per line");
        let dist: isize = dist.parse().expect("Distance should be numeric");
        let (dx, dy) = match dir {
            "R" => (1, 0),
            "D" => (0, 1),
            "L" => (-1, 0),
            "U" => (0, -1),
            _ => panic!("Directions should be U,D,L or R, not {dir}"),
        };
        for _ in 0..dist {
            x += dx;
            y += dy;
            lagoon.write(x, y, Cube::Trench);
        }
    }
    let start = inside(&lagoon, 0);
    flood(&mut lagoon, start);
    let s = lagoon.count(|&&x| x == Cube::Trench);
    println!("Lagoon can hold {s} cubic metres");
}

fn lookup(horiz: &[isize], vert: &[isize], x: isize, y: isize) -> (isize, isize) {
    let Ok(x) = horiz.binary_search(&x) else {
        panic!("Couldn't find {x} in {horiz:?}");
    };
    let Ok(y) = vert.binary_search(&y) else {
        panic!("Couldn't find {y} in {vert:?}");
    };
    (2 * x as isize, 2 * y as isize)
}

use std::cmp::Ordering;
fn draw(lagoon: &mut Lagoon, from: (isize, isize), to: (isize, isize)) {
    let dx = match from.0.cmp(&to.0) {
        Ordering::Less => 1,
        Ordering::Greater => -1,
        Ordering::Equal => 0,
    };
    let dy = match from.1.cmp(&to.1) {
        Ordering::Less => 1,
        Ordering::Greater => -1,
        Ordering::Equal => 0,
    };

    let mut x = from.0;
    let mut y = from.1;
    while x != to.0 || y != to.1 {
        lagoon.write(x, y, Cube::Trench);
        x += dx;
        y += dy;
    }
}

fn size(lagoon: &Lagoon, horiz: &[isize], vert: &[isize]) -> Number {
    let mut n = 0;
    for (x, y) in lagoon.find(|c| c == Cube::Trench) {
        // The odd numbers represent potentially large spreads

        let width = if x & 1 == 1 {
            let left = (x / 2) as usize;
            let right = (x / 2 + 1) as usize;
            (horiz[right] - horiz[left] - 1) as Number
        } else {
            1
        };
        let height = if y & 1 == 1 {
            let top = (y / 2) as usize;
            let bottom = (y / 2 + 1) as usize;
            (vert[bottom] - vert[top] - 1) as Number
        } else {
            1
        };

        n += width * height;
    }
    n
}

pub fn b() {
    let ctxt = readfile("18");
    let mut x = 0;
    let mut y = 0;
    let mut coords = Vec::new();

    for line in ctxt.lines() {
        let (_, hex) = line
            .rsplit_once(' ')
            .expect("Should be at least one space per line");
        let code = hex.strip_prefix("(#").unwrap().strip_suffix(')').unwrap();
        let (dist, dir) = code.split_at(5);
        let dist =
            isize::from_str_radix(dist, 16).expect("Distance should be a hexadecimal number");
        match dir {
            "0" => x += dist,
            "1" => y += dist,
            "2" => x -= dist,
            "3" => y -= dist,
            _ => panic!("Directions should be 0,1,2 or 3, not {dir}"),
        };
        coords.push((x, y));
    }
    let mut horiz: Vec<isize> = coords.iter().map(|&(x, _)| x).collect();
    horiz.sort();
    horiz.dedup();
    let mut vert: Vec<isize> = coords.iter().map(|&(_, y)| y).collect();
    vert.sort();
    vert.dedup();

    let mut lagoon: Lagoon = Map::new();
    let mut from: (isize, isize) = lookup(&horiz, &vert, 0, 0);
    for (x, y) in coords {
        let to = lookup(&horiz, &vert, x, y);
        draw(&mut lagoon, from, to);
        from = to;
    }
    let (_, y) = lookup(&horiz, &vert, 0, 0);
    let start = inside(&lagoon, y);
    flood(&mut lagoon, start);
    let s = size(&lagoon, &horiz, &vert);
    println!("Now, lagoon can hold {s} cubic metres");
}

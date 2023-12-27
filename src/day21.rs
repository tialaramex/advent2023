use sky::map::Map;
use sky::readfile;

type Number = u32;

#[derive(Copy, Clone, Default, Eq, PartialEq)]
enum Plot {
    #[default]
    Outside,
    Start,
    Garden,
    Rock,
    Elf,
}

impl From<char> for Plot {
    fn from(ch: char) -> Self {
        match ch {
            'S' => Self::Start,
            '.' => Self::Garden,
            '#' => Self::Rock,
            _ => panic!("Input should only include S.#"),
        }
    }
}

type Garden = Map<Plot>;

use std::fmt::{Display, Formatter};

impl Display for Plot {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Outside => f.write_str("?"),
            Self::Start => f.write_str("S"),
            Self::Garden => f.write_str("."),
            Self::Rock => f.write_str("#"),
            Self::Elf => f.write_str("O"),
        }
    }
}

fn steps(map: &mut Garden, n: Number) {
    let origin = map.find(|g| g == Plot::Start);
    let &(x, y) = origin
        .first()
        .expect("The map should show a starting location");
    let mut todo: Vec<(isize, isize)> = if n & 1 == 1 {
        let mut next: Vec<(isize, isize)> = Vec::new();
        for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            let x = x + dx;
            let y = y + dy;
            if Some(Plot::Garden) == map.read(x, y) {
                next.push((x, y));
            }
        }
        next
    } else {
        vec![(x, y)]
    };

    for _ in 0..(n / 2) {
        let mut next: Vec<(isize, isize)> = Vec::new();
        for (x, y) in todo {
            // Beaten here, skip
            if let Some(Plot::Elf) = map.read(x, y) {
                continue;
            }
            map.write(x, y, Plot::Elf);
            for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                let x = x + dx;
                let y = y + dy;
                if Some(Plot::Garden) != map.read(x, y) {
                    continue;
                }
                for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                    let x = x + dx;
                    let y = y + dy;
                    if Some(Plot::Garden) == map.read(x, y) {
                        next.push((x, y));
                    }
                }
            }
        }
        todo = next;
    }
    for (x, y) in todo {
        map.write(x, y, Plot::Elf);
    }
}

pub fn a() {
    let ctxt = readfile("21");
    let mut map: Garden = ctxt.value().parse().unwrap();
    steps(&mut map, 64);
    let elves = map.count(|&&g| g == Plot::Elf);
    println!("After walking 64 steps the elf could reach {elves} garden plots");
}

fn make_even_tile(map: &Garden) -> Garden {
    let mut map = map.clone();
    let origin = map.find(|g| g == Plot::Start);
    let &(x, y) = origin
        .first()
        .expect("The map should show a starting location");
    map.write(x, y, Plot::Garden);
    let mut todo: Vec<(isize, isize)> = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];

    while !todo.is_empty() {
        let mut next: Vec<(isize, isize)> = Vec::new();
        for (x, y) in todo {
            if let Some(Plot::Elf) = map.read(x, y) {
                continue;
            }
            map.write(x, y, Plot::Elf);
            for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                let x = x + dx;
                let y = y + dy;
                if Some(Plot::Garden) != map.read(x, y) {
                    continue;
                }
                for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                    let x = x + dx;
                    let y = y + dy;
                    if Some(Plot::Garden) == map.read(x, y) {
                        next.push((x, y));
                    }
                }
            }
        }
        todo = next;
    }

    map
}

fn make_odd_tile(map: &Garden) -> Garden {
    let mut map = map.clone();
    let origin = map.find(|g| g == Plot::Start);
    let &(x, y) = origin
        .first()
        .expect("The map should show a starting location");
    map.write(x, y, Plot::Garden);
    let mut todo: Vec<(isize, isize)> = vec![(x, y)];

    while !todo.is_empty() {
        let mut next: Vec<(isize, isize)> = Vec::new();
        for (x, y) in todo {
            if let Some(Plot::Elf) = map.read(x, y) {
                continue;
            }
            map.write(x, y, Plot::Elf);
            for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                let x = x + dx;
                let y = y + dy;
                if Some(Plot::Garden) != map.read(x, y) {
                    continue;
                }
                for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                    let x = x + dx;
                    let y = y + dy;
                    if Some(Plot::Garden) == map.read(x, y) {
                        next.push((x, y));
                    }
                }
            }
        }
        todo = next;
    }

    map
}

fn make_tile_from(map: &Garden, x: isize, y: isize, count: usize) -> Garden {
    let mut map = map.clone();
    let origin = map.find(|g| g == Plot::Start);
    let &(ox, oy) = origin
        .first()
        .expect("The map should show a starting location");
    map.write(ox, oy, Plot::Garden);

    let mut todo: Vec<(isize, isize)> = if count & 1 == 1 {
        let mut next: Vec<(isize, isize)> = Vec::new();
        for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
            let x = x + dx;
            let y = y + dy;
            if Some(Plot::Garden) == map.read(x, y) {
                next.push((x, y));
            }
        }
        next
    } else {
        vec![(x, y)]
    };

    let count = count / 2;

    for _ in 0..=count {
        let mut next: Vec<(isize, isize)> = Vec::new();
        for (x, y) in todo {
            if let Some(Plot::Elf) = map.read(x, y) {
                continue;
            }
            map.write(x, y, Plot::Elf);
            for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                let x = x + dx;
                let y = y + dy;
                if Some(Plot::Garden) != map.read(x, y) {
                    continue;
                }
                for (dx, dy) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
                    let x = x + dx;
                    let y = y + dy;
                    if Some(Plot::Garden) == map.read(x, y) {
                        next.push((x, y));
                    }
                }
            }
        }
        todo = next;
    }

    map
}

fn make_count_from(map: Garden) -> usize {
    map.count(|&g| g == &Plot::Elf)
}

fn diamond(map: &Garden, radius: usize) -> usize {
    let tile_size = map.x().end() - map.x().start() + 1;
    assert!(radius as isize > tile_size);

    // Made up of repeated map tiles

    let even = make_count_from(make_even_tile(map));
    let odd = make_count_from(make_odd_tile(map));
    let left = make_count_from(make_tile_from(map, 130, 65, 130));
    let right = make_count_from(make_tile_from(map, 0, 65, 130));
    let top = make_count_from(make_tile_from(map, 65, 130, 130));
    let bottom = make_count_from(make_tile_from(map, 65, 0, 130));
    let tls_small = make_count_from(make_tile_from(map, 130, 130, 64));
    let bls_small = make_count_from(make_tile_from(map, 130, 0, 64));
    let brs_small = make_count_from(make_tile_from(map, 0, 0, 64));
    let trs_small = make_count_from(make_tile_from(map, 0, 130, 64));

    let tls_large = make_count_from(make_tile_from(map, 130, 130, 195));
    let bls_large = make_count_from(make_tile_from(map, 130, 0, 195));
    let brs_large = make_count_from(make_tile_from(map, 0, 0, 195));
    let trs_large = make_count_from(make_tile_from(map, 0, 130, 195));

    let blocks = (radius - 65) / 131;
    // Algorithm Only works for whole blocks
    assert_eq!(blocks * 131, radius - 65);
    let fewer = blocks - 1;

    let edges = top
        + (tls_small * blocks)
        + (tls_large * fewer)
        + left
        + (bls_small * blocks)
        + (bls_large * fewer)
        + bottom
        + (brs_small * blocks)
        + (brs_large * fewer)
        + right
        + (trs_small * blocks)
        + (trs_large) * fewer;

    (fewer * fewer * even) + (blocks * blocks * odd) + edges
}

pub fn b() {
    let ctxt = readfile("21");
    let map: Garden = ctxt.value().parse().unwrap();

    const STEPS: usize = 26501365;

    let elves = diamond(&map, STEPS);

    println!("After walking {STEPS} steps the elf could reach {elves} garden plots");
}

use sky::readfile;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Ident {
    tlc: [u8; 3],
}

use std::str::FromStr;
impl FromStr for Ident {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        if bytes.len() != 3 {
            return Err("Incorrect node ID length");
        }
        let tlc: [u8; 3] = bytes.try_into().unwrap();

        Ok(Ident { tlc })
    }
}

const AAA: Ident = Ident {
    tlc: [b'A', b'A', b'A'],
};
const ZZZ: Ident = Ident {
    tlc: [b'Z', b'Z', b'Z'],
};

type Choice = (Ident, Ident);

use std::collections::HashMap;
fn network<'a>(lines: impl Iterator<Item = &'a str>) -> HashMap<Ident, Choice> {
    let mut map = HashMap::new();
    for line in lines {
        let Some((ident, rest)) = line.split_once(" = ") else {
            panic!("Network line lacks equals separator");
        };
        let ident: Ident = ident.parse().expect("Network should have node");
        let Some((left, right)) = rest.split_once(", ") else {
            panic!("Network line lacks comma separator");
        };
        let left = left.strip_prefix('(').expect("Left side should start (");
        let right = right.strip_suffix(')').expect("Right side should start )");
        let left: Ident = left.parse().expect("Left should be a node identifier");
        let right: Ident = right.parse().expect("Right should be a node identifier");
        map.insert(ident, (left, right));
    }
    map
}

fn steps<D>(
    mut directions: impl Iterator<Item = char>,
    network: &HashMap<Ident, Choice>,
    start: Ident,
    mut done: D,
) -> usize
where
    D: FnMut(Ident) -> bool,
{
    let mut steps = 0;
    let mut pos = start;
    while !done(pos) {
        steps += 1;
        let choice = network
            .get(&pos)
            .expect("Looking up {pos} should find a node in the network");
        pos = match directions.next() {
            Some('L') => choice.0,
            Some('R') => choice.1,
            _ => panic!("All decisions should be L or R"),
        }
    }
    steps
}

pub fn a() {
    let ctxt = readfile("08");
    let mut lines = ctxt.lines();
    let Some(lr) = lines.next() else {
        panic!("No LR line");
    };
    lines.next(); // Throw away blank line

    let n = network(lines);
    let lr = lr.chars().cycle();
    let answer = steps(lr, &n, AAA, |p| p == ZZZ);
    println!("After {answer} steps we reach ZZZ");
}

fn find_starts(network: &HashMap<Ident, Choice>) -> Vec<Ident> {
    network
        .keys()
        .copied()
        .filter(|i| i.tlc[2] == b'A')
        .collect()
}

pub fn b() {
    let ctxt = readfile("08");
    let mut lines = ctxt.lines();
    let Some(lr) = lines.next() else {
        panic!("No LR line");
    };
    let pace = lr.len();
    lines.next(); // Throw away blank line

    let n = network(lines);
    let lr = lr.chars().cycle();
    let starts = find_starts(&n);
    // In theory they could all converge earlier, but let's assume not
    let mut best = pace;
    for pos in starts {
        let n = steps(lr.clone(), &n, pos, |p| p.tlc[2] == b'Z');
        best = num::Integer::lcm(&best, &n);
    }
    println!("After {best} steps all ghosts reach nodes ending in Z simultaneously");
}

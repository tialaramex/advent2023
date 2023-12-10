use sky::map::Map;
use sky::readfile;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
enum Pipe {
    #[default]
    Ground,
    Vert,
    Horiz,
    Ne,
    Nw,
    Sw,
    Se,
    Start,
}

impl From<char> for Pipe {
    fn from(ch: char) -> Self {
        match ch {
            '.' => Self::Ground,
            '|' => Self::Vert,
            '-' => Self::Horiz,
            'L' => Self::Ne,
            'J' => Self::Nw,
            '7' => Self::Sw,
            'F' => Self::Se,
            'S' => Self::Start,
            _ => panic!("Invalid map symbol {ch}"),
        }
    }
}

type Pipes = Map<Pipe>;

fn fix_start(pipes: &mut Pipes, x: isize, y: isize) {
    let west = matches!(
        pipes.read(x - 1, y).unwrap_or_default(),
        Pipe::Horiz | Pipe::Ne | Pipe::Se
    );
    let east = matches!(
        pipes.read(x + 1, y).unwrap_or_default(),
        Pipe::Horiz | Pipe::Nw | Pipe::Sw
    );
    let north = matches!(
        pipes.read(x, y - 1).unwrap_or_default(),
        Pipe::Vert | Pipe::Sw | Pipe::Se
    );
    let south = matches!(
        pipes.read(x, y + 1).unwrap_or_default(),
        Pipe::Vert | Pipe::Ne | Pipe::Nw
    );

    match (north, west, south, east) {
        (true, true, false, false) => pipes.write(x, y, Pipe::Nw),
        (true, false, true, false) => pipes.write(x, y, Pipe::Vert),
        (true, false, false, true) => pipes.write(x, y, Pipe::Ne),
        (false, true, false, true) => pipes.write(x, y, Pipe::Horiz),
        (false, true, true, false) => pipes.write(x, y, Pipe::Sw),
        (false, false, true, true) => pipes.write(x, y, Pipe::Se),
        _ => panic!("Pipes around the start don't make sense to me"),
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

// Loop length
fn length(pipes: &Pipes, sx: isize, sy: isize) -> usize {
    let mut x: isize = sx;
    let mut y: isize = sy;
    let mut distance = 0;
    let mut facing = match pipes.read(sx, sy).unwrap() {
        Pipe::Horiz => Direction::West,
        Pipe::Vert => Direction::North,
        Pipe::Nw => Direction::North,
        Pipe::Ne => Direction::West,
        Pipe::Sw => Direction::South,
        Pipe::Se => Direction::East,
        _ => panic!("Start pipe is weird"),
    };

    loop {
        distance += 1;

        match facing {
            Direction::North => y -= 1,
            Direction::South => y += 1,
            Direction::West => x -= 1,
            Direction::East => x += 1,
        }

        if x == sx && y == sy {
            // Loop completed
            return distance;
        }

        facing = match (pipes.read(x, y).unwrap(), facing) {
            (Pipe::Horiz, _) => facing,
            (Pipe::Vert, _) => facing,
            (Pipe::Se, Direction::West) => Direction::South,
            (Pipe::Se, Direction::North) => Direction::East,
            (Pipe::Sw, Direction::East) => Direction::South,
            (Pipe::Sw, Direction::North) => Direction::West,
            (Pipe::Ne, Direction::South) => Direction::East,
            (Pipe::Ne, Direction::West) => Direction::North,
            (Pipe::Nw, Direction::South) => Direction::West,
            (Pipe::Nw, Direction::East) => Direction::North,
            _ => panic!("Impossible combination"),
        }
    }
}

fn just_loop(pipes: &Pipes, sx: isize, sy: isize) -> Pipes {
    let mut x: isize = sx;
    let mut y: isize = sy;
    let mut just = Map::new();
    let mut facing = match pipes.read(sx, sy).unwrap() {
        Pipe::Horiz => Direction::West,
        Pipe::Vert => Direction::North,
        Pipe::Nw => Direction::North,
        Pipe::Ne => Direction::West,
        Pipe::Sw => Direction::South,
        Pipe::Se => Direction::East,
        _ => panic!("Start pipe is weird"),
    };

    loop {
        just.write(x, y, pipes.read(x, y).unwrap());

        match facing {
            Direction::North => y -= 1,
            Direction::South => y += 1,
            Direction::West => x -= 1,
            Direction::East => x += 1,
        }

        if x == sx && y == sy {
            // Loop completed
            return just;
        }

        facing = match (pipes.read(x, y).unwrap(), facing) {
            (Pipe::Horiz, _) => facing,
            (Pipe::Vert, _) => facing,
            (Pipe::Se, Direction::West) => Direction::South,
            (Pipe::Se, Direction::North) => Direction::East,
            (Pipe::Sw, Direction::East) => Direction::South,
            (Pipe::Sw, Direction::North) => Direction::West,
            (Pipe::Ne, Direction::South) => Direction::East,
            (Pipe::Ne, Direction::West) => Direction::North,
            (Pipe::Nw, Direction::South) => Direction::West,
            (Pipe::Nw, Direction::East) => Direction::North,
            _ => panic!("Impossible combination"),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum State {
    None,
    North,
    South,
}

// If there are an *odd* number of Vertical pipes left of us, we're inside
// if a pipe comes initially from the North, then tends East or West, but eventually turns South,
// that's equivalent to a Vertical pipe, whereas if it turns back North it is not
fn interior(pipes: &Pipes) -> usize {
    let mut n = 0;
    for y in pipes.y() {
        let mut inside = false;
        let mut state = State::None;
        for x in pipes.x() {
            let pipe = pipes.read(x, y).unwrap_or_default();
            match (state, pipe) {
                (State::None, Pipe::Ground) => {
                    if inside {
                        n += 1;
                    }
                }
                (State::None, Pipe::Vert) => {
                    inside = !inside;
                }
                (State::None, Pipe::Se) => {
                    state = State::South;
                }
                (State::None, Pipe::Ne) => {
                    state = State::North;
                }

                // Horizontal lines make no difference to anything
                (State::North | State::South, Pipe::Horiz) => {}

                // U-turns
                (State::South, Pipe::Sw) | (State::North, Pipe::Nw) => {
                    state = State::None;
                }

                // Form a vertical line
                (State::South, Pipe::Nw) | (State::North, Pipe::Sw) => {
                    inside = !inside;
                    state = State::None;
                }

                _ => {
                    panic!("Unexpected sequence {state:?} {pipe:?}");
                }
            }
        }
    }
    n
}

pub fn a() {
    let ctxt = readfile("10");
    let mut pipes: Pipes = ctxt.value().parse().expect("Should be a map of pipes");
    let s = pipes.find(|p| p == Pipe::Start);
    if s.len() != 1 {
        panic!("Pipe networks with more than one Start can't be solved");
    }
    let &(sx, sy) = s.first().unwrap();
    fix_start(&mut pipes, sx, sy);
    let len = length(&pipes, sx, sy);
    println!("Furthest from start is {} moves", len / 2);
}

pub fn b() {
    let ctxt = readfile("10");
    let mut pipes: Pipes = ctxt.value().parse().expect("Should be a map of pipes");
    let s = pipes.find(|p| p == Pipe::Start);
    if s.len() != 1 {
        panic!("Pipe networks with more than one Start can't be solved");
    }
    let &(sx, sy) = s.first().unwrap();
    fix_start(&mut pipes, sx, sy);
    let just = just_loop(&pipes, sx, sy);
    let count = interior(&just);
    println!("{count} tiles are enclosed by the loop");
}

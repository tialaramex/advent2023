use sky::map::Map;
use sky::readfile;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
enum Pixel {
    #[default]
    Space,
    Galaxy,
}

impl From<char> for Pixel {
    fn from(ch: char) -> Self {
        match ch {
            '.' => Self::Space,
            '#' => Self::Galaxy,
            _ => panic!("Invalid map symbol {ch}"),
        }
    }
}

type Pixels = Map<Pixel>;

fn check_vertical(sky: &Pixels) -> Vec<isize> {
    let mut v: Vec<isize> = Vec::new();
    for x in sky.x() {
        if sky
            .y()
            .all(|y| sky.read(x, y).unwrap_or_default() == Pixel::Space)
        {
            v.push(x);
        }
    }
    v
}

fn check_horizontal(sky: &Pixels) -> Vec<isize> {
    let mut h: Vec<isize> = Vec::new();
    for y in sky.y() {
        if sky
            .x()
            .all(|x| sky.read(x, y).unwrap_or_default() == Pixel::Space)
        {
            h.push(y);
        }
    }
    h
}

fn basic(v: &[isize], h: &[isize], from: (isize, isize), to: (isize, isize)) -> isize {
    distance(v, h, from, to, 1)
}

fn distance(v: &[isize], h: &[isize], from: (isize, isize), to: (isize, isize), f: usize) -> isize {
    let x = (to.0 - from.0).abs();
    let y = (to.1 - from.1).abs();
    let vert = v
        .iter()
        .filter(|&&x| (x > to.0 && x < from.0) || (x > from.0 && x < to.0))
        .count()
        * f;
    let horz = h
        .iter()
        .filter(|&&y| (y > to.1 && y < from.1) || (y > from.1 && y < to.1))
        .count()
        * f;
    let extra = vert + horz;
    x + y + (extra as isize)
}

pub fn a() {
    let ctxt = readfile("11");
    let sky: Pixels = ctxt.value().parse().expect("Should be a map of the sky");
    let v = check_vertical(&sky);
    let h = check_horizontal(&sky);
    let mut galaxies = sky.find(|p| p == Pixel::Galaxy);
    let mut sum = 0;
    while let Some((ox, oy)) = galaxies.pop() {
        for &(nx, ny) in galaxies.iter() {
            sum += basic(&v, &h, (ox, oy), (nx, ny));
        }
    }
    println!("Distances between all galaxy pairs add up to {sum}");
}

pub fn b() {
    let ctxt = readfile("11");
    let sky: Pixels = ctxt.value().parse().expect("Should be a map of the sky");
    let v = check_vertical(&sky);
    let h = check_horizontal(&sky);
    let mut galaxies = sky.find(|p| p == Pixel::Galaxy);
    let mut sum = 0;
    while let Some((ox, oy)) = galaxies.pop() {
        for &(nx, ny) in galaxies.iter() {
            sum += distance(&v, &h, (ox, oy), (nx, ny), 999_999);
        }
    }
    println!("Now, distances between all galaxy pairs add up to {sum}");
}

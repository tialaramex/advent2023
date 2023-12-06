use sky::readfile;

type Number = u64;

fn beat(duration: Number, best: Number) -> usize {
    (0..=duration)
        .map(|t| t * (duration - t))
        .filter(|&d| d > best)
        .count()
}

pub fn a() {
    let ctxt = readfile("06");
    let mut lines = ctxt.lines();
    let Some(t) = lines.next() else {
        panic!("Should have a line of times");
    };
    let Some(d) = lines.next() else {
        panic!("Should also have a line of distances");
    };

    let pairs = t.split_ascii_whitespace().zip(d.split_ascii_whitespace());
    let mut product = 1;
    for (time, distance) in pairs.skip(1) {
        let Ok(time): Result<Number, _> = time.parse() else {
            panic!("{time} isn't a number");
        };
        let Ok(distance): Result<Number, _> = distance.parse() else {
            panic!("{distance} isn't a number");
        };
        product *= beat(time, distance);
    }
    println!("Multiplying together how many ways I could win these races gives {product}");
}

// Strip off the prefix, fix the kerning, convert to a Number
fn fix_everything(s: &str) -> Number {
    let Some((_, s)) = s.split_once(' ') else {
        panic!("{s} doesn't have even a single space character");
    };
    let s = s.trim().replace(' ', "");
    let Ok(n) = s.parse() else {
        panic!("{s} is not an integer");
    };
    n
}

pub fn b() {
    let ctxt = readfile("06");
    let mut lines = ctxt.lines();
    let Some(t) = lines.next() else {
        panic!("Should have a line of times");
    };
    let Some(d) = lines.next() else {
        panic!("Should also have a line of distances");
    };
    let t = fix_everything(t);
    let d = fix_everything(d);
    let answer = beat(t, d);
    println!("Can beat the real race {answer:?} ways");
}

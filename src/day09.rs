use sky::readfile;

type Number = i32;

fn diff(values: &[Number]) -> Vec<Number> {
    let mut d = Vec::with_capacity(values.len() - 1);

    for pair in values.windows(2) {
        d.push(pair[1] - pair[0]);
    }
    d
}

fn guess_next(values: &[Number]) -> Number {
    let d = diff(values);
    if d.iter().all(|&n| n == 0) {
        *values.last().unwrap()
    } else {
        values.last().unwrap() + guess_next(&d)
    }
}

fn numbers(line: &str) -> Vec<Number> {
    line.split(' ')
        .map(|s| s.parse::<Number>().unwrap())
        .collect()
}

pub fn a() {
    let ctxt = readfile("09");
    let mut sum = 0;
    for line in ctxt.lines() {
        let g = guess_next(&numbers(line));
        sum += g;
    }
    println!("{sum}");
}

fn guess_prev(values: &[Number]) -> Number {
    let d = diff(values);
    if d.iter().all(|&n| n == 0) {
        *values.first().unwrap()
    } else {
        values.first().unwrap() - guess_prev(&d)
    }
}

pub fn b() {
    let ctxt = readfile("09");
    let mut sum = 0;
    for line in ctxt.lines() {
        let g = guess_prev(&numbers(line));
        sum += g;
    }
    println!("{sum}");
}

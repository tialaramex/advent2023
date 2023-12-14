use sky::readfile;

type Number = u32;

fn check_v_mirror(rows: &[String], mirror: usize) -> bool {
    let mut top = mirror - 1;
    let mut bottom = mirror;
    loop {
        if rows[top] != rows[bottom] {
            return false;
        }
        if top == 0 {
            return true;
        }
        top -= 1;
        bottom += 1;
        if bottom == rows.len() {
            return true;
        }
    }
}

fn check_mirror(rows: &[&str], mirror: usize) -> bool {
    let mut top = mirror - 1;
    let mut bottom = mirror;
    loop {
        if rows[top] != rows[bottom] {
            return false;
        }
        if top == 0 {
            return true;
        }
        top -= 1;
        bottom += 1;
        if bottom == rows.len() {
            return true;
        }
    }
}

fn columnize(rows: &[&str]) -> Vec<String> {
    let mut v = Vec::new();
    v.resize(rows[0].len(), String::new());
    for row in rows {
        for (k, c) in row.chars().enumerate() {
            v[k].push(c);
        }
    }
    v
}

fn assess(rows: &[&str]) -> Number {
    for r in 1..rows.len() {
        if rows[r] == rows[r - 1] && check_mirror(rows, r) {
            return (r as Number) * 100;
        }
    }

    let cols = columnize(rows);
    for c in 1..cols.len() {
        if cols[c] == cols[c - 1] && check_v_mirror(&cols, c) {
            return c as Number;
        }
    }
    0
}

pub fn a() {
    let ctxt = readfile("13");
    let mut rows = Vec::new();
    let mut sum = 0;

    for line in ctxt.lines() {
        if line.is_empty() {
            sum += assess(&rows);
            rows = Vec::new();
        } else {
            rows.push(line);
        }
    }
    sum += assess(&rows);
    println!("{sum}");
}

fn ham_v_mirror(rows: &[String], mirror: usize) -> usize {
    let mut ham = 0;
    let mut top = mirror - 1;
    let mut bottom = mirror;
    loop {
        ham += rows[top]
            .chars()
            .zip(rows[bottom].chars())
            .filter(|(t, b)| t != b)
            .count();
        if top == 0 {
            return ham;
        }
        top -= 1;
        bottom += 1;
        if bottom == rows.len() {
            return ham;
        }
    }
}

fn ham_mirror(rows: &[&str], mirror: usize) -> usize {
    let mut ham = 0;
    let mut top = mirror - 1;
    let mut bottom = mirror;
    loop {
        ham += rows[top]
            .chars()
            .zip(rows[bottom].chars())
            .filter(|(t, b)| t != b)
            .count();
        if top == 0 {
            return ham;
        }
        top -= 1;
        bottom += 1;
        if bottom == rows.len() {
            return ham;
        }
    }
}

fn reassess(rows: &[&str]) -> Number {
    for r in 1..rows.len() {
        if ham_mirror(rows, r) == 1 {
            return (r as Number) * 100;
        }
    }

    let cols = columnize(rows);
    for c in 1..cols.len() {
        if ham_v_mirror(&cols, c) == 1 {
            return c as Number;
        }
    }
    0
}

pub fn b() {
    let ctxt = readfile("13");
    let mut rows = Vec::new();
    let mut sum = 0;

    for line in ctxt.lines() {
        if line.is_empty() {
            sum += reassess(&rows);
            rows = Vec::new();
        } else {
            rows.push(line);
        }
    }
    sum += reassess(&rows);
    println!("{sum}");
}

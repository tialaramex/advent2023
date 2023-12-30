use sky::readfile;
use std::collections::HashMap;
use std::collections::HashSet;

fn extra(done: &HashSet<&str>, hm: &HashMap<&str, usize>, links: &[&str]) -> usize {
    let mut count = 0;
    for link in links {
        if !done.contains(link) && !hm.contains_key(link) {
            count += 1;
        }
    }
    count
}

fn clinks(hm: &HashMap<&str, usize>) -> usize {
    hm.iter().map(|(_, v)| v).sum()
}

pub fn a() {
    let ctxt = readfile("25");
    let mut components: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in ctxt.lines() {
        let (fto, rest) = line
            .split_once(": ")
            .expect("Line should have ': ' separator");
        for node in rest.split(' ') {
            let entry = components.entry(fto).or_default();
            entry.push(node);
            entry.sort();
            let entry = components.entry(node).or_default();
            entry.push(fto);
            entry.sort();
        }
    }

    let keys: Vec<&str> = components.keys().copied().collect();
    let chosen = keys.first().unwrap();
    let mut mine: HashSet<&str> = HashSet::new();
    mine.insert(chosen);

    let mut outward: HashMap<&str, usize> = HashMap::new();
    let links = &components[chosen];
    for link in links {
        outward.insert(link, 1);
    }

    while clinks(&outward) >= 4 {
        let mut fewest = 1000;
        let mut best: Option<&str> = None;
        for (to, far) in outward.iter() {
            let extras = extra(&mine, &outward, &components[to]);
            if extras < fewest || *far >= extras {
                fewest = extras;
                best = Some(to);
            }
        }
        let best = best.unwrap();
        outward.remove(best);
        mine.insert(best);
        let links = &components[best];
        for link in links {
            if !mine.contains(link) {
                let n = outward.entry(link).or_default();
                *n += 1;
            }
        }
    }
    let a = mine.len();
    let b = keys.len() - a;
    println!(
        "Sizes of each group multiplied together: {a} x {b} = {}",
        a * b
    );
}

pub fn b() {
    println!("Happy Christmas!");
}

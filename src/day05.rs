use sky::readfile;

type Number = u64;

// Inclusive
#[derive(Copy, Clone, Debug)]
struct Numbers {
    from: Number,
    to: Number,
}

impl Numbers {
    // cut gives three optional Numbers::  before, transformed, after
    fn cut(
        self,
        src: Number,
        len: Number,
        dest: Number,
    ) -> (Option<Self>, Option<Self>, Option<Self>) {
        if src > self.to {
            return (Some(self), None, None);
        }
        if src + len < self.from {
            return (None, None, Some(self));
        }

        let before = if src > self.from {
            Some(Self {
                from: self.from,
                to: src - 1,
            })
        } else {
            None
        };

        let after = if src + len < self.to {
            Some(Self {
                from: src + len + 1,
                to: self.to,
            })
        } else {
            None
        };

        let left = if src > self.from { src } else { self.from };
        let right = if src + len < self.to {
            src + len
        } else {
            self.to
        };

        let transformed = Some(Self {
            from: left + dest - src,
            to: right + dest - src,
        });

        (before, transformed, after)
    }
}

#[derive(Debug)]
struct Map {
    dest: Vec<Number>,
    src: Vec<Number>,
    len: Vec<Number>,
}

impl Map {
    fn read(lines: &mut dyn Iterator<Item = &str>) -> Self {
        let mut dest = Vec::new();
        let mut src = Vec::new();
        let mut len = Vec::new();

        let Some(_) = lines.next() else {
            panic!("Missing map");
        };
        for line in lines {
            if line.is_empty() {
                break;
            }
            let Some((d, rest)) = line.split_once(' ') else {
                panic!("Map line missing first space");
            };
            let Ok(d): Result<Number, _> = d.parse() else {
                panic!("Destination is not an integer");
            };
            let Some((s, l)) = rest.split_once(' ') else {
                panic!("Map line missing second space");
            };
            let Ok(s): Result<Number, _> = s.parse() else {
                panic!("Source is not an integer");
            };
            let Ok(l): Result<Number, _> = l.parse() else {
                panic!("Length is not an integer");
            };
            dest.push(d);
            src.push(s);
            len.push(l);
        }
        Map { dest, src, len }
    }

    fn apply(&self, n: Number) -> Number {
        for ((&s, &l), &d) in self.src.iter().zip(self.len.iter()).zip(self.dest.iter()) {
            if n >= s && n <= s + l {
                return (n + d) - s;
            }
        }
        // If unmapped, the same
        n
    }

    fn ranges(&self, mut input: Vec<Numbers>) -> Vec<Numbers> {
        let mut out = Vec::new();

        for ((&s, &l), &d) in self.src.iter().zip(self.len.iter()).zip(self.dest.iter()) {
            let mut residue = Vec::new();
            for numbers in input {
                let (before, transformed, after) = numbers.cut(s, l, d);
                if let Some(before) = before {
                    residue.push(before);
                }
                if let Some(transformed) = transformed {
                    out.push(transformed);
                }
                if let Some(after) = after {
                    residue.push(after);
                }
            }
            input = residue;
        }

        out.append(&mut input);
        out
    }
}

fn seed_list(s: &str) -> Vec<Number> {
    let list = s.split(' ').skip(1);
    list.filter_map(|s| s.parse().ok()).collect()
}

fn revised_seed_list(s: &str) -> Vec<Numbers> {
    let mut list = s.split(' ').skip(1);
    let mut seeds = Vec::new();
    while let Some(start) = list.next().and_then(|start| start.parse().ok()) {
        let Some(count): Option<Number> = list.next().and_then(|count| count.parse().ok()) else {
            panic!("Uneveen seed list cannot work as described in problem");
        };
        seeds.push(Numbers {
            from: start,
            to: start + count - 1,
        });
    }

    seeds
}

pub fn a() {
    let ctxt = readfile("05");
    let mut lines = ctxt.lines();
    let Some(seeds) = lines.next() else {
        panic!("No input lines");
    };
    let seeds = seed_list(seeds);
    let Some("") = lines.next() else {
        panic!("Expected blank line");
    };
    let seed_to_soil = Map::read(&mut lines);
    let soils: Vec<Number> = seeds.iter().map(|&n| seed_to_soil.apply(n)).collect();
    let soil_to_fert = Map::read(&mut lines);
    let ferts: Vec<Number> = soils.iter().map(|&n| soil_to_fert.apply(n)).collect();
    let fert_to_water = Map::read(&mut lines);
    let waters: Vec<Number> = ferts.iter().map(|&n| fert_to_water.apply(n)).collect();
    let water_to_light = Map::read(&mut lines);
    let lights: Vec<Number> = waters.iter().map(|&n| water_to_light.apply(n)).collect();
    let light_to_temp = Map::read(&mut lines);
    let temps: Vec<Number> = lights.iter().map(|&n| light_to_temp.apply(n)).collect();
    let temp_to_humid = Map::read(&mut lines);
    let humids: Vec<Number> = temps.iter().map(|&n| temp_to_humid.apply(n)).collect();
    let humid_to_locn = Map::read(&mut lines);
    let locns: Vec<Number> = humids.iter().map(|&n| humid_to_locn.apply(n)).collect();
    let lowest = locns
        .iter()
        .min()
        .expect("There should be at least one location");
    println!("Lowest location number is {lowest}");
}

pub fn b() {
    let ctxt = readfile("05");
    let mut lines = ctxt.lines();
    let Some(seeds) = lines.next() else {
        panic!("No input lines");
    };
    let mut seeds = revised_seed_list(seeds);
    let Some("") = lines.next() else {
        panic!("Expected blank line");
    };
    for _ in 0..7 {
        let step = Map::read(&mut lines);
        seeds = step.ranges(seeds);
    }
    let lowest = seeds
        .iter()
        .map(|n| n.from)
        .min()
        .expect("Should be at least one seed");
    println!("Lowest location number was {lowest}");
}

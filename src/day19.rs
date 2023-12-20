use sky::readfile;
use std::collections::HashMap;

type Number = u64;

#[derive(Clone, Debug)]
enum Rule {
    Always(String),
    Xless(Number, String),
    Xmore(Number, String),
    Mless(Number, String),
    Mmore(Number, String),
    Aless(Number, String),
    Amore(Number, String),
    Sless(Number, String),
    Smore(Number, String),
}

impl Rule {
    fn flow(&self) -> &str {
        match self {
            Rule::Always(s) => &s,
            Rule::Xless(_, s) => &s,
            Rule::Xmore(_, s) => &s,
            Rule::Mless(_, s) => &s,
            Rule::Mmore(_, s) => &s,
            Rule::Aless(_, s) => &s,
            Rule::Amore(_, s) => &s,
            Rule::Sless(_, s) => &s,
            Rule::Smore(_, s) => &s,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Ratings {
    x: Number,
    m: Number,
    a: Number,
    s: Number,
}

impl Ratings {
    fn total(&self) -> Number {
        self.x + self.m + self.a + self.s
    }
}

use std::str::FromStr;
impl FromStr for Ratings {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((x, rest)) = s.split_once(",m=") else {
            return Err("No m rating");
        };
        let Some((m, rest)) = rest.split_once(",a=") else {
            return Err("No a rating");
        };
        let Some((a, s)) = rest.split_once(",s=") else {
            return Err("No s rating");
        };
        let Some(x) = x.strip_prefix("{x=") else {
            return Err("Does not start with {x=");
        };
        let Ok(x): Result<Number, _> = x.parse() else {
            return Err("Rating x is not an integer");
        };
        let Ok(m): Result<Number, _> = m.parse() else {
            return Err("Rating m is not an integer");
        };
        let Ok(a): Result<Number, _> = a.parse() else {
            return Err("Rating a is not an integer");
        };
        let Some(s) = s.strip_suffix('}') else {
            return Err("Does not end with }");
        };
        let Ok(s): Result<Number, _> = s.parse() else {
            return Err("Rating s is not an integer");
        };
        Ok(Ratings { x, m, a, s })
    }
}

fn parse(s: &str) -> (&str, Vec<Rule>) {
    let (name, rest) = s.split_once('{').expect("Workflow should have braces");
    let mut rules = Vec::new();
    let rest = rest
        .strip_suffix('}')
        .expect("Workflow should have braces surrounding rules");
    for rule in rest.split(',') {
        let Some((condition, target)) = rule.split_once(':') else {
            rules.push(Rule::Always(String::from(rule)));
            continue;
        };
        let (what, num) = condition.split_at(2);
        let num: Number = num.parse().expect("Numerics should be integers");
        let target = String::from(target);
        let rule = match what {
            "x<" => Rule::Xless(num, target),
            "x>" => Rule::Xmore(num, target),
            "m<" => Rule::Mless(num, target),
            "m>" => Rule::Mmore(num, target),
            "a<" => Rule::Aless(num, target),
            "a>" => Rule::Amore(num, target),
            "s<" => Rule::Sless(num, target),
            "s>" => Rule::Smore(num, target),
            _ => panic!("Impossible rule {rule}"),
        };
        rules.push(rule);
    }

    (name, rules)
}

type Rules<'t> = HashMap<&'t str, Vec<Rule>>;

fn accept(rules: &Rules, part: Ratings) -> bool {
    let mut name = "in";
    loop {
        let conditions = rules.get(name).expect("Named rule should in rules list");
        for cond in conditions {
            match cond {
                Rule::Always(n) => {
                    name = &n;
                    break;
                }
                Rule::Xless(num, n) => {
                    if part.x < *num {
                        name = &n;
                        break;
                    }
                }
                Rule::Xmore(num, n) => {
                    if part.x > *num {
                        name = &n;
                        break;
                    }
                }
                Rule::Mless(num, n) => {
                    if part.m < *num {
                        name = &n;
                        break;
                    }
                }
                Rule::Mmore(num, n) => {
                    if part.m > *num {
                        name = &n;
                        break;
                    }
                }
                Rule::Aless(num, n) => {
                    if part.a < *num {
                        name = &n;
                        break;
                    }
                }
                Rule::Amore(num, n) => {
                    if part.a > *num {
                        name = &n;
                        break;
                    }
                }
                Rule::Sless(num, n) => {
                    if part.s < *num {
                        name = &n;
                        break;
                    }
                }
                Rule::Smore(num, n) => {
                    if part.s > *num {
                        name = &n;
                        break;
                    }
                }
            }
        }

        if name == "A" {
            return true;
        }
        if name == "R" {
            return false;
        }
    }
}

use core::ops::RangeInclusive;

#[derive(Clone, Debug)]
struct Combs {
    x: RangeInclusive<Number>,
    m: RangeInclusive<Number>,
    a: RangeInclusive<Number>,
    s: RangeInclusive<Number>,
}

impl Combs {
    const fn new() -> Self {
        const FULL: RangeInclusive<Number> = 1..=4000;
        Self {
            x: FULL,
            m: FULL,
            a: FULL,
            s: FULL,
        }
    }

    fn size(&self) -> Number {
        let x = self.x.end() - self.x.start() + 1;
        let m = self.m.end() - self.m.start() + 1;
        let a = self.a.end() - self.a.start() + 1;
        let s = self.s.end() - self.s.start() + 1;
        x * m * a * s
    }

    // passed rule  vs failed rule
    fn split(self, rule: &Rule) -> (Option<Self>, Option<Self>) {
        match rule {
            Rule::Always(_) => (Some(self), None),
            Rule::Xless(num, _) => {
                let start = self.x.start().clone();
                let end = self.x.end().clone();
                if num <= &start {
                    return (None, Some(self));
                }
                if num >= &end {
                    return (Some(self), None);
                }
                (
                    Some(Self {
                        x: start..=(num - 1),
                        ..self.clone()
                    }),
                    Some(Self {
                        x: *num..=end,
                        ..self
                    }),
                )
            }
            Rule::Xmore(num, _) => {
                let start = self.x.start().clone();
                let end = self.x.end().clone();
                if num <= &start {
                    return (Some(self), None);
                }
                if num >= &end {
                    return (None, Some(self));
                }
                (
                    Some(Self {
                        x: (num + 1)..=end,
                        ..self.clone()
                    }),
                    Some(Self {
                        x: start..=*num,
                        ..self
                    }),
                )
            }
            Rule::Mless(num, _) => {
                let start = self.m.start().clone();
                let end = self.m.end().clone();
                if num <= &start {
                    return (None, Some(self));
                }
                if num >= &end {
                    return (Some(self), None);
                }
                (
                    Some(Self {
                        m: start..=(num - 1),
                        ..self.clone()
                    }),
                    Some(Self {
                        m: *num..=end,
                        ..self
                    }),
                )
            }
            Rule::Mmore(num, _) => {
                let start = self.m.start().clone();
                let end = self.m.end().clone();
                if num <= &start {
                    return (Some(self), None);
                }
                if num >= &end {
                    return (None, Some(self));
                }
                (
                    Some(Self {
                        m: (num + 1)..=end,
                        ..self.clone()
                    }),
                    Some(Self {
                        m: start..=*num,
                        ..self
                    }),
                )
            }
            Rule::Aless(num, _) => {
                let start = self.a.start().clone();
                let end = self.a.end().clone();
                if num <= &start {
                    return (None, Some(self));
                }
                if num >= &end {
                    return (Some(self), None);
                }
                (
                    Some(Self {
                        a: start..=(num - 1),
                        ..self.clone()
                    }),
                    Some(Self {
                        a: *num..=end,
                        ..self
                    }),
                )
            }
            Rule::Amore(num, _) => {
                let start = self.a.start().clone();
                let end = self.a.end().clone();
                if num <= &start {
                    return (Some(self), None);
                }
                if num >= &end {
                    return (None, Some(self));
                }
                (
                    Some(Self {
                        a: (num + 1)..=end,
                        ..self.clone()
                    }),
                    Some(Self {
                        a: start..=*num,
                        ..self
                    }),
                )
            }
            Rule::Sless(num, _) => {
                let start = self.s.start().clone();
                let end = self.s.end().clone();
                if num <= &start {
                    return (None, Some(self));
                }
                if num >= &end {
                    return (Some(self), None);
                }
                (
                    Some(Self {
                        s: start..=(num - 1),
                        ..self.clone()
                    }),
                    Some(Self {
                        s: *num..=end,
                        ..self
                    }),
                )
            }
            Rule::Smore(num, _) => {
                let start = self.s.start().clone();
                let end = self.s.end().clone();
                if num <= &start {
                    return (Some(self), None);
                }
                if num >= &end {
                    return (None, Some(self));
                }
                (
                    Some(Self {
                        s: (num + 1)..=end,
                        ..self.clone()
                    }),
                    Some(Self {
                        s: start..=*num,
                        ..self
                    }),
                )
            }
        }
    }
}

type Attempt<'a> = (&'a str, Combs);

fn combinations(rules: &Rules) -> Number {
    let mut accepted = 0;
    let mut todo: Vec<Attempt> = vec![("in", Combs::new())];

    while let Some((flow, comb)) = todo.pop() {
        if flow == "A" {
            accepted += comb.size();
            continue;
        }
        if flow == "R" {
            continue;
        }
        let mut onward = comb;
        for rule in rules.get(flow).expect("Should be a valid rule") {
            let (pass, fail) = onward.split(rule);
            let flow = rule.flow();
            if let Some(pass) = pass {
                todo.push((flow, pass));
            }
            let Some(fail) = fail else {
                break;
            };
            onward = fail;
        }
    }

    accepted
}

pub fn a() {
    let ctxt = readfile("19");
    let mut lines = ctxt.lines();
    let mut rules: Rules = HashMap::new();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let (id, v) = parse(line);
        rules.insert(id, v);
    }

    let mut sum = 0;
    for line in lines {
        let part: Ratings = line
            .parse()
            .expect("Should be ratings below the blank line");
        if accept(&rules, part) {
            sum += part.total();
        }
    }
    println!("Total ratings of accepted parts: {sum}");
}

pub fn b() {
    let ctxt = readfile("19");
    let mut rules: Rules = HashMap::new();

    for line in ctxt.lines() {
        if line.is_empty() {
            break;
        }
        let (id, v) = parse(line);
        rules.insert(id, v);
    }

    let com = combinations(&rules);
    println!("{com} distinct combinations of ratings are accepted");
}

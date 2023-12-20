use sky::readfile;
use std::collections::HashMap;

type Number = u64;

#[derive(Copy, Clone, Debug)]
enum MoreOrLess {
    Less,
    More,
}

#[derive(Copy, Clone, Debug)]
enum Xmas {
    X,
    M,
    A,
    S,
}

#[derive(Clone, Debug)]
struct Rule {
    target: String,
    kind: MoreOrLess,
    letter: Xmas,
    num: Number,
}

impl Rule {
    fn unconditional(target: String) -> Self {
        Rule {
            target,
            kind: MoreOrLess::More,
            letter: Xmas::X,
            num: 0,
        }
    }

    fn flow(&self) -> &str {
        &self.target
    }
}

#[derive(Copy, Clone, Debug)]
struct Ratings {
    x: Number,
    m: Number,
    a: Number,
    s: Number,
}

impl std::ops::Index<Xmas> for Ratings {
    type Output = Number;

    fn index(&self, index: Xmas) -> &Self::Output {
        match index {
            Xmas::X => &self.x,
            Xmas::M => &self.m,
            Xmas::A => &self.a,
            Xmas::S => &self.s,
        }
    }
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
            rules.push(Rule::unconditional(String::from(rule)));
            continue;
        };
        let (what, num) = condition.split_at(2);
        let num: Number = num.parse().expect("Numerics should be integers");
        let target = String::from(target);
        let rule = match what {
            "x<" => Rule {
                target,
                kind: MoreOrLess::Less,
                letter: Xmas::X,
                num,
            },
            "x>" => Rule {
                target,
                kind: MoreOrLess::More,
                letter: Xmas::X,
                num,
            },
            "m<" => Rule {
                target,
                kind: MoreOrLess::Less,
                letter: Xmas::M,
                num,
            },
            "m>" => Rule {
                target,
                kind: MoreOrLess::More,
                letter: Xmas::M,
                num,
            },
            "a<" => Rule {
                target,
                kind: MoreOrLess::Less,
                letter: Xmas::A,
                num,
            },
            "a>" => Rule {
                target,
                kind: MoreOrLess::More,
                letter: Xmas::A,
                num,
            },
            "s<" => Rule {
                target,
                kind: MoreOrLess::Less,
                letter: Xmas::S,
                num,
            },
            "s>" => Rule {
                target,
                kind: MoreOrLess::More,
                letter: Xmas::S,
                num,
            },
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
        let conditions = rules.get(name).expect("Named rule should be in rules list");
        for cond in conditions {
            match cond.kind {
                MoreOrLess::Less => {
                    if part[cond.letter] < cond.num {
                        name = &cond.target;
                        break;
                    }
                }
                MoreOrLess::More => {
                    if part[cond.letter] > cond.num {
                        name = &cond.target;
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

    fn edges(&self, letter: Xmas) -> (Number, Number) {
        match letter {
            Xmas::X => (*self.x.start(), *self.x.end()),
            Xmas::M => (*self.m.start(), *self.m.end()),
            Xmas::A => (*self.a.start(), *self.a.end()),
            Xmas::S => (*self.s.start(), *self.s.end()),
        }
    }

    // passed rule  vs failed rule
    fn split(self, rule: &Rule) -> (Option<Self>, Option<Self>) {
        let (start, end) = self.edges(rule.letter);
        match rule.kind {
            MoreOrLess::Less => {
                if rule.num <= start {
                    return (None, Some(self));
                }
                if rule.num >= end {
                    return (Some(self), None);
                }
                match rule.letter {
                    Xmas::X => (
                        Some(Self {
                            x: start..=(rule.num - 1),
                            ..self.clone()
                        }),
                        Some(Self {
                            x: rule.num..=end,
                            ..self
                        }),
                    ),
                    Xmas::M => (
                        Some(Self {
                            m: start..=(rule.num - 1),
                            ..self.clone()
                        }),
                        Some(Self {
                            m: rule.num..=end,
                            ..self
                        }),
                    ),
                    Xmas::A => (
                        Some(Self {
                            a: start..=(rule.num - 1),
                            ..self.clone()
                        }),
                        Some(Self {
                            a: rule.num..=end,
                            ..self
                        }),
                    ),
                    Xmas::S => (
                        Some(Self {
                            s: start..=(rule.num - 1),
                            ..self.clone()
                        }),
                        Some(Self {
                            s: rule.num..=end,
                            ..self
                        }),
                    ),
                }
            }
            MoreOrLess::More => {
                if rule.num <= start {
                    return (Some(self), None);
                }
                if rule.num >= end {
                    return (None, Some(self));
                }
                match rule.letter {
                    Xmas::X => (
                        Some(Self {
                            x: (rule.num + 1)..=end,
                            ..self.clone()
                        }),
                        Some(Self {
                            x: start..=(rule.num),
                            ..self
                        }),
                    ),
                    Xmas::M => (
                        Some(Self {
                            m: (rule.num + 1)..=end,
                            ..self.clone()
                        }),
                        Some(Self {
                            m: start..=(rule.num),
                            ..self
                        }),
                    ),
                    Xmas::A => (
                        Some(Self {
                            a: (rule.num + 1)..=end,
                            ..self.clone()
                        }),
                        Some(Self {
                            a: start..=(rule.num),
                            ..self
                        }),
                    ),
                    Xmas::S => (
                        Some(Self {
                            s: (rule.num + 1)..=end,
                            ..self.clone()
                        }),
                        Some(Self {
                            s: start..=(rule.num),
                            ..self
                        }),
                    ),
                }
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

    for line in lines.by_ref() {
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

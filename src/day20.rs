use sky::readfile;

type Number = u64;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Pulse {
    Low,
    High,
}

#[derive(Copy, Clone, Default, Eq, PartialEq, Hash)]
struct Id {
    bytes: [u8; 2],
}

use std::fmt::{Debug, Formatter};

impl Debug for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        if self.bytes == [0; 2] {
            f.write_str("Id:broadcaster")
        } else {
            f.write_str("Id:")?;
            for byte in self.bytes {
                f.write_fmt(format_args!("{}", char::from_u32(byte as u32).unwrap()))?
            }
            Ok(())
        }
    }
}

use std::str::FromStr;
impl FromStr for Id {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "broadcaster" {
            Ok(Id { bytes: [0; 2] })
        } else if s.len() == 2 {
            let mut id: Id = Default::default();
            id.bytes.copy_from_slice(s.as_bytes());
            Ok(id)
        } else if s.len() == 1 {
            let mut id: Id = Default::default();
            id.bytes[0..=0].copy_from_slice(s.as_bytes());
            Ok(id)
        } else if s.len() > 2 {
            let mut id: Id = Default::default();
            id.bytes.copy_from_slice(&s.as_bytes()[0..=1]);
            Ok(id)
        } else {
            Err("Unsuitable identifier, should be two ASCII symbols")
        }
    }
}

impl Id {
    const BUTTON: Self = Self { bytes: [b'_'; 2] };
    const BROADCAST: Self = Self { bytes: [0; 2] };
    const RX: Self = Self {
        bytes: [b'r', b'x'],
    };
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcast,
}

#[derive(Clone, Debug)]
struct Module {
    kind: ModuleType,
    dest: Vec<Id>,
    src: Vec<Id>,
    state: u32,
}

fn linear_search(haystack: &[Id], needle: Id) -> Option<usize> {
    for (n, &id) in haystack.iter().enumerate() {
        if id == needle {
            return Some(n);
        }
    }
    None
}

impl Module {
    fn warn(&mut self, sender: Id) {
        self.src.push(sender);
        if self.kind == ModuleType::Conjunction {
            self.state = (self.state << 1) | 0x1;
        }
    }

    fn reset(&mut self) {
        if self.kind == ModuleType::Conjunction {
            self.state = (1 << self.src.len()) - 1;
        } else {
            self.state = 0;
        }
    }

    fn signal(&mut self, from: Id, signal: Pulse) -> Option<(Pulse, Vec<Id>)> {
        match (self.kind, signal) {
            (ModuleType::Broadcast, input) => Some((input, self.dest.clone())),
            (ModuleType::FlipFlop, Pulse::High) => None,
            (ModuleType::FlipFlop, Pulse::Low) => {
                if self.state == 0 {
                    self.state = 1;
                    Some((Pulse::High, self.dest.clone()))
                } else {
                    self.state = 0;
                    Some((Pulse::Low, self.dest.clone()))
                }
            }
            (ModuleType::Conjunction, input) => {
                // We're representing memory of low signals in Conjunction as 1 bits.
                let from =
                    linear_search(&self.src, from).expect("Should not send signals unexpectedly");
                match input {
                    Pulse::Low => {
                        self.state |= 0x1 << from;
                    }
                    Pulse::High => {
                        self.state &= !(0x1 << from);
                    }
                }
                // ... because that makes this test easier
                if self.state == 0 {
                    Some((Pulse::Low, self.dest.clone()))
                } else {
                    Some((Pulse::High, self.dest.clone()))
                }
            }
        }
    }
}

fn parse(line: &str) -> (Id, Module) {
    let (name, list) = line
        .split_once(" -> ")
        .expect("Every line should have an arrow");
    let dest: Vec<Id> = list.split(", ").map(|n| n.parse::<Id>().unwrap()).collect();
    let src = Vec::new();

    if let Some(name) = name.strip_prefix('%') {
        let id: Id = name.parse().unwrap();
        return (
            id,
            Module {
                kind: ModuleType::FlipFlop,
                dest,
                src,
                state: 0,
            },
        );
    }

    if let Some(name) = name.strip_prefix('&') {
        let id: Id = name.parse().unwrap();
        return (
            id,
            Module {
                kind: ModuleType::Conjunction,
                dest,
                src,
                state: 0,
            },
        );
    }

    assert!(name == "broadcaster");
    (
        Id::BROADCAST,
        Module {
            kind: ModuleType::Broadcast,
            dest,
            src,
            state: 0,
        },
    )
}

use std::collections::HashMap;
#[derive(Clone, Debug, Default)]
struct System {
    modules: HashMap<Id, Module>,
}

use std::collections::VecDeque;

impl System {
    fn insert(&mut self, id: Id, module: Module) {
        self.modules.insert(id, module);
    }

    fn reset(&mut self) {
        for (_, module) in self.modules.iter_mut() {
            module.reset();
        }
    }

    /// Initialization should happen after inserting modules, not before
    fn init(&mut self) {
        let mut v: Vec<(Id, Id)> = Vec::new();

        for (&from, module) in self.modules.iter() {
            for &to in module.dest.iter() {
                v.push((from, to));
            }
        }
        for (from, to) in v {
            if let Some(module) = self.modules.get_mut(&to) {
                module.warn(from);
            }
        }
    }

    /// Vec of Ids which send to this target Id
    fn by_target(&self, target: Id) -> Vec<Id> {
        let mut sources = Vec::new();

        for (&source, module) in self.modules.iter() {
            if module.dest.contains(&target) {
                sources.push(source);
            }
        }
        sources
    }

    fn button(&mut self) -> (Number, Number) {
        let mut todo: VecDeque<(Pulse, Id, Id)> = VecDeque::new();
        let mut low = 0;
        let mut high = 0;

        // Begin with a single button push -> Low signal to Broadcaster
        todo.push_back((Pulse::Low, Id::BUTTON, Id::BROADCAST));

        while let Some((signal, from, to)) = todo.pop_front() {
            match signal {
                Pulse::Low => low += 1,
                Pulse::High => high += 1,
            }
            if let Some(module) = self.modules.get_mut(&to) {
                match module.signal(from, signal) {
                    None => {}
                    Some((signal, ids)) => {
                        for id in ids {
                            todo.push_back((signal, to, id));
                        }
                    }
                }
            }
        }

        (low, high)
    }

    fn cycle(&mut self, check: Id) -> bool {
        let mut todo: VecDeque<(Pulse, Id, Id)> = VecDeque::new();

        // Begin with a single button push -> Low signal to Broadcaster
        todo.push_back((Pulse::Low, Id::BUTTON, Id::BROADCAST));

        while let Some((signal, from, to)) = todo.pop_front() {
            if from == check && signal == Pulse::High {
                return true;
            }
            if let Some(module) = self.modules.get_mut(&to) {
                match module.signal(from, signal) {
                    None => {}
                    Some((signal, ids)) => {
                        for id in ids {
                            todo.push_back((signal, to, id));
                        }
                    }
                }
            }
        }
        false
    }
}

pub fn a() {
    let ctxt = readfile("20");
    let mut sys: System = Default::default();
    for line in ctxt.lines() {
        let (id, module) = parse(line);
        sys.insert(id, module);
    }
    sys.init();
    let mut low = 0;
    let mut high = 0;
    for _ in 0..1000 {
        let (l, h) = sys.button();
        low += l;
        high += h;
    }
    println!("Low pulses x High pulses: {low}x{high} ={}", low * high);
}

pub fn b() {
    let ctxt = readfile("20");
    let mut sys: System = Default::default();
    for line in ctxt.lines() {
        let (id, module) = parse(line);
        sys.insert(id, module);
    }
    sys.init();

    // find who signals 'rx'
    // trace that back to N senders

    let from = sys.by_target(Id::RX);
    assert_eq!(from.len(), 1);
    let &from = from.first().unwrap();
    let senders = sys.by_target(from);

    let mut cycle: Number = 1;

    // for each sender, find cycle length and use lcm to calculate when cycles align
    'cycles: for id in senders {
        sys.reset();
        for k in 1..20_000 {
            if sys.cycle(id) {
                cycle = num::Integer::lcm(&cycle, &k);
                continue 'cycles;
            }
        }
        panic!("No cycle detected for {id:?}");
    }

    println!("After {cycle} buttom presses the 'rx' module gets a Low pulse");
}

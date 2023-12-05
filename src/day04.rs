use sky::readfile;

type Number = u32;

#[derive(Debug)]
struct Card {
    left: Vec<Number>,
    right: Vec<Number>,
}

use std::str::FromStr;
impl FromStr for Card {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((_, rest)) = s.split_once(": ") else {
            return Err("Not a scratch card");
        };

        let Some((left, right)) = rest.split_once(" | ") else {
            return Err("Lacks dividing bar");
        };

        let left: Vec<Number> = left.split(' ').filter_map(|n| n.parse().ok()).collect();
        let right: Vec<Number> = right.split(' ').filter_map(|n| n.parse().ok()).collect();

        Ok(Card { left, right })
    }
}

impl Card {
    fn points(&self) -> Number {
        let mut p = 0;
        for n in &self.right {
            if self.left.contains(n) {
                if p == 0 {
                    p = 1;
                } else {
                    p *= 2;
                }
            }
        }
        p
    }

    fn matches(&self) -> Number {
        let mut p = 0;
        for n in &self.right {
            if self.left.contains(n) {
                p += 1;
            }
        }
        p
    }
}

pub fn a() {
    let ctxt = readfile("04");
    let mut sum = 0;
    for line in ctxt.lines() {
        let Ok(card): Result<Card, _> = line.parse() else {
            panic!("Invalid card: {line}");
        };
        sum += card.points();
    }
    println!("Scratch cards are worth {sum} points in total");
}

pub fn b() {
    let ctxt = readfile("04");
    let mut cards = Vec::new();
    for line in ctxt.lines() {
        let Ok(card): Result<Card, _> = line.parse() else {
            panic!("Invalid card: {line}");
        };
        cards.push(card);
    }
    let mut copies = Vec::new();
    copies.resize(cards.len(), 1);

    let mut count = 0;
    for k in 0..cards.len() {
        let n = copies[k];
        let m = cards[k].matches() as usize;
        for j in 1..=m {
            copies[k + j] += n;
        }

        count += n;
    }

    println!("Processed {count} scratch cards");
}

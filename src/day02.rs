use sky::readfile;

type Number = u32;

#[derive(Copy, Clone, Debug)]
struct Round {
    red: Number,
    green: Number,
    blue: Number,
}

use std::str::FromStr;
impl FromStr for Round {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        let mut split = s.split(", ");
        while let Some(pair) = split.next() {
            let Some((amount, color)) = pair.split_once(' ') else {
                return Err("No space in pair");
            };
            let Ok(amount): Result<Number, _> = amount.parse() else {
                return Err("Amount isn't a number");
            };
            match color {
                "red" => {
                    red += amount;
                }
                "green" => {
                    green += amount;
                }
                "blue" => {
                    blue += amount;
                }
                _ => {
                    return Err("Unacceptable color");
                }
            }
        }

        Ok(Round { red, green, blue })
    }
}

#[derive(Debug)]
struct Game {
    id: Number,
    rounds: Vec<Round>,
}

impl FromStr for Game {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((game, rounds)) = s.split_once(": ") else {
            return Err("Cannot discern game ID");
        };
        let Some(id) = game.strip_prefix("Game ") else {
            return Err("Game ID missing");
        };
        let Ok(id): Result<Number, _> = id.parse() else {
            return Err("Game ID not numeric");
        };
        let mut split = rounds.split("; ");
        let mut rounds = Vec::new();
        while let Some(round) = split.next() {
            let Ok(round): Result<Round, _> = round.parse() else {
                return Err("Can't parse round");
            };
            rounds.push(round);
        }

        Ok(Game { id, rounds })
    }
}

impl Game {
    fn possible(&self, red: Number, green: Number, blue: Number) -> bool {
        for r in &self.rounds {
            if r.red > red || r.green > green || r.blue > blue {
                return false;
            }
        }
        true
    }

    fn power(&self) -> Number {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for r in &self.rounds {
            red = std::cmp::max(red, r.red);
            green = std::cmp::max(green, r.green);
            blue = std::cmp::max(blue, r.blue);
        }
        red * green * blue
    }
}

pub fn a() {
    let ctxt = readfile("02");
    let mut sum = 0;
    for line in ctxt.lines() {
        let Ok(game): Result<Game, &str> = line.parse() else {
            panic!("Couldn't parse: {line}");
        };
        if game.possible(12, 13, 14) {
            sum += game.id
        }
    }
    println!("Sum of IDs of possible games is {sum}");
}

pub fn b() {
    let ctxt = readfile("02");
    let mut sum = 0;
    for line in ctxt.lines() {
        let Ok(game): Result<Game, &str> = line.parse() else {
            panic!("Couldn't parse: {line}");
        };
        sum += game.power();
    }
    println!("Sum of powers of games is {sum}");
}

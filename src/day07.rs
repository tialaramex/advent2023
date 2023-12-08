use sky::readfile;

type Number = u32;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

use std::str::FromStr;
impl FromStr for Card {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2" => Ok(Self::Two),
            "3" => Ok(Self::Three),
            "4" => Ok(Self::Four),
            "5" => Ok(Self::Five),
            "6" => Ok(Self::Six),
            "7" => Ok(Self::Seven),
            "8" => Ok(Self::Eight),
            "9" => Ok(Self::Nine),
            "T" => Ok(Self::T),
            "J" => Ok(Self::J),
            "Q" => Ok(Self::Q),
            "K" => Ok(Self::K),
            "A" => Ok(Self::A),
            _ => panic!("{s} is not a card"),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Type {
    High,
    Pair,
    Two,
    Three,
    Boat,
    Four,
    Five,
}

impl Type {
    fn strength(mut cards: [Card; 5]) -> Self {
        cards.sort();
        if cards[0] == cards[4] {
            return Self::Five;
        }
        if cards[0] == cards[3] || cards[1] == cards[4] {
            return Self::Four;
        }
        if cards[0] == cards[2] && cards[3] == cards[4] {
            return Self::Boat;
        }
        if cards[0] == cards[1] && cards[2] == cards[4] {
            return Self::Boat;
        }
        if cards[0] == cards[2] || cards[1] == cards[3] || cards[2] == cards[4] {
            return Self::Three;
        }
        // ABBCC
        if cards[1] == cards[2] && cards[3] == cards[4] {
            return Self::Two;
        }
        // BBACC
        if cards[0] == cards[1] && cards[3] == cards[4] {
            return Self::Two;
        }
        // BBCCA
        if cards[0] == cards[1] && cards[2] == cards[3] {
            return Self::Two;
        }

        if cards[0] == cards[1]
            || cards[1] == cards[2]
            || cards[2] == cards[3]
            || cards[3] == cards[4]
        {
            return Self::Pair;
        }

        Self::High
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Hand {
    kind: Type,
    cards: [Card; 5],
    bid: Number,
}

impl FromStr for Hand {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((cards, bid)) = s.split_once(' ') else {
            return Err("No space in line");
        };
        let Ok(bid): Result<Number, _> = bid.parse() else {
            return Err("Bid was not an integer");
        };
        let Ok(cards): Result<[Card; 5], _> = cards
            .matches(|_c| true)
            .filter_map(|s| str::parse::<Card>(s).ok())
            .collect::<Vec<Card>>()
            .try_into()
        else {
            panic!("Wrong number of cards in {cards}?");
        };
        let kind = Type::strength(cards);
        Ok(Self { kind, cards, bid })
    }
}

pub fn a() {
    let ctxt = readfile("07");
    let mut hands: Vec<Hand> = ctxt
        .lines()
        .filter_map(|s| str::parse::<Hand>(s).ok())
        .collect();
    hands.sort();
    let mut winnings = 0;
    for (n, hand) in hands.into_iter().enumerate() {
        let rank: Number = 1 + (n as Number);
        winnings += hand.bid * rank;
    }
    println!("Total winnings from these hands are {winnings}");
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum NewCard {
    Wildcard,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    Q,
    K,
    A,
}

impl FromStr for NewCard {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2" => Ok(Self::Two),
            "3" => Ok(Self::Three),
            "4" => Ok(Self::Four),
            "5" => Ok(Self::Five),
            "6" => Ok(Self::Six),
            "7" => Ok(Self::Seven),
            "8" => Ok(Self::Eight),
            "9" => Ok(Self::Nine),
            "T" => Ok(Self::T),
            "J" => Ok(Self::Wildcard),
            "Q" => Ok(Self::Q),
            "K" => Ok(Self::K),
            "A" => Ok(Self::A),
            _ => panic!("{s} is not a card"),
        }
    }
}

impl Type {
    fn new_kind(cards: [NewCard; 5]) -> Self {
        let mut vec: Vec<NewCard> = cards.into_iter().collect();
        vec.sort();
        vec.retain(|&c| c != NewCard::Wildcard);
        let wildcards = 5 - vec.len();
        match wildcards {
            0 => Self::easy(&vec),
            1 => Self::hard(&vec),
            2 => {
                if vec[0] == vec[2] {
                    Self::Five
                } else if vec[0] == vec[1] || vec[1] == vec[2] {
                    Self::Four
                } else {
                    Self::Three
                }
            }
            3 => {
                if vec[0] == vec[1] {
                    Self::Five
                } else {
                    Self::Four
                }
            }
            4 => Self::Five,
            5 => Self::Five,
            _ => panic!("Impossible card ranking {cards:?}"),
        }
    }

    fn easy(cards: &[NewCard]) -> Self {
        if cards[0] == cards[4] {
            return Self::Five;
        }
        if cards[0] == cards[3] || cards[1] == cards[4] {
            return Self::Four;
        }
        if cards[0] == cards[2] && cards[3] == cards[4] {
            return Self::Boat;
        }
        if cards[0] == cards[1] && cards[2] == cards[4] {
            return Self::Boat;
        }
        if cards[0] == cards[2] || cards[1] == cards[3] || cards[2] == cards[4] {
            return Self::Three;
        }
        // ABBCC
        if cards[1] == cards[2] && cards[3] == cards[4] {
            return Self::Two;
        }
        // BBACC
        if cards[0] == cards[1] && cards[3] == cards[4] {
            return Self::Two;
        }
        // BBCCA
        if cards[0] == cards[1] && cards[2] == cards[3] {
            return Self::Two;
        }

        if cards[0] == cards[1]
            || cards[1] == cards[2]
            || cards[2] == cards[3]
            || cards[3] == cards[4]
        {
            return Self::Pair;
        }

        Self::High
    }

    fn hard(cards: &[NewCard]) -> Self {
        if cards[0] == cards[3] {
            return Self::Five;
        }
        if cards[0] == cards[2] || cards[1] == cards[3] {
            return Self::Four;
        }
        if cards[0] == cards[1] && cards[2] == cards[3] {
            return Self::Boat;
        }
        if cards[0] == cards[1] || cards[1] == cards[2] || cards[2] == cards[3] {
            return Self::Three;
        }

        Self::Pair
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct NewHand {
    kind: Type,
    cards: [NewCard; 5],
    bid: Number,
}

impl FromStr for NewHand {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((cards, bid)) = s.split_once(' ') else {
            return Err("No space in line");
        };
        let Ok(bid): Result<Number, _> = bid.parse() else {
            return Err("Bid was not an integer");
        };
        let Ok(cards): Result<[NewCard; 5], _> = cards
            .matches(|_c| true)
            .filter_map(|s| str::parse::<NewCard>(s).ok())
            .collect::<Vec<NewCard>>()
            .try_into()
        else {
            panic!("Wrong number of cards in {cards}?");
        };
        let kind = Type::new_kind(cards);
        Ok(Self { kind, cards, bid })
    }
}

pub fn b() {
    let ctxt = readfile("07");
    let mut hands: Vec<NewHand> = ctxt
        .lines()
        .filter_map(|s| str::parse::<NewHand>(s).ok())
        .collect();
    hands.sort();
    let mut winnings = 0;
    for (n, hand) in hands.into_iter().enumerate() {
        let rank: Number = 1 + (n as Number);
        winnings += hand.bid * rank;
    }
    println!("Total winnings from these hands with Jacks wild are {winnings}");
}

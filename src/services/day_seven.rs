use std::fs;
use std::collections::{HashMap};
use std::ptr::null;

const FILE_PATH: &str = "src/services/inputs/day_seven__input.txt";

const CARDS: [char;13] = [
    'A', 'K', 'Q',
    'T', '9', '8', '7', 
    '6', '5', '4', '3', 
    '2', 'J',
];

#[derive(Debug)]
enum HandPower {
    FiveKind, // if has joker, all cards are jokers; doesn't get any better (1)
    FourKind, // if has joker, can become FiveKind (2)
    FullHouse, // if has joker, can become FiveKind (2)
    ThreeKind, // if has joker, can become FourKind (3)
    TwoPair, // if has 2 jokers, can become FourKind; if has 1 joker, can become FullHouse (3)
    OnePair, // if has joker, can become ThreeKind (4)
    HighCard, // if has joker, can become OnePair (5)
}

impl HandPower {
    fn value(&self) -> i32 {
        match *self {
            HandPower::FiveKind => 6,
            HandPower::FourKind => 5,
            HandPower::FullHouse => 4,
            HandPower::ThreeKind => 3,
            HandPower::TwoPair => 2,
            HandPower::OnePair => 1,
            HandPower::HighCard => 0,
        }
    }
}

#[derive(Debug)]

struct Hand {
    cards: String,
    power: HandPower,
    bid: i32,
}

impl Hand {
    fn get_differing_char_index(&self, other: &Self) -> i32 {
        let self_chars: Vec<char> = self.cards.chars().collect();
        let other_chars: Vec<char> = other.cards.chars().collect();
        
        for (i, c) in self_chars.iter().enumerate() {
            if self_chars[i] != other_chars[i] {
                return i as i32;
            }
        }

        -1
    }

    fn joker_count(&self) -> i32 {
        let mut ret = 0;

        for b in self.cards.as_bytes() {
            if *b == b'J' { ret += 1 } 
        }

        ret
    }

    fn get_card(&self, index: usize) -> char {
        self
            .cards
            .chars()
            .nth(index)
            .unwrap()
    }

    fn has_equal_power(&self, other: &Self) -> bool {
        self.power.value() == other.power.value()
    }

    fn is_stronger(&self, other: &Self) -> bool {
        self.power.value() > other.power.value()
    }
}

pub fn get_total_winnings() -> () {
    let input = fs::read_to_string(FILE_PATH).expect("SHOULDVE READ JUST FINE");
    let mut hands = parse_input(input);
    let mut card_power: HashMap<char, i32> = HashMap::new();

    for (i, card) in CARDS.iter().enumerate() {
        card_power.insert(*card, i as i32 * -1);
    }

    hands.sort_by(|a, b| {
        if a.has_equal_power(b) {
            let differing_index = a.get_differing_char_index(b);

            if differing_index == -1 {
                // happens if they are the EXACT SAME HAND
                return std::cmp::Ordering::Equal;
            } else {
                let a_val = card_power.get(&a.get_card(differing_index as usize)).unwrap();
                let b_val = card_power.get(&b.get_card(differing_index as usize)).unwrap();

                a_val.cmp(b_val)
            }
        } else {
            return if a.is_stronger(b) { 
                std::cmp::Ordering::Greater 
            } else {
                std::cmp::Ordering::Less
            }
        }
    });

    let mut total_winnings = 0;

    for (i, hand) in hands.iter().enumerate() {
        total_winnings += (i + 1) as i32 * hand.bid;
    }

    println!("TOTAL WINNINGS: {total_winnings}")
}

fn parse_input(input: String) -> Vec<Hand> {
    let parsed_input: Vec<Hand> = input
        .split("\n")
        .map(|l| {
            let tokens: Vec<&str> = l.split(" ").collect();
            let cards = tokens[0];

            Hand {
                cards: cards.to_string(),
                power: get_cards_power(cards),
                bid: tokens[1].parse::<i32>().unwrap(),
            }
        })
        .collect();
    
    parsed_input
}

fn joker_count(cards: &str) -> i32 {
    let mut ret = 0;

    for card_byte in cards.as_bytes() {
        if *card_byte == b'J' {
            ret += 1;
        }
    }

    ret
}

fn get_cards_power(cards: &str) -> HandPower {
    let mut cards_map: HashMap<char, usize> = HashMap::new();

    for c in cards.chars() {
        cards_map.insert(
            c, 
            1 + if cards_map.contains_key(&c) { cards_map[&c] } else { 0 },
        );
    }

    let num_cards = cards_map.keys().len();
    let hand_power = match_num_cards(num_cards, cards_map);
    let joker_count = joker_count(cards);

    if joker_count != 0 {
        return match_num_cards_with_joker(hand_power, joker_count);
    }

    hand_power
}

fn match_num_cards_with_joker(hand_power: HandPower, joker_count: i32) -> HandPower {
    match hand_power {
        HandPower::FiveKind => HandPower::FiveKind,
        HandPower::FourKind => HandPower::FiveKind,
        HandPower::FullHouse => HandPower::FiveKind,
        HandPower::ThreeKind => HandPower::FourKind,
        HandPower::TwoPair => {
            if joker_count == 2 { return HandPower::FourKind; } else { return HandPower::FullHouse; }
        },
        HandPower::OnePair => HandPower::ThreeKind,
        HandPower::HighCard => HandPower::OnePair,
    }
}

fn match_num_cards(num_cards: usize, map: HashMap<char, usize>) -> HandPower {
    match num_cards {
        1 => HandPower::FiveKind,
        4 => HandPower::OnePair,
        5 => HandPower::HighCard,
        2 => {
            let first_val = map.values().next().unwrap();
            
            if *first_val == 2 || *first_val == 3 {
                return HandPower::FullHouse;
            }

            HandPower::FourKind
        },
        3 => {
            let vals: Vec<&usize> = map.values().collect();
            if vals.contains(&(&3)) {
                return HandPower::ThreeKind;
            }

            HandPower::TwoPair
        },
        _ => panic!("BRUHHHH"),
    }
}


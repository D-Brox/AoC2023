use std::collections::HashMap;

use itertools::Itertools;
use pest::Parser;
use pest_derive::Parser as DeriveParser;

#[derive(DeriveParser)]
#[grammar = "days/day7.pest"]
pub struct BetParser;

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Debug)]
enum JokerCard {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => unreachable!(),
        }
    }
}

impl From<char> for JokerCard {
    fn from(value: char) -> Self {
        match value {
            'J' => JokerCard::Joker,
            '2' => JokerCard::Two,
            '3' => JokerCard::Three,
            '4' => JokerCard::Four,
            '5' => JokerCard::Five,
            '6' => JokerCard::Six,
            '7' => JokerCard::Seven,
            '8' => JokerCard::Eight,
            '9' => JokerCard::Nine,
            'T' => JokerCard::Ten,
            'Q' => JokerCard::Queen,
            'K' => JokerCard::King,
            'A' => JokerCard::Ace,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
enum Hand {
    High,
    Pair,
    DoublePair,
    Three,
    Full,
    Four,
    Five,
}

fn parse(input: String, joker: bool) -> (Hand, Vec<char>, u64) {
    let mut pairs = BetParser::parse(Rule::bet, &input)
        .unwrap()
        .next()
        .unwrap()
        .into_inner();
    let hand = pairs.next().unwrap().as_str().chars().collect_vec();
    let bet: u64 = pairs.next().unwrap().as_str().parse().unwrap();
    let cardcount: HashMap<char, u32> = hand
        .iter()
        .into_group_map_by(|&&x| x)
        .into_iter()
        .map(|(k, v)| (k, v.len() as u32))
        .collect();

    let mut double = 0;
    let mut triple = false;
    let mut quadra = false;
    let mut penta = false;

    for (c, n) in cardcount.clone() {
        if joker && c == 'J' {
            continue;
        }
        match n {
            1 => (),
            2 => double += 1,
            3 => triple = true,
            4 => quadra = true,
            5 => penta = true,
            _ => unreachable!(),
        }
    }

    let jokers = if joker { cardcount.get(&'J').unwrap_or(&0).to_owned() } else { 0 };

    if penta
        || (quadra && jokers == 1)
        || (triple && jokers == 2)
        || (double == 1 && jokers == 3)
        || jokers >= 4
    {
        (Hand::Five, hand, bet)
    } else if quadra || (triple && jokers == 1) || (double == 1 && jokers == 2) || jokers == 3 {
        (Hand::Four, hand, bet)
    } else if (triple && double == 1) || (double == 2 && jokers == 1) {
        (Hand::Full, hand, bet)
    } else if triple || (double == 1 && jokers == 1) || jokers == 2 {
        (Hand::Three, hand, bet)
    } else if double == 2 {
        (Hand::DoublePair, hand, bet)
    } else if double == 1 || jokers == 1 {
        (Hand::Pair, hand, bet)
    } else {
        (Hand::High, hand, bet)
    }
}

pub fn solution1(input: Vec<String>) -> u64 {
    let mut bets = Vec::new();

    for line in input {
        bets.push(parse(line, false));
    }

    bets.sort_by_key(|(h, c, _)| {
        (
            h.clone(),
            c.iter().map(|&c| c.into()).collect::<Vec<Card>>(),
        )
    });

    bets.iter()
        .enumerate()
        .map(|(i, (_, _, b))| ((i + 1) as u64) * b)
        .sum()
}

pub fn solution2(input: Vec<String>) -> u64 {
    let mut bets = Vec::new();

    for line in input {
        bets.push(parse(line, true));
    }

    bets.sort_by_key(|(h, c, _)| {
        (
            h.clone(),
            c.iter().map(|&c| c.into()).collect::<Vec<JokerCard>>(),
        )
    });

    bets.iter()
        .enumerate()
        .map(|(i, (_, _, b))| ((i + 1) as u64) * b)
        .sum()
}

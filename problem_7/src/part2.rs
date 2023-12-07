#![allow(unused_imports)]  // TODO: remove this after it's complete
use itertools::Itertools;
use std::collections::HashSet;
use std::fmt::Display;
use std::fs;
use std::iter;
use std::str::FromStr;
use counter::Counter;


/// value: J(joker)=1, 2-9 = 2-9;  T=10, Q=12, K=13, A=14
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Card {
    value: u8
}
impl Card {
    fn new(value: u8) -> Card {
        assert!(1 <= value && value <= 14 && value != 11);
        Card {value}
    }
}

// here it's very handy that the derive() for the Ord is exactly what we want
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct CardList {
    cards: Vec<Card>
}
impl CardList {
    fn new(cards: Vec<Card>) -> Self {
        assert_eq!(cards.len(), 5);
        Self { cards }
    }

    fn categorize(&self) -> HandCategory {
        let normal_cards = (2..=10).chain(12..=14).map(Card::new).collect_vec();
        let joker_indices = self.cards
            .iter()
            .enumerate()
            .filter_map(|(i, card)| (card == &Card::new(1)).then_some(i)).collect_vec();
        if joker_indices.len() == 0 { return self.categorize_simple(); }
        let card_combos_to_try = (0..joker_indices.len())
            .map(|ji_i| {
                let ji = joker_indices[ji_i];
                normal_cards.iter().map(move |c| (ji, *c))
            })
            .multi_cartesian_product();
        card_combos_to_try.map(|subs| {
            // substitute subs
            let mut cards_cp = self.cards.clone();
            for (i, sub) in subs.iter() {
                cards_cp[*i] = *sub;
            }
            // determine key
            let categ = Self::new(cards_cp).categorize_simple();
            categ  // only care about category as the key (rest is the same no matter how the J is sub'd)
        }).max().expect("Expected substitutable Jokers (should've checked for simple previously)")
    }

    fn categorize_simple(&self) -> HandCategory {
        let cards_set: Counter<_> = self.cards.iter().collect();
        assert!(!cards_set.contains_key(&Card::new(1)));  // assert no jokers in categorize_simple
        let most_common = cards_set.most_common();
        match most_common.as_slice() {
            [(_, 5), ..] => HandCategory::FiveOfKind,
            [(_, 4), ..] => HandCategory::FourOfKind,
            [(_, 3), (_, 2)] => HandCategory::FullHouse,
            [(_, 3), ..] => HandCategory::ThreeOfKind,
            [(_, 2), (_, 2), ..] => HandCategory::TwoPair,
            [(_, 2), ..] => HandCategory::OnePair,
            [..] => HandCategory::HighCard,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum HandCategory {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    category: HandCategory,
    cards: CardList,
}
impl Hand {
    fn from_cards(cards: CardList) -> Self {
        let category = cards.categorize();
        Self { category, cards }
    }
}


fn parse_card(c: char) -> Card {
    Card::new(match c {
        'T' => 10,
        'J' => 1,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        num if ('2' <= num && num <= '9') => num.to_digit(10).unwrap() as u8,
        _ => panic!("Bad card char")
    })
}
fn parse_hand(s: &str) -> Hand {
    Hand::from_cards(CardList::new(s.trim().chars().map(|c| parse_card(c)).collect_vec()))
}

pub fn part2() {
    let contents = fs::read_to_string("./src/input.txt").expect("Should've been able to read the file");
    let lines_s = contents.lines().map(|x| x.trim()).filter(|x| x.len() > 0);
    let lines_v = lines_s.map(|ln| {
        if let Some((hand_s, bid_s)) = ln.split_whitespace().collect_tuple() {
            let hand = parse_hand(hand_s);
            let bid: u32 = bid_s.parse().expect("Expected number as bid");
            (hand, bid)
        } else {
            panic!("line format should be '<cards> <number>'")
        }
    });
    let lines_with_rank =  lines_v
        .sorted()
        .enumerate()
        .map(|(rank_0based, v)| (rank_0based + 1, v));
    let ranks_and_bids = lines_with_rank.map(|(rank, (_hand, bid))| (rank, bid));
    let products = ranks_and_bids.map(|(rank, bid)| rank * bid as usize);
    let s: usize = products.sum();
    println!("Part 2: {}", s);
}

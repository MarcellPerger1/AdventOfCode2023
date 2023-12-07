use itertools::Itertools;
use std::fs;
use counter::Counter;

mod part2;

fn main() {
    part1();
    part2::part2();
}


/// value: 2-9 = 2-9;  T=10, J=11, Q=12, K=13, A=14
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Card {
    value: u8
}
impl Card {
    fn new(value: u8) -> Card {
        assert!(2 <= value && value <= 14);
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
        let cards_set: Counter<_> = self.cards.iter().collect();
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
        Self { category: cards.categorize(), cards }
    }
}


fn parse_card(c: char) -> Card {
    Card::new(match c {
        'T' => 10,
        'J' => 11,
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

fn part1() {
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
    println!("Part 1: {}", s);
}

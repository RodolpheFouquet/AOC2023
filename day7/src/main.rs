use std::cmp::Ordering;
use std::collections::HashMap;


const TEST : &str =  r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

#[derive(Eq, Hash, PartialEq, Debug, Clone)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    C9,
    C8,
    C7,
    C6,
    C5,
    C4,
    C3,
    C2
}

const CARD_ORDER : &'static [Card] = &[
    Card::A, 
    Card::K, 
    Card::Q, 
    Card::J, 
    Card::T, 
    Card::C9, 
    Card::C8,
    Card::C7,
    Card::C6,
    Card::C5,
    Card::C4,
    Card::C3,
    Card::C2
];

const CARD_ORDER_2 : &'static [Card] = &[
    Card::A, 
    Card::K, 
    Card::Q, 
    Card::T, 
    Card::C9, 
    Card::C8,
    Card::C7,
    Card::C6,
    Card::C5,
    Card::C4,
    Card::C3,
    Card::C2,
    Card::J 
];

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let idx_self = CARD_ORDER_2.iter().position(|c| c == self).unwrap();
        //let idx_self = CARD_ORDER.iter().position(|c| c == self).unwrap();
        let idx_other = CARD_ORDER_2.iter().position(|c| c == other).unwrap();
        //let idx_other = CARD_ORDER.iter().position(|c| c == other).unwrap();

        Some(idx_other.cmp(&idx_self))
    }
}

impl From<char> for Card {
    fn from(c: char) -> Self { 
        match c {
            'A' => Card::A,
            'K' => Card::K,
            'Q' => Card::Q,
            'J' => Card::J,
            'T' => Card::T,
            '9' => Card::C9,
            '8' => Card::C8,
            '7' => Card::C7,
            '6' => Card::C6,
            '5' => Card::C5,
            '4' => Card::C4,
            '3' => Card::C3,
            '2' => Card::C2,
            _ => unreachable!()
        }
    }
}

#[derive(Debug, PartialEq)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    High = 1

}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let s = self.typ() as u8;
        let o = other.typ() as u8;
        let pairs : Vec<_> = self.cards.iter().zip(other.cards.iter()).collect(); 
        match s.partial_cmp(&o) {
            Some(e) if e == Ordering::Equal => {
                let pos = pairs.iter().position(|(a, b)| a.partial_cmp(b).unwrap() != Ordering::Equal);
                match pos {
                    Some(p) => {
                        let (card_a, card_b) = pairs[p];
                        card_a.partial_cmp(&card_b)
                    },
                    None => Some(Ordering::Equal)
                }

            },
            Some(e) => Some(e),
            None => None

        }
    }
}
impl Hand {
    fn typ(&self) -> HandType {
        let mut map : HashMap<Card, usize> = HashMap::new();

        for c in &self.cards {
            
            map
            .entry(c.clone())
            .and_modify(|count| *count += 1)
            .or_insert(1);
        }
        let count_joker = map.get(&Card::J).unwrap_or(&0).clone();
        
        if count_joker != 0 && count_joker != self.cards.len() {
            let (mut max_card, mut max_card_count) = (Card::J, 0);
            map.clone().into_iter().for_each(|(key, value)| {
                if value > max_card_count && key != Card::J {
                    (max_card, max_card_count) = (key, value);
                }
            });
            map.entry(max_card).and_modify(|c| *c += count_joker);
            map.entry(Card::J).and_modify(|c| *c = 0);

        }
        let mut counts = map.values().cloned().collect::<Vec<usize>>();
        counts.sort();

        if *counts.last().unwrap() == 5 {
            HandType::FiveOfAKind
        } else if *counts.last().unwrap() == 4{
            HandType::FourOfAKind
        } else if counts[counts.len()-1] == 3 && counts[counts.len()-2] == 2 {
            HandType::FullHouse
        } else if *counts.last().unwrap() == 3 {
            HandType::ThreeOfAKind
        } else if counts[counts.len()-1] == 2 && counts[counts.len()-2] == 2 {
            HandType::TwoPair
        } else if counts[counts.len()-1] == 2 {
            HandType::OnePair
        } else {
            HandType::High
        }
    }
} 

impl From<&str> for Hand {

    fn from(s: &str) -> Self { 
        Self {cards: s.chars().map(|c| c.into()).collect()}
    }
}

impl PartialOrd for Play {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.hand.partial_cmp(&other.hand)
    }
}

impl Ord for Play {
    fn cmp(&self, other: &Self) -> Ordering {
       self.partial_cmp(&other).unwrap() 
    }
}


#[derive(Debug, PartialEq, Eq)]
struct Play {
    hand: Hand,
    bid: usize
}

impl From<&str> for Play {

    fn from(s: &str) -> Self { 
        let comps = s.split_whitespace().collect::<Vec<&str>>();

        Self {
            bid: comps[1].parse::<usize>().unwrap(),
            hand: comps[0].into()
        }
    }
}


fn main() {
    let mut plays = include_str!("../input.txt").lines().map(|l| l.into()).collect::<Vec<Play>>();
    //let mut plays = TEST.lines().map(|l| l.into()).collect::<Vec<Play>>();
    plays.sort();

    let total = plays.len();
    //plays.iter().rev().enumerate().for_each(|(pos, p)| println!("{pos} {:?} {:?}", p.hand, p.hand.typ()));
    let score : usize = plays.iter().rev().enumerate().map(|(pos, p)| p.bid*(total-pos)).sum();

    println!("{score}");
}

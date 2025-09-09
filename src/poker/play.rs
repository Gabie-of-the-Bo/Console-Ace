use std::{cmp::Ordering, collections::{HashMap, HashSet}};

use lazy_static::lazy_static;

use crate::poker::card::Card;

#[derive(PartialEq, Eq, Debug)]
pub enum Play {
    Highest(Vec<usize>),
    Pair(usize, Vec<usize>),
    DoublePair(usize, usize, Vec<usize>),
    ThreeOfAKind(usize, Vec<usize>),
    Straight(usize),
    Flush(Vec<usize>),
    FullHouse(usize, usize), // Three of a kind, Pair
    FourOfAKind(usize, Vec<usize>),
    StraightFlush(usize),
    RoyalFlush
}

fn value_to_str(v: usize) -> String {
    match v {
        11 => "Jack".into(),
        12 => "Queen".into(),
        13 => "King".into(),
        14 => "Ace".into(),
        n => n.to_string(),
    }
}

impl Play {
    pub fn priority(&self) -> usize {
        match self {
            Play::Highest(..) => 0,
            Play::Pair(..) => 1,
            Play::DoublePair(..) => 2,
            Play::ThreeOfAKind(..) => 3,
            Play::Straight(..) => 4,
            Play::Flush(..) => 5,
            Play::FullHouse(..) => 6,
            Play::FourOfAKind(..) => 7,
            Play::StraightFlush(..) => 8,
            Play::RoyalFlush => 9,
        }
    }

    pub fn name(&self) -> String {
        match self {
            Play::Highest(k) => format!("High card {}", value_to_str(k[4])),
            Play::Pair(p, ..) => format!("Pair of {}s", value_to_str(*p)),
            Play::DoublePair(p1, p2, ..) => format!("Two Pairs: {}s and {}s", value_to_str(*p1), value_to_str(*p2)),
            Play::ThreeOfAKind(t, ..) => format!("Three {}s", value_to_str(*t)),
            Play::Straight(s) => format!("Straight to {}", value_to_str(*s)),
            Play::Flush(..) => format!("Flush"),
            Play::FullHouse(t, p) => format!("{}s full of {}s", value_to_str(*t), value_to_str(*p)),
            Play::FourOfAKind(f, ..) => format!("Four {}s", value_to_str(*f)),
            Play::StraightFlush(s, ..) => format!("Straight Flush to {}", value_to_str(*s)),
            Play::RoyalFlush => format!("Royal Flush"),
        }
    }
}

impl PartialOrd for Play {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Play::*;

        match self.priority().cmp(&other.priority()) {
            Ordering::Equal => {
                match (self, other) {
                    (Highest(k1), Highest(k2)) => Some(compare_kickers(k1, k2)),
                    (Pair(p1, k1), Pair(p2, k2)) => Some(p1.cmp(p2).then_with(|| compare_kickers(k1, k2))),
                    (DoublePair(p11, p12, k1), DoublePair(p21, p22, k2)) => Some(
                        p11.cmp(p21)
                        .then_with(|| p12.cmp(p22))
                        .then_with(|| compare_kickers(k1, k2))
                    ),
                    (ThreeOfAKind(t1, k1), ThreeOfAKind(t2, k2)) => Some(t1.cmp(t2).then_with(|| compare_kickers(k1, k2))),
                    (Straight(h1), Straight(h2)) => Some(h1.cmp(h2)),
                    (Flush(k1), Flush(k2)) => Some(compare_kickers(k1, k2)),
                    (FullHouse(t1, p1), FullHouse(t2, p2)) => Some(t1.cmp(t2).then_with(|| p1.cmp(p2))),
                    (FourOfAKind(f1, k1), FourOfAKind(f2, k2)) => Some(f1.cmp(f2).then_with(|| compare_kickers(k1, k2))),
                    (StraightFlush(h1), StraightFlush(h2)) => Some(h1.cmp(h2)),
                    (RoyalFlush, RoyalFlush) => Some(Ordering::Equal),

                    _ => unreachable!()
                }    
            },

            c => Some(c)
        }
    }
}

impl Ord for Play {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn compare_kickers(a: &Vec<usize>, b: &Vec<usize>) -> Ordering {
    for (i, j) in a.iter().zip(b).rev() {
        match i.cmp(j) {
            Ordering::Equal => { },
            c => { return c; },
        }
    }

    Ordering::Equal
}

lazy_static! {
    static ref STRAIGHTS: Vec<Vec<usize>> = valid_straights();
}

fn valid_straights() -> Vec<Vec<usize>> {
    let mut straights = (2..=10).rev()
        .map(|i| (i..i + 5).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    straights.push(vec!(14, 2, 3, 4, 5)); // Ace-low straight
    
    straights
}

pub fn analyze_play(hand: &[Card], community: &[Card]) -> Play {
    let mut all = hand.iter().chain(community).collect::<Vec<_>>();
    all.sort_by_key(|c| c.value());

    // Create number map
    let mut numbers = HashMap::<_, Vec<_>>::new();

    for c in all.iter() {
        numbers.entry(c.value()).or_default().push(*c);
    }

    // RoyalFlush / StraightFlush / Straight
    let mut straight_found = None;

    for ns in STRAIGHTS.iter() {
        let has_all = ns.iter().all(|i| numbers.contains_key(&i));

        if has_all {
            let suits = ns.iter()
                .map(|i| numbers.get(&i).unwrap())
                .map(|s| s.iter().map(|c| c.suit.clone()).collect::<HashSet<_>>())
                .reduce(|a, b| a.intersection(&b).cloned().collect())
                .unwrap();

            if suits.is_empty() {
                if straight_found.is_none() {
                    straight_found = Some(ns[4]);
                }

            } else if ns[4] == 14 {
                return Play::RoyalFlush;
            
            } else {
                return Play::StraightFlush(ns[4]);
            }
        }
    }

    // Four of a kind
    let four = numbers.iter()
        .filter(|(_, cs)|cs.len() == 4)
        .max_by_key(|(i, _)| *i);

    if let Some((v, _)) = four {
        let kickers = all.iter()
            .map(|c| c.value())
            .filter(|c| c != v)
            .rev()
            .take(1)
            .collect::<Vec<_>>();

        return Play::FourOfAKind(*v, kickers);
    }    

    // Flush (used later)
    let mut suits = HashMap::<_, Vec<_>>::new();

    for c in all.iter() {
        suits.entry(c.suit.clone()).or_default().push(*c);
    }

    let flush = suits.iter()
        .map(|(_, i)| i)
        .find(|&cs| cs.len() >= 5)
        .cloned()
        .map(|mut cs| {
            cs.sort_by_key(|i| i.value());
            cs
        });

    // FullHouse / ThreeOfAKind
    let three = numbers.iter()
        .filter(|(_, cs)|cs.len() == 3)
        .max_by_key(|(i, _)| *i);

    if let Some((t, _)) = three {
        let p = numbers.iter()
            .filter(|(_, cs)| cs.len() >= 2)
            .filter(|&(i, _)| i != t)
            .map(|i| *i.0)
            .max();

        if let Some(i) = p {
            return Play::FullHouse(*t, i);
        }

        if let Some(f) = flush {
            return Play::Flush(f[f.len() - 5..].iter().map(|c| c.value()).collect());
        }

        let mut kickers = all.iter()
            .map(|c| c.value())
            .filter(|c| c != t)
            .rev()
            .take(2)
            .collect::<Vec<_>>();

        kickers.reverse();

        return Play::ThreeOfAKind(*t, kickers);
    }

    if let Some(f) = flush {
        return Play::Flush(f[f.len() - 5..].iter().map(|c| c.value()).collect());
    }

    if let Some(s) = straight_found {
        return Play::Straight(s);
    }

    // DoublePair / Pair
    let mut pairs = numbers.iter()
        .filter(|(_, cs)| cs.len() == 2)
        .map(|i| *i.0)
        .collect::<Vec<_>>();

    pairs.sort();

    if pairs.len() >= 2 {
        let num_pairs = pairs.len();

        let kickers = all.iter()
            .map(|c| c.value())
            .filter(|c| *c != pairs[num_pairs - 1])
            .filter(|c| *c != pairs[num_pairs - 2])
            .rev()
            .take(1)
            .collect::<Vec<_>>();

        return Play::DoublePair(pairs[num_pairs - 1], pairs[num_pairs - 2], kickers);
    }

    if pairs.len() == 1 {
        let mut kickers = all.iter()
            .map(|c| c.value())
            .filter(|c| *c != pairs[0])
            .rev()
            .take(3)
            .collect::<Vec<_>>();

        kickers.reverse();

        return Play::Pair(pairs[0], kickers);
    }

    // Return highest card
    Play::Highest(all[all.len() - 5..].iter().map(|c| c.value()).collect())
}